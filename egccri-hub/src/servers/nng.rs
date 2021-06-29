use log::info;
use nng::{Protocol, Socket};

pub async fn start() {
    info!("nng server starting...");
    let socket = Socket::new(Protocol::Rep0).unwrap();
    bind(&socket, "tcp://127.0.0.1:5555").await;
    tokio::spawn(accept_loop(socket));
    info!("nng server started.");
}

pub async fn bind(socket: &Socket, url: &str) {
    socket.listen(url).expect("Nng bind 5555 failure");
}

pub async fn accept_loop(socket: Socket) {
    loop {
        // Sleep for a little bit before sending the next message.
        // todo!
    }
}
