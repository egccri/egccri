use anyhow::Result;
use clap::error::ErrorKind;
use clap::Parser;
use egccri_cli::commands::RunCommand;

/// Lightweight edge connect and compute runtime for various edge device!
///
/// Egccri is a edge program for connect devices, support modbus, mqtt, etc.
/// And it support compute with UDS(User define service).
#[derive(Parser, Debug)]
#[command(name = "Egccri")]
#[command(author = "Curtis Yang <zifeng.1024@gmail.com>")]
#[command(version = "1.0")]
#[command(about, long_about = None)]
enum Egccri {
    /// Runs egccri modules.
    Run(RunCommand),
}

impl Egccri {
    /// Executes the command.
    pub fn execute(self) -> Result<()> {
        match self {
            Self::Run(c) => c.execute(),
        }
    }
}

fn main() {
    // parse or else default run command.

}
