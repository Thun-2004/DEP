mod lib;
use lib::{get_args, run, json_to_bson, read_file};
use std::{fs::File, io::Read};
use serde_json::{Value};

use std::fmt; 
use bson::{Bson, Document, raw::RawBson};
use std::io::{self, Write};
use serde::de::Error;


fn main() {
    if let Err(err) = lib::get_args().and_then(lib::run){
        eprint!("Error: {}", err);
        std::process::exit(1);
    }
    
    // print_bson(&doc);
    //println!("{:?}", doc); //pretty-printed BSON just like JSON format
}








