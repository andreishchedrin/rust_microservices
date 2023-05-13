
use app::consumer::Consumer;

mod app;
mod models;

// #[tokio::main(flavor = "multi_thread", worker_threads = 2)]
#[tokio::main]
async fn main() {
    println!("Start app");
    let db_instance = app::db::init();
    let consumer_instance = app::consumer::Rabbit::new().await;
    let app_instance = app::init(db_instance, consumer_instance);

    app_instance.start_consumer().await
}
