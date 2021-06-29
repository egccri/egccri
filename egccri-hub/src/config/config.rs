use std::str::FromStr;

#[derive(Clone)]
pub struct EdgeHubConfig {
    server_protocol: ServerProtocol,
    client_protocol: ClientProtocol,
}

#[allow(dead_code)]
pub struct EdgeCoreConfig {
    // message qos,default is EOS, when need higher performance, you should set a lower message qos.
    message_qos: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerProtocol {
    ZMQ,
    WS,
    QUIC,
    MQTT,
    NNG,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClientProtocol {
    KAFKA,
    PULSAR,
}

impl FromStr for ServerProtocol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ZMQ" => Ok(ServerProtocol::ZMQ),
            "WS" => Ok(ServerProtocol::WS),
            "QUIC" => Ok(ServerProtocol::QUIC),
            "MQTT" => Ok(ServerProtocol::MQTT),
            "NNG" => Ok(ServerProtocol::NNG),
            _ => Err(()),
        }
    }
}

impl FromStr for ClientProtocol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "KAFKA" => Ok(ClientProtocol::KAFKA),
            "PULSAR" => Ok(ClientProtocol::PULSAR),
            _ => Err(()),
        }
    }
}

impl EdgeHubConfig {
    pub fn new(server_protocol: ServerProtocol, client_protocol: ClientProtocol) -> Self {
        EdgeHubConfig {
            server_protocol,
            client_protocol,
        }
    }

    pub fn get_hub_enabled_server_protocol(&self) -> &ServerProtocol {
        &self.server_protocol
    }

    pub fn get_hub_enabled_client_protocol(&self) -> &ClientProtocol {
        &self.client_protocol
    }
}
