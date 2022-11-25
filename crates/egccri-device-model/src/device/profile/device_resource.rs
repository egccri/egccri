use crate::device::profile::profile_property::ProfileProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum ResourceScheme {
    Property,
    Telemetry,
    Command,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceResource {
    id: i32,
    name: String,
    description: String,
    resource_scheme: ResourceScheme,
    property: ProfileProperty,
}
