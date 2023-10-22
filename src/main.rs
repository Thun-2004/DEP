mod lib;
use lib::{get_args, run, json_to_bson, bin_to_base64, base64_to_bin};
use serde::Serializer;
use std::{fs::File, io::Read};
use serde_json::{Value};

use std::fmt; 
use bson::{Bson, Document, raw::RawBson};
use std::io::{self, Write};
use serde::de::Error;

use rmp_serde::encode;
use rmp_serde::Serializer as OtherSerializer;


fn main() {
    if let Err(err) = lib::get_args().and_then(lib::run){
        eprint!("Error: {}", err);
        std::process::exit(1);
    }
    // print_bson(&doc);
    //println!("{:?}", doc); //pretty-printed BSON just like JSON format
    // let input = vec![0b01101000, 0b01100101, 0b01101100, 0b01101100, 0b01101111]; 
    // let output = bin_to_base64(input);
    // println!("{:?}", output);
    // let reverse: Result<Vec<u8>, Box<dyn Error>> = base64_to_bin(output.unwrap());
    // println!("{:?}", reverse);
}

// 0x82, 0xA4, 0x6E, 0x61, 0x6D, 0x65, 0xA4, 0x4A, 0x6F, 0x68, 0x6E, 0xA3, 0x61, 0x67, 0x65, 0x1E

//value gone
// 0xA4, 0x4A, 0x6F, 0x68, 0x6E

//ending 
//0x1E
