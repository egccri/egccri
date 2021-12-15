pub mod builder;
mod handle;
mod message;
mod proto;
mod transport;
mod channel;
mod frame;

use crate::builder::WhoBuilder;
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
    address: IpAddr,
}

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

    fn bind(address: IpAddr) {}

    fn send(message: Box<dyn Any>) {}

    fn dial(address: IpAddr) {}
}

#[cfg(test)]
mod tests {

}
