//! Utilities specific to Linux.

use version::OsVersion;
use info::{OsInfo, OsMeta};

pub mod debian;
pub mod ubuntu;

pub(crate) fn get_info() -> OsInfo {
    let version = OsVersion::get();
    OsInfo {
        meta: OsMeta::UnknownLinux {},
        version,
    }
}
