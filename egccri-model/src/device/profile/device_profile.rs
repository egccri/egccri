use crate::device::profile::device_resource::DeviceResource;
use std::collections::HashMap;

pub struct DeviceProfile {
    id: i32,
    name: String,
    manufacturer: String,
    namespace: String,
    resources: Vec<DeviceResource>,
    labels: HashMap<String, String>,
}

impl DeviceProfile {
    pub fn new(id: i32, resources: Vec<DeviceResource>) -> Self {
        DeviceProfile {
            id,
            name: "Pascal".to_string(),
            manufacturer: "HUAWEI".to_string(),
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
