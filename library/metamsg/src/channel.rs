/// Support a common channel to exchange message with features like QoS,timeout,reconnect,heartbeat.
/// Channel used by proto, split in_channel and out_channel.

mod qos_handler;
mod timeout_handler;
mod heartbeat_handler;
mod connection;