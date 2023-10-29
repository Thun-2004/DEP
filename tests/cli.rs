use DEF::{read_json_file, 
    read_bson_file, 
    read_msgp_file, 
    read_raw_binary, 
    read_base64_file, 
    json_to_bson, 
    json_to_msgpack, 
    bson_to_json, 
    bson_to_msgpack, 
    msgpack_to_bson, 
    msgpack_to_json, 
    bin_to_base64, 
    msgpack_to_string, 
    write_msgp_file, 
    write_json_file
};

use bson::Document;
use serde_json::Value; 
use std::error::Error; 

#[derive(PartialEq, Debug)]
enum Content{
    Json(Value),
    Bson(Document),
    Msgpack(String),
    B64(String)
}

type TestResult = Result<(), Box<dyn Error>>; 

fn run(input_file: &str, expected_file: &str) -> TestResult{
    let from = input_file.split('.').last().unwrap();
    let to = expected_file.split('.').last().unwrap();
    let result_content; 
    let output_content;
    match from{
        "json" => {
            match to{
                "bson" => {
                    let result = read_json_file(input_file)?;
                    let bson_result = json_to_bson(&result)?;
                    result_content = Content::Bson(bson_result);
                    output_content = Content::Bson(read_bson_file(expected_file)?);
                },
                "msgpack" => {
                    let result = read_json_file(input_file)?;
                    let msg_result = json_to_msgpack(&result)?;
                    let _ = write_msgp_file("tests/expected/temp_json.msgpack", &msg_result);
                    result_content = Content::Msgpack(msgpack_to_string("tests/expected/temp_json.msgpack"));
                    output_content = Content::Msgpack(msgpack_to_string(expected_file));
                }, 
                _ => panic!("Invalid conversion")
            }
        }, 
        "bson" => {
            match to{
                "json" => {
                    let result = read_bson_file(input_file)?;
                    let json_result = bson_to_json(&result)?;
                    result_content = Content::Json(json_result);
                    output_content = Content::Json(read_json_file(expected_file)?);
                }, 
                "msgpack" => {
                    let result = read_bson_file(input_file)?;
                    let bson_result = bson_to_msgpack(&result)?;
                    let _ = write_msgp_file("tests/expected/temp_json2.msgpack", &bson_result);
                    result_content = Content::Msgpack(msgpack_to_string("tests/expected/temp_json2.msgpack"));
                    output_content = Content::Msgpack(msgpack_to_string(expected_file));
 
                },
                _ => panic!("Invalid conversion")
            }
        }, 
        "msgpack" => {
            //main function should be msgp_to_bson
            match to{
                "bson" => { 
                    let msg = msgpack_to_string(input_file);
                    let msg_json: Value = serde_json::from_str(&msg)?;
                    let result = json_to_bson(&msg_json)?;
                    let bson = read_bson_file(&expected_file)?;
                    result_content = Content::Bson(result);
                    output_content = Content::Bson(bson);
                },
                "json" => {
                    let msg = msgpack_to_string(input_file);
                    let msg_json: Value = serde_json::from_str(&msg)?;
                    let json: Value = read_json_file(expected_file)?;
                    result_content = Content::Json(msg_json);
                    output_content = Content::Json(json);
                },
                _ => panic!("Invalid conversion")
            }
        },
        "bin" => {
            match to{
                "b64" => {
                    let result = read_raw_binary(input_file)?;
                    let bin_result = bin_to_base64(result)?;
                    result_content = Content::B64(bin_result);
                    output_content = Content::B64(read_base64_file(expected_file)?);
                }, 
                _ => panic!("Invalid conversion")
            }
        }, 
        _ => panic!("Invalid conversion")
    }
    assert_eq!(result_content, output_content);
    Ok(())
}

#[test]
fn test_empty_json_to_bson() -> TestResult{
    let input = "tests/inputs/empty_json.json"; 
    run(input, "tests/expected/empty_json.bson")
}

#[test]
fn test_blank_json_to_bson() -> TestResult{
    let input = "tests/inputs/blank_json.json";
    run(input, "tests/expected/blank_json.bson")
}

#[test]
fn test_full_json_to_bson() -> TestResult{
    let input = "./tests/inputs/full_json.json";
    run(input, "./tests/expected/full_json.bson")
}

#[test]
fn test_full_json_to_msgpack() -> TestResult{
    let input = "tests/inputs/full_json.json"; 
    run(input, "tests/expected/full_json.msgpack")
}

#[test]
fn test_empty_bson_to_json() -> TestResult{
    let input = "./tests/inputs/empty_bson.bson";
    run(input, "./tests/expected/empty_bson.json")
}

#[test]
fn test_full_bson_to_json() -> TestResult{
    let input = "./tests/inputs/full_bson.bson";
    run(input, "./tests/expected/full_bson.json")
}

#[test]
fn test_empty_bson_to_msgpack() -> TestResult{
    let input = "./tests/inputs/empty_bson.bson";
    run(input, "./tests/expected/empty_bson.msgpack")
}

#[test]
fn test_full_bson_to_msgpack() -> TestResult{
    let input = "./tests/inputs/full_bson.bson";
    run(input, "./tests/expected/full_bson.msgpack")
}


#[test]
fn test_empty_msgpack_to_json() -> TestResult{
    let input = "./tests/inputs/empty_msgpack.msgpack";
    run(input, "./tests/expected/empty_msgpack.json")
}

#[test]
fn test_full_msgpack_to_json() -> TestResult{
    let input = "./tests/inputs/full_msgpack.msgpack";
    run(input, "./tests/expected/full_msgpack.json")
}


#[test]
fn test_empty_msgpack_to_bson() -> TestResult{
    let input = "./tests/inputs/empty_msgpack.msgpack";
    run(input, "./tests/expected/empty_msgpack.bson")
}

#[test]
fn test_full_msgpack_to_bson() -> TestResult{
    let input: &str = "./tests/inputs/full_msgpack.msgpack";
    run(input, "./tests/expected/full_msgpack.bson")
}

#[test]
fn test_full_bin_to_base64() -> TestResult{
    let input = "./tests/inputs/raw_bin.bin";
    run(input, "./tests/expected/raw_bin.b64")
}

#[test]
fn test_empty_bin_to_base64() -> TestResult{
    let input = "./tests/inputs/empty_bin.bin";
    run(input, "./tests/expected/empty_bin.b64")
}


