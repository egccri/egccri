use egccri_device_manager::DeviceManager;
use egccri_storage_sqlite::StorageSqlite;
use micro_async_module::{run_block_on, Config, Module};
use tracing::{info, warn};

pub mod cli;
mod networks;
mod server;
mod shadow;

pub fn start() {
    // 1. init store
    StorageSqlite.start();
    // 2. init core conn to the edge-hub
    // 4. inti networks.server(mqtt)
    // 5. setup client server
    EgccriConnect.start();
    // 3. fetch shadow information and controller run
    DeviceManager.start();
}

/// Connect edge-hub and recv command from client with unix domain socket.
pub struct EgccriConnect;

impl Module for EgccriConnect {
    fn config(&self) -> Config {
        Config {
            name: "Connect".to_string(),
            max_threads: 4,
        }
    }

    fn start(&self) {
        run_block_on(init(), self.config());
    }

    fn context(&self) {
        todo!()
    }
}

async fn init() {
    // 1. init core conn to the edge-hub

    // 2. inti networks.server(mqtt)
    inti_mqtt_server().await;

    // 3. setup client server
    server::init_client_socket_server().await;
}

async fn inti_mqtt_server() {
    #[cfg(not(feature = "mqtt"))]
    warn!("The connect module mqtt feature is not enable!");
    #[cfg(feature = "mqtt")]
    use networks::servers::mqtt_server;
    #[cfg(feature = "mqtt")]
    mqtt_server::start_mqtt_server("0.0.0.0:1883").await;
}
