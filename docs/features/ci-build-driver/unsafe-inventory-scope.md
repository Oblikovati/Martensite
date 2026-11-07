# Unsafe inventory scope

PR validation selects the source ratchet and expanded compiler inventory
separately (#411). Unrelated driver modules and safety documentation no longer
trigger a clean compilation of the entire workspace. The ordinary affected-package
build, lint and test selection remains in place.

Both inventories run for changes anywhere under `crates/` or `external/`, including
non-Rust inputs, added files and deletions. Build scripts and include macros can
consume arbitrary bytes. Manifests, lockfiles, build scripts, toolchain and Cargo
configuration, native installers and CI setup also trigger both checks.

Changes to the source baseline or source collector run the source ratchet.
Changes to compiler baseline JSON files or the compiler collector run expanded
capture. Shared invocation, routing, reporting and error-handling modules trigger
both conservatively. The path rules live in
`scripts/build/src/inventory_impact.rs`; new generator input roots and collector
dependencies must be added there when introduced. Historical measurements and
prose under `docs/features/safe-toolkits/` do not select either inventory.

A selected expanded capture still cleans its dedicated target directory before
collecting workspace diagnostics for all targets. Cargo membership determines
ownership; revision, working-tree status and compiler identity remain recorded.
The optimization does not reuse cached diagnostics or change baseline acceptance.

PR validation removes `latest.json`, `compiler.stdout.jsonl` and
`compiler.stderr.txt` from `target/unsafe-inventory/` before querying Git or
running lint checks. Artifact upload therefore cannot publish an earlier
revision's reports when validation fails early or capture is skipped. Failure to
remove an existing report fails validation. This cleanup retains compilation
artifacts; only a selected capture cleans the dedicated build directory.
Release inventory capture remains mandatory and unconditional.

Regression controls cover independent baseline selection, arbitrary input paths,
added and deleted inputs, mixed change sets, fresh capture ordering, compiler
failure, early Git and lint failures, and stale-report removal. Commands are recorded by the existing fake
runner; filesystem fixtures use temporary directories.
