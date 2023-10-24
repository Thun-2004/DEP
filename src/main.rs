mod lib;
use lib::{get_args, run, json_to_bson, write_json_file,read_msgp_file, bin_to_base64, base64_to_bin, json_to_msgpack, msgpack_to_json};
use serde::Serializer;
use std::{fs::File, io::Read};
use serde_json::{Value, Map};

use std::fmt; 
use bson::{Bson, Document, raw::RawBson};
use std::io::{self, Write};
use serde::de::Error;

use rmp_serde::encode;
use rmp_serde::Serializer as OtherSerializer;


fn main() {
    // let input = vec![0b01101000, 0b01100101, 0b01101100, 0b01101100, 0b01101111]; 
    // let mut file = File::create("input.bin").expect("Unable to create file");
    // // let bytes = input.as_slice();
    // file.write_all(&input).expect("Unable to write data");

    if let Err(err) = lib::get_args().and_then(lib::run){
        eprint!("Error: {}", err);
        std::process::exit(1);
    }

    // print_bson(&doc);
    //println!("{:?}", doc); //pretty-printed BSON just like JSON format
    // let input = vec![0b01101000, 0b01100101, 0b01101100, 0b01101100, 0b01101111]; 
    // let mut file = File::create("input.bin").expect("Unable to create file");
    // let bytes = input.as_slice();
    // for i in bytes {
    //     println!("{:b}", i);
    // }
    // file.write_all(&bytes).expect("Unable to write data");
    // //convert byte to binary 
    // let mut file = File::open("input.bin").expect("Unable to open file");
    // let mut buffer = Vec::new();
    // file.read_to_end(&mut buffer).expect("Unable to read data");

    // let output = bin_to_base64(buffer).unwrap();
    // println!("{:?}", output);
    // let reverse: Vec<u8> = base64_to_bin(output).unwrap();
    // println!("{:?}", reverse);
    

    // let input_msgpack: &[u8] = &[
    //     0x81, 0xA4, 0x6E, 0x61, 0x6D, 0x65, 0xA4, 0x4A, 0x6F, 0x68, 0x6E
    // ];
    // let mut buffer = read_msgp_file("input.msgpack").unwrap();
    // let result = msgpack_to_json(&buffer).unwrap();
    // // println!("{:?}", result);
    // let _ = write_json_file("msgoutput.json", result).unwrap();

}

// 0x82, 0xA4, 0x6E, 0x61, 0x6D, 0x65, 0xA4, 0x4A, 0x6F, 0x68, 0x6E, 0xA3, 0x61, 0x67, 0x65, 0x1E

//value gone
// 0xA4, 0x4A, 0x6F, 0x68, 0x6E

//ending 
//0x1E
