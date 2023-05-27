use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::fields::Field;

pub struct MessageVersionQuery {
    pub message_id: i32,
    pub version_id: i32,
}

pub struct MessageUniqueIdQuery {
    pub unique_id: String
}

pub trait IMessageVerionQuery: Into<String> {
    
}

impl IMessageVerionQuery for MessageVersionQuery { }
impl IMessageVerionQuery for MessageUniqueIdQuery { }

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

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageSpec {
    pub message_id: i32,
    pub version_id: i32,
    pub unique_id: Option<String>,
    _generated_unique_id: Option<String>,
    pub fields: Vec<Field>,
}

impl MessageSpec {
    #[allow(unused)]
    pub fn new(message_id: i32, version_id: i32) -> Self {
        Self {
            message_id: message_id,
            version_id: version_id,
            unique_id: None,
            _generated_unique_id: None,
            fields: Vec::new()
        }
    }

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

pub enum MessageDataSourceEnum {
    Json,
}
pub struct LoadFromJson {
    pub json_data: String
}

pub trait IMessageDataSource {
    fn source_type(&self) -> MessageDataSourceEnum;
    fn source_data(&self) -> String;
}

impl IMessageDataSource for LoadFromJson {
    fn source_data(&self) -> String {
        self.json_data.to_owned()
    }

    fn source_type(&self) -> MessageDataSourceEnum {
        MessageDataSourceEnum::Json
    }
}

pub struct Message<'a> {
    pub message_spec: &'a MessageSpec,
    pub embedded_specs: Vec<&'a MessageSpec>,
}

pub struct MessageFactory<'a> {
    messages: Option<HashMap<String, MessageSpec>>,
    _cached_fetched_messages: HashMap<String, Message<'a>>,
}

impl<'a> MessageFactory<'a> {
    pub fn new(hash_map: HashMap<String, Message<'a>>) -> Self {
        Self {
            messages: None,
            _cached_fetched_messages: hash_map,
        }
    }

    pub fn load<T>(&mut self, the_data: T) where T: IMessageDataSource {
        let source_type = the_data.source_type();
        match source_type {
            MessageDataSourceEnum::Json => {
                let data = the_data.source_data();
                let d: Vec<MessageSpec> = serde_json::from_str(data.as_str()).unwrap();
                if self.messages.is_none() {
                    self.messages = Some(HashMap::new());
                }
                for mut m in d {
                    self.messages.as_mut().unwrap().insert(m.get_unique_id(), m);
                }
            }
        }
    }

    pub fn fetch<T>(&mut self, unique_id_query: T) -> Result<&Message, String>
        where T: IMessageVerionQuery {
        let unique_id = unique_id_query.into() as String;
        
        // return the cached value, speeding up the process
        if self._cached_fetched_messages.contains_key(&unique_id) {
            return Ok(&self._cached_fetched_messages[&unique_id]);
        }

        if self.messages.is_none() {
            return Err(format!("No messages were loaded, could not find {}", unique_id))
        }

        let mut v: Vec<&MessageSpec> = Vec::new();
        let mut selected_message_ids: Vec<String> = Vec::new();
        let mut parent_message: Option<&MessageSpec> = None;
        if let Some(message) = self._fetch_message(&unique_id) {
            parent_message = Some(message);
            // for embedded reasons, go ahead and add it
            v.push(message);
        } else {
            return Err(format!("No message '{}' was found", unique_id));
        }

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

        // remove the parent spec
        v.remove(0);

        // now return the full message with all its embedded message specs
        let full_message = Message {
            message_spec: parent_message.unwrap(),
            embedded_specs: v,
        };
        // this is pain.  How in the world are you able to cache things then??
        let unique_id = parent_message.unwrap().get_unique_id();
        self._cached_fetched_messages.insert(unique_id, full_message);
        return Ok(&self._cached_fetched_messages[&unique_id]);
    }

    fn _fetch_message<'b>(&'b self, spec_id: &String) -> Option<&'b MessageSpec> {
        let m = self.messages.as_ref().unwrap();
        if m.contains_key(spec_id) {
            let c = m.get(spec_id).unwrap();
            return Some(c);
            // messages.push(c);
        } else {
            None
        }
    }

    fn _fetch_next_spec_id<'b>(&'b self, messages: &Vec<&'a MessageSpec>, selected_message_ids: &Vec<String>) -> Result<String, &str> {
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
