use crate::device::profile::property_value::AccessMode::{ReadOnly, ReadWrite};
use std::fmt::{self, Display, Formatter};

/// + type - Required. The data type of the value. Supported types are bool, int8 - int64, uint8 - uint64, float32, float64, string, binary and arrays of the primitive types (ints, floats, bool). Arrays are specified as eg. float32array, boolarray etc.
/// + readWrite - R, RW, or W indicating whether the value is readable or writable.
/// + defaultValue - a value used for PUT requests which do not specify one.
/// + base - a value to be raised to the power of the raw reading before it is returned.
/// + scale - a factor by which to multiply a reading before it is returned.
/// + offset - a value to be added to a reading before it is returned.
/// + mask - a binary mask which will be applied to an integer reading.
/// + shift - a number of bits by which an integer reading will be shifted right.
pub struct PropertyValue {
    value_type: Type,
    access_mode: AccessMode,
    default_value: String,
    base: String,
    scale: f64,
    offset: u64,
}

pub enum Type {
    STRING(String),
    INI(i32),
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum AccessMode {
    ReadOnly = 0,
    ReadWrite = 1,
}

impl Display for AccessMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ReadOnly => write!(f, "0 (access mode is read only)."),
            ReadWrite => write!(f, "1 (access mode is read write)."),
        }
    }
}

impl AccessMode {
    pub fn from_bits(bits: u8) -> AccessMode {
        match bits {
            0 => ReadOnly,
            1 => ReadWrite,
            _ => panic!("Invalid access mode."),
        }
    }
}
