use clap::{App, Arg};
use std::error::Error;
use serde_json::{Value, Number, json};
use bson::{Document , Bson}; 
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use rmp::{encode, decode::RmpRead}; 

use rmp::Marker; 
use std::io::Cursor;
use rmp::encode::{write_nil, write_bool, write_f32, write_f64, write_i32, write_i64, write_str, write_array_len, write_map_len};

// use rmp::Value as RmpValue; 


//input format
#[derive(Debug)]
pub struct Config{
    filename : String, 
    filetype : String, 
    desired_type: String
}

type MyResult<T> = Result<T, Box<dyn Error>>;
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
                .help("Convert data to Base64")
                .takes_value(false)
                .required(false)
        )
        .get_matches();
    
    let filename = matches.value_of("files").unwrap().to_string(); 
    let filetype = filename.split(".").last().unwrap().to_string();
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
    let mut file = File::create("output.bson")?; 
    let content = read_json_file(&config.filename)?;
    //for json to bson 
    // let doc = json_to_bson(&content);
    // let bytes: Vec<u8> = bson::to_vec(&doc).unwrap();
    // file.write_all(&bytes).expect("Write failed");

    //for json to msgpack
    let json_content: Value = serde_json::from_str(&content).expect("Failed to parse to JSON");
    let result = json_to_msgpack2(json_content);
    println!("{:?}", result);
    Ok(())
}

pub fn read_json_file(filepath: &str) -> MyResult<String>{
    let content = fs::read_to_string(filepath).expect("Unable to read the file");
    Ok(content)
}

// pub fn write_file(config: Config, content: &Document) -> Result<(), Box<dyn Error>>{
//     let filename = format!("{}.{}", config.filename, config.desired_type);
//     let mut output = File::create(filename)?;
//     output.write_all(content).expect("Write failed");
//     Ok(())
// }

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
        //see if there's any other type 
        serde_json::Value::Null => bson::Bson::Null,
        serde_json::Value::Bool(b) => bson::Bson::Boolean(b),
        serde_json::Value::Number(n) => {
            if n.is_f64(){
                bson::Bson::Double(n.as_f64().unwrap())
            } else {
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

// BSON to JSON 
// add read bson and write json
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
            Bson::RegularExpression(_) => todo!(),
            Bson::JavaScriptCode(_) => todo!(),
            Bson::JavaScriptCodeWithScope(_) => todo!(),
            Bson::Int32(n) => serde_json::Value::Number(Number::from(*n)),
            Bson::Int64(n) => serde_json::Value::Number(Number::from(*n)),
            Bson::Timestamp(t) => serde_json::Value::Number(Number::from(t.timestamp())),
            Bson::Binary(_) => todo!(),
            Bson::ObjectId(objid) => Value::String(objid.to_hex()),
            Bson::DateTime(dt) => serde_json::Value::String(dt.try_to_rfc3339_string().unwrap()),
            Bson::Symbol(s) => serde_json::Value::String(s.clone()),
            Bson::Decimal128(d) => serde_json::Value::String(d.to_string()),
            Bson::Undefined => serde_json::Value::Null,
            Bson::MaxKey => serde_json::Value::Object(serde_json::from_str(r#"{"$maxKey":1}"#).unwrap()),
            Bson::MinKey => serde_json::Value::Object(serde_json::from_str(r#"{"$maxKey":1}"#).unwrap()),
            Bson::DbPointer(p) => serde_json::Value::Object(serde_json::from_str(&format!(r#"{{"$dbPointer":{},"$id":"{}"}}}}"#, p.get("namespace"), p.id)).unwrap())
        };
        json_map.insert(key.to_string(), json_value); 
    }
    Value::Object(json_map)
}


fn json_to_msgpack2(value: Value) -> Vec<String> {
// fn json_to_msgpack2(value: Value) -> Vec<u8> {
    let mut buf = Vec::new();
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
                json_to_msgpack2(elem);
            }
        }, 
        Value::Object(map) => {
            let len = map.len() as u32; 
            write_map_len(&mut buf, len).unwrap(); 
            for (key, value) in map{
                write_str(&mut buf, &key).unwrap();
                json_to_msgpack2(value);
            }
        }
    }
    let new_buf: Vec<String> = buf.iter().map(|b| format!("0x{:02X}", b)).collect();
    new_buf
}

//Messagepack to Json 
fn msgpack_to_json_string(msgpack: &[u8]) -> Result<String, &'static str> {
    let value = msgpack_to_json_value(msgpack)?;
    let json_string = serde_json::to_string(&value).map_err(|_| "Failed to convert to JSON")?;
    Ok(json_string)
}

fn msgpack_to_json_value(msgpack: &[u8]) -> Result<Value, &'static str> {
    let cursor = Cursor::new(msgpack);

    match RmpRead::read_exact_buf(&mut cursor).unwrap() {
        Ok((marker, data)) => match marker {
            Marker::Null => Ok(Value::Null),
            Marker::False => Ok(Value::Bool(false)),
            Marker::True => Ok(Value::Bool(true)),
            Marker::I32 => Ok(Value::Number(data.as_i64().unwrap().into())),
            Marker::Str8 | Marker::Bin8 => {
                let s: String = data.read_str().map_err(|_| "Failed to read string")?;
                Ok(Value::String(s))
            }
            Marker::FixArray(size) => {
                let mut array = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    array.push(msgpack_to_json_value(msgpack)?);
                }
                Ok(Value::Array(array))
            }
            Marker::Array32 => {
                let mut array = Vec::with_capacity(data.read_array_len() as usize);
                for _ in 0..data.read_array_len() {
                    array.push(msgpack_to_json_value(msgpack)?);
                }
                Ok(Value::Array(array))
            }
            Marker::Map32 => {
                let mut map = serde_json::Map::new();
                for _ in 0..data.read_map_len().unwrap() {
                    let key = msgpack_to_json_value(msgpack)?;
                    let value = msgpack_to_json_value(msgpack)?;

                    if let Value::String(key_str) = key {
                        map.insert(key_str, value);
                    } else {
                        return Err("Map keys must be strings");
                    }
                }
                Ok(Value::Object(map))
            }
            _ => Err("Unsupported MsgPack type"),
        },
        Err(ValueReadError::InvalidMarkerRead(_)) | Err(ValueReadError::InsufficientBytes(_)) => {
            Err("Failed to read MsgPack value")
        }
    }
}


//Bson to Msp

//Msg to Bson



//encode raw binary data to Base64 
// pub fn bin_to_base64()

//Decode Base64 to raw binary data
// pub fn base64_to_bin()



















