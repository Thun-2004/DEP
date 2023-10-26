mod lib;
use DEF::write_bson_file;
use bson::doc;
use lib::{get_args, run, json_to_bson,write_bson_file2, write_base64, read_raw_binary,  bson_to_json, write_msgp_file, write_json_file,read_msgp_file,read_json_file,  bin_to_base64, base64_to_bin, json_to_msgpack, msgpack_to_json};
use serde::{Serialize, Deserialize};
use std::{fs::File, io::Read};
use serde_json::{Value, Map};

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
    // let input = vec![0b01101000, 0b01100101, 0b01101100, 0b01101100, 0b01101111]; 
    // let mut file = File::create("input.bin").expect("Unable to create file");
    // // let bytes = input.as_slice();
    // file.write_all(&input).expect("Unable to write data");

    if let Err(err) = lib::get_args().and_then(lib::run){
        eprint!("Error: {}", err);
        std::process::exit(1);
    }
    

    // let mut file = read_json_file("input.json").expect("Unable to open file");
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

}

