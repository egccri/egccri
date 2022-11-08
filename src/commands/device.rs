use std::path::Path;
use anyhow::Result;

pub struct DeviceCommand {
    /// load a device profile from file path.
    load: Path
}

impl DeviceCommand {
    /// Executes the command.
    pub fn execute(&self) -> Result<()> {
        OK(())
    }
}
