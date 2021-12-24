use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::channel::Channel;
use crate::handle::Handle;

/// The last common handler, covert stream to message
pub struct StreamHandler;

impl Handle<Channel> for StreamHandler {

    type Stream = ();
    type Error = crate::BoxError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Stream, Self::Error>>>>;

    fn poll_ready(&mut self, cx: Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn process(&mut self, channel: Channel) -> Self::Future {
        Box::pin(async move {

        })
    }
}