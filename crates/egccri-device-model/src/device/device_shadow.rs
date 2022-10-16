use crate::device::profile::device_profile;
use std::collections::HashMap;

pub struct DeviceShadow {
    id: String,
    name: String,
    address: Addressable,
    namespace: String,
    profile: device_profile::DeviceProfile,
    properties: HashMap<String, String>,
}

struct Addressable {}

impl DeviceShadow {
    pub fn new(id_str: &str, profile: device_profile::DeviceProfile) -> Self {
        DeviceShadow {
            id: id_str.to_string(),
            name: "Pascal".to_string(),
            address: Addressable {},
            namespace: "Namespace".to_string(),
            profile,
            properties: HashMap::new(),
        }
    }

    pub fn add_device_properties(
        &mut self,
        field_name: String,
        field_value: String,
    ) -> Option<String> {
        self.properties.insert(field_name, field_value)
    }
}
