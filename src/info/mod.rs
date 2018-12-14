//! Utilities for querying OS information, such as version, name, and other
//! things.

use version::OsVersion;

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

    /// Debian Linux.
    #[cfg(target_os = "linux")]
    Debian {
        /// The Debian release name, if known.
        release: Option<os::debian::OsRelease>
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

impl OsMeta {
    /// Returns the string representation of the operating system's release
    /// version name, if one is known.
    pub fn release_name(&self) -> Option<&'static str> {
        #[cfg(target_os = "linux")]
        match self {
            OsMeta::Ubuntu { release, .. } => release.map(Into::into),
            OsMeta::Debian { release, .. } => release.map(Into::into),
            OsMeta::UnknownLinux { .. } => None,
        }

        #[cfg(not(target_os = "linux"))]
        match self {
            #[cfg(target_os = "macos")]
            OsMeta::MacOs { release, .. } => release.map(Into::into),
            #[cfg(target_os = "windows")]
            OsMeta::Windows { release, .. } => release.map(Into::into),
        }
    }
}

/// Information about the host operating system.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OsInfo {
    /// Metadata for the host OS.
    pub meta: OsMeta,
    /// The operating system version.
    pub version: Option<OsVersion>,
}

impl OsInfo {
    /// Queries information about the host operating system.
    pub fn get() -> OsInfo {
        os::get_info()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_os_info() {
        let info = OsInfo::get();

        #[cfg(target_os = "macos")]
        {
            assert!(info.version.is_some());
            match info.meta {
                OsMeta::MacOs { release, .. } => {
                    assert!(release.is_some());
                },
            }
        }
    }
}
