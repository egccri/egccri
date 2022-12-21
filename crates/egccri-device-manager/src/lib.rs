use micro_async_module::{Config, Module};

pub mod cli;
mod controller;
mod storage;

/// Fetch shadow information and controller run
pub struct DeviceManager;

impl Module for DeviceManager {
    fn config(&self) -> Config {
        todo!()
    }

    fn start(&self) {
        todo!()
    }

    fn context(&self) {
        todo!()
    }
}
