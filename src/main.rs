mod lib;
use DEF::write_bson_file;
use bson::doc;
use lib::{get_args, run, json_to_bson,write_bson_file2, write_base64, read_raw_binary,  bson_to_json, write_msgp_file, write_json_file,read_msgp_file,read_json_file,  bin_to_base64, base64_to_bin, json_to_msgpack, msgpack_to_json};
use serde::{Serialize, Deserialize};
use std::{fs::File, io::{Read, Cursor}};
use serde_json::{Value, Map};
use msgpack_simple::{MsgPack, MapElement, Extension};


use std::fmt; 
use bson::{Bson, Document, raw::RawBson};
use std::io::{self, Write};
use serde::de::Error;
use base64::{encode};

use rmp_serde::{encode, to_vec, Deserializer, Serializer};


#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u32,
    married: bool,
    pet: Option<serde_json::Value>,
    children: Vec<Child>,
    address: Address,
    car_model: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Child {
    name: String,
    age: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Address {
    street: String,
    city: String,
    state: String,
    postalCode: String,
}
fn main() {
    // if let Err(err) = lib::get_args().and_then(lib::run){
    //     eprint!("Error: {}", err);
    //     std::process::exit(1);
    // }
    let input = r#"{
        "name" : "John", 
        "age" : 30, 
        "married": false, 
        "pet": null, 
        "children": [
            {
                "name": "Ann", 
                "age": 5
            },
            {
                "name": "Sally", 
                "age": 7
            }
        ],
        "address": {
            "street": "21 2nd Street", 
            "city": "New York", 
            "state": "NY", 
            "postalCode": "10021"
        },
        "car_model" : " "
    }"#;
    // let input: &str = r#"{}"#;
    let value: serde_json::Value = serde_json::from_str(input).unwrap();
    let content = to_vec(&value).unwrap();
    let _ = write_msgp_file("input.msgpack", &content);

    let file = File::open("input.msgpack").unwrap();
    let mut de: Deserializer<rmp_serde::decode::ReadReader<File>> = Deserializer::new(file);
    let json_value: Value = Deserialize::deserialize(&mut de).unwrap();
    let bson = json_to_bson(&json_value).unwrap();
    let _ = write_bson_file("output.bson", bson);

    //json to msgpack
    //test bson
    // let person = Person{
    //     name: "John".to_string(), 
    //     age: 30,
    //     married: false,
    //     pet: None,
    //     children: vec![
    //         Child{
    //             name: "Ann".to_string(), 
    //             age: 5
    //         }, 
    //         Child{
    //             name: "Sally".to_string(), 
    //             age: 7
    //         }
    //     ],
    //     address: Address{
    //         street: "21 2nd Street".to_string(), 
    //         city: "New York".to_string(), 
    //         state: "NY".to_string(), 
    //         postalCode: "10021".to_string()
    //     },
    //     car_model: " ".to_string()
    // };
    //normally convert to msgpack then binary 


}

