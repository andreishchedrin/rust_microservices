use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc;

pub mod db;
pub mod consumer;
pub mod mapper;

pub struct App<T: db::DB, U: consumer::Consumer, O: mapper::Mapper> {
    db: T,
    consumer: U,
    mapper: O
}

pub fn init<T: db::DB, U: consumer::Consumer, O: mapper::Mapper>(db: T, consumer: U, mapper: O) -> App<T, U, O> {
    App{
        db,
        consumer,
        mapper
    }
}

impl<T: db::DB, U: consumer::Consumer + std::marker::Sync + std::marker::Send + 'static, O: mapper::Mapper> App<T, U, O> {
    pub async fn start_consumer(&'static self) {
        let (tx, mut rx) = mpsc::channel(1);
        let consumer = &self.consumer;

        tokio::spawn(async move {
            consumer.consume_messages(tx).await
        });

        loop {
            if let Some(message) = rx.recv().await {
                let msg = self.mapper.to_message(message);
                self.db.insert_message(&msg);
            } else {
                panic!("Receive message error.")
            }
        }
    }
}
