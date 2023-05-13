use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicPublishArguments, QueueBindArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};

use tokio::time;

#[tokio::main]
async fn main() {
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

    for n in 1..101 {
        let content = String::from(
            r#"
        {
            "publisher": "example"
            "data": "Hello, amqprs! [test]"
        }
    "#,
        ).replace("test", &n.to_string())
            .into_bytes();

        // create arguments for basic_publish
        let args = BasicPublishArguments::new(exchange_name, routing_key);

        channel
            .basic_publish(BasicProperties::default(), content, args)
            .await
            .unwrap();
    }

    // keep the `channel` and `connection` object from dropping before pub/sub is done.
    // channel/connection will be closed when drop.
    time::sleep(time::Duration::from_secs(1)).await;
    // explicitly close
    channel.close().await.unwrap();
    connection.close().await.unwrap();
}
