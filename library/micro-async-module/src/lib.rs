//! Support async runtime and config app.
//!
//! Support a module to run and context store.
//!
//! Use metamsg or memory channel to send and recv message.

///
pub trait Module{
    fn config();

    fn start();

    fn context();
}

struct Config{
    name: String,

}

fn run_block_on() {}


