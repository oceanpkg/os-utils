//! Simple version information.

use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

/// A simple `MAJOR.MINOR.PATCH` version.
///
/// Unlike [`semver::Version`](https://docs.rs/semver/0.9.*/semver/struct.Version.html),
/// this value does not include a pre-release version identifier or build
/// metadata.
#[derive(Clone, Copy, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Version {
    /// The `x.0.0` version number.
    pub major: u64,
    /// The `0.y.0` version number.
    pub minor: u64,
    /// The `0.0.z` version number.
    pub patch: u64,
}

impl From<Version> for (u64, u64, u64) {
    #[inline]
    fn from(Version { major, minor, patch }: Version) -> Self {
        (major, minor, patch)
    }
}

impl From<(u64, u64, u64)> for Version {
    #[inline]
    fn from((major, minor, patch): (u64, u64, u64)) -> Self {
        Version { major, minor, patch }
    }
}

impl From<(u64, u64)> for Version {
    #[inline]
    fn from((major, minor): (u64, u64)) -> Self {
        Version { major, minor, ..Default::default() }
    }
}

impl From<(u64,)> for Version {
    #[inline]
    fn from((major,): (u64,)) -> Self {
        major.into()
    }
}

impl From<u64> for Version {
    #[inline]
    fn from(major: u64) -> Self {
        Version { major, ..Default::default() }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Version, ParseVersionError> {
        use self::ParseVersionError::*;

        let mut vers = Version::default();
        let mut iter = s.split('.');

        let major = iter.next().ok_or(EmptyInput)?;
        let minor = iter.next();
        let patch = iter.next();

        if iter.next().is_some() {
            return Err(ExtraInput);
        }

        vers.major = major.parse().map_err(|e| MajorInt(e))?;

        if let Some(minor) = minor {
            vers.minor = minor.parse().map_err(|e| MinorInt(e))?;
        }

        if let Some(patch) = patch {
            vers.patch = patch.parse().map_err(|e| PatchInt(e))?;
        }

        Ok(vers)
    }
}

impl Version {
    /// Queries the current macOS version.
    pub fn macos() -> Option<Version> {
        #[cfg(not(target_os = "macos"))]
        { None }

        #[cfg(target_os = "macos")]
        {
            use cocoa::appkit::*;
            use cocoa::base::nil;
            use cocoa::foundation::{NSInteger, NSProcessInfo};

            let version = unsafe { NSAppKitVersionNumber };

            let version = if version < NSAppKitVersionNumber10_7 {
                return None;
            } else if version < NSAppKitVersionNumber10_7_2 {
                (10, 7, 0)
            } else if version < NSAppKitVersionNumber10_7_3 {
                (10, 7, 2)
            } else if version < NSAppKitVersionNumber10_7_4 {
                (10, 7, 3)
            } else if version < NSAppKitVersionNumber10_8 {
                (10, 7, 4)
            } else if version < NSAppKitVersionNumber10_9 {
                (10, 8, 0)
            } else if version < NSAppKitVersionNumber10_10 {
                (10, 9, 0)
            } else {
                // https://developer.apple.com/documentation/foundation/nsoperatingsystemversion?language=objc
                #[repr(C)]
                struct NSOperatingSystemVersion {
                    major: NSInteger,
                    minor: NSInteger,
                    patch: NSInteger,
                }

                // Available in Obj-C as of macOS 10.10+
                let version: NSOperatingSystemVersion = unsafe {
                    let proc_info = NSProcessInfo::processInfo(nil);
                    msg_send![proc_info, operatingSystemVersion]
                };

                (version.major as u64, version.minor as u64, version.patch as u64)
            };
            Some(version.into())
        }
    }

    /// Creates a new instance from the three values.
    #[inline]
    pub fn new(major: u64, minor: u64, patch: u64) -> Version {
        Version { major, minor, patch }
    }

    /// Converts the version string formatted as `MAJOR.MINOR.PATCH`.
    #[inline]
    pub fn parse(version: &str) -> Result<Version, ParseVersionError> {
        version.parse()
    }
}

/// An error returned when parsing a version string fails.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseVersionError {
    /// Failed to parse major integer value.
    MajorInt(ParseIntError),
    /// Failed to parse minor integer value.
    MinorInt(ParseIntError),
    /// Failed to parse patch integer value.
    PatchInt(ParseIntError),
    /// Parse input is empty.
    EmptyInput,
    /// Parse input had an extra period and maybe more.
    ExtraInput,
}

#[cfg(test)]
mod tests {
    use super::*;

    type VersionTriple = (u64, u64, u64);

    #[test]
    fn cmp() {
        let pairs: &[(VersionTriple, VersionTriple)] = &[
            ((0, 0, 1), (0, 0, 0)),
            ((0, 1, 0), (0, 0, 1)),
            ((0, 1, 1), (0, 1, 0)),
            ((1, 0, 0), (0, 0, 1)),
            ((1, 0, 0), (0, 1, 0)),
        ];
        for &(a, b) in pairs {
            let a = Version::from(a);
            let b = Version::from(b);
            assert!(a > b);
            assert!(a != b);
            assert!(b < a);
        }
    }

    #[test]
    fn parse_success() {
        let pairs: &[(&str, VersionTriple)] = &[
            ("0",     Default::default()),
            ("0.1",   ( 0, 1, 0)),
            ("0.1.2", ( 0, 1, 2)),
            ("16.04", (16, 4, 0)),
        ];
        for &(string, version) in pairs {
            let version = Version::from(version);
            let strings: [&str; 2] = [string, &version.to_string()];

            for string in strings.iter() {
                assert_eq!(string.parse(), Ok(version));
            }
        }
    }

    #[test]
    fn parse_failure() {
        let strings: &[&str] = &[
            "",
            ".",
            "..",
            "...",
            "....",
            "0.",
            "0.0.",
            "0.0.0.",
            "0.0.0.0",
        ];
        for s in strings {
            Version::parse(s).expect_err(&format!("parsing {:?}", s));
        }
    }
}
