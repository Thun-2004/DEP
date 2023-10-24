use bson::spec::BinarySubtype;
use clap::{App, Arg};
use rmp::decode::{RmpRead, read_map_len, read_str};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;
use serde_json::{Value, Number, json};
use bson::{Document , Bson, doc}; 
use bson::Binary; 
use std::fs;
use std::fs::File;
use std::io::{self, Write, Read, BufWriter, BufReader, BufRead};
use rmp::{encode::ValueWriteError, decode}; 
use rmp_serde::{from_read, Deserializer};
use rmp::Marker; 
use std::io::Cursor;
use rmp::encode::{write_nil, write_bool, write_f32, write_f64, write_u32, write_i32, write_i64, write_str, write_array_len, write_map_len, write_bin};

// use rmp::Value as RmpValue; 

type MyResult<T> = Result<T, Box<dyn Error>>;//input format

#[derive(Debug)]
pub struct Config{
    filename : String, 
    filetype : String, 
    desired_type: String
}

//run : cargo run -- --help
pub fn get_args() -> MyResult<Config>{
    //help add matches
    let matches = App::new("DEF")
        .version("0.1.0")
        .author("Thunyaphon")
        .about("Data exchage format")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Sets the input file to use")
                .takes_value(true)
                //multiple(true)
                .required(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("json")
                .short("json")
                .long("json")
                .value_name("JSON")
                .help("Convert data to JSON")
                .takes_value(false)
                .required(false)
        )
        .arg(
            Arg::with_name("bson")
                .short("bson")
                .long("bson")
                .value_name("BSON")
                .help("Convert data to BSON")
                .takes_value(false)
                .required(false)
        )
        .arg(
            Arg::with_name("msgpack")
                .short("msgp")
                .long("msgpack")
                .value_name("MSGP")
                .help("Convert data to MessagePack")
                .takes_value(false)
                .required(false)
        )
        .arg(
            Arg::with_name("B64")
                .short("x64")
                .long("base64")
                .value_name("B64")
                .help("Convert raw binary data to Base64")
                .takes_value(false)
                .required(false)
        )
        
        .get_matches();

    let info = matches.value_of("files").map(|s| s.split(".").collect::<Vec<&str>>()).unwrap();
    let filename = info[0].to_string(); 
    let filetype = info[1].to_string();
    let desired_type = if matches.is_present("json"){
        "json".to_string()
    } else if matches.is_present("bson"){
        "bson".to_string()
    } else if matches.is_present("msgpack"){
        "msgpack".to_string()
    } else if matches.is_present("B64"){
        "B64".to_string()
    } else {
        "invalid".to_string()
    };
    
    Ok(Config {
        filename,
        filetype,
        desired_type
    })
        
}

//help rewrite to make it cleaner when call run function
//process data from Struct Config and save to file 
pub fn run(config: Config) -> MyResult<()>{
    println!("{:?}", config);
    let input_file = format!("{}.{}", config.filename, config.filetype);
    let output_file = format!("{}.{}", config.filename, config.desired_type);

    match config.desired_type.as_str(){
        "json" => {
            match config.filetype.as_str(){
                "bson" => {
                    let content = read_bson_file(&input_file)?;
                    let json_content = bson_to_json(&content); 
                    let _ = write_json_file(&output_file, json_content);
                },
                "msgpack" => {
                    let content = read_msgp_file(&input_file).unwrap();
                    let json_content: Value = msgpack_to_json(&content).unwrap();
                    let _ = write_json_file(&output_file, json_content);
                },
                _ => {
                    println!("Invalid type");
                }
            }
        },
        "bson" => {
            match config.filetype.as_str(){
                "json" => {
                    let content = read_json_file(&input_file)?;
                    let doc = json_to_bson(&content); 
                    println!("{:?}", &doc);
                    let _ = write_bson_file(&output_file, doc);
                },
                "msgpack" => {
                    let content = read_msgp_file(&input_file).unwrap();
                    let json_content: Value = msgpack_to_json(&content).unwrap();
                    let doc = json_to_bson(&json_content.to_string()); 
                    let _ = write_bson_file(&output_file, doc);
                },
                _ => {
                    println!("Invalid type");
                }
            }
        },
        "msgpack" => {
            match config.filetype.as_str(){
                "json" => {
                    let content = read_json_file(&input_file)?;
                    let doc = json_to_msgpack(content); 
                    let _ = write_msgp_file(&output_file, &doc);
                },
                "bson" => {
                    //help not sure
                    let content = read_bson_file(&input_file)?;
                    let doc = bson_to_msgpack(&content).unwrap(); 
                    let msgpack_content = json_to_msgpack(doc.into()); 
                    let _ = write_msgp_file(&output_file, &msgpack_content);
                },
                _ => {
                    println!("Invalid type");
                }
            }
        }
        "B64" => {
            let content = read_raw_binary(&input_file)?;
            let base64_content = bin_to_base64(content)?;
            let _ = write_base64(&output_file, base64_content);
        }
        _ => {
            println!("Invalid type");
        }
    }
    Ok(())
}

