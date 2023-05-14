extern crate serde;
use serde_json::{Value};

use crate::models::message::Message;

pub trait Mapper{
    fn to_message(&self, message: &str) -> Message;
}

pub struct RabbitMapper {}

impl Mapper for RabbitMapper {
    fn to_message(&self, message: &str) -> Message {
        let v: Value = match serde_json::from_str(message) {
            Ok(r) =>  r,
            Err(e) => panic!("JSON decode error: {}", e),
        };

        return Message{
            id: 0,
            consumer: "rabbit".to_string(),
            data: v["data"].to_string(),
            created_at: String::from(""),
            sent_at: v["sent_at"].to_string(),
        }
    }
}

impl RabbitMapper {
    pub fn new() -> RabbitMapper {
        return RabbitMapper{}
    }
}