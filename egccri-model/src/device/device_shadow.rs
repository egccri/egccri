use crate::device::profile::device_profile;
use std::collections::HashMap;

struct DeviceShadow {
    id: i32,
    name: String,
    address: Addressable,
    namespace: String,
    profile: device_profile::DeviceProfile,
    properties: HashMap<String, String>,
}

struct Addressable {}

impl DeviceShadow {
    fn new(id: i32, profile: device_profile::DeviceProfile) -> Self {
        DeviceShadow {
            id,
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
