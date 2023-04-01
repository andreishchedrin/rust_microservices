extern crate postgres;

use postgres::{Connection, ConnectParams, ConnectTarget, SslMode, UserInfo};

fn main() {
    println!("Start app");
     db_init()
}

fn db_init() {
    let params = ConnectParams {
        target: ConnectTarget::Tcp("postgres".to_string()),
        port: Some(5432),
        user: Some(UserInfo {
            user: "test_user".to_string(),
            password: None,
        }),
        database: Some("test_db".to_string()),
        options: vec![],
    };

    let conn =
        Connection::connect(
            params,
            &SslMode::None).unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS messages (
                       id              SERIAL PRIMARY KEY,
                       data            VARCHAR NOT NULL,
                       created_at      TIMESTAMP DEFAULT NOW(),
                       sent_at         TIMESTAMP
                   )", &[]).unwrap();

    println!("Database init done!");
}