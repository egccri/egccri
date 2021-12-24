use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use pin_project_lite::pin_project;

use crate::handle::{Handle, MessageError};
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct Timeout<H> {
    inner: H,
    timeout: Duration
}

impl<H> Timeout<H> {
    pub fn new(inner: H, timeout: Duration) -> Timeout<H> {
        Timeout {
            inner,
            timeout,
        }
    }
}

impl<H, Channel> Handle<Channel> for Timeout<H>
where
    H: Handle<Channel>,
    H::Error: Into<crate::BoxError>,
{
    type Stream = H::Stream;

    type Error = crate::BoxError;

    type Future = StreamFuture<H::Future>;

    fn poll_ready(&mut self, cx: Context<'_>) -> Poll<Result<(), Self::Error>> {
        match self.inner.poll_ready(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(r) => Poll::Ready(r.map_err(Into::into)),
        }
    }

    fn process(&mut self, channel: Channel) -> Self::Future {
        // process inner first
        let stream = self.inner.process(channel);
        // create a future to execute
        StreamFuture::new(stream)
    }
}

pin_project! {
    #[derive(Debug)]
    pub struct StreamFuture<T> {
        #[pin]
        stream: T,
    }
}

impl<T> StreamFuture<T> {
    pub(crate) fn new(stream: T) -> Self {
        StreamFuture { stream }
    }
}

impl<F, T, E> Future for StreamFuture<F>
    where
        F: Future<Output = Result<T, E>>,
        E: Into<crate::BoxError>,
{
    type Output = Result<T, crate::BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        // First, try polling the future
        match this.stream.poll(cx) {
            Poll::Ready(v) => return Poll::Ready(v.map_err(Into::into)),
            Poll::Pending => {}
        }

        // Now check the sleep
        match this.sleep.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => Poll::Ready(Err(Box::g(()))),
        }
    }
}

