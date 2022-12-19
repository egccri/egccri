use futures::{AsyncReadExt, AsyncWriteExt};
use parity_tokio_ipc::{dummy_endpoint, Endpoint};
use tokio::io::split;
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

pub fn start() {
    StorageSqlite.start();
    EgccriConnect.start();
}

async fn init() {
    // 1. init store

    // 2. init core conn to the edge-hub

    // 3. fetch shadow information

    // 4. inti mqtt networks.servers
    inti_mqtt_server().await;

    // 5. setup client server
    init_client_socket_server().await;
}

async fn inti_mqtt_server() {
    #[cfg(not(feature = "mqtt"))]
    warn!("The connect module mqtt feature is not enable!");
    #[cfg(feature = "mqtt")]
    use networks::servers::mqtt_server;
    #[cfg(feature = "mqtt")]
    mqtt_server::start_mqtt_server("0.0.0.0:1883").await;
}

async fn init_client_socket_server() {
    let mut endpoint = Endpoint::new(dummy_endpoint());
    let incoming = endpoint.incoming()
        .expect("failed to open up a new pipe/socket");
    futures::pin_mut!(incoming);

    while let Some(result) = incoming.next().await {
        match result {
            Ok(stream) => {
                let (mut reader, mut writer) = split(stream);
                let mut buf = [0u8; 5];
                reader.read_exact(&mut buf).await.expect("unable to read from socket");
                writer.write_all(&buf[..]).await.expect("unable to write to socket");
            }
            _ => unreachable!("ideally")
        }
    };
}
