use crate::message::Message;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::future::Future;

struct Subscriber {}

struct PubSocket {}

#[derive(Debug)]
pub struct PubError;

impl Display for PubError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for PubError {}

pub trait A {}

// 此handler为pub专用，用来消费消息
trait PubSubHandler {
    type Future: Future<Output = Result<PubFuture, PubError>>;

    fn on_message<T>(&mut self, message: T) -> Self::Future
    where
        T: A;
}

pub struct PubFuture {}
