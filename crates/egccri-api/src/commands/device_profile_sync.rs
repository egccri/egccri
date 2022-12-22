pub struct DeviceProfileSyncOpt {
    command_flag: i8,
}

pub enum Command {
    SyncOpt(DeviceProfileSyncOpt),
}
