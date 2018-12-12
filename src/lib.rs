//! Cross-platform utilities for querying information about the host operating
//! system.

#![deny(missing_docs)]

#[macro_use]
extern crate cfg_if;

pub mod version;
pub mod info;

pub use info::{OsInfo, OsMeta};
