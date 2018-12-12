//! Cross-platform utilities for querying information about the host operating
//! system.

#![deny(missing_docs)]

#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(target_os = "macos")] {
        extern crate cocoa;
        #[macro_use]
        extern crate objc;
    } else if #[cfg(target_os = "windows")] {
        extern crate winapi;
    }
}

pub mod info;
pub mod version;

pub use info::{OsInfo, OsMeta};
pub use version::OsVersion;
