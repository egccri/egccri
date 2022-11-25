//! 存取shadow
//!
//! device_model转换为streaming_schema
use anyhow::{Error, Result};
use egccri_device_model::device::device_shadow;
use egccri_device_model::device::profile::device_profile::DeviceProfile;
use serde_json::Value;

/// 获取设备`Profile`
fn get_shadow_profile() -> Result<DeviceProfile, Error> {
    Ok(DeviceProfile::new("1000".to_string(), vec![]))
}

fn save_shadow(json: &Value) -> Result<(), Error> {
    Ok(())
}

/// 删除后重建，考虑可用性，如何更新不影响使用，阻塞数据直到更新成功
///
/// 此接口供`internal.controller`调用，由`internal.controller` `watch` `edge-hub`，然后创建或更新本地
fn update_shadow(json: &Value) -> Result<(), Error> {
    Ok(())
}
