use std::any::Any;
use std::net::IpAddr;

pub enum Protocol {
    PUSH,
    PULL,
    PUB,
    SUB,
    REQ,
    REP,
}

pub struct Socket {
    protocol: Protocol,
    address: IpAddr,
}

/// reconnect
/// heartbeat
/// QOS
#[allow(dead_code)]
impl Socket {
    pub fn new(protocol: Protocol, ip: IpAddr) -> Self {
        Socket {
            protocol,
            address: ip,
        }
    }

    pub fn bind(address: IpAddr) {}

    pub fn send(message: Box<dyn Any>) {}

    pub fn dial(address: IpAddr) {}
}
