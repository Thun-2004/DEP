
// fn read_msgpack_marker(data: &[u8]) -> Option<Marker>{
//     if data.is_empty(){
//         return None; 
//     }
//     match data[0]{
//         0xc0 => Some(Marker::Null), 
//         0xc2 => Some(Marker::False),
//         0xc3 => Some(Marker::True),
//         0xd0 => Some(Marker::I32),
//         0xd1 => Some(Marker::I64),
//         0xd2 => Some(Marker::F32),
//         0xd3 => Some(Marker::F64),
//         0xd9 => Some(Marker::Str8),
//         0xda => Some(Marker::Str16),
//         0xdb => Some(Marker::Str32),
//         0xc4 => Some(Marker::Bin8),
//         0xc5 => Some(Marker::Bin16),
//         0xc6 => Some(Marker::Bin32),
//         0xdc => Some(Marker::Array16),
//         0xdd => Some(Marker::Array32),
//         0xde => Some(Marker::Map16),
//         0xdf => Some(Marker::Map32),
//         0x00..=0x7f => Some(Marker::FixPos(data[0])),
//         0xe0..=0xff => Some(Marker::FixNeg(data[0] as i8)),
//         _ => None
//     }

// }

// fn msgpack_to_json_value(msgpack_data: &[u8]){
//     let mut json_map: serde_json::Map<String, Value> = serde_json::Map::new(); 
//     let doc =  extract_msgpack_key_value_pairs2(msgpack_data).unwrap(); 
//     for (key, value) in doc{
//         let json_value = match value{
//             Marker::Null => serde_json::Value::Null,
//             Marker::False => Ok(Value::Bool(false)),
//             Marker::True => Ok(Value::Bool(true)),
//             Marker::I32 => Ok(Value::Number(data.as_i64().unwrap().into())),
//             Marker::Str8 | Marker::Bin8 => {
//                 let s: String = data.read_str().map_err(|_| "Failed to read string")?;
//                 return Ok(Value::String(s));
//             }
//             Marker::FixArray(size) => {
//                 let mut arr = Vec::with_capacity(size as usize);
//                 for _ in 0..size {
//                     arr.push(msgpack_to_json_value(msgpack)?);
//                 }
//                 Ok(arr)
//             }
//             Marker::FixPos(_) => todo!(),
//             Marker::FixNeg(_) => todo!(),
//             Marker::U8 => todo!(),
//             Marker::U16 => todo!(),
//             Marker::U32 => todo!(),
//             Marker::U64 => todo!(),
//             Marker::I8 => todo!(),
//             Marker::I16 => todo!(),
//             Marker::I64 => todo!(),
//             Marker::F32 => todo!(),
//             Marker::F64 => todo!(),
//             Marker::FixStr(_) => todo!(),
//             Marker::Str16 => todo!(),
//             Marker::Str32 => todo!(),
//             Marker::Bin16 => todo!(),
//             Marker::Bin32 => todo!(),
//             Marker::Array16 => todo!(),
//             Marker::Array32 => todo!(),
//             Marker::FixMap(_) => todo!(),
//             Marker::Map16 => todo!(),
//             Marker::Map32 => todo!(),
//             Marker::FixExt1 => todo!(),
//             Marker::FixExt2 => todo!(),
//             Marker::FixExt4 => todo!(),
//             Marker::FixExt8 => todo!(),
//             Marker::FixExt16 => todo!(),
//             Marker::Ext8 => todo!(),
//             Marker::Ext16 => todo!(),
//             Marker::Ext32 => todo!(),
//             Marker::Reserved => todo!(),
//         };
//         json_map.insert(key.to_string(), json_value); 
//     }
// }


// pub fn bson_to_msgpack(bson_doc: &Document) -> Result<Vec<u8>, Box<dyn std::error::Error>>{
//     let mut buf = Vec::new(); 
//     let elements = bson_doc.iter();
//     write_map_len(&mut buf, elements.count() as u32)?;

//     for(key, value) in elements{
//         write_str(&mut buf, key)?;
//         match value{
//             Bson::Array(array) => {
//                 write_array_len(&mut buf, array.len() as u32)?;
//                 for elem in array.iter(){
//                     bson_value_to_msgpack(elem, &mut buf)?;
//                 }
//             }
//             Bson::Document(sub_doc) => {
//                 bson_to_msgpack(sub_doc)?;
//             }
//             _ => {
//                 return Err(Box::new(std::io::Error::new(
//                     std::io::ErrorKind::InvalidData,
//                     "Invalid data",
//                 )))
//             }
//         }
//     }
//     Ok(buf)
// }

