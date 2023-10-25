
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