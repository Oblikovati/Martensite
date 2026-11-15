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
including transitive Rust modules. Custom build scripts, production targets
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
