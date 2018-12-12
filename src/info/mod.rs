//! Utilities for querying OS information, such as version, name, and other
//! things.

use version::Version;

pub mod os;

/// Metadata for a specific operating system.
///
/// Information for each OS in non-exhaustive. Fields may be added later and it
/// won't be considered breaking backwards compatibility.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OsMeta {
    /// Apple macOS.
    #[cfg(target_os = "macos")]
    MacOs {
        /// The macOS release name, if known.
        release: Option<os::OsRelease>
    },

    /// Microsoft Windows.
    #[cfg(target_os = "windows")]
    Windows {
        /// The Windows release name, if known.
        release: Option<os::OsRelease>
    },

    /// Ubuntu Linux.
    #[cfg(target_os = "linux")]
    Ubuntu {
        /// The Ubuntu release name, if known.
        release: Option<os::ubuntu::OsRelease>
    },

    /// Some unknown Linux operating system.
    #[cfg(target_os = "linux")]
    UnknownLinux {},
}

/// Information about the host operating system.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OsInfo {
    /// Metadata for the host OS.
    pub meta: OsMeta,
    /// The operating system version.
    pub version: Option<Version>,
}

impl OsInfo {
    /// Queries information about the host operating system.
    pub fn get() -> OsInfo {
        unimplemented!()
    }

    /// Returns the string representation of the operating system's release
    /// version name, if one is known.
    pub fn release_name_str(&self) -> Option<&'static str> {
        #[cfg(target_os = "linux")]
        match self.meta {
            OsMeta::Ubuntu { release, .. } => release.map(Into::into),
            OsMeta::UnknownLinux { .. } => None,
        }

        #[cfg(not(target_os = "linux"))]
        match self.meta {
            #[cfg(target_os = "macos")]
            OsMeta::MacOs { release, .. } => release.map(Into::into),
            #[cfg(target_os = "windows")]
            OsMeta::Windows { release, .. } => release.map(Into::into),
        }
    }
}