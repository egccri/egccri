use anyhow::Result;
use clap::{error::ErrorKind, Parser};
use egccri_cli::commands::{DeviceCommand, RunCommand};
use tracing::info;
use tracing_subscriber;

const EGCCRI_LONG_ABOUT: &str =
    "Egccri is a edge program for connect devices, support modbus, mqtt, etc.
        And it support compute with UDS(User define service)";

/// Lightweight edge connect and compute runtime for various edge device!
///
/// Egccri is a edge program for connect devices, support modbus, mqtt, etc.
/// And it support compute with UDS(User define service).
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = EGCCRI_LONG_ABOUT)] // Read from `Cargo.toml`
enum Egccri {
    /// Runs egccri modules.
    #[command(subcommand)]
    Run(RunCommand),
    /// Config device.
    Device(DeviceCommand),
}

impl Egccri {
    /// Executes the command.
    pub fn execute(self) -> Result<()> {
        match self {
            Self::Run(command) => command.execute(),
            Self::Device(command) => command.execute(),
        }
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // parse or else default run command.
    Egccri::try_parse()
        .unwrap_or_else(|e| match e.kind() {
            ErrorKind::InvalidSubcommand | ErrorKind::UnknownArgument => {
                Egccri::Run(RunCommand::All)
            }
            _ => e.exit(),
        })
        .execute()
}
