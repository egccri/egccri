use crate::device::profile::property_value::PropertyValue;
use crate::device::profile::units::Units;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileProperty {
    value: PropertyValue,
    units: Units,
}