// pub fn bson_value_to_msgpack(value: &Bson, buf: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>>{
// // pub fn bson_value_to_msgpack(value: &Bson, buf: &mut Vec<u8>) -> Result<(), ValueWriteError>{
//     match value{
//         Bson::Double(d) => write_f64( buf, *d)?,
//         Bson::String(s) => write_str(buf, s)?,
//         Bson::Array(array) => {
//             write_array_len(buf, array.len() as u32)?;
//             for element in array.iter(){
//                 bson_value_to_msgpack(element, buf)?;
//             }
//         }
//         Bson::Document(doc) => {
//             bson_to_msgpack(doc);
//         }
//         Bson::Null => write_nil(buf)?,
//         Bson::Boolean(b) => write_bool(buf, *b)?,
//         Bson::RegularExpression(reg) => {
//             write_str(buf, &reg.pattern)?;
//             write_str(buf, &reg.options)?;
//         },  
//         Bson::JavaScriptCode(js) => write_str(buf, js).unwrap(), 
//         Bson::JavaScriptCodeWithScope(js_c) => {
//             write_str(buf, &js_c.code)?;
//             bson_to_msgpack(&js_c.scope)?;
//         },  
//         Bson::Int32(n) => write_i32(buf, *n)?,
//         Bson::Int64(n) => write_i64(buf, *n)?,
//         Bson::Timestamp(t) => {
//             write_u32(buf, t.time)?;
//             write_u32(buf, t.increment)?;
//         },
//         Bson::Binary(b) => {
//             let binary = b.bytes.clone();
//             write_bin(buf, &binary).unwrap()
//         }
//         Bson::ObjectId(objid) => write_str(buf, &objid.to_hex())?,
//         Bson::DateTime(dt) => write_str(buf, &dt.try_to_rfc3339_string().unwrap())?,
//         Bson::Symbol(s) => write_str(buf, s)?,
//         Bson::Decimal128(d) => write_str(buf, &d.to_string())?,
//         Bson::Undefined => write_nil(buf)?,
//         Bson::MaxKey => write_str(buf, "$maxKey")?,
//         Bson::MinKey => write_str(buf, "$minKey")?,
//         Bson::DbPointer(p) => write_nil(buf)?
//     }
//     Ok(())
// }

// pub fn msgpack_to_bson(input: &[u8]) -> MyResult<Document>{
//     let mut reader = Cursor::new(input);
//     let mut de = Deserializer::new(reader);
//     let key_value_pairs: BTreeMap<String, Value> = Deserialize::deserialize(&mut de)?;
//     let mut bson_doc = Document::new();
//     for (key, value) in key_value_pairs.iter().rev() {
//         bson_doc.insert(key.to_string(), value.clone());
//     }
//     Ok(bson_doc)
// }



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
    // let mut buf = Vec::new();
    // let mut buf2 = Vec::new();
    //normally convert to msgpack then binary    
    
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




    //create bson doc for test 
    // let mut document = Document::new();
    // document.insert("name", Bson::String("John".to_string()));
    // document.insert("age", Bson::Int32(30));
    // document.insert("married", Bson::Boolean(false));
    // document.insert("pet", Bson::Null);
    // document.insert("children", Bson::Array(vec![
    //     Bson::Document(doc!{
    //         "name": Bson::String("Ann".to_string()), 
    //         "age": Bson::Int32(5)
    //     }), 
    //     Bson::Document(doc!{
    //         "name": Bson::String("Sally".to_string()), 
    //         "age": Bson::Int32(7)
    //     })
    // ]));
    // document.insert("address", Bson::Document(doc!{
    //     "street": Bson::String("21 2nd Street".to_string()), 
    //     "city": Bson::String("New York".to_string()), 
    //     "state": Bson::String("NY".to_string()), 
    //     "postalCode": Bson::String("10021".to_string())
    // }));
    // document.insert("car_model", Bson::String(" ".to_string()));

    // let _ = write_bson_file2("input.bson", &document);

    // //create json from preexisiting method 
    // let json:Value = serde_json::from_str(&"{}").unwrap();
    // let _ = write_json_file("output2.json", json);



    //create bin for test
    // let mut file = File::create("input.bin").unwrap();
    // let data: [u8; 0] = [];  
    // // let data: [u8; 5] = [0x41, 0x42, 0x43, 0x44, 0x45];  
    // file.write_all(&data).unwrap();

    // let mut file = File::open("input.bin").unwrap();
    // let mut b64 = read_raw_binary("input.bin").unwrap();
    // let b64 = encode(&b64);
    // let _ = write_base64("output2.b64", b64);


