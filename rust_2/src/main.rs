// extern crate postgres;
//
// use postgres::{Connection, ConnectParams, ConnectTarget, SslMode, UserInfo};

mod app;
// mod db;
// mod consumer;

fn main() {
    println!("Start app");
    let db_instance = app::db::init();
    let consumer_instance = app::consumer::rabbit_init();
    let app_instance = app::init(db_instance, consumer_instance);
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
