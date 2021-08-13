mod mqtt;

pub mod server {
    use tokio::sync::{broadcast, mpsc};

    pub fn run() {
        println!("I'm mqtt server.");
        let (ctrl_c_tx, ctrl_c_rx) = broadcast::channel(5);
        let (client_tx, client_rx) = mpsc::channel(32);
    }
}