//should be converted to Value not String 
pub fn read_json_file(filepath: &str) -> MyResult<Value>{
    let content = fs::read_to_string(filepath).expect("Unable to read the file");
    let content  = serde_json::from_str(&content)?;
    Ok(content)
}

pub fn write_bson_file(filepath: &str, content: Document) -> MyResult<()>{
    let mut file = File::create(filepath)?; 
    let bytes: Vec<u8> = bson::to_vec(&content).unwrap();
    file.write_all(&bytes).expect("Write failed");
    Ok(())
}

pub fn read_bson_file(filepath: &str) -> MyResult<Document>{
    let file = File::open(filepath).expect("Unable to read the file");
    let mut v: Vec<u8> = Vec::new(); 
    file.bytes().for_each(|b| v.push(b.unwrap()));
    let mut reader: Cursor<Vec<u8>> = Cursor::new(v.clone());
    let doc = bson::Document::from_reader(&mut reader)?;
    Ok(doc)
}

pub fn write_json_file(filepath: &str, content: Value) -> MyResult<()>{
    let mut file = File::create(filepath)?;
    let mut writer = BufWriter::new(file); 
    serde_json::to_writer(&mut writer, &content)?;
    writer.flush().expect("Flush failed");
    Ok(())
}

pub fn read_msgp_file(filepath: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let file_path = Path::new(filepath);
    let mut hex_values = Vec::new();

    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let hex_strings: Vec<&str> = line.split_whitespace().collect();
                for hex_str in hex_strings {
                    if let Ok(parsed_byte) = u8::from_str_radix(hex_str, 16) {
                        hex_values.push(parsed_byte);
                    } else {
                        eprintln!("Failed to parse the hex value: {}", hex_str);
                    }
                }
            }
        }
    }
    Ok(hex_values)
}

pub fn write_msgp_file(filepath: &str, content: &[u8]) -> MyResult<()>{
    let mut file = File::create(filepath)?;
    file.write_all(content)?;
    Ok(())
}

pub fn read_raw_binary(filepath: &str) -> MyResult<Vec<u8>>{
    let mut file = File::open(filepath)?;
    let mut data = Vec::new(); 
    file.read_to_end(&mut data)?;
    Ok(data)
}

