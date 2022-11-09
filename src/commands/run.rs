use anyhow::Result;

// TODO Split connect and compute run.
/// sub command run egccri modules.
#[derive(clap::Parser, Debug)]
pub struct RunCommand {}

impl RunCommand {
    /// execute command
    pub fn execute(&self) -> Result<()> {
        // initial storage.

        // run connect.

        // run compute.

        Ok(())
    }
}
