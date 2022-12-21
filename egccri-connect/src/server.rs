use futures::StreamExt;
use parity_tokio_ipc::{dummy_endpoint, Endpoint};
use tokio::io::{split, AsyncReadExt, AsyncWriteExt};

pub(crate) async fn init_client_socket_server() {
    let mut endpoint = Endpoint::new(dummy_endpoint());
    let incoming = endpoint
        .incoming()
        .expect("failed to open up a new pipe/socket");
    futures::pin_mut!(incoming);

    while let Some(result) = incoming.next().await {
        match result {
            Ok(stream) => {
                let (mut reader, mut writer) = split(stream);
                let mut buf = [0u8; 5];
                reader
                    .read_exact(&mut buf)
                    .await
                    .expect("unable to read from socket");
                writer
                    .write_all(&buf[..])
                    .await
                    .expect("unable to write to socket");
            }
            _ => unreachable!("ideally"),
        }
    }
}
