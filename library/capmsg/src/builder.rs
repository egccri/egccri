use crate::Protocol;
use std::net::IpAddr;

#[derive(Default)]
pub struct SocketBuilder {
    protocol: Protocol,
    address: IpAddr,
}
