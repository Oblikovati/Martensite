# Release source and QA scope

Bug [#466](https://github.com/Oblikovati/Martensite/issues/466) blocked release
preparation because Vulkan had 21 changed files under `tests/`, despite unchanged
`src/` and package manifest. The release classifier previously treated every
package file as a publication source change.

Release scope now separates eligible QA-only paths from publication inputs.
Changes outside `tests/` retain their existing classification, including
`build.rs`, examples, runtime assets and substantive manifest edits. Test changes
still select their package and affected dependents for QA. An unchanged published
package identity reuses its verified historical archive, including its historical
tests; current checkout tests run against current dependencies during validation.

A test path remains a publication input when production Rust references it,
including transitive Rust modules. Unreviewed custom build scripts, production targets
outside `src/`, unreadable or unparseable sources and unresolved file includes
retain the conservative source-changed result. Tokenization reuses the build
driver's existing exact `proc-macro2` dependency; no dependency component or
version was added.

For a dynamic `include_*!(concat!(...))`, a fixed final filename extension can
prove disjointness from changed file extensions. A `.pixels` include cannot read
a changed `.rs` file. Matching extensions and unbounded expressions remain
uncertain. Extension comparison is case-insensitive, and only a nonempty ASCII
alphanumeric extension supplies this proof. Direct fixture paths are checked
independently of that optimization. Single-arm literal macro parameters can also
supply a finite extension bound from their invocation arguments. Alias or export
uncertainty disables that proof. Calls inside other macro bodies are included;
unsupported matchers and nonliteral arguments remain uncertain. Nested inline
module path attributes remain source changes because their directory base is not
inferred. Conditional or otherwise wrapped path attributes fail closed at every
attribute depth, including inner attributes. Explicit module and `include!`
source paths must identify regular package-owned `.rs` files; other suffixes,
external sources and dynamic `include!` expressions remain uncertain. Byte and
string includes retain their separate data-file handling. Referenced Rust source
is never followed outside the package through a symlink.

Packages with known non-test source edits bypass this analysis. Unfiltered change
sets are borrowed, so ordinary release classification adds no change-list copies.

The release regression exercises `ReleaseScope::load` with an unchanged package
version and a test-only diff: publication scope is empty, while QA scope still
contains the package. Additional controls cover production fixture includes,
transitive modules, custom builds and targets, ambiguous paths and bounded dynamic
includes. No package versions or artifact immutability rules changed.

The process-policy rustix constraint update is a separate substantive manifest
change. Its historical 0.4.0 archive pins rustix 1.1.5 and cannot resolve alongside
the patched TKernel's exact 1.1.6 constraint. Ignoring that change or rewriting
published bytes would not fix the release graph. Its version decision is separate
from this test-only classifier correction.

## Test targets and the reviewed budget marker — #481

A separate Cargo integration-test target is a QA change. The publication comparison now ignores differences confined to `[[test]]` entries with `name`, `path` and `harness` fields after proving that production inputs do not retain `Cargo.toml`. It does not alter the parsed manifest or the test-impact comparison. Dependency, feature, build, library, binary and unknown-field changes still count as source changes.

Literal production references to `Cargo.toml` retain publication impact. A dynamic include with a `.toml` suffix remains uncertain whenever comparing manifest differences, including differences from a historical archive absent from the latest Git diff. Unreadable inputs and unsupported expressions fail closed.

The Boolean-case package has a build script that reads only `OPT_LEVEL` and `CARGO_CFG_TARGET_OS` and emits the budget cfg and a `rerun-if-changed=build.rs` directive. Its reviewed LF-normalized SHA-256 is `dddebb2e5a4e8cfbf22875ca33cd1890f22b750466b38063feab8d798d2bfb29`. Only this exact program permits the QA-only proof. An edited script, a different build path or an unknown program disables it. LF and CRLF checkouts receive the same decision. This is a bound on one inspected program's inputs, not a filename or package-name exemption.

Regression coverage includes test-only targets, both line endings, changed scripts, production manifest includes, historical manifest comparisons, real manifest changes and unchanged test selection. Focused read-only controls also evaluate the actual Boolean package source. No package versions or dependencies change. Release validation remains hosted; no local release rehearsal is used for this correction.

## Explicit production target paths — #482

The previous target guard accepted any path starting with `src/`. That included `src/../tests/runtime.rs`, whose actual source lies in the skipped QA directory, and `src/runtime.input`, whose extension the Rust-source scanner does not read. Either target could retain manifest bytes without appearing in the proof.

Explicit library and binary targets now remain uncertain unless they use a forward-slash path under `src/`, contain only normal path components, reject colon-based stream syntax and end in the exact `.rs` extension covered by the scanner. The change rejects unsupported paths rather than broadening the scanner. Regression fixtures cover both library and binary targets, parent traversal, unsupported extensions, manifest includes and ordinary eligible targets. The negative control failed before the guard changed. No package versions or modelling behavior change.

The portable-path controls also refuse `src/runtime.rs:hidden.rs`, which can name an NTFS alternate data stream outside directory enumeration. This is a conservative filesystem-proof guard; Cargo compilation from such a stream was not reproduced.

Symlinked package roots, source files and source directories disable the proof. The scanner does not follow them. Owned Unix fixtures cover all three forms; the package-root control failed before its explicit refusal was added. Native Windows link creation is not covered by these Unix-only fixtures; the shared refusal uses the standard library's link classification.
