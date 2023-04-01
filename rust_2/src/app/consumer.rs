pub trait Consumer {
    fn consume_message();
}

pub struct Rabbit {
}

impl Consumer for Rabbit {
    fn consume_message() {
        todo!()
    }
}

pub struct Kafka {
}

impl Consumer for Kafka {
    fn consume_message() {
        todo!()
    }
}

pub fn rabbit_init() -> Rabbit {
    Rabbit{}
}