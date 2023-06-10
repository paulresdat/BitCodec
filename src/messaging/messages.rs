use std::{hash::{Hash, Hasher}, collections::{hash_map::DefaultHasher, HashMap}, vec};
use serde::{Serialize, Deserialize};
use crate::fields::Field;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageSpec {
    pub message_id: i32,
    pub version_id: i32,
    pub unique_id: Option<String>,
    _generated_unique_id: Option<String>,
    pub fields: Vec<Field>,
}

impl Hash for MessageSpec {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.message_id.hash(state);
        self.version_id.hash(state);
        self.unique_id.hash(state);
        self._generated_unique_id.hash(state);
        self.fields.hash(state);
    }
}

impl PartialEq for MessageSpec {
    fn eq(&self, other: &Self) -> bool {
        self.message_id == other.message_id && 
        self.version_id == other.version_id && 
        self.unique_id == other.unique_id
        // self._generated_unique_id == other._generated_unique_id
        // self.fields == other.fields
    }
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

    pub fn get_unique_id(&self) -> String {
        if let Some(id) = &self.unique_id {
            return id.to_owned();
        }
        return "".to_string();
    }

    // pub fn assign_unique_id(message: &mut MessageSpec) {
    //     match message.unique_id {
    //         None => {
    //             let s = format!("{}.{}", message.message_id, message.version_id).to_string();
    //             message.unique_id = Some(s);
    //         }
    //         _ => (),
    //     }
    // }
}

#[derive(Clone)]
pub struct Message {
    pub message_spec: Box<MessageSpec>,
    pub embedded_specs: Vec<MessageSpec>,
    // _cached_hashcode: u64,
}

impl Message {
    fn calculate_hash(&self, val: &Self) -> u64 {
        let mut s = DefaultHasher::new();
        val.hash(&mut s);
        s.finish()
    }
}

impl Hash for Message {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.message_spec.hash(state);
        self.embedded_specs.hash(state);
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.calculate_hash(self) == self.calculate_hash(other)
    }
}

pub struct MessageFactory {
    messages: Option<HashMap<String, Box<MessageSpec>>>,
    _cached_messages: HashMap<String, Box<Message>>,
    // _cached_fetched_messages: CachedMessages<'a>
}

impl MessageFactory {
    pub fn new() -> Self {
        Self {
            messages: None,
            _cached_messages: HashMap::new(),
            // _cached_fetched_messages: CachedMessages::new(hash_map),
        }
    }

    fn new_message(&mut self, index: String, message: Message) {
        self._cached_messages.insert(index, Box::new(message));
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
                for mut message in d {
                    self.assign_unique_id(&mut message);
                    let unique_id: String = message.get_unique_id();
                    self.messages.as_mut().unwrap().insert(unique_id, Box::new(message));
                }
            }
        }

        // compile messages here instead maybe?
        let mut selected_message_ids: Vec<String> = Vec::new();
        let referrable_messages = self.messages.as_ref().unwrap();
        let mut vec_of_messages: Vec<&Box<MessageSpec>> = Vec::new();
        for v in referrable_messages.values() {
            vec_of_messages.push(v);
        }

        for m in referrable_messages.values() {
            let mut v: Vec<MessageSpec> = Vec::new();
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
                if let Ok(spec_id) = self._fetch_next_spec_id(&vec_of_messages, &selected_message_ids) {
                    if let Some(message) = self._fetch_message(&spec_id) {
                        selected_message_ids.push(spec_id);
                        v.push(message.clone());
                    } else {
                        panic!("An internall message id of '{}' was found but but no matching unique ids were found", spec_id);
                    }
                } else {
                    no_more_specs = true;
                }
                iteration_depth += 1;
            }

            if let Some(id) = &m.unique_id {
                let b = Box::new(Message {
                    message_spec: m.clone(),
                    embedded_specs: v,
                });
                self._cached_messages.insert(id.to_string(), b);
            }
        }
    }

    fn _fetch_next_spec_id<'b>(&'b self, messages: &Vec<&Box<MessageSpec>>, selected_message_ids: &Vec<String>)
        -> Result<String, &str> {
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

    // this must live outside, mutable computed properties are a pain to work with
    fn assign_unique_id(&self, message: &mut MessageSpec) {
        match message.unique_id {
            None => {
                let s = format!("{}.{}", message.message_id, message.version_id).to_string();
                message.unique_id = Some(s);
            }
            _ => (),
        }
    }

    pub fn fetch<T>(&mut self, unique_id_query: T) -> Result<&Message, String>
        where T: IMessageVerionQuery {
        let unique_id = unique_id_query.into() as String;
        
        // return the cached value, speeding up the process
        if self._cached_messages.contains_key(&unique_id) {
            let m = &self._cached_messages[&unique_id];
            return Ok(m);
        }
        return Err("Unknown message type".to_string());
    }

    fn _fetch_message<'b>(&'b self, spec_id: &String) //, messages: &'b HashMap<String, Box<MessageSpec>>)
        -> Option<&'b MessageSpec> {
        if let Some(ms) = &self.messages {
            let id = ms.get(spec_id);
            let c = id.unwrap();
            return Some(c);
        }
        None
    }
}

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
