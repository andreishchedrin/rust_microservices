pub mod db;
pub mod consumer;

pub struct App<T: db::DB, U: consumer::Consumer> {
    db: T,
    consumer: U,
}

// impl<T, U> App<T, U>
//     where T: db::DB, U: consumer::Consumer
// {
//    pub fn init(db: T, consumer: U) -> Box<Self> {
//        Box::new(App{
//            db,
//            consumer
//        })
//    }
// }

pub fn init<T: db::DB, U: consumer::Consumer>(db: T, consumer: U) -> App<T, U> {
    App{
        db,
        consumer
    }
}


