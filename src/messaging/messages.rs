use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::fields::Field;

pub trait IFieldGenerator {
    fn flatten_fields<'a>(messages: &'a Vec<&'a Message>) -> Vec<Field>;
}

pub struct FieldGenerator {

}

pub struct MessageVersionQuery {
    pub message_id: i32,
    pub version_id: i32,
}

pub struct MessageUniqueIdQuery {
    pub unique_id: String
}

// pub trait MessageVersionQuery {
//     fn get_unique_id(&self) -> String;
// }

// impl MessageVersionQuery for MessageVersion {
//     fn get_unique_id(&self) -> String {
//         format!("{}.{}", self.message_id, self.version_id)
//     }
// }

// impl MessageVersionQuery for MessageUniqueIdQuery {
//     fn get_unique_id(&self) -> String {
//         self.unique_id
//     }
// }

// pub trait UniqueMessageId {
//     type DataType: MessageVersionQuery;
//     fn get_unique_id(self) -> Self::DataType;
// }

pub trait IMessageVerionQuery: Into<String> {
    
}

impl IMessageVerionQuery for MessageVersionQuery { }

impl Into<String> for MessageVersionQuery {
    fn into(self) -> String {
        format!("{}.{}", self.message_id, self.version_id)
    }
}

impl Into<String> for MessageUniqueIdQuery {
    fn into(self) -> String {
        self.unique_id
    }
}

// impl UniqueMessageId for MessageVersion {
//     type DataType: MessageVersionQuery;

//     fn testing(self) -> MessageVersionQuery {
//         todo!()
//     }
// }

// impl UniqueMessageId for MessageVersion {
//     type DataType;
//     fn testing(self) -> Self::DataType {
//         todo!()
//     }
// }

impl IFieldGenerator for FieldGenerator {
    fn flatten_fields<'a>(messages: &'a Vec<&'a Message>) -> Vec<Field> {
        let mut v = Vec::new();

        for m in messages {
            for f in &m.spec.fields {
                let s = f.clone();
                v.push(s);
            }
        }

        return v;
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
    pub version_id: i32,
    pub unique_id: Option<String>,
    _generated_unique_id: Option<String>,
    pub fields: Vec<Field>,
}

impl MessageSpec {
    pub fn get_unique_id(&mut self) -> String {
        match &self.unique_id {
            Some(v) => return v.to_string(),
            None => {
                match &self._generated_unique_id {
                    Some(v) => return v.to_string(),
                    None => {
                        let s = format!("{}.{}", self.message_id, self.version_id).to_string();
                        self._generated_unique_id = Some(s.clone());
                        return s;
                    }
                };
            }
        };
    }
}


pub struct MessageFactory {
    messages: Option<HashMap<String, MessageSpec>>,
}

impl MessageFactory {
    pub fn new() -> Self {
        Self {
            messages: None,
        }
    }

    pub fn load(&mut self, the_data: &String) {
        let d: Vec<MessageSpec> = serde_json::from_str(the_data).unwrap();
        if self.messages.is_none() {
            self.messages = Some(HashMap::new());
        }
        for mut m in d {
            self.messages.as_mut().unwrap().insert(m.get_unique_id(), m);
        }
    }

    pub fn fetch<T>(&mut self, unique_id_query: T) -> Result<Vec<&MessageSpec>, String>
        where T: IMessageVerionQuery {
        let unique_id = unique_id_query.into() as String;

        if self.messages.is_none() {
            return Err(format!("No messages were loaded, could not find {}", unique_id))
        }

        let mut v: Vec<&MessageSpec> = Vec::new();
        let mut selected_message_ids: Vec<String> = Vec::new();

        if let Some(message) = self._fetch_message(&unique_id) {
            v.push(message);
        } else {
            return Err(format!("No message '{}' was found", unique_id));
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

    fn _fetch_message<'a>(&'a self, spec_id: &String) -> Option<&'a MessageSpec> {
        let m = self.messages.as_ref().unwrap();
        if m.contains_key(spec_id) {
            let c = m.get(spec_id).unwrap();
            return Some(c);
            // messages.push(c);
        } else {
            None
        }
    }

    fn _fetch_next_spec_id<'a>(&'a self, messages: &Vec<&'a MessageSpec>, selected_message_ids: &Vec<String>) -> Result<String, &str> {
        for m in messages {
            for f in &m.fields {
                if let Some(spec_id) = &f.repeating_spec_id {
                    if !selected_message_ids.contains(&spec_id) {
                        return Ok(spec_id.clone());
                    }
                }
            }
        }

        // error message exists only for an if condition
        return Err("No more to be found");
    }
}

// #[derive(Debug)]
// pub struct Message {
//     pub header: MessageHeader,
//     pub body: Option<Vec<u8>>,
//     // body_t: T,
// }

// #[derive(Debug)]
// pub struct MessageHeader {
//     pub message_id: u32,
//     pub unix_time: i64,
// }

// impl Message {
//     // priv unix_datestamp: Options<DateTime<Utc>>;
//     pub fn date(&self) -> DateTime<Utc> {
//         return Utc.timestamp_opt(self.header.unix_time, 0).unwrap()
//     }
// }

// pub struct MessageBuilder {
//     header_message_id: u32,
//     header_unix_time: i64,
//     body_data: Option<Vec<u8>>,
//     // pub message: Message
// }

// impl MessageBuilder {
//     pub fn new() -> MessageBuilder {
//         MessageBuilder {
//             header_message_id: 0,
//             header_unix_time: 0,
//             body_data: None
//         }
//     }

//     pub fn default(&mut self) -> &mut MessageBuilder {
//         self.header_message_id = 0;
//         self.header_unix_time = 0;
//         self.body_data = None;
//         self
//     }

//     pub fn with_message_id(&mut self, message_id: u32) -> &mut MessageBuilder {
//         self.header_message_id = message_id;
//         self
//     }

//     pub fn with_unix_time(&mut self, unix_time: i64) -> &mut MessageBuilder {
//         self.header_unix_time = unix_time;
//         self
//     }

//     pub fn with_body(&mut self, byte_data: Vec<u8>) -> &mut MessageBuilder {
//         self.body_data = Some(byte_data);
//         self
//     }

//     pub fn build(&self) -> Message {
//         Message {
//             header: MessageHeader {
//                 message_id: self.header_message_id,
//                 unix_time: self.header_unix_time,
//             },
//             body: Some(self.body_data.as_ref().unwrap().clone()),
//         }
//     }
// }
