use anyhow::Result;
use egccri_connect::{App, DeviceManager, EgccriConnect, StorageSqlite};
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
    /// 1. init store
    /// 2. init core conn to the edge-hub
    /// 3. fetch shadow information and controller run
    /// 4. inti networks.server(mqtt)
    /// 5. setup client server
    pub fn execute(self) -> Result<()> {
        let app = App::new("egccri".to_string(), 4, 100);
        app.add_module(StorageSqlite)
            .add_module(EgccriConnect)
            .add_module(DeviceManager)
            .run();
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
