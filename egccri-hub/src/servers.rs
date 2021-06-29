mod nng;

use crate::config::config::{EdgeHubConfig, ServerProtocol};
use crate::context::tenant::TenantId;
use crate::message::Message;

pub struct EdgeCoreTwin {
    name: String,
    tenant_id: TenantId,
    // every edge core use a single queue for cache messages
    message_queue: tokio::sync::mpsc::Sender<Message>,
    status: EdgeCoreStatus,
}

pub enum EdgeCoreStatus {
    ONLINE,
    OFFLINE,
}

impl EdgeCoreTwin {
    fn new(
        name: String,
        tenant_id: TenantId,
        message_queue: tokio::sync::mpsc::Sender<Message>,
    ) -> Self {
        EdgeCoreTwin {
            name,
            tenant_id,
            message_queue,
            status: EdgeCoreStatus::ONLINE,
        }
    }
}

pub async fn start(edge_hub_config: &EdgeHubConfig) {
    let protocol = EdgeHubConfig::get_hub_enabled_server_protocol(edge_hub_config);

    match protocol {
        ServerProtocol::ZMQ => {}
        ServerProtocol::NNG => nng::start().await,
        ServerProtocol::WS => {}
        ServerProtocol::QUIC => {}
        ServerProtocol::MQTT => {}
    }
}

pub trait Server {
    fn start();
}
