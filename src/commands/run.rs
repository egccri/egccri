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
            RunCommand::All => {
                info!("Run all module");
                Ok(())
            }
        }
    }
}

/// run connect module
#[derive(clap::Parser, Debug)]
pub struct RunConnect {
    /// enable proto
    proto: Vec<String>,
}

impl RunConnect {
    /// execute run_connect command
    pub fn execute(self) -> Result<()> {
        // initial storage.
        egccri_connect::start();
        Ok(())
    }
}

/// run compute module
#[derive(clap::Parser, Debug)]
pub struct RunCompute {}

impl RunCompute {
    /// execute run_compute command
    pub fn execute(self) -> Result<()> {
        Ok(())
    }
}
