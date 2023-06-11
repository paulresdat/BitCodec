mod fields;
pub mod encoder;
mod messaging;
mod benching;
use std::collections::HashMap;

use fields::FieldValue;
use messaging::{messages::{MessageFactory, LoadFromJson, MessageUniqueIdQuery}};
use encoder::{bitcodec::{BitCodec, IBitCodec}};

fn main() {
    let data = r#"
    [
        {
            "message_id": 1,
            "version_id": 1,
            "fields": [
                {
                    "name": "field1",
                    "data_type": "Byte",
                    "field_length": 2,
                    "field_length_type": "Bits",
                    "field_type": "Standard"
                },
                {
                    "name": "field2",
                    "data_type": "Bool",
                    "field_length": 1,
                    "field_length_type": "Bits",
                    "field_type": "Standard"
                },
                {
                    "name": "field3",
                    "data_type": "U16",
                    "field_length": 2,
                    "field_length_type": "Bits",
                    "field_type": "Standard"
                },
                {
                    "name": "field4",
                    "data_type": "U32",
                    "field_length": 2,
                    "field_length_type": "Bits",
                    "field_type": "Standard"
                },
                {
                    "name": "field5",
                    "data_type": "Bool",
                    "field_length": 1,
                    "field_length_type": "Bits",
                    "field_type": "Repeating"
                }
            ]
        }
    ]
    "#;

    let mut m = MessageFactory::new();
    let n = LoadFromJson{ json_data: data.to_string() };
    m.load(n);

    let q = MessageUniqueIdQuery { unique_id: "1.1".to_string() };
    let m2 = m.fetch(q).unwrap();

    let bc = BitCodec::new();
    let data: Vec<u8> = vec![0b0011_1111];
    let mut values: HashMap<String, FieldValue> = HashMap::new();
    if let Ok(_) = bc.decode(&data, &m2, &mut values) {
        print!("Successfully decoded");
        println!("{:?}", values);
    }
}
