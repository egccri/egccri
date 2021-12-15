use crate::Protocol;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug)]
pub struct WhoBuilder {
    protocol: Protocol,
    address: IpAddr,
}

impl Default for WhoBuilder {
    fn default() -> Self {
        WhoBuilder {
            protocol: Protocol::default(),
            address: IpAddr::V4(Ipv4Addr::LOCALHOST),
        }
    }
}



#[test]
fn test_default() {
    let socket_default = WhoBuilder {
        protocol: Protocol::PULL,
        ..Default::default()
    };
    println!("{:?}", socket_default);
}

