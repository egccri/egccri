use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

/// sub command device
#[derive(clap::Parser, Debug)]
pub struct DeviceCommand {
    /// check device profile from json file.
    #[arg(short = 'c', long = "check-profile")]
    check_profile: Option<PathBuf>,

    /// load a device profile from json file and store to the storage
    #[arg(short = 'l', long = "load-profile")]
    load_profile: Option<PathBuf>,
}

impl DeviceCommand {
    /// Executes the command.
    pub fn execute(self) -> Result<()> {
        if let Some(profile_file) = self.check_profile {
            info!("Check device profile from path: {:?}", profile_file.as_os_str());
            egccri_connect::cli::load_profile_check(profile_file);
        }
        if let Some(profile_file) = self.load_profile {
            egccri_connect::cli::load_profile_and_store(profile_file);
        }
        Ok(())
    }
}
