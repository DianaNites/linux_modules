# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

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
[Unreleased]: https://github.com/DianaNites/linux_modules/compare/v0.1.5...HEAD
[0.1.5]: https://github.com/DianaNites/linux_modules/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/DianaNites/linux_modules/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/DianaNites/linux_modules/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/DianaNites/linux_modules/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/DianaNites/linux_modules/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/DianaNites/linux_modules/releases/tag/v0.1.0
