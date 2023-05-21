mod fields;
use std::{collections::HashMap};
use fields::{FieldDataType, Field, BitLengthType, FieldValue, FieldType};
use serde::{Serialize, Deserialize};

pub trait IFieldGenerator {
    fn flatten_fields<'a>() -> Vec<&'a Field>;
}

pub struct FieldGenerator {

}

impl IFieldGenerator for FieldGenerator {
    fn flatten_fields<'a>() -> Vec<&'a Field> {
        Vec::new()
    }
}

pub struct Message
{
    pub spec: MessageSpec,
    pub embedded_specs: Vec<MessageSpec>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageSpec {
    pub message_id: i32,
    pub fields: Vec<Field>,
}

impl MessageSpec { }

pub trait IMessageFactory {
    fn new() -> Self;
    fn load(&self);
}

pub struct MessageFactory {
    messages: Option<HashMap<i32, MessageSpec>>,
}

impl MessageFactory {
    pub fn new() -> Self {
        Self {
            messages: None,
        }
    }

    pub fn load(&mut self) {
        let the_data = r#"[{
            "message_id": 1,
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
                "repeating_spec_id": 2
            }]
        }, {
            "message_id": 2,
            "fields": [{
                "name": "field3",
                "data_type": "Byte",
                "field_length": 2,
                "field_length_type": "Bits",
                "field_type": "Standard"
            }]
        }, {
            "message_id": 3,
            "fields": [{
                "name": "field4",
                "data_type": "Byte",
                "field_length": 2,
                "field_length_type": "Bits",
                "field_type": "Standard"
            }]
        }]"#;

        let d: Vec<MessageSpec> = serde_json::from_str(the_data).unwrap();
        if self.messages.is_none() {
            self.messages = Some(HashMap::new());
        }
        for m in d {
            self.messages.as_mut().unwrap().insert(m.message_id, m);
        }
    }

    pub fn fetch(&mut self, message_id: i32) -> Result<Vec<&MessageSpec>, String> {
        // if not found
        if self.messages.is_none() {
            return Err(format!("No messages were loaded, could not find {}", message_id))
        }
        let mut v: Vec<&MessageSpec> = Vec::new();
        let mut selected_message_ids: Vec<i32> = Vec::new();

        if let Some(message) = self._fetch_message(&message_id) {
            v.push(message);
        } else {
            return Err(format!("No message '{}' was found", message_id));
        }
        // self._current_message_ids.push(message_id);
        let mut no_more_specs = false;
        let mut iteration_depth = 0;
        let max_iteration_depth = 1000;
        // This gives us about 1000 embedded specs for each message, that should more than enough
        // instead of recursion, we're using a while loop with a max depth.
        // This essentially flattens embedded specs into a single list of message ids.
        while no_more_specs == false {
            if iteration_depth == max_iteration_depth {
                break;
            }
            if let Ok(spec_id) = self._fetch_next_spec_id(&v, &selected_message_ids) {
                if let Some(message) = self._fetch_message(&spec_id) {
                    selected_message_ids.push(spec_id);
                    v.push(message);
                } else {
                    return Err(format!("An internal message id was specified but no matching message ids were found: {}", spec_id));
                }
            } else {
                no_more_specs = true;
            }
            iteration_depth += 1;
        }

        return Ok(v);
    }

    fn _fetch_message<'a>(&'a self, message_id: &i32) -> Option<&'a MessageSpec> {
        let m = self.messages.as_ref().unwrap();
        if m.contains_key(message_id) {
            let c = m.get(message_id).unwrap();
            return Some(c);
            // messages.push(c);
        } else {
            None
        }
    }

    fn _fetch_next_spec_id<'a>(&'a self, messages: &Vec<&'a MessageSpec>, selected_message_ids: &Vec<i32>) -> Result<i32, &str> {
        for m in messages {
            for f in &m.fields {
                if let Some(spec_id) = f.repeating_spec_id {
                    if !selected_message_ids.contains(&spec_id) {
                        return Ok(spec_id);
                    }
                }
            }
        }

        // error message exists only for an if condition
        return Err("No more to be found");
    }
}

// pub trait IMessage {
//     pub fields: Vec<&'a mut Field>,
//     pub message_id: i32,
//     pub encoded: Vec<u8>,
// }
// pub struct Message<'a> {
//     pub fields: Vec<&'a mut Field>,
//     pub message_id: i32,
//     pub encoded: Vec<u8>,
// }

// impl<'a> std::ops::Deref for Message<'a> {
//     type Target = Vec<&'a mut dyn IField>;

//     fn deref(&self) -> &Self::Target {
//         &self.fields
//     }
// }

// impl<'a> Message<'a> {
//     fn push(&mut self, field: &'a mut Field) {
//         self.fields.push(field);
//     }
// }

// pub trait IMessageFactory {
//     // fn fetch<'a>(message_id: u32) -> Message<'a>;
//     fn fetch(message_id: u32) -> Box<dyn Message>;
// }

// pub struct MessageFactory<'a> {
//     hash: HashMap<u32, Message<'a>>,
// }

