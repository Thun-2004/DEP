use clap::{App, Arg, ArgMatches};
use std::error::Error;
struct Config{
    filename : String, 
    filetype : String, 
    desired_type: String
}

//run : cargo run -- --help
pub fn get_args() -> Result<(), Box<dyn Error + 'static>>{
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
                .takes_value(true)
        )
        .arg(
            Arg::with_name("bson")
                .short("bson")
                .long("bson")
                .value_name("BSON")
                .help("Convert data to BSON")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("msgpack")
                .short("msgp")
                .long("msgpack")
                .value_name("MSGP")
                .help("Convert data to MessagePack")
                .takes_value(true)
        )
        .get_matches();
    Ok(())
        
}

