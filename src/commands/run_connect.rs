use anyhow::Result;

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

        Ok(())
    }
}
