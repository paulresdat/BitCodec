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
