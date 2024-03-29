use connect_mqtt::server;
use connect_mqtt::server::MqttSettings;

pub async fn start_mqtt_server(mqtt_address: &str) {
    server::run(MqttSettings::new(mqtt_address)).await;
}
