use crate::Protocol;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug)]
pub struct SocketBuilder {
    protocol: Protocol,
    address: IpAddr,
}

impl Default for SocketBuilder {
    fn default() -> Self {
        SocketBuilder {
            protocol: Protocol::default(),
            address: IpAddr::V4(Ipv4Addr::LOCALHOST),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::SocketBuilder;
    use crate::Protocol;

    #[test]
    fn test_default() {
        let socket_default = SocketBuilder {
            protocol: Protocol::PULL,
            ..Default::default()
        };
        println!("{:?}", socket_default);
    }
}
