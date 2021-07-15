pub mod builder;

use std::any::Any;
use std::net::IpAddr;
use crate::builder::SocketBuilder;

pub enum Protocol {
    PUSH,
    PULL,
    PUB,
    SUB,
    REQ,
    REP,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum QoS {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

#[derive(Debug, PartialEq)]
pub struct Socket {
    protocol: Protocol,
    address: IpAddr,
    qos: QoS,
}

/// reconnect
/// heartbeat
/// QOS
impl Socket {

    pub fn builder() -> FooBuilder {
        SocketBuilder::default()
    }

    pub fn bind(address: IpAddr) {}

    pub fn send(message: Box<dyn Any>) {}

    pub fn dial(address: IpAddr) {}
}
