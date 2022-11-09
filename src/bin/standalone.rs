use anyhow::Result;
use clap::{error::ErrorKind, Parser};
use egccri_cli::commands::RunCommand;
use tracing::info;
use tracing_subscriber;

/// Lightweight edge connect and compute runtime for various edge device!
///
/// Egccri is a edge program for connect devices, support modbus, mqtt, etc.
/// And it support compute with UDS(User define service).
#[derive(clap::Parser, Debug)]
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
    tracing_subscriber::fmt::init();

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");
    let egccri = Egccri::parse();
}
