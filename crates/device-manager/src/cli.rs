mod device;

use device_model::device::profile::DeviceProfile;
use std::fs::File;
use std::path::PathBuf;
use tracing::{debug, info};

pub fn load_profile_check(path: PathBuf) {
    let profile_json = File::open(path).expect("Profile config file open error!");
    let profile: DeviceProfile = serde_json::from_reader(profile_json).unwrap();
    info!("Device profile json file load check pass!");
    info!("The output: {:?}", profile)
}

pub fn load_profile_and_store(path: PathBuf) {
    let profile_json = File::open(path).expect("Profile config file open error!");
    let profile: DeviceProfile = serde_json::from_reader(profile_json).unwrap();
    debug!("The output: {:?}", profile)
}
