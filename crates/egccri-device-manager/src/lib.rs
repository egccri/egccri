use micro_async_module::{AsyncRuntime, Module};
use std::fmt::Debug;

pub mod cli;
mod controller;
mod storage;

const MODULE_NAME: &str = "device_manager";

/// Fetch shadow information and controller run
#[derive(Debug)]
pub struct DeviceManager;

impl Module for DeviceManager {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    fn start(&self, runtime: AsyncRuntime) {
        todo!()
    }

    fn send(&self) {
        todo!()
    }

    fn recv(&self) {
        todo!()
    }
}
