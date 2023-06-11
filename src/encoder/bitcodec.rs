use std::collections::HashMap;

use crate::{fields::{Field, FieldValue, FieldDataType}, messaging::messages::Message};

use super::bitwise_ops::{get_bits_left_pad, get_bits_byte_pad_left, IntConverter};


pub struct BitCodecError {
    error: CodecResultCode,
    error_str: String
}

pub enum CodecResultCode
{
    EndExceedsBitLength,
    MessageNotFullyParsed,
}

pub trait IBitCodec {
    fn decode<'a>(&self, data: &'a Vec<u8>, message: &'a Message, values: &'a mut HashMap<String, FieldValue>)
        -> Result<String, BitCodecError>;
}

pub struct BitCodec {

}

impl BitCodec {
    pub fn new() -> Self {
        Self { }
    }
    fn _get_value(&self, f: &Field, data: &Vec<u8>, start: usize, end: usize) -> Result<FieldValue, String> {
        let mut d: Option<FieldValue> = None;
        match f.data_type {
            FieldDataType::Bool => {
                let bits = get_bits_left_pad(data, start, end);
                d = Some(FieldValue::Bool(bits[0] == 1));
            }
            FieldDataType::Byte => {
                let bits = get_bits_left_pad(data, start, end-1);
                d = Some(FieldValue::Byte(bits[0]));
            }
            FieldDataType::U16 => {
                let bytes = get_bits_byte_pad_left(data, start, end-1, 2);
                d = Some(FieldValue::U16(IntConverter::to_uint16(bytes)));
            },
            FieldDataType::U32 => {
                let bytes = get_bits_byte_pad_left(data, start, end-1, 4);
                d = Some(FieldValue::U32(IntConverter::to_uint32(bytes)));
            },
            FieldDataType::U64 => {
                let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
                d = Some(FieldValue::U64(IntConverter::to_uint64(bytes)));
            },
            FieldDataType::U128 => {
                let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
                d = Some(FieldValue::U128(IntConverter::to_uint128(bytes)));
            },
            FieldDataType::I8 => {
                let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
                d = Some(FieldValue::I8(IntConverter::to_int8(bytes)));
            },
            FieldDataType::I16 => {
                let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
                d = Some(FieldValue::I16(IntConverter::to_int16(bytes)));
            },
            FieldDataType::I32 => {
                let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
                d = Some(FieldValue::I32(IntConverter::to_int32(bytes)));
            },
            FieldDataType::I64 => {
                let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
                d = Some(FieldValue::I64(IntConverter::to_int64(bytes)));
            },
            FieldDataType::I128 => {
                let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
                d = Some(FieldValue::I128(IntConverter::to_int128(bytes)));
            },
            FieldDataType::F32 => {
                let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
                d = Some(FieldValue::F32(IntConverter::to_f32(bytes)));
            },
            FieldDataType::F64 => {
                let bytes = get_bits_byte_pad_left(data, start, end-1, 8);
                d = Some(FieldValue::F64(IntConverter::to_f64(bytes)))
            },
            FieldDataType::String => {
                panic!("Not implemented");
            },
            FieldDataType::Utc => {
                panic!("Not implemented");
            },
        }

        if let Some(v) = d {
            return Ok(v);
        }

        return Err("None type found for value, not implemented likely".to_owned());
    }
}

impl IBitCodec for BitCodec {
    fn decode<'a>(&self, data: &'a Vec<u8>, message: &'a Message, values: &'a mut HashMap<String, FieldValue>)
        -> Result<String, BitCodecError>
    {
        let mut start: usize = 0;
        let mut end: usize = 0;
        let total_bits = data.len() * 8;

        for f in &message.message_spec.fields {
            end = start + (f.field_length as usize);

            // unreversed raw bits
            if end > total_bits {
                let bits_left = total_bits - start;
                let overflow_count = end - total_bits;
                return Err(BitCodecError {
                    error: CodecResultCode::EndExceedsBitLength,
                    error_str: format!("The end exceeds what bits are left. Bits left: {bits_left}, Overflow: {overflow_count} bits")
                });
            }
            // println!("Start: {}, End: {}", start, end);
            let r = self._get_value(f, data, start, end);
            match r {
                Ok(fv) => values.insert(f.name.clone(), fv),
                Err(_) => panic!("invalid data type {}", f.data_type),
            };

            start = end;
        }

        if end < total_bits {
            return Err(BitCodecError {
                error: CodecResultCode::MessageNotFullyParsed,
                error_str: format!("The message was not full parsed: Ended at bit {end} where total bits is: {total_bits}")
            });
        }

        return Ok("Successfully parsed message: results updated fields".to_owned());
    }
}
