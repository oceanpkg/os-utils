//! Utilities specific to Debian Linux.

use version::Version;

use self::OsRelease::*;

/// The release name of a known Debian version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OsRelease {
    /// Buzz (Debian 1.1).
    Buzz,
    /// Rex (Debian 1.2).
    Rex,
    /// Bo (Debian 1.3).
    Bo,
    /// Hamm (Debian 2.0).
    Hamm,
    /// Slink (Debian 2.1).
    Slink,
    /// Potato (Debian 2.2).
    Potato,
    /// Woody (Debian 3.0).
    Woody,
    /// Sarge (Debian 3.1).
    Sarge,
    /// Etch (Debian 4.0).
    Etch,
    /// Lenny (Debian 5.0).
    Lenny,
    /// Squeeze (Debian 6.0).
    Squeeze,
    /// Wheezy (Debian 7.0).
    Wheezy,
    /// Jessie (Debian 8.0).
    Jessie,
    /// Stretch (Debian 9.0).
    Stretch,
    // We assume this value will never be used, so `unreachable_unchecked()` is
    // fine to use
    #[doc(hidden)]
    _NonExhaustive,
}

impl OsRelease {
    /// The minimum supported OS release.
    pub const MIN: OsRelease = Buzz;

    /// The most recent OS release.
    pub const LATEST: OsRelease = Stretch;

    // This might be a large table; monomorphize here
    fn _new(Version { major, minor, .. }: Version) -> Option<Self> {
        match (major, minor) {
            (1, 1) => Some(Buzz),
            (1, 2) => Some(Rex),
            (1, 3) => Some(Bo),
            (2, 0) => Some(Hamm),
            (2, 1) => Some(Slink),
            (2, 2) => Some(Potato),
            (3, 0) => Some(Woody),
            (3, 1) => Some(Sarge),
            (4, 0) => Some(Etch),
            (5, 0) => Some(Lenny),
            (6, 0) => Some(Squeeze),
            (7, 0) => Some(Wheezy),
            (8, 0) => Some(Jessie),
            (9, 0) => Some(Stretch),
            _ => None,
        }
    }

    /// Returns the corresponding release for the Ubuntu version number.
    pub fn new<V: Into<Version>>(version: V) -> Option<Self> {
        Self::_new(version.into())
    }
}

impl From<OsRelease> for Version {
    fn from(release: OsRelease) -> Version {
        match release {
            Buzz           => (1, 1),
            Rex            => (1, 2),
            Bo             => (1, 3),
            Hamm           => (2, 0),
            Slink          => (2, 1),
            Potato         => (2, 2),
            Woody          => (3, 0),
            Sarge          => (3, 1),
            Etch           => (4, 0),
            Lenny          => (5, 0),
            Squeeze        => (6, 0),
            Wheezy         => (7, 0),
            Jessie         => (8, 0),
            Stretch        => (9, 0),
            _NonExhaustive => unsafe { std::hint::unreachable_unchecked() },
        }.into()
    }
}
