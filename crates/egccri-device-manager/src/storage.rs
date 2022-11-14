use egccri_device_model::device::device_shadow::DeviceShadow;
use egccri_device_model::device::profile::device_profile::DeviceProfile;
use egccri_storage_sqlite::StorageHandler;

pub struct StorageDeviceProfile(DeviceProfile);

pub struct StorageDeviceShadow(DeviceShadow);

impl StorageHandler for StorageDeviceProfile {
    fn on_add(&self) {
        todo!()
    }

    fn on_update(&self) {
        todo!()
    }

    fn on_delete(&self) {
        todo!()
    }
}

impl StorageHandler for StorageDeviceShadow {
    fn on_add(&self) {
        todo!()
    }

    fn on_update(&self) {
        todo!()
    }

    fn on_delete(&self) {
        todo!()
    }
}
