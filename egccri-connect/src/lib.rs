use micro_async_module::{run_async_block_on, AsyncRuntime, Module};
use tracing::{info, warn};

pub mod cli;
mod networks;
mod server;
mod shadow;

pub use egccri_device_manager::DeviceManager;
pub use egccri_storage_sqlite::StorageSqlite;
pub use micro_async_module::App;

const MODULE_NAME: &str = "egccri_connect";

/// Connect edge-hub and recv command from client with unix domain socket.
#[derive(Debug)]
pub struct EgccriConnect;

impl Module for EgccriConnect {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    fn start(&self, runtime: AsyncRuntime) {
        run_async_block_on(init(), runtime);
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