pub fn write_base64(filepath: &str, content: String) -> MyResult<()>{
    let mut file = File::create(filepath)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

//JSON to BSON : cargo run -- "input.json" --bson
pub fn json_to_bson(str_content: &str) -> Document{
    let json_content: Value = serde_json::from_str(str_content).expect("Failed to parse to JSON");
    match json_content {
        Value::Object(map) => {
            let mut doc = Document::new();
            for (key, value) in map{
                doc.insert(key, str_to_json(value));
            }
            doc
        }
        _ => panic!("Top level JSON must be an object")
    }
}


fn str_to_json(value: Value) -> bson::Bson{
    match value {
        serde_json::Value::Null => bson::Bson::Null,
        serde_json::Value::Bool(b) => bson::Bson::Boolean(b),
        serde_json::Value::Number(n) => {
            if n.is_f64(){
                bson::Bson::Double(n.as_f64().unwrap())
            }else if n.is_i64(){
                bson::Bson::Int64(n.as_i64().unwrap())
            }
            else {
                bson::Bson::Int64(n.as_i64().unwrap())
            }
        }
        serde_json::Value::String(s) => bson::Bson::String(s), 
        serde_json::Value::Array(vec) => {
            let bson_array: Vec<bson::Bson> = vec.into_iter().map(str_to_json).collect();
            bson::Bson::Array(bson_array)
        }
        serde_json::Value::Object(map) => {
            let mut doc = Document::new();
            for (key, value2) in map{
                doc.insert(key, str_to_json(value2));
            }
            bson::Bson::Document(doc)
        }
    }
}

#[test]
fn test_json_to_bson() {
    let json_str = r#"{
        "name": "John",
        "age": 27
    }"#;

    let result_doc = json_to_bson(json_str);
    let expected_doc = doc! {
        "name": "John",
        "age": 27 as i64
    };
    assert_eq!(result_doc, expected_doc);
}