// impl<'a> MessageFactory<'a> {
    
// }

// impl<'a> IMessageFactory for MessageFactory<'a> {
//     fn fetch<'b>(message_id: u32) -> Box<Message> {
//         let mut f1 = Field {
//             name: "field1".to_owned(),
//             data_type: FieldDataType::Byte,
//             field_length: 2,
//             field_length_type: BitLengthType::Bits,
//             field_type: FieldType::Standard,
//             repeating_length_name: None,
//             value: None
//         };

//         let mut f2 = Field {
//             name: "field2".to_owned(),
//             data_type: FieldDataType::Byte,
//             field_length: 2,
//             field_length_type: BitLengthType::Bits,
//             field_type: FieldType::Standard,
//             repeating_length_name: None,
//             value: None
//         };
    
//         let mut f3 = Field {
//             name: "field3".to_owned(),
//             data_type: FieldDataType::I16,
//             field_length: 4,
//             field_length_type: BitLengthType::Bits,
//             field_type: FieldType::Standard,
//             repeating_length_name: None,
//             value: None
//         };
//         let mut fields2: Vec<&mut Field> = Vec::new();
//         fields2.push(&mut f1 as &mut Field);
//         fields2.push(&mut f2 as &mut Field);
//         fields2.push(&mut f3 as &mut Field);
        
//         return Some(Message { fields: fields2, message_id: 1, encoded: Vec::new() });
//     }
// }


fn main() {
    let mut mf = MessageFactory { messages: None };
    mf.load();
    // if let Ok(m) = mf.fetch(1) {
    //     println!("{:?}", m);
    // } else {
    //     println!("An error ocurred");
    // }
    match mf.fetch(1) {
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

// fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
//     v.try_into()
//         .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
// }

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
//     fn decode<'a>(&self, data: &'a Vec<u8>, fields: &'a mut Vec<&'a mut Field>) -> Result<String, BitCodecError>;
// }

// pub trait IBitCodec {
//     fn decode<'a>(&self, data: &'a Vec<u8>, fields: &'a mut Vec<&'a dyn IField>) -> Result<&'a Vec<&'a dyn IField>, BitCodecError>;
//     // fn encode<T>(&self, fields: &Vec<Field<T>>) -> Vec<u8>;
// }

// struct BitCodec {}

// impl BitCodec {
//     // private methods here
//     fn _repeater<'a>(&self, data: &'a Vec<u8>, fields: &'a mut Vec<&'a mut Field>) -> Option<String> {
//         None
//     }
// }

// impl IBitCodec for BitCodec {
//     fn decode<'a>(&self, data: &'a Vec<u8>, fields: &'a mut Vec<&'a mut Field>)
//         -> Result<String, BitCodecError>
//     {
//         // let mut f1 = &mut fields[0];
//         // f1.set_value(FieldValue::Byte(3));
//         // let mut new_fields = fields.to_vec();
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
//             match v.data_type {
//                 FieldDataType::Byte => {
//                     let bits = get_bits_left_pad(data, start, end-1);
//                     let fv = FieldValue::Byte(bits[0]);
//                     v.set_value(fv);
//                 }
//                 FieldDataType::U16 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 2);
//                     v.set_value(FieldValue::U16(IntConverter::to_uint16(bytes)))
//                 },
//                 FieldDataType::U32 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 4);
//                     v.set_value(FieldValue::U32(IntConverter::to_uint32(bytes)))
//                 },
//                 FieldDataType::U64 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     v.set_value(FieldValue::U64(IntConverter::to_uint64(bytes)))
//                 },
//                 FieldDataType::U128 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     v.set_value(FieldValue::U128(IntConverter::to_uint128(bytes)))
//                 },
//                 FieldDataType::I8 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     v.set_value(FieldValue::I8(IntConverter::to_int8(bytes)))
//                 },
//                 FieldDataType::I16 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     v.set_value(FieldValue::I16(IntConverter::to_int16(bytes)))
//                 },
//                 FieldDataType::I32 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     v.set_value(FieldValue::I32(IntConverter::to_int32(bytes)))
//                 },
//                 FieldDataType::I64 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     v.set_value(FieldValue::I64(IntConverter::to_int64(bytes)))
//                 },
//                 FieldDataType::I128 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     v.set_value(FieldValue::I128(IntConverter::to_int128(bytes)))
//                 },
//                 FieldDataType::F32 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     v.set_value(FieldValue::F32(IntConverter::to_f32(bytes)))
//                 },
//                 FieldDataType::F64 => {
//                     let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     v.set_value(FieldValue::F64(IntConverter::to_f64(bytes)))
//                 },
//                 FieldDataType::String => {
//                     panic!("Not implemented");
//                     // let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     v.set_value(FieldValue::String("".to_owned()));
//                 },
//                 FieldDataType::Utc => {
//                     panic!("Not implemented");
//                     // let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
//                     v.set_value(FieldValue::Utc(Utc::now()))
//                 },
//             }
//             // if v.get_field_length_type() == BitLengthType::Bits {
//             //     // v.value = Some(0_i32)
//             // }
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
