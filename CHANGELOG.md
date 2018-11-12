# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2018-11-11
### Added
- Added more implementations of the trait up to size 128, and some powers of 2 and 10.

## [0.2.0] - 2018-04-26
### Added
- This change log.
- `IsSliceomorphic` now explicitly supports the use case of impls on wrapper types around arrays, so that I can sleep at night.
- `<[T]>::to_array()`, because type inference hates `as_array().clone()`.

### Changed
- `IsSliceomorphic::array_len` has been replaced with `IsSliceomorphic::LEN`.
- Accordingly, the minimum supported version of Rust has bumped to... hell, idunno.

## 0.1.1 - 2017-10-13
### Added
- `<[[T; n]]>::flat`
- `<[T]>::nest`
- `<[T]>::as_array`
- ...and `mut` variants.

[Unreleased]: https://github.com/ExpHP/slice-of-array/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/ExpHP/slice-of-array/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/ExpHP/slice-of-array/compare/v0.1.1...v0.2.0