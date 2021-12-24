pub mod builder;
mod handle;
mod message;
mod proto;
mod transport;
pub mod channel;
mod frame;

use crate::builder::WhoBuilder;
use std::any::Any;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;

/// a build server example to run handler.
/// ```
/// use tokio::net::TcpListener;
/// use metamsg::channel::Channel;
/// use metamsg::channel::stream_handler::StreamHandler;
/// use metamsg::channel::timeout_handler::Timeout;
///
/// let listener = TcpListener::bind("127.0.0.1");
/// let stream_handler = StreamHandler;
/// let handle = Timeout::new(
///     stream_handler, Duration::from_micros(1000)
/// );
/// loop {
///     let mut connection = listener.accept().await;
///     let channel = Channel::new(connection);
///     match handle.call(channel).await {
///         Ok(()) => (),
///         Err(()) => (),
///     }
/// }
/// ```
///
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Protocol {
    PUSH,
    PULL,
    PUB,
    SUB,
    REQ,
    REP,
}

pub enum Who {
    Peer,
    Router,
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::REP
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum QoS {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    protocol: Protocol,
    address: Address,
}

#[derive(Debug, PartialEq)]
pub enum Address {
    Tcp(Host, Port),
    Ipc(Option<PathBuf>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Host {
    /// An IPv4 address
    Ipv4(Ipv4Addr),
    /// An Ipv6 address
    Ipv6(Ipv6Addr),
    /// A domain name, such as `example.com` in `tcp://example.com:4567`.
    Domain(String),
}

pub type Port = u16;

pub struct Peer {
    node: Node,
    qos: QoS, // QoS is determined by the peer, and negotiation between two peers.
}

pub struct Router {
    node: Node,
}

/// reconnect
/// heartbeat
/// QOS
trait  Socket {

    fn bind(node: Node) {}

    fn send(message: Box<dyn Any>) {}

    fn dial(node: Node) {}
}


#[cfg(test)]
mod tests {

}
