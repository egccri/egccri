enum ResourceScheme {
    Property,
    Telemetry,
    Command,
}

pub struct DeviceResource {
    id: i32,
    name: String,
    description: String,
    resource_scheme: ResourceScheme,
}
