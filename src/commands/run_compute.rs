use anyhow::Result;

/// run compute module
#[derive(clap::Parser, Debug)]
pub struct RunCompute {}

impl RunCompute {
    /// execute run_compute command
    pub fn execute(self) -> Result<()> {
        Ok(())
    }
}
