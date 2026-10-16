# Changelog

All notable changes to this project are documented in this file. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project
follows [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- Added checked borrowed input with state masks, position capability and owned typed
  causes, sharing extraction with existing owned streams (#142).

- Added owned storage-stream output seeking for header backpatching while preserving
  EOF, input counts and formatting state (#136).

### Fixed

- Checked toolkit unsafe source counts during PR validation, including inactive
  platform code and test fixtures (#81).
- Checked expanded unsafe counts against target-specific compiler baselines and
  retained separate workspace and dependency reports in CI (#81).
- Removed stale compiler inventory reports and streams before starting a new
  capture, including captures that failed before compilation (#118).

## [0.3.1] - 2026-10-16

### Added

- Added the `martensite-tkmath` crate, the math toolkit: geometric primitives
  and transformations, vectors and matrices, linear algebra, root finders,
  minimizers and integrators, B-spline and polynomial libraries, conversion of
  conics and elementary surfaces to B-splines, bounding boxes and bounding
  volume hierarchies, triangulations and polygons, locations, and exact
  predicates for sketch incidences and certified Bezier roots ([#76]).

### Changed

- Changed the `cargo xtask release` semver check for versions before 1.0.0 to
  cargo's compatibility rule: an additive change (`Added`, `Deprecated`, a
  non-breaking `Changed`) needs a patch bump and a breaking change (a removal
  or incompatible change) a minor bump. From 1.0.0 the rules are unchanged
  ([#76]).

### Fixed

- Fixed false incompatible-release errors for Common, Heap and Aligned storage
  on Windows, preserving coherent ring removal and arena-release protection ([#78]).

- Fixed cancelled sequence-clear tokens being accepted by later clear
  invocations, preventing live values from being abandoned ([#77]).

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
[#76]: https://github.com/Oblikovati/Martensite/issues/76
[#77]: https://github.com/Oblikovati/Martensite/issues/77
[#78]: https://github.com/Oblikovati/Martensite/issues/78