pub fn bson_to_json(doc: &Document) -> serde_json::Value{
    let mut json_map = serde_json::Map::new();
    for (key, value) in doc.iter(){
        let json_value = match value{
            Bson::Double(d) => serde_json::Value::Number(serde_json::Number::from_f64(*d).unwrap()), 
            Bson::String(s) => serde_json::Value::String(s.clone()), 
            Bson::Array(arr) => {
                let doc_arr = arr.iter().map(|b| match b{
                    Bson::Document(document) => Some(document), 
                    _ => None, 
                }).collect::<Vec<Option<&Document>>>();
                let json_array: Vec<serde_json::Value> = doc_arr.iter().map(|elem| bson_to_json(elem.unwrap())).collect(); 
                serde_json::Value::Array(json_array)
            }

            Bson::Document(doc) => bson_to_json(doc), 
            Bson::Null => serde_json::Value::Null,
            Bson::Boolean(b) => serde_json::Value::Bool(*b),
            Bson::RegularExpression(regex) => {
                let mut json_map = serde_json::Map::new(); 
                json_map.insert(
                    "$regex".to_string(), 
                    serde_json::Value::String(regex.pattern.clone())
                ); 
                json_map.insert(
                    "$options".to_string(),
                    serde_json::Value::String(regex.options.clone())
                );
                serde_json::Value::Object(json_map)
            },  
            Bson::JavaScriptCode(js) => serde_json::Value::String(js.clone()),  
            Bson::JavaScriptCodeWithScope(js_c) => {
                let mut json_map = serde_json::Map::new(); 
                json_map.insert(
                    "$code".to_string(), 
                    serde_json::Value::String(js_c.code.clone())
                ); 
                json_map.insert(
                    "$scope".to_string(),
                    bson_to_json(&js_c.scope)
                );
                serde_json::Value::Object(json_map)
            },  
            Bson::Int32(n) => serde_json::Value::Number(Number::from(*n)),
            Bson::Int64(n) => serde_json::Value::Number(Number::from(*n)),
            Bson::Timestamp(t) => {
                let mut json_map  = serde_json::Map::new();
                json_map.insert(
                    "$timestamp".to_string(), 
                    serde_json::Value::Number(Number::from(t.time))
                );
                json_map.insert(
                    "$increment".to_string(), 
                    serde_json::Value::Number(Number::from(t.increment))
                );
                serde_json::Value::Object(json_map)
            },
            Bson::Timestamp(t) => serde_json::Value::Null,
            Bson::Binary(b) => serde_json::Value::String(bin_to_base64(b.bytes.to_vec()).unwrap()), //needed
            Bson::ObjectId(objid) => Value::String(objid.to_hex()),
            Bson::DateTime(dt) => serde_json::Value::String(dt.try_to_rfc3339_string().unwrap()),
            Bson::Symbol(s) => serde_json::Value::String(s.clone()),
            Bson::Decimal128(d) => serde_json::Value::String(d.to_string()),
            Bson::Undefined => serde_json::Value::Null,
            Bson::MaxKey => serde_json::Value::Object(serde_json::from_str(r#"{"$maxKey":1}"#).unwrap()),
            Bson::MinKey => serde_json::Value::Object(serde_json::from_str(r#"{"$maxKey":1}"#).unwrap()),
            Bson::DbPointer(p) => serde_json::Value::Null
        };
        json_map.insert(key.to_string(), json_value); 
    }
    Value::Object(json_map)
}


// problem
pub fn json_to_msgpack(value: Value) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    match value{
        Value::Null => write_nil(&mut buf).unwrap(), 
        Value::Bool(b) => write_bool(&mut buf, b).unwrap(),
        Value::String(s) => write_str(&mut buf, &s).unwrap(),
        Value::Number(n) => {
            if let Some(n) = n.as_f64(){
                write_f64(&mut buf, n).unwrap();
            } else if let Some(n) = n.as_i64(){
                write_i64(&mut buf, n).unwrap();
            } else {
                write_f32(&mut buf, n.as_f64().unwrap() as f32).unwrap();
            }
        },
        Value::Array(arr) => {
            let len = arr.len() as u32; 
            write_array_len(&mut buf, len).unwrap();
            for elem in arr{
                json_to_msgpack(elem);
            }
        }, 

        Value::Object(map) => {
            let len = map.len() as u32; 
            write_map_len(&mut buf, len).unwrap(); 
            for (key, value) in map{
                write_str(&mut buf, &key).unwrap();
                json_to_msgpack(value);
            }
        }
    }
    // let new_buf: Vec<String> = buf.iter().map(|b| format!("0x{:02X}", b)).collect();
    // new_buf
    buf
}


pub fn msgpack_to_json(msgpack_data: &[u8]) -> Result<Value, Box<dyn Error>> {
    let mut reader = Cursor::new(msgpack_data);
    let mut de = Deserializer::new(reader);

    let key_value_pairs: BTreeMap<String, Value> = Deserialize::deserialize(&mut de)?;
    let mut json_map: serde_json::Map<String, Value> = serde_json::Map::new();
    
    for (key, value) in key_value_pairs.iter().rev() {
        json_map.insert(key.to_string(), value.clone());
    }
    let json_value = Value::Object(json_map);
    // let json_string = serde_json::to_string(&json_value).unwrap();
    Ok(json_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msgpack_to_json_string() {
        let input_msgpack: &[u8] = &[
            0x82, 0xA3, 0x6E, 0x61, 0x6D, 0x65, 0xA4, 0x4A, 0x6F, 0x68, 0x6E
        ];

        let expected_json = r#"{"name":"John"}"#;
        let result = msgpack_to_json(input_msgpack).unwrap();
        match result {
            json_string => {
                assert_eq!(json_string, expected_json);
            }
            _ => {
                panic!("Test failed. Failed to convert MessagePack to JSON.");
            }
        }
    }
}

//Bson to Msp
pub fn bson_to_msgpack(bson_doc: &Document) -> Result<Vec<u8>, Box<dyn std::error::Error>>{
    let mut buf = Vec::new(); 
    let elements = bson_doc.iter();
    write_map_len(&mut buf, elements.count() as u32)?;

    for(key, value) in elements{
        write_str(&mut buf, key)?;
        match value{
            Bson::Array(array) => {
                write_array_len(&mut buf, array.len() as u32)?;
                for elem in array.iter(){
                    bson_value_to_msgpack(elem, &mut buf)?;
                }
            }
            Bson::Document(sub_doc) => {
                bson_to_msgpack(sub_doc)?;
            }
            _ => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid data",
                )))
            }
        }
    }
    Ok(buf)
}

