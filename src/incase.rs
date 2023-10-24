
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
