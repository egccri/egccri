use egccri_storage_sqlite::StorageSqlite;
use micro_async_module::{run_block_on, Config, Module};
use tracing::{info, warn};

pub mod cli;
mod networks;
mod shadow;

pub struct EgccriConnect;

impl Module for EgccriConnect {
    fn config(&self) -> Config {
        Config {
            name: "Connect".to_string(),
            max_threads: 2,
        }
    }

    fn start(&self) {
        run_block_on(inti_mqtt_server(), self.config());
    }

    fn context(&self) {
        todo!()
    }
}

pub fn start() {
    StorageSqlite.start();
    EgccriConnect.start();
}

async fn init() {
    // 1. init store

    // 2. init core conn to the edge-hub
    networks::clients::nng_push::push("tcp://127.0.0.1:5555", "Hub, i got you!")
        .expect("nng error");
    // 3. fetch shadow information

    // 4. inti mqtt networks.servers
    inti_mqtt_server().await;
}

async fn inti_mqtt_server() {
    #[cfg(not(feature = "mqtt"))]
    warn!("The connect module mqtt feature is not enable!");
    #[cfg(feature = "mqtt")]
    use networks::servers::mqtt_server;
    #[cfg(feature = "mqtt")]
    mqtt_server::start_mqtt_server("0.0.0.0:1883").await;
}
