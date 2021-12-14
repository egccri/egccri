pub mod builder;
mod handle;
mod message;
mod proto;
mod transport;

use crate::builder::SocketBuilder;
use std::any::Any;
use std::net::IpAddr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Protocol {
    PUSH,
    PULL,
    PUB,
    SUB,
    REQ,
    REP,
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
    address: IpAddr,
}

pub struct Peer {
    node: Node,
    qos: QoS, // QoS is determined by the peer, and negotiation between two peers.
}

pub struct Router {
    node: Node,
}

struct Socket {}

/// reconnect
/// heartbeat
/// QOS
impl Socket {
    pub fn builder() -> SocketBuilder {
        SocketBuilder::default()
    }

    pub fn bind(address: IpAddr) {}

    pub fn send(message: Box<dyn Any>) {}

    pub fn dial(address: IpAddr) {}
}
