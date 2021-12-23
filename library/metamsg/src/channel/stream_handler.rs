use std::task::{Context, Poll};
use crate::channel::Channel;
use crate::handle::Handle;

/// The last common handler, covert stream to message
pub struct StreamHandler;

impl Handle<Channel> for StreamHandler {

    type Stream = ();
    type Error = ();
    type Future = ();

    fn poll_ready(&mut self, cx: Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn process(&mut self, channel: Channel) -> Self::Future {
        todo!()
    }
}