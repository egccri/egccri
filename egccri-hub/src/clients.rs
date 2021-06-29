pub mod kafka;

use super::config::config::{ClientProtocol, EdgeHubConfig};
use crate::context::tenant::TenantId;
use crate::message::Message;
use log::info;

#[allow(dead_code)]
pub struct ClientTwin {
    name: String,
    tenant_id: TenantId,
    message_queue: tokio::sync::mpsc::Receiver<Message>,
}

pub fn start(edge_hub_config: &EdgeHubConfig) {
    let protocol = EdgeHubConfig::get_hub_enabled_client_protocol(edge_hub_config);

    match protocol {
        ClientProtocol::KAFKA => {
            info!("Kafka client is enabled!")
        }
        ClientProtocol::PULSAR => {
            info!("Pulsar client is enabled!")
        }
    }
}
