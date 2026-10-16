# Changelog

All notable changes to this project are documented in this file. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project
follows [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Fixed

- Fixed cancelled sequence-clear tokens completing a later invocation and
  abandoning its live values; stale tokens now fail before changing state ([#77]).
- Fixed `cargo xtask pr` accepting fatal Git errors as clean untracked-file
  whitespace checks; failed comparisons now retain their diagnostics ([#75]).

## [0.3.0] - 2026-10-16

### Added

- Added `install-linux.sh`, `install-macos.sh` and `install-windows.ps1`, which
  install or update every development dependency with the platform's package
  manager.
- Added the `cargo xtask` build driver and the single `Martensite` CI workflow
  for PR validation and multi-target releases.
- Added the `martensite-tkernel` crate, the foundation toolkit: typed errors,
  memory managers and allocators, collections, ASCII and extended strings,
  operating-system services, messages, quantities, units and resources
  ([#72]).
- Added the `martensite-cmath` crate, which calls the platform C math library
  for `asinh`, `acosh`, `atanh` and `remainder` so results match the C
  functions bit for bit ([#72]).

[#72]: https://github.com/Oblikovati/Martensite/pull/72
[#75]: https://github.com/Oblikovati/Martensite/issues/75
[#77]: https://github.com/Oblikovati/Martensite/issues/77
