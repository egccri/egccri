use anyhow::Result;
use clap::Parser;
use egccri_cli::commands::{DeviceCommand, StorageCommand};
use tracing::{info, warn};

/// Egccri Cli commands.
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Cli {
    /// Config device.
    Device(DeviceCommand),
    /// Config storage
    Storage(StorageCommand),
}

impl Cli {
    /// Executes the command.
    pub fn execute(self) -> Result<()> {
        match self {
            Self::Device(command) => command.execute(),
            Self::Storage(_) => {
                info!("Storage command execute.");
                Ok(())
            }
        }
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // parse or else default run command.
    Cli::parse().execute()
}
