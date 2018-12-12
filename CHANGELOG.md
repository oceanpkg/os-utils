# Changelog [![Crates.io][crate-badge]][crate]

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## Unreleased

## 0.0.1 - 2018-12-12

- Added:

  ```
  os_utils (lib.rs)
  ├── info
  │   ├── OsInfo
  │   ├── OsMeta
  │   └── os
  │       ├── linux
  │       │  └── ubuntu
  │       │      └── OsRelease
  │       ├── macos
  │       │   └── OsRelease
  │       └── windows
  │           └── OsRelease
  └── version
      ├── Version
      └── OsVersion
  ```

- Added `OsVersion::get()` to query the platform-specific OS version
    - Contains build number on Windows

- Added `OsInfo::get()` to query all available information about the OS

- Added `Version::parse()`, which calls the `FromStr` impl

- Added conversions between `Version` and `OsVersion`

    - A `Version` reference can be taken straight from an `OsVersion`

[crate]:       https://crates.io/crates/os_utils
[crate-badge]: https://img.shields.io/crates/v/os_utils.svg