pub fn bson_value_to_msgpack(value: &Bson, buf: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>>{
// pub fn bson_value_to_msgpack(value: &Bson, buf: &mut Vec<u8>) -> Result<(), ValueWriteError>{
    match value{
        Bson::Double(d) => write_f64( buf, *d)?,
        Bson::String(s) => write_str(buf, s)?,
        Bson::Array(array) => {
            write_array_len(buf, array.len() as u32)?;
            for element in array.iter(){
                bson_value_to_msgpack(element, buf)?;
            }
        }
        Bson::Document(doc) => {
            bson_to_msgpack(doc);
        }
        Bson::Null => write_nil(buf)?,
        Bson::Boolean(b) => write_bool(buf, *b)?,
        Bson::RegularExpression(reg) => {
            write_str(buf, &reg.pattern)?;
            write_str(buf, &reg.options)?;
        },  
        Bson::JavaScriptCode(js) => write_str(buf, js).unwrap(), 
        Bson::JavaScriptCodeWithScope(js_c) => {
            write_str(buf, &js_c.code)?;
            bson_to_msgpack(&js_c.scope)?;
        },  
        Bson::Int32(n) => write_i32(buf, *n)?,
        Bson::Int64(n) => write_i64(buf, *n)?,
        Bson::Timestamp(t) => {
            write_u32(buf, t.time)?;
            write_u32(buf, t.increment)?;
        },
        Bson::Binary(b) => {
            let binary = b.bytes.clone();
            write_bin(buf, &binary).unwrap()
        }
        Bson::ObjectId(objid) => write_str(buf, &objid.to_hex())?,
        Bson::DateTime(dt) => write_str(buf, &dt.try_to_rfc3339_string().unwrap())?,
        Bson::Symbol(s) => write_str(buf, s)?,
        Bson::Decimal128(d) => write_str(buf, &d.to_string())?,
        Bson::Undefined => write_nil(buf)?,
        Bson::MaxKey => write_str(buf, "$maxKey")?,
        Bson::MinKey => write_str(buf, "$minKey")?,
        Bson::DbPointer(p) => write_nil(buf)?
    }
    Ok(())
}

//Msg to Bson add 
pub fn msgpack_to_bson(input: &[u8]) -> MyResult<Document, Box<dyn Error>>{

}

//encode raw binary data to Base64 
pub fn bin_to_base64(binary: Vec<u8>) -> MyResult<String>{
    for i in &binary{
        println!("{:08b}", i);
    }
    const BASE64_CHARS: [u8; 64] = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
        b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
        b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
        b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
        b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/'
    ];
    let mut result = String::with_capacity((binary.len() + 2 / 3) * 4);
    let mut buffer = 0;
    let mut buffer_len = 0; 

    for byte in binary{
        buffer = (buffer << 8) | (byte as u32);
        buffer_len += 8; 

        while buffer_len >= 6{
            let index = (buffer >> (buffer_len - 6)) & 0x3F;
            result.push(BASE64_CHARS[index as usize] as char);
            buffer_len -= 6;
        }
    }
    
    if buffer_len > 0{
        let index = (buffer << (6 - buffer_len)) & 0x3F; 
        result.push(BASE64_CHARS[index as usize] as char); 
    }

    while result.len() % 4 != 0{
        result.push('=');
    }
    println!("{:?}", result);
    Ok(result)
}

//Decode Base64 to raw binary data
pub fn base64_to_bin(base64: String) -> MyResult<Vec<u8>>{
    const BASE64_CHARS: [u8; 64] = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
        b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
        b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
        b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
        b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/'
    ];
    let mut result = Vec::with_capacity(base64.len() * 3/4);
    let mut buffer = 0; 
    let mut buffer_len = 0; 

    for byte in base64.bytes(){
        if byte == b'='{
            break;
        }
        let value = BASE64_CHARS.iter().position(|&c| c==byte).unwrap() as u32;
        buffer = (buffer << 6) | value; 
        buffer_len += 6; 

        if buffer_len >= 8{
            buffer_len -= 8; 
            let byte = (buffer >> buffer_len) & 0xFF;
            result.push(byte as u8);
        }
    }
    Ok(result)
}



















