mod broker;
mod mqtt;

pub mod server {
    use crate::broker::manager::Manager;
    use crate::mqtt::listener::MqttListener;
    use futures::future::join_all;
    use log::{debug, info};
    use tokio::signal;
    use tokio::sync::{broadcast, mpsc};

    #[derive(Debug)]
    pub struct MqttSettings {
        pub listeners_tcp: Vec<String>,
    }

    impl MqttSettings {
        pub fn new(mqtt_address: &str) -> MqttSettings {
            let mut addresses = Vec::new();
            addresses.push(mqtt_address.to_string());
            MqttSettings {
                listeners_tcp: addresses,
            }
        }
    }

    // 异步方法集成方式
    // 提取ctrl_c,研究优雅停止方案
    pub async fn run(mqtt_settings: MqttSettings) {
        let (ctrl_c_tx, ctrl_c_rx) = broadcast::channel(5);

        let (client_tx, client_rx) = mpsc::channel(32);

        let manager = Manager::new(client_rx, ctrl_c_rx);
        let manager_future = tokio::spawn(manager.run());

        let mut listeners = Vec::new();

        for bind_address in mqtt_settings.listeners_tcp {
            let listener = MqttListener::bind(
                bind_address.as_str(),
                client_tx.clone(),
                ctrl_c_tx.subscribe(),
            )
            .await
            .unwrap();

            listeners.push(tokio::spawn(listener.start_accepting()));
        }

        info!("Initialized egccri connect mqtt module");

        signal::ctrl_c().await.unwrap();

        info!("Stopping egccri connect mqtt module");
        ctrl_c_tx.send(()).unwrap();

        join_all(listeners).await;

        manager_future.await.unwrap();

        info!("egccri connect mqtt module stopped");
    }
}
