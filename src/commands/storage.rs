use std::path::PathBuf;

/// sub command device
#[derive(clap::Parser, Debug)]
pub struct StorageCommand {
    /// load a device profile from file path.
    file_name: PathBuf,
}
