use crate::Protocol;
use std::net::{IpAddr, Ipv4Addr};

pub struct SocketBuilder {
    protocol: Protocol,
    address: IpAddr,
}

impl Default for SocketBuilder {
    fn default() -> Self {
        SocketBuilder {
            protocol: Protocol::default(),
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
        }
    }
}