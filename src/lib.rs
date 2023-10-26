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

type MyResult<T> = Result<T, Box<dyn Error>>;//input format

#[derive(Debug)]
pub struct Config{
    filename : String, 
    filetype : String, 
    desired_type: String
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

//addtion 

//run : cargo run -- --help
pub fn get_args() -> MyResult<Config>{
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
                .short("msgp") //cant use have to use long
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
        //might add binary 
        
        .get_matches();

    let info = matches.value_of("files").map(|s| s.split(".").collect::<Vec<&str>>()).unwrap_or(Vec::new());
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


pub fn run(config: Config) -> MyResult<()>{
    println!("{:?}", config);
    let input_file = format!("{}.{}", config.filename, config.filetype);
    let output_file = format!("output.{}", config.desired_type);

    match config.desired_type.as_str(){
        "json" => {
            match config.filetype.as_str(){
                "bson" => {
                    let content = read_bson_file(&input_file)?;
                    let json_content = bson_to_json(&content); 
                    let _ = write_json_file(&output_file, json_content.unwrap());
                },
                "msgpack" => {
                    let content = read_msgp_file(&input_file).unwrap_or(Vec::new());
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
                    let _ = write_bson_file(&output_file, doc.unwrap());
                },
                "msgpack" => {
                    let content = read_msgp_file(&input_file).unwrap_or(Vec::new());
                    let json_content: Value = msgpack_to_json(&content).unwrap();
                    let doc = json_to_bson(&json_content); 
                    let _ = write_bson_file(&output_file, doc.unwrap());
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
                    let doc = json_to_msgpack(&content)?; 
                    let _ = write_msgp_file(&output_file, &doc);
                },
                "bson" => {
                    //help not sure
                    let content = read_bson_file(&input_file)?;
                    let doc = bson_to_msgpack(&content).unwrap_or(Vec::new()); 
                    let msgpack_content = json_to_msgpack(&doc.into())?; 
                    let _ = write_msgp_file(&output_file, &msgpack_content);
                },
                _ => {
                    println!("Invalid type");
                }
            }
        }
        //only convert binary to base64
        "B64" => {
            let content = read_raw_binary(&input_file)?;
            let base64_content = bin_to_base64(content)?;
            let _ = write_base64(&output_file, base64_content);
        } //might add B64 to binary 
        _ => {
            println!("Invalid type");
        }
    }
    Ok(())
}

pub fn read_json_file(filepath: &str) -> MyResult<Value>{
    let content = fs::read_to_string(filepath).expect("Unable to read the file");
    let content  = serde_json::from_str(&content)?;
    Ok(content)
}

pub fn write_bson_file(filepath: &str, content: Document) -> MyResult<()>{
    let mut file = File::create(filepath)?; 
    let bytes: Vec<u8> = bson::to_vec(&content).unwrap_or(Vec::new());
    file.write_all(&bytes).expect("Write failed");
    Ok(())
}

pub fn write_bson_file2(filepath: &str, content: &Document) -> MyResult<()>{
    let mut file = File::create(filepath)?; 
    let bytes: Vec<u8> = bson::to_vec(&content).unwrap_or(Vec::new());
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

pub fn read_msgp_file(filepath: &str) -> MyResult<Vec<u8>> {
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

pub fn read_base64_file(filepath: &str) -> MyResult<String>{
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

//JSON to BSON : cargo run -- "input.json" --bson
pub fn json_to_bson(json_content: &Value) -> MyResult<Document>{
    match json_content {
        Value::Object(map) => {
            let mut doc = Document::new();
            for (key, value) in map{
                doc.insert(key, json_to_bson2(value.clone()));
            }
            Ok(doc)
        }
        _ => panic!("Top level JSON must be an object")
    }
}


fn json_to_bson2(value: Value) -> bson::Bson{
    match value {
        serde_json::Value::Null => bson::Bson::Null,
        serde_json::Value::Bool(b) => bson::Bson::Boolean(b),
        serde_json::Value::Number(n) => {
            if n.is_f64(){
                bson::Bson::Double(n.as_f64().unwrap())
            }else if n.is_i64(){
                bson::Bson::Int32((n.as_i64().unwrap()) as i32)
            }
            else {
                bson::Bson::Int64(n.as_i64().unwrap())
            }
        }
        serde_json::Value::String(s) => bson::Bson::String(s), 
        serde_json::Value::Array(vec) => {
            let bson_array: Vec<bson::Bson> = vec.into_iter().map(json_to_bson2).collect();
            bson::Bson::Array(bson_array)
        }
        serde_json::Value::Object(map) => {
            let mut doc = Document::new();
            for (key, value2) in map{
                doc.insert(key, json_to_bson2(value2));
            }
            bson::Bson::Document(doc)
        }
    }
}

pub fn bson_to_json(doc: &Document) -> MyResult<serde_json::Value>{
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
                let json_array: Vec<serde_json::Value> = doc_arr.iter().map(|elem| bson_to_json(elem.unwrap()).unwrap()).collect(); 
                serde_json::Value::Array(json_array)
            },
            Bson::Document(doc) => bson_to_json(doc)?, 
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
                    bson_to_json(&js_c.scope)?
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
            Bson::Binary(b) => serde_json::Value::String(bin_to_base64(b.bytes.to_vec())?), //needed
            Bson::ObjectId(objid) => Value::String(objid.to_hex()),
            Bson::DateTime(dt) => serde_json::Value::String(dt.try_to_rfc3339_string()?),
            Bson::Symbol(s) => serde_json::Value::String(s.clone()),
            Bson::Decimal128(d) => serde_json::Value::String(d.to_string()),
            Bson::Undefined => serde_json::Value::Null,
            Bson::MaxKey => serde_json::Value::Object(serde_json::from_str(r#"{"$maxKey":1}"#)?),
            Bson::MinKey => serde_json::Value::Object(serde_json::from_str(r#"{"$maxKey":1}"#)?),
            Bson::DbPointer(p) => serde_json::Value::Null
        };
        json_map.insert(key.to_string(), json_value); 
    }
    Ok(Value::Object(json_map))
}

pub fn json_to_msgpack(value: &Value) -> MyResult<Vec<u8>>{
    match value{
        Value::Object(map) => {
            let mut buf = Vec::new();
            for (key, value) in map{
                let key_bytes = json_to_msgpack2(&serde_json::Value::String(key.clone()))?;
                buf.extend_from_slice(&key_bytes);

                let msg_value = json_to_msgpack2(value)?;
                buf.extend_from_slice(&msg_value);
            }
            Ok(buf)
        }
        _ => panic!("Top level JSON must be an object")
    }
}

pub fn json_to_msgpack2(value: &Value) -> MyResult<Vec<u8>>{
    let mut buf: Vec<u8> = Vec::new();
    match value{
        Value::Null => write_nil(&mut buf).unwrap(), 
        Value::Bool(b) => write_bool(&mut buf, *b)?,
        Value::String(s) => write_str(&mut buf, &s)?,
        Value::Number(n) => {
            if let Some(n) = n.as_f64(){
                write_f64(&mut buf, n)?;
            } else if let Some(n) = n.as_i64(){
                write_i32(&mut buf, n as i32)?;
            } else {
                write_f32(&mut buf, n.as_f64().unwrap() as f32)?;
            }
        },
        Value::Array(arr) => {
            let len = arr.len() as u32; 
            write_array_len(&mut buf, len)?;
            for elem in arr{
                let _ = json_to_msgpack2(elem);
            }
        }, 

        Value::Object(map) => {
            let len = map.len() as u32; 
            write_map_len(&mut buf, len)?; 
            for (key, value) in map{
                write_str(&mut buf, &key)?;
                let _ = json_to_msgpack2(value);
            }
        }
    }
    Ok(buf)
}

// pub fn json_to_msgpack2(value: &Value) -> MyResult<Vec<u8>> {
//     let mut buf: Vec<u8> = Vec::new();
//     match value{
//         Value::Null => write_nil(&mut buf).unwrap(), 
//         Value::Bool(b) => write_bool(&mut buf, *b)?,
//         Value::String(s) => write_str(&mut buf, &s)?,
//         Value::Number(n) => {
//             if let Some(n) = n.as_f64(){
//                 write_f64(&mut buf, n)?;
//             } else if let Some(n) = n.as_i64(){
//                 write_i64(&mut buf, n)?;
//             } else {
//                 write_f32(&mut buf, n.as_f64().unwrap() as f32)?;
//             }
//         },
//         Value::Array(arr) => {
//             let len = arr.len() as u32; 
//             write_array_len(&mut buf, len)?;
//             for elem in arr{
//                 let _ = json_to_msgpack2(elem);
//             }
//         }, 

//         Value::Object(map) => {
//             let len = map.len() as u32; 
//             write_map_len(&mut buf, len)?; 
//             for (key, value) in map{
//                 write_str(&mut buf, &key)?;
//                 let _ = json_to_msgpack2(value);
//             }
//         }
//     }
//     Ok(buf)
// }


pub fn msgpack_to_json(msgpack_data: &[u8]) -> MyResult<Value> {
    let mut reader = Cursor::new(msgpack_data);
    let mut de = Deserializer::new(reader);

    let key_value_pairs: BTreeMap<String, Value> = Deserialize::deserialize(&mut de)?;
    let mut json_map: serde_json::Map<String, Value> = serde_json::Map::new();
    
    for (key, value) in key_value_pairs.iter().rev() {
        json_map.insert(key.to_string(), value.clone());
    }
    let json_value: Value = Value::Object(json_map);
    // let json_string = serde_json::to_string(&json_value).unwrap();
    Ok(json_value)
}

pub fn bson_to_msgpack(bson_doc: &Document) -> MyResult<Vec<u8>>{
    let json_value = bson_to_json(bson_doc)?;
    let msgpack_content = json_to_msgpack(&json_value)?;
    Ok(msgpack_content)
}

pub fn msgpack_to_bson(input: &[u8]) -> MyResult<Document>{
    let json_content: Value = msgpack_to_json(&input)?;
    let doc = json_to_bson(&json_content)?;
    Ok(doc)
}

pub fn bin_to_base64(binary: Vec<u8>) -> MyResult<String>{
    for i in &binary{
        println!("{:08b}", i);
    }
    
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

pub fn base64_to_bin(base64: String) -> MyResult<Vec<u8>>{
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


