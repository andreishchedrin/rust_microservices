// extern crate postgres;
//
// use postgres::{Connection, ConnectParams, ConnectTarget, SslMode, UserInfo};

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicConsumeArguments, QueueBindArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    consumer::DefaultConsumer,
};

mod app;
mod models;
// mod db;
// mod consumer;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
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
        .queue_declare(QueueDeclareArguments::default())
        .await
        .unwrap()
        .unwrap();

    // bind the queue to exchange
    let rounting_key = "amqprs.example";
    let exchange_name = "amq.topic";
    channel
        .queue_bind(QueueBindArguments::new(
            &queue_name,
            exchange_name,
            rounting_key,
        ))
        .await
        .unwrap();

    print!("{channel}, {queue_name}")
}