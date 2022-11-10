use anyhow::Result;

/// sub command device
#[derive(clap::Parser, Debug)]
pub struct DeviceCommand {
    /// load a device profile from file path.
    load: String,
}

impl DeviceCommand {
    /// Executes the command.
    pub fn execute(self) -> Result<()> {
        Ok(())
    }
}
