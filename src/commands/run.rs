use crate::commands::{RunCompute, RunConnect};
use anyhow::Result;
use tracing::info;

/// sub command run egccri modules.

#[derive(clap::Subcommand, Debug)]
pub enum RunCommand {
    /// run sub command connect
    Connect(RunConnect),
    /// run sub command compute
    Compute(RunCompute),
    /// run both connect and compute module
    All,
}

impl RunCommand {
    /// execute run sub command
    pub fn execute(self) -> Result<()> {
        // pre config and check
        match self {
            RunCommand::Connect(command) => command.execute(),
            RunCommand::Compute(command) => command.execute(),
            RunCommand::All => { info!("Run all module"); Ok(()) },
        }
    }
}
