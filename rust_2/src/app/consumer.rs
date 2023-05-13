use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicCancelArguments, BasicConsumeArguments, Channel, QueueBindArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    consumer::DefaultConsumer,
};
use async_trait::async_trait;
use tokio::sync::Notify;
use tokio::time;
use std::str;

#[async_trait]
pub trait Consumer {
    async fn consume_messages(&self);
}

pub struct Rabbit {
    channel: Channel,
    queue_name: String,
}

impl Rabbit {
    pub async fn new() -> Rabbit {
        // open a connection to RabbitMQ server
        let connection = Connection::open(&OpenConnectionArguments::new(
            "rabbitmq",
            5672,
            "guest",
            "guest",
        ))
            .await
            .unwrap();

        connection
            .register_callback(DefaultConnectionCallback)
            .await
            .unwrap();

        // open a channel on the connection
        let channel = connection.open_channel(None).await.unwrap();
        channel
            .register_callback(DefaultChannelCallback)
            .await
            .unwrap();

        // declare a queue
        let (queue_name, _, _) = channel
            .queue_declare(QueueDeclareArguments::new("tasks.queue").finish())
            .await
            .unwrap()
            .unwrap();

        print!("Queue declared: {queue_name}\n");

        // bind the queue to exchange
        let routing_key = "tasks";
        let exchange_name = "amq.topic";
        channel
            .queue_bind(QueueBindArguments::new(
                &queue_name,
                exchange_name,
                routing_key,
            ))
            .await
            .unwrap();

        print!("Channel {channel} init complete!\n");

        Rabbit{
            channel,
            queue_name
        }
    }
}

#[async_trait]
impl Consumer for Rabbit {
    async fn consume_messages(&self) {
        let args = BasicConsumeArguments::new(
            &self.queue_name,
            "tasks_pub_sub"
        );

        let (ctag, mut messages_rx) = self.channel.basic_consume_rx(args).await.unwrap();

        // you will need to run this in `tokio::spawn` or `tokio::task::spawn_blocking`
        // if you want to do other things in parallel of message consumption.
        tokio::spawn(async move {
            while let Some(msg) = messages_rx.recv().await {
                let r = msg.content.unwrap();
                let s = match str::from_utf8(&r) {
                    Ok(v) =>  v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                print!("Result {:?}\n", s);
            }
        });

        // Only needed when `messages_rx.recv().await` hasn't yet returned `None`
        if let Err(e) = self.channel.basic_cancel(BasicCancelArguments::new(&ctag)).await {
            print!("Result error {e}\n");
        };

        // consume forever
        println!("consume forever..., ctrl+c to exit");
        let guard = Notify::new();
        guard.notified().await;
    }
}

pub struct Kafka {
}

#[async_trait]
impl Consumer for Kafka {
    async fn consume_messages(&self) {
        todo!()
    }
}

