# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.2.2] - 2020-04-05

### Added

- Automatic underscore conversion for `nms remove`.
  `nms remove module-name` is equivalent to `nms remove module_name`

## [0.2.1] - 2020-04-05

### Fixed

- `nms list` broke due to clap/structopt oddities. Fixed now.

## [0.2.0] - 2020-03-26

### Changed

- ***BREAKING*** Renamed binary to `nms`.
- Updated example in readme.

## [0.1.7] - 2020-03-26

### Added

- Version flags

## [0.1.6] - 2020-03-26

### Added

- Kernel modprobe support. Can be used in `/proc/sys/kernel/modprobe`.

### Changed

- Tables use fancy UTF-8 characters for pretty tables

## [0.1.5] - 2020-02-26

### Fixed

- `linux_modules list` panicking

## [0.1.4] - 2020-02-19

### Fixed

- Failure to get information on some xz compressed modules, such as `vivid`
- Modules with long parameter descriptions, such as `vivid`, displaying incorrectly.

### Changed

- Module parameters now appear in alphabetical order, instead of random.

## [0.1.3] - 2020-02-18

### Added

- `uname` command flag, so you can get information about modules from
  other kernel versions. This can happen if you upgrade your kernel,
  causing current module files to be removed. Arch Linux does this.

### Fixed

- `linux_modules info <module>` now correctly identifies signatures
- `linux_modules info <module>` now works for modules without parameters

### Changed

- Proper error messages instead of rust panics

## [0.1.2] - 2020-02-16

### Changed

- Updated `linapi`

## [0.1.1] - 2020-02-16

### Added

- Support for compressed modules

## [0.1.0] - 2020-02-16

### Added

- CLI Interface for adding, removing listing, and getting information on modules.

<!-- next-url -->
[Unreleased]: https://github.com/DianaNites/linux_modules/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/DianaNites/linux_modules/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/DianaNites/linux_modules/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/DianaNites/linux_modules/compare/v0.1.7...v0.2.0
[0.1.7]: https://github.com/DianaNites/linux_modules/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/DianaNites/linux_modules/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/DianaNites/linux_modules/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/DianaNites/linux_modules/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/DianaNites/linux_modules/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/DianaNites/linux_modules/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/DianaNites/linux_modules/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/DianaNites/linux_modules/releases/tag/v0.1.0
