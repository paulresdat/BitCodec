mod fields;
pub mod encoder;
mod messaging;
mod benching;
use std::{collections::HashMap};
use benching::{bencher::{bencher}, benchmarks::benchmark_bit_pad_left};
use encoder::bitwise_ops::{get_bits_left_pad, get_bits_byte_pad_left, IntConverter};
use fields::{FieldDataType, Field, FieldValue};
use messaging::messages::{MessageFactory, MessageVersionQuery};

fn main() {
    let mut mf = MessageFactory::new();
    let the_data = r#"
[
    {
        "message_id": 1,
        "version_id": 1,
        "fields": [{
            "name": "field1",
            "data_type": "Byte",
            "field_length": 2,
            "field_length_type": "Bits",
            "field_type": "Standard"
        },
        {
            "name": "field2",
            "data_type": "Byte",
            "field_length": 0,
            "field_length_type": "Bits",
            "field_type": "Repeating",
            "repeating_spec_id": "2.1"
        }]
    },
    {
        "message_id": 2,
        "version_id": 1,
        "fields": [{
            "name": "field3",
            "data_type": "Byte",
            "field_length": 2,
            "field_length_type": "Bits",
            "field_type": "Standard"
        }]
    },
    {
        "message_id": 3,
        "version_id": 1,
        "unique_id": "message3",
        "fields": [{
            "name": "field4",
            "data_type": "Byte",
            "field_length": 2,
            "field_length_type": "Bits",
            "field_type": "Standard"
        }]
    }
]"#;

    mf.load(&the_data.to_string());

    let query = MessageVersionQuery { message_id: 1, version_id: 1};
    match mf.fetch(query) {
        Ok(m) => println!("Hooray! {:?}", m),
        Err(s) => println!("An error occurred: {}", s),
    }

    // let t = HashMap::from([(1, f3)]);
    // this is called '"fat" pointer including vtable'
    // let mut fields2: Vec<&mut Field> = Vec::new();
    // fields2.push(&mut f1 as &mut Field);
    // fields2.push(&mut f2 as &mut Field);
    // fields2.push(&mut f3 as &mut Field);
    // let mut fields2 = vec![
    //     &mut f1 as &mut dyn IField,
    //     &mut f2 as &mut dyn IField,
    //     &mut f3 as &mut dyn IField,
    // ];
    // let a = 0b0011_1111;
    // let bit_codec = BitCodec {};
    // let v = vec![a];
    // match bit_codec.decode(&v, fields2) {
    //     Err(err) => panic!("{}", err.error_str),
    //     Ok(_) => println!("Yay!"),
    // }
}

// pub struct BitCodecError {
//     error: CodecResultCode,
//     error_str: String
// }

// pub enum CodecResultCode
// {
//     EndExceedsBitLength,
//     MessageNotFullyParsed,
// }

// pub trait IBitCodec {
//     fn decode<'a>(&self, data: &'a Vec<u8>, fields: &'a Vec<&Field>, values: &'a mut HashMap<String, FieldValue>)
//         -> Result<String, BitCodecError>;
// }

// struct BitCodec {}

// impl BitCodec {
//     // private methods here
//     // fn _repeater<'a>(&self, data: &'a Vec<u8>, fields: &'a mut Vec<&'a mut Field>) -> Option<String> {
//     //     None
//     // }
// }

// impl IBitCodec for BitCodec {
//     fn decode<'a>(&self, data: &'a Vec<u8>, fields: &'a Vec<&Field>, values: &'a mut HashMap<String, FieldValue>)
//         -> Result<String, BitCodecError>
//     {
//         let mut start: usize = 0;
//         let mut end: usize = 0;
//         let total_bits = data.len() * 8;
//         for v in fields {
//             end = start + (v.field_length as usize);

//             //println!("Start {} :: End {}", start, end);
//             // unreversed raw bits
//             if end > total_bits {
//                 let bits_left = total_bits - start;
//                 let overflow_count = end - total_bits;
//                 return Err(BitCodecError {
//                     error: CodecResultCode::EndExceedsBitLength,
//                     error_str: format!("The end exceeds what bits are left. Bits left: {bits_left}, Overflow: {overflow_count} bits")
//                 });
//             }

//             let mut d: Option<FieldValue> = None;
//             match v.data_type {
//                 FieldDataType::Byte => {
//                     let bits = get_bits_left_pad(data, start, end-1);
//                     d = Some(FieldValue::Byte(bits[0]));
//                 }
//                 FieldDataType::U16 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 2);
//                     d = Some(FieldValue::U16(IntConverter::to_uint16(bytes)));
//                 },
//                 FieldDataType::U32 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 4);
//                     d = Some(FieldValue::U32(IntConverter::to_uint32(bytes)));
//                 },
//                 FieldDataType::U64 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     d = Some(FieldValue::U64(IntConverter::to_uint64(bytes)));
//                 },
//                 FieldDataType::U128 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     d = Some(FieldValue::U128(IntConverter::to_uint128(bytes)));
//                 },
//                 FieldDataType::I8 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     d = Some(FieldValue::I8(IntConverter::to_int8(bytes)));
//                 },
//                 FieldDataType::I16 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     d = Some(FieldValue::I16(IntConverter::to_int16(bytes)));
//                 },
//                 FieldDataType::I32 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     d = Some(FieldValue::I32(IntConverter::to_int32(bytes)));
//                 },
//                 FieldDataType::I64 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     d = Some(FieldValue::I64(IntConverter::to_int64(bytes)));
//                 },
//                 FieldDataType::I128 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     d = Some(FieldValue::I128(IntConverter::to_int128(bytes)));
//                 },
//                 FieldDataType::F32 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     d = Some(FieldValue::F32(IntConverter::to_f32(bytes)));
//                 },
//                 FieldDataType::F64 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     d = Some(FieldValue::F64(IntConverter::to_f64(bytes)))
//                 },
//                 FieldDataType::String => {
//                     panic!("Not implemented");
//                 },
//                 FieldDataType::Utc => {
//                     panic!("Not implemented");
//                 },
//             }

//             match d {
//                 Some(fv) => values.insert(v.name.clone(), fv),
//                 None => panic!("invalid data type {}", v.data_type),
//             };

//             start = end;
//         }

//         if end < total_bits {
//             return Err(BitCodecError {
//                 error: CodecResultCode::MessageNotFullyParsed,
//                 error_str: format!("The message was not full parsed: Ended at bit {end} where total bits is: {total_bits}")
//             });
//         }
//         // fields.to_vec()
//         // new_fields.to_vec()
//         return Ok("Successfully parsed message: results updated fields".to_owned());
//     }
// }
