
use crate::models::message::Message;

pub trait Mapper{}

pub struct RabbitMapper {}

impl Mapper for RabbitMapper {}