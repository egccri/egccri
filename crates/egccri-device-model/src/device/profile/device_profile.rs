use crate::device::profile::device_resource::DeviceResource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceProfile {
    id: String,
    name: String,
    manufacturer: Option<String>,
    namespace: String,
    resources: Vec<DeviceResource>,
    labels: HashMap<String, String>,
}

impl DeviceProfile {
    pub fn new(id: String, resources: Vec<DeviceResource>) -> Self {
        DeviceProfile {
            id,
            name: "Pascal".to_string(),
            manufacturer: Option::from("HUAWEI".to_string()),
            namespace: "Namespace".to_string(),
            resources,
            labels: HashMap::new(),
        }
    }

    pub fn add_device_profile_labels(
        &mut self,
        field_name: String,
        field_value: String,
    ) -> Option<String> {
        self.labels.insert(field_name, field_value)
    }
}
