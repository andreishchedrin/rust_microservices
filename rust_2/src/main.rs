
mod app;
mod models;

// #[tokio::main(flavor = "multi_thread", worker_threads = 2)]
#[tokio::main]
async fn main() {
    println!("Start app");
    let db_instance = app::db::init();
    let mapper_instance = app::mapper::RabbitMapper::new();
    let consumer_instance = app::consumer::Rabbit::new().await;
    let app_instance = app::init(db_instance, consumer_instance, mapper_instance);

    app_instance.start_consumer().await
}
