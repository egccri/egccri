use crate::channel::connection::Connection;

/// Support a common channel to exchange message with features like QoS,timeout,reconnect,heartbeat.
/// Channel used by proto, split in_channel and out_channel.

pub mod qos_handler;
pub mod timeout_handler;
pub mod heartbeat_handler;
pub mod connection;
pub mod stream_handler;

pub struct Channel {
    conn: connection::Connection,

}

impl Channel {
    // create a channel from socket.
    pub fn new(connection: Connection) {

    }
}