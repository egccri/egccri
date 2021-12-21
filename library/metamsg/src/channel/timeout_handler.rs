use std::future::Future;
use std::time::Duration;
use crate::channel::connection::Channel;
use crate::handle::{Handle, MessageError};
use crate::message::Message;

#[derive(Debug, Clone)]
struct Timeout<H> {
    inner: H,
    timeout: Duration
}

impl<H> Timeout<H> {
    fn new(inner: H, timeout: Duration) -> Timeout<H> {
        Timeout {
            inner,
            timeout,
        }
    }
}

impl<H, Channel> Handle<Channel> for Timeout<H>
where
    H: Handle<Channel>,
{

    type Future = Future<Output=Result<Channel, MessageError>>;

    fn on_message(&mut self, message: Channel) -> Box<Self::Future> {
        todo!()
    }
}
