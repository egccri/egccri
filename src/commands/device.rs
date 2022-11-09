use anyhow::Result;
use std::path::Path;

/// sub command device
#[derive(Debug)]
pub struct DeviceCommand {
    /// load a device profile from file path.
    load: Path,
}

impl DeviceCommand {
    /// Executes the command.
    pub fn execute(&self) -> Result<()> {
        Ok(())
    }
}
