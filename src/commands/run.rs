use anyhow::Result;
use clap::Parser;

// TODO Split connect and compute run.
#[derive(Parser)]
pub struct RunCommand {}

impl RunCommand {
    pub fn execute(&self) -> Result<()> {
        // initial storage.

        // run connect.

        // run compute.

        OK(())
    }
}
