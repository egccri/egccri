mod networks;

#[tokio::main]
async fn main() {
    log4rs::init_file("egccri-core/log4rs.yaml", Default::default()).unwrap();

    // 1. init internal.store

    // 2. init core conn to the edge-hub
    networks::clients::nng_push::push("tcp://127.0.0.1:5555", "Hub, i got you!");
    // 3. fetch shadow information

    // 4. inti mqtt networks.servers
    inti_mqtt_server().await;
}

async fn inti_mqtt_server() {
    #[cfg(feature = "mqtt")]
    use networks::servers::mqtt_server;
    #[cfg(feature = "mqtt")]
    mqtt_server::start_mqtt_server("0.0.0.0:1883").await;
}
