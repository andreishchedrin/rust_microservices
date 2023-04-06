// extern crate postgres;
//
// use postgres::{Connection, ConnectParams, ConnectTarget, SslMode, UserInfo};

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicCancelArguments, BasicConsumeArguments, QueueBindArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    consumer::DefaultConsumer,
};

use tokio::time;

use std::str;

mod app;
mod models;
// mod db;
// mod consumer;

// #[tokio::main(flavor = "multi_thread", worker_threads = 2)]
#[tokio::main]
async fn main() {
    println!("Start app");
    let db_instance = app::db::init();
    let consumer_instance = app::consumer::rabbit_init();
    let app_instance = app::init(db_instance, consumer_instance);

    rabbit_init().await
}

// fn db_init() -> Connection {
//     let params = ConnectParams {
//         target: ConnectTarget::Tcp("postgres".to_string()),
//         port: Some(5432),
//         user: Some(UserInfo {
//             user: "test_user".to_string(),
//             password: None,
//         }),
//         database: Some("test_db".to_string()),
//         options: vec![],
//     };
//
//     let conn =
//         Connection::connect(
//             params,
//             &SslMode::None).unwrap();
//
//     conn.execute("CREATE TABLE IF NOT EXISTS messages (
//                        id              SERIAL PRIMARY KEY,
//                        data            VARCHAR NOT NULL,
//                        created_at      TIMESTAMP DEFAULT NOW(),
//                        sent_at         TIMESTAMP
//                    )", &[]).unwrap();
//
//     println!("Database init done!");
//     conn
// }

async fn rabbit_init() {
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

    let args = BasicConsumeArguments::new(
        &queue_name,
        "tasks_pub_sub"
    );

    // let result = channel
    //     .basic_consume(DefaultConsumer::new(args.no_ack), args)
    //     .await
    //     .unwrap();
    //
    // print!("Result {result}\n");

    let (ctag, mut messages_rx) = channel.basic_consume_rx(args).await.unwrap();

    // you will need to run this in `tokio::spawn` or `tokio::task::spawn_blocking`
    // if you want to do other things in parallel of message consumption.
    while let Some(msg) = messages_rx.recv().await {
        let r = msg.content.unwrap();
        let s = match str::from_utf8(&r) {
            Ok(v) =>  v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        print!("Result {:?}\n", s);
    }

    // Only needed when `messages_rx.recv().await` hasn't yet returned `None`
    if let Err(e) = channel.basic_cancel(BasicCancelArguments::new(&ctag)).await {
        print!("Result error {e}\n");
    };

    // keep the `channel` and `connection` object from dropping before pub/sub is done.
    // channel/connection will be closed when drop.
    time::sleep(time::Duration::from_secs(1)).await;
    // explicitly close
    channel.close().await.unwrap();
    connection.close().await.unwrap();
}