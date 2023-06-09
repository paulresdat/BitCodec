use core::fmt;
use std::hash::Hash;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub name: String,
    pub data_type: FieldDataType,
    pub field_length: u32,
    pub field_length_type: BitLengthType,
    pub field_type: FieldType,
    pub repeating_length_name: Option<String>,
    pub repeating_spec_id: Option<String>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum FieldDataType {
    Bool,
    Byte, // U8
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    // not sure yet how to handle this
    String,
    Utc,
}

impl Hash for FieldDataType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

#[derive(Debug)]
pub enum FieldValue {
    Bool(bool),
    Byte(u8), // U8
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    F64(f64),
    // not sure yet how to handle this
    String(String),
    Utc(DateTime<Utc>),

    // for repeating fields
    VecU8(Vec<u8>),
    VecU16(Vec<u16>),
    VecU32(Vec<u32>),
    VecU64(Vec<u64>),
    VecU128(Vec<u128>),
    VecI8(Vec<i8>),
    VecI16(Vec<i16>),
    VecI32(Vec<i32>),
    VecI64(Vec<i64>),
    VecI128(Vec<i128>),
    VecF32(Vec<f32>),
    VecF64(Vec<f64>),
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum FieldType {
    Standard,
    Repeating,
}

impl Hash for FieldType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl fmt::Display for FieldDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FieldDataType::Bool => write!(f, "FieldDataType::Bool"),
            FieldDataType::Byte => write!(f, "FieldDataType::Byte"),
            FieldDataType::U16 => write!(f, "FieldDataType::U16"),
            FieldDataType::U32 => write!(f, "FieldDataType::U32"),
            FieldDataType::U64 => write!(f, "FieldDataType::U64"),
            FieldDataType::U128 => write!(f, "FieldDataType::U128"),
            FieldDataType::I8 => write!(f, "FieldDataType::I8"),
            FieldDataType::I16 => write!(f, "FieldDataType::I16"),
            FieldDataType::I32 => write!(f, "FieldDataType::I32"),
            FieldDataType::I64 => write!(f, "FieldDataType::I64"),
            FieldDataType::I128 => write!(f, "FieldDataType::I128"),
            FieldDataType::F32 => write!(f, "FieldDataType::F32"),
            FieldDataType::F64 => write!(f, "FieldDataType::F64"),
            FieldDataType::String => write!(f, "FieldDataType::String"),
            FieldDataType::Utc => write!(f, "FieldDataType::Utc"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum BitLengthType {
    Bits,
    Bytes,
    None
}

impl Hash for BitLengthType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}


impl Hash for Field {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.data_type.hash(state);
        self.field_length.hash(state);
        self.field_length_type.hash(state);
        self.field_type.hash(state);
        self.repeating_length_name.hash(state);
        self.repeating_spec_id.hash(state);
    }
}

impl Field {
//     fn get_data_type(&self) -> FieldDataType {
//         self.data_type
//     }

//     fn get_field_length(&self) -> u32 {
//         self.field_length
//     }

//     fn get_field_length_type(&self) -> BitLengthType {
//         self.field_length_type
//     }

    // pub fn set_value(&mut self, v: FieldValue) {
    //     self.value = Some(v);
    // }
}

// #[derive(Clone)]
// pub enum Fields {
//     Fieldu8(Field<u8>),
//     Fieldu16(Field<u16>),
//     Fieldu32(Field<u32>),
//     Fieldu64(Field<u64>),

//     Fieldi8(Field<i8>),
//     Fieldi16(Field<i16>),
//     Fieldi32(Field<i32>),
//     Fieldi64(Field<i64>),

//     Fieldf32(Field<f32>),
//     Fieldf64(Field<f64>),

//     FieldString(Field<String>),
//     FieldUtc(Field<Utc>),
// }
