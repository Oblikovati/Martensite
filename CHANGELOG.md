# Changelog

All notable changes to this project are documented in this file. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project
follows [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- Added the `martensite-tkcdf` crate for application documents, references between
  documents, document metadata, format-driver lookup, container header reading and a
  light XML parser and writer.
- Added the `martensite-tklcaf` crate for the application data framework: labels and
  attributes, transactions with undo and redo, standard attributes, the function
  mechanism, documents, external links and multi-document transactions.
- Added the `martensite-tkservice` crate for visualization services: windows and input
  events, pixel maps, fonts and text layout, the presentation data model, shader
  program generation and the graphic driver interfaces. Linux is complete; the Windows,
  macOS and canvas windows are defined interfaces that return a typed
  `Standard_NotImplemented` error until their backends exist. FreeType, fontconfig and
  Xlib are loaded at run time.

### Fixed

- Recorded the Intel macOS compiler-inventory baseline from a clean native capture (#198).

- Corrected release coverage to Linux x64, Windows x64, and both macOS architectures, with native Intel macOS validation replacing Linux ARM64 (#198).

- Preserved frozen fixture bytes during Windows checkouts with automatic line-ending conversion (#197).

## [0.3.3] - 2026-10-19

### Added

- Added geometric construction, conversion, approximation, extrema, bounds and mass-property tools with retained numerical fixtures (#193).

- Added spatial curves, surfaces, adaptors, evaluation grids and retained numerical fixtures (#192).

- Added checked borrowed input with state masks, position capability and owned typed
  causes, sharing extraction with existing owned streams and binary container
  metadata decoding with checked offset-range rejection (#142, #181).

- Added explicit fallible printer and ordered messenger delivery with owned typed
  failures, preserving existing infallible entry points (#112).

- Added the `martensite-tkexpress` crate for symbolic expressions, differentiation,
  simplification, named functions, relations and text interpretation (#100).
- Added the `martensite-tkg2d` crate for planar curves, splines, offsets, adaptors,
  derivative and grid evaluation, and quantized geometry hashes (#105).

### Changed

- Computed every transcendental function (trigonometric, hyperbolic, exponential,
  logarithmic, power, cube root and `hypot`) with one portable, correctly rounded
  library, so finite transcendental results are bit-identical on Linux x64,
  Windows x64 and both macOS architectures. This is numerical behavior version 2
  (#201).

  Migration from behavior version 1 (0.3.2 and earlier): a result changes only
  where the platform C math library did not round correctly. On Linux, 971 rows
  in 73 retained fixtures and the expected values of 3 test literals and 20
  fixture seals changed. Each change was traced to individual function results
  that MPFR confirms the new library rounds correctly and glibc did not.
  Composite results such as curves, projections, iteration counts and formatted
  text can differ by more than one unit in the last place where they amplify
  such a difference. Apart from the documented invalid-operation NaN below, NaN
  sign and payload remain unspecified.
  Version 1 cannot be selected inside this release; applications that need its
  exact results stay on the 0.3.2 crates. The maintainer approved behavior
  version 2 with this recorded migration instead of version-1 replay; the
  evidence is in `docs/features/portable-transcendentals/attribution/`.

- Returned one NaN word, `0xfff8000000000000` (the x86-64 default NaN), for
  invalid divisions in expression evaluation and for directions normalized from
  infinite components. Arm targets previously produced a positive NaN here, so
  their text and words differed from x86-64; x86-64 results are unchanged. The
  new `martensite_cmath::quotient` and `INVALID_OPERATION_NAN` provide this (#201).

- Spelled infinities and NaNs in stream text (`format_general` and `IoStream`
  real insertion) by their sign alone (`inf`, `-inf`, `nan`, `-nan`) on every
  target, as the glibc layout it documents. Windows previously printed the
  x86-64 default NaN as UCRT's `-nan(ind)` there; the C `sprintf` emulation
  keeps the host spelling (#201).

### Fixed

- Preserved serialized inverse-hyperbolic domain errors: `acosh(x < 1)` and
  `atanh(|x| > 1)` return `INVALID_OPERATION_NAN` (`0xfff8000000000000`) on every
  target, while NaN operands stay classification-only and valid results stay
  correctly rounded (#203).

- Guarded single-precision powers with NaN arguments before dependency evaluation, while preserving zero-exponent and unit-base identities (#202).

- Fixed symbolic-expression message failures to honor configured stack capture while
  preserving explicitly supplied stacks (#141).

## [0.3.2] - 2026-10-18

### Added

- Added owned storage-stream output seeking for header backpatching while preserving
  EOF, input counts and formatting state (#136).

### Fixed

- Shipped `LICENSE-APACHE` in the binary release archives and `LICENSE-GPL` in
  every source package; the release had still bundled the removed `LICENSE`
  file (#176).
- Checked toolkit unsafe source counts during PR validation, including inactive
  platform code and test fixtures (#81).
- Checked expanded unsafe counts against target-specific compiler baselines and
  retained separate workspace and dependency reports in CI (#81).
- Removed stale compiler inventory reports and streams before starting a new
  capture, including captures that failed before compilation (#118).

### Changed

- Replaced selected toolkit raw ownership, record initialization and native calls
  with safe ownership and established safe interfaces; preserved public APIs and
  excluded candidates with rejected or unproven performance. Recorded the partial
  migration results and outstanding constraints in the safe-toolkit closeout (#80).

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
