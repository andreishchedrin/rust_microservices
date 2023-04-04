use crate::models::message::Message;

pub trait Consumer {
    fn consume_message() -> Message;
}

pub struct Rabbit {
}

impl Consumer for Rabbit {
    fn consume_message() -> Message {
        todo!()
    }
}

pub struct Kafka {
}

impl Consumer for Kafka {
    fn consume_message() -> Message {
        todo!()
    }
}

pub fn rabbit_init() -> Rabbit {
    Rabbit{}
}