//incorrect version of json to msgpack 
// pub fn json_to_msgpack(value: &Value) -> MyResult<Vec<u8>>{
//     match value{
//         Value::Object(map) => {
//             let mut buf = Vec::new();
//             for (key, value) in map{
//                 let key_bytes: Vec<u8> = json_to_msgpack2(&serde_json::Value::String(key.clone()))?;
//                 buf.extend_from_slice(&key_bytes);

//                 let msg_value = json_to_msgpack2(value)?;
//                 buf.extend_from_slice(&msg_value);
//             }
//             Ok(buf)
//         }
//         _ => panic!("Top level JSON must be an object")
//     }
// }

// pub fn json_to_msgpack2(value: &Value) -> MyResult<Vec<u8>>{
//     let mut buf: Vec<u8> = Vec::new();
//     match value{
//         Value::Null => write_nil(&mut buf).unwrap(), 
//         Value::Bool(b) => write_bool(&mut buf, *b)?,
//         Value::String(s) => write_str(&mut buf, &s)?,
//         Value::Number(n) => {
//             if let Some(n) = n.as_f64(){
//                 write_f64(&mut buf, n)?;
//             } else if let Some(n) = n.as_i64(){
//                 write_i32(&mut buf, n as i32)?;
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

// let message: MsgPack = MsgPack::Map(vec![
//     MapElement {
//         key: MsgPack::String("name".to_string()),
//         value: MsgPack::String("John".to_string()),
//     },
//     MapElement {
//         key: MsgPack::String("age".to_string()),
//         value: MsgPack::Int(30),
//     },
//     MapElement {
//         key: MsgPack::String("married".to_string()),
//         value: MsgPack::Boolean(false),
//     },
//     MapElement {
//         key: MsgPack::String("pet".to_string()),
//         value: MsgPack::Nil,
//     },
//     MapElement {
//         key: MsgPack::String("children".to_string()),
//         value: MsgPack::Array(vec![
//             MsgPack::Map(vec![
//                 MapElement {
//                     key: MsgPack::String("name".to_string()),
//                     value: MsgPack::String("Ann".to_string()),
//                 },
//                 MapElement {
//                     key: MsgPack::String("age".to_string()),
//                     value: MsgPack::Int(5),
//                 },
//             ]),
//             MsgPack::Map(vec![
//                 MapElement {
//                     key: MsgPack::String("name".to_string()),
//                     value: MsgPack::String("Sally".to_string()),
//                 },
//                 MapElement {
//                     key: MsgPack::String("age".to_string()),
//                     value: MsgPack::Int(7),
//                 },
//             ]),
//         ]),
//     },
//     MapElement {
//         key: MsgPack::String("address".to_string()),
//         value: MsgPack::Map(vec![
//             MapElement {
//                 key: MsgPack::String("street".to_string()),
//                 value: MsgPack::String("21 2nd Street".to_string()),
//             },
//             MapElement {
//                 key: MsgPack::String("city".to_string()),
//                 value: MsgPack::String("New York".to_string()),
//             },
//             MapElement {
//                 key: MsgPack::String("state".to_string()),
//                 value: MsgPack::String("NY".to_string()),
//             },
//             MapElement {
//                 key: MsgPack::String("postalCode".to_string()),
//                 value: MsgPack::String("10021".to_string()),
//             },
//         ]),
//     }
// ]);


// let input = r#"{
//     "name" : "John", 
//     "age" : 30, 
//     "married": false, 
//     "pet": null, 
//     "children": [
//         {
//             "name": "Ann", 
//             "age": 5
//         },
//         {
//             "name": "Sally", 
//             "age": 7
//         }
//     ],
//     "address": {
//         "street": "21 2nd Street", 
//         "city": "New York", 
//         "state": "NY", 
//         "postalCode": "10021"
//     },
//     "car_model" : " "
// }"#;
// // let input: &str = r#"{}"#;
// let value: serde_json::Value = serde_json::from_str(input).unwrap();
// let content = to_vec(&value).unwrap();
// let _ = write_msgp_file("input.msgpack", &content);

// let file = File::open("input.msgpack").unwrap();
// let mut de: Deserializer<rmp_serde::decode::ReadReader<File>> = Deserializer::new(file);
// let json_value: Value = Deserialize::deserialize(&mut de).unwrap();
// let bson = json_to_bson(&json_value).unwrap();
// let _ = write_bson_file("output.bson", bson);

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