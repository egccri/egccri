use crate::message::Message;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::future::Future;

#[derive(Debug)]
pub struct MessageError;

impl Display for MessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// 不同协议的Handler可能不同，动态适配或者继承
// 此handler为公共handler，用来解决重连，超时等
trait Handler {
    type Future: Future<Output = Result<Message, MessageError>>;

    fn on_message(&mut self, message: Message) -> Self::Future;
}
