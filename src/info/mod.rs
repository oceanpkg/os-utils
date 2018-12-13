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

/// Information about the host operating system.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OsInfo {
    /// Metadata for the host OS.
    pub meta: OsMeta,
    /// The operating system version.
    pub version: Option<OsVersion>,
}

impl OsInfo {
    #[cfg(target_os = "macos")]
    fn _get() -> OsInfo {
        let version = OsVersion::get();
        let release = version.and_then(os::OsRelease::new);
        OsInfo {
            meta: OsMeta::MacOs {
                release,
            },
            version,
        }
    }

    #[cfg(target_os = "windows")]
    fn _get() -> OsInfo {
        let version = OsVersion::get();
        let release = version.and_then(os::OsRelease::new);
        OsInfo {
            meta: OsMeta::Windows {
                release,
            },
            version,
        }
    }

    #[cfg(target_os = "linux")]
    fn _get() -> OsInfo {
        let version = OsVersion::get();
        OsInfo {
            meta: OsMeta::UnknownLinux {},
            version,
        }
    }

    /// Queries information about the host operating system.
    pub fn get() -> OsInfo {
        OsInfo::_get()
    }

    /// Returns the string representation of the operating system's release
    /// version name, if one is known.
    pub fn release_name_str(&self) -> Option<&'static str> {
        #[cfg(target_os = "linux")]
        match self.meta {
            OsMeta::Ubuntu { release, .. } => release.map(Into::into),
            OsMeta::Debian { release, .. } => release.map(Into::into),
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
