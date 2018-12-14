use std::mem;

use info::{OsInfo, OsMeta};
use version::{Version, OsVersion};
use self::OsRelease::*;

const MIN_MINOR_VERSION: u64 = 7; // Lion
const MAX_MINOR_VERSION: u64 = MIN_MINOR_VERSION + OsRelease::LATEST as u64;

pub(crate) fn get_info() -> OsInfo {
    let version = OsVersion::get();
    let release = version.and_then(OsRelease::new);
    OsInfo {
        meta: OsMeta::MacOs {
            release,
        },
        version,
    }
}

/// The release name of a known macOS version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OsRelease {
    /// macOS 10.7.
    ///
    /// This is the minimum version supported by Rust, according to
    /// [tier 1 platforms](https://forge.rust-lang.org/platform-support.html#tier-1).
    Lion,
    /// macOS 10.8.
    MountainLion,
    /// macOS 10.9.
    Mavericks,
    /// macOS 10.10.
    ///
    /// This is the minimum version that supports querying the version with
    /// [`operatingSystemVersion`](https://developer.apple.com/documentation/foundation/nsprocessinfo/1410906-operatingsystemversion?language=objc).
    Yosemite,
    /// macOS 10.11.
    ElCapitan,
    /// macOS 10.12.
    Sierra,
    /// macOS 10.13.
    HighSierra,
    /// macOS 10.14.
    Mojave,
    // We assume this value will never be used, so `unreachable_unchecked()` is
    // fine to use
    #[doc(hidden)]
    _NonExhaustive,
}

impl<'a> From<OsRelease> for &'a str {
    fn from(release: OsRelease) -> Self {
        match release {
            Lion           => "Lion",
            MountainLion   => "Mountain Lion",
            Mavericks      => "Mavericks",
            Yosemite       => "Yosemite",
            ElCapitan      => "El Capitan",
            Sierra         => "Sierra",
            HighSierra     => "High Sierra",
            Mojave         => "Mojave",
            _NonExhaustive => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

impl OsRelease {
    /// The minimum supported OS release.
    pub const MIN: OsRelease = Lion;

    /// The most recent OS release.
    pub const LATEST: OsRelease = Mojave;

    /// Returns the corresponding release for the macOS version number.
    pub fn new<V: Into<Version>>(version: V) -> Option<Self> {
        let Version { major, minor, .. } = version.into();
        match (major, minor) {
            (10, MIN_MINOR_VERSION..=MAX_MINOR_VERSION) => unsafe {
                Some(mem::transmute((minor - MIN_MINOR_VERSION) as u8))
            },
            _ => None,
        }
    }
}

impl From<OsRelease> for Version {
    fn from(release: OsRelease) -> Version {
        (10, MIN_MINOR_VERSION + release as u64).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_conversion() {
        let minor_end = MAX_MINOR_VERSION + 1;
        let major = 10;

        assert_eq!(OsRelease::new((major, minor_end)), None);

        for minor in MIN_MINOR_VERSION..minor_end {
            let vers = Version::from((major, minor));
            let name = OsRelease::new(vers).unwrap();
            assert_eq!(Version::from(name), vers);
        }
    }

    #[test]
    fn cmp() {
        assert!(OsRelease::LATEST > OsRelease::MIN);
    }
}
