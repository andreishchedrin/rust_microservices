extern crate postgres;

use postgres::{Connection, ConnectParams, ConnectTarget, SslMode, UserInfo};

fn db_init() -> Connection {
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
    conn
}

pub struct Pg {
    conn: Connection
}

pub fn init() -> Pg {
    Pg { conn: db_init() }
}

pub trait DB {
    fn insert_message(&self, data: &Message);
}

impl DB for Pg {
    fn insert_message(&self, message: &Message) {
        self.conn.execute("INSERT INTO messages (data, sent_at) VALUES ($1, $2)",
                     &[&message.data, &message.sent_at]).unwrap();
    }
}

pub struct Message {
    id: i64,
    data: String,
    created_at: String,
    sent_at: String,
}