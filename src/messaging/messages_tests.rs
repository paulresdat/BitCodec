#[cfg(test)]
mod messages_tests {
    use std::{collections::{
        hash_map::DefaultHasher
    },
    hash::{
        Hash,
        Hasher
    }};

    use crate::{
        messaging::messages::{
            MessageSpec,
            // Message,
            MessageFactory,
            LoadFromJson,
            MessageUniqueIdQuery, MessageVersionQuery
        },
        fields::{FieldDataType, FieldType, BitLengthType}
    };

    #[test]
    fn unique_id_is_given_if_specified() {
        let mut m = MessageSpec::new(1, 1);
        m.unique_id = Some("TESTING 123".to_string());
        assert_eq!("TESTING 123", m.get_unique_id());
    }

    #[test]
    fn hash_code_uniqueness_given_for_messages() {
        let m1 = MessageSpec::new(1, 1);
        let m2 = MessageSpec::new(1, 1);
        let mut hash = DefaultHasher::new();
        m1.hash(&mut hash);
        let id1 = hash.finish();
        let mut hash = DefaultHasher::new();
        m2.hash(&mut hash);
        let id2 = hash.finish();
        println!("{} {}", id1, id2);
        assert_eq!(id1, id2);
    }

    #[test]
    fn hash_code_uniqueness_given_for_messages2() {
        let m1 = MessageSpec::new(1, 1);
        let m2 = MessageSpec::new(1, 2);
        let mut hash = DefaultHasher::new();
        m1.hash(&mut hash);
        let id1 = hash.finish();
        let mut hash = DefaultHasher::new();
        m2.hash(&mut hash);
        let id2 = hash.finish();
        println!("{} {}", id1, id2);
        assert_ne!(id1, id2);
    }

    #[test]
    fn equality_of_message_spec() {
        let m1 = MessageSpec::new(1, 1);
        let m2 = MessageSpec::new(1, 2);
        assert!(m1 != m2);
    }

    #[test]
    fn messages_can_be_loaded_from_json() {
        let s = r#"
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
                    "field_length": 1,
                    "field_length_type": "Bits",
                    "field_type": "Standard"
                }]
            }
        ]
        "#;

        let mut m = MessageFactory::new();
        let n = LoadFromJson{ json_data: s.to_string() };
        m.load(n);

        let q = MessageUniqueIdQuery { unique_id: "1.1".to_string() };
        match m.fetch(q) {
            Ok(m) => {
                let e = &m.message_spec;
                assert_eq!(0, m.embedded_specs.len());
                assert_eq!(2, e.fields.len());

                assert_eq!(1, e.message_id);
                assert_eq!(1, e.version_id);
                let fields = &e.fields;
                // field one
                assert_eq!("field1", fields[0].name);
                assert_eq!(FieldDataType::Byte, fields[0].data_type);
                assert_eq!(2, fields[0].field_length);
                assert_eq!(BitLengthType::Bits, fields[0].field_length_type);
                assert_eq!(FieldType::Standard, fields[0].field_type);
                assert_eq!(None, fields[0].repeating_spec_id);
                assert_eq!(None, fields[0].repeating_length_name);

                assert_eq!("field2", fields[1].name);
                assert_eq!(FieldDataType::Byte, fields[1].data_type);
                assert_eq!(1, fields[1].field_length);
                assert_eq!(BitLengthType::Bits, fields[1].field_length_type);
                assert_eq!(FieldType::Standard, fields[1].field_type);
                assert_eq!(None, fields[1].repeating_spec_id);
                assert_eq!(None, fields[1].repeating_length_name);
            }
            Err(s) => {
                println!("{}", s);
                assert!(false);
            }
        }

        let q2 = MessageVersionQuery {
            message_id: 1,
            version_id: 1
        };

        match m.fetch(q2) {
            Ok(m) => {
                let e = &m.message_spec;
                assert_eq!(0, m.embedded_specs.len());
                assert_eq!(2, e.fields.len());
                assert_eq!(1, e.message_id);
                assert_eq!(1, e.version_id);
            }
            Err(s) => {
                println!("{}", s);
                assert!(false);
            }
        }
    }

    #[test]
    fn repeating_spec_ids_are_returned_with_a_message() {
        let s = r#"
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
                    "field_length": 1,
                    "field_length_type": "Bits",
                    "field_type": "Repeating",
                    "repeating_spec_id": "2.1"
                }]
            },
            {
                "message_id": 2,
                "version_id": 1,
                "fields": [{
                    "name": "field1",
                    "data_type": "Byte",
                    "field_length": 8,
                    "field_length_type": "Bits",
                    "field_type": "Standard"
                }]
            }
        ]
        "#;

        let mut m = MessageFactory::new();
        let n = LoadFromJson{ json_data: s.to_string() };
        m.load(n);

        let q = MessageUniqueIdQuery { unique_id: "1.1".to_string() };
        match m.fetch(q) {
            Ok(m) => {
                let e = &m.message_spec;
                println!("{:?}", m.embedded_specs);
                assert_eq!(1, m.embedded_specs.len());
                assert_eq!(2, e.fields.len());
                assert_eq!(1, e.message_id);
                assert_eq!(1, e.version_id);
                assert_eq!(2, m.embedded_specs[0].message_id);
                assert_eq!(1, m.embedded_specs[0].version_id);
            }
            Err(s) => {
                println!("{}", s);
                assert!(false);
            }
        }
    }
}
