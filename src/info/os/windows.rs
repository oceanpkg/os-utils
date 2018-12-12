use version::Version;

/// The release name of a known Windows version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OsRelease {
    /// Windows 7.
    ///
    /// This is the minimum version supported by Rust, according to
    /// [tier 1 platforms](https://forge.rust-lang.org/platform-support.html#tier-1).
    Windows7,
    /// Windows 8.
    Windows8,
    /// Windows 8.1.
    Windows8_1,
    /// Windows 10.
    Windows10,
    // We assume this value will never be used, so `unreachable_unchecked()` is
    // fine to use
    #[doc(hidden)]
    _NonExhaustive,
}

impl<'a> From<OsRelease> for &'a str {
    fn from(release: OsRelease) -> Self {
        match release {
            OsRelease::Windows7       => "7",
            OsRelease::Windows8       => "8",
            OsRelease::Windows8_1     => "8.1",
            OsRelease::Windows10      => "10",
            OsRelease::_NonExhaustive => unsafe {
                std::hint::unreachable_unchecked()
            },
        }
    }
}

impl OsRelease {
    /// The minimum supported OS release.
    pub const MIN: OsRelease = OsRelease::Windows7;

    /// The most recent OS release.
    pub const LATEST: OsRelease = OsRelease::Windows10;

    /// Returns the corresponding release for the Windows version number.
    pub fn new<V: Into<Version>>(version: V) -> Option<Self> {
        let Version { major, minor, .. } = version.into();
        match (major, minor) {
            (06, 1) => Some(OsRelease::Windows7),
            (06, 2) => Some(OsRelease::Windows8),
            (06, 3) => Some(OsRelease::Windows8_1),
            (10, 0) => Some(OsRelease::Windows10),
            _ => None
        }
    }
}

impl From<OsRelease> for Version {
    fn from(release: OsRelease) -> Version {
        let pair = match release {
            OsRelease::Windows7       => (06, 1),
            OsRelease::Windows8       => (06, 2),
            OsRelease::Windows8_1     => (06, 3),
            OsRelease::Windows10      => (10, 0),
            OsRelease::_NonExhaustive => unsafe {
                std::hint::unreachable_unchecked()
            },
        };
        pair.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp() {
        assert!(OsRelease::LATEST > OsRelease::MIN);
    }
}
