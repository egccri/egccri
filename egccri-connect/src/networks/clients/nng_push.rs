use nng::{Error, Protocol, Socket};
use std::thread;
use std::time::Duration;

pub fn push(url: &str, arg: &str) -> Result<(), Error> {
    let s = Socket::new(Protocol::Push0)?;
    s.dial(url)?;

    println!("PUSH: SENDING \"{}\"", arg);
    s.send(arg.as_bytes())?;

    // Wait for messages to flush before shutting down.
    thread::sleep(Duration::from_secs(1));
    Ok(())
}
