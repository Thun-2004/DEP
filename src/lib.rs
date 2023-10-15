use clap::{App, Arg};
use std::error::Error;
use serde_json::{Value, Number};
use bson::{Document , Bson}; 
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use rmp::encode; 
use rmp::encode::Encoder; 
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

//process data from Struct Config and save to file 
pub fn run(config: Config) -> MyResult<()>{
    println!("{:?}", config);
    let mut file = File::create("output.bson")?; 
    let content = read_json_file(&config.filename)?;
    let doc = json_to_bson(&content);
    let bytes = bson::to_vec(&doc).unwrap();
    file.write_all(&bytes).expect("Write failed");
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
        Value::Null => bson::Bson::Null,
        Value::Bool(b) => bson::Bson::Boolean(b),
        Value::Number(n) => {
            if n.is_f64(){
                bson::Bson::Double(n.as_f64().unwrap())
            } else {
                bson::Bson::Int64(n.as_i64().unwrap())
            }
        }
        Value::String(s) => bson::Bson::String(s), 
        Value::Array(vec) => {
            let bson_array: Vec<bson::Bson> = vec.into_iter().map(str_to_json).collect();
            bson::Bson::Array(bson_array)
        }
        Value::Object(map) => {
            let mut doc = Document::new();
            for (key, value2) in map{
                doc.insert(key, str_to_json(value2));
            }
            bson::Bson::Document(doc)
        }
    }
}

//BSON to JSON 
//add read bson and write json
pub fn bson_to_json(doc: &Document) -> Value{
    let mut json_map = serde_json::Map::new();
    for (key, value) in doc.iter(){
        let json_value = match value{
            Bson::Double(d) => Value::Number(serde_json::Number::from_f64(*d).unwrap()), 
            Bson::String(s) => Value::String(s.clone()), 
            Bson::Array(arr) => {
                let doc_arr = arr.iter().map(|b| match b{
                    Bson::Document(document) => Some(document), 
                    _ => None, 
                }).collect::<Vec<Option<&Document>>>();
            
                let json_array: Vec<Value> = doc_arr.iter().map(|elem| bson_to_json(elem.unwrap())).collect(); 
                Value::Array(json_array)
            }
            Bson::Document(doc) => bson_to_json(doc), 
            Bson::Null => Value::Null,
            Bson::Boolean(b) => Value::Bool(*b),
            Bson::RegularExpression(_) => todo!(),
            Bson::JavaScriptCode(_) => todo!(),
            Bson::JavaScriptCodeWithScope(_) => todo!(),
            Bson::Int32(n) => Value::Number::from(n),
            Bson::Int64(_) => todo!(),
            Bson::Timestamp(_) => todo!(),
            Bson::Binary(_) => todo!(),
            Bson::ObjectId(_) => todo!(),
            Bson::DateTime(_) => todo!(),
            Bson::Symbol(_) => todo!(),
            Bson::Decimal128(_) => todo!(),
            Bson::Undefined => todo!(),
            Bson::MaxKey => todo!(),
            Bson::MinKey => todo!(),
            Bson::DbPointer(_) => todo!(), 
        };
        json_map.insert(key.to_string(), json_value); 
    }
    Value::Object(json_map)
}

//JSON to MessagePack
pub fn json_to_msgpack<W: Write>(encoder: &mut encode::Encoder<W>, value: Value) {
    match value{
        Value::Null => encoder.encoder_nil(), 
        Value::Bool(b) => encoder.encoder_bool(b), 
        Value::Number(n) => {
            if n.is_f64(){
                encoder.encoder_f64(n.as_f64().unwrap());
            }else if n.is_i64(){
                encoder.encoder_i64(n.as_i64().unwrap());
            }else {
                encoder.encoder_f64*(n.as_f64().unwrap() as f32);
            }
        }
        Value::String(s) => encoder.encode_str(&s), 
        Value::Array(arr) => {
            let len = arr.len() as u32;
            encoder.encode_array_len(len).unwrap(); 
            for i in arr{
                json_to_msgpack(encoder, i);
            }
        }
        Value::Object(map) => {
            let len = map.len() as u32; 
            encoder.encode_map_len(len).unwrap();
            for (key, value) in map{
                encoder.encode_str(key); 
                json_to_msgpack(encoder, value)
            }
                
        }

    }
}
//Messagepack to Json 


//JSON to Base64 

//Base64 to json 













