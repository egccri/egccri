use parity_tokio_ipc::Endpoint;
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct Client {
    host: String,
}

impl Client {
    pub async fn send_command<Command>(&self, command: Command) {}

    pub async fn send(path: PathBuf) {
        let mut client = Endpoint::connect(&path)
            .await
            .expect("Failed to connect client.");

        loop {
            let mut buf = [0u8; 4];
            println!("SEND: PING");
            client
                .write_all(b"ping")
                .await
                .expect("Unable to write message to client");
            client
                .read_exact(&mut buf[..])
                .await
                .expect("Unable to read buffer");
            if let Ok("pong") = std::str::from_utf8(&buf[..]) {
                println!("RECEIVED: PONG");
            } else {
                break;
            }

            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }
}
