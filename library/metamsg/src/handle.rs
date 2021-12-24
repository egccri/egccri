use std::fmt::{Display, Formatter};
use std::future::Future;
use std::task::{Context, Poll};
use std::time::Duration;
use crate::channel::Channel;
use crate::channel::stream_handler::StreamHandler;

use crate::message::Message;

#[derive(Debug)]
pub struct MessageError;

impl Display for MessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// 不同协议的Handler可能不同，动态适配或者继承
// 此handler为公共handler，用来解决重连，超时等
// 一个特殊的Handle，转换Stream到Message
pub trait Handle<Channel> {

    type Stream;

    type Error;

    type Future: Future<Output = Result<Self::Stream, Self::Error>>;

    fn poll_ready(&mut self, cx: Context<'_>) -> Poll<Result<(), Self::Error>>;

    fn process(&mut self, channel: Channel) -> Self::Future;
}

#[test]
fn test_consume_handle() {
    let stream_handler = StreamHandler;
    let handle = crate::channel::timeout_handler::Timeout::new(
        stream_handler, Duration::from_micros(1000)
    );
}