# Production-library source SDK

Release assets include `martensite-<release>-source-workspace.tar.gz` and a
SHA-256 companion. This bundle contains the committed workspace sources,
configuration and lockfile, plus vendored locked Cargo dependencies. Each crate
keeps its own package version. The bundle's release number identifies the whole
workspace snapshot (#279).

Install the release's Rust toolchain and native platform dependencies first.
Verify the downloaded archive against its checksum, then extract it. Inside the
extracted top directory, build the production libraries with:

```bash
cargo build --locked --offline --workspace --lib --all-features
```

The bundle's `.cargo/config.toml` resolves registry dependencies from `vendor/`.
Cargo needs no registry download for this command. A Rust toolchain, compiler
linker and platform libraries are installed prerequisites.

For an application, depend on the extracted crates by path. Preserve their
relative directories. Cargo configuration is discovered from the application's
working directory, so an external application must also select the bundle's
vendor source. Copy its source-replacement tables into the application's Cargo
configuration and make the `directory` point to the extracted `vendor/` directory.
Keep the application's lockfile and use `--locked --offline` after generating
that lockfile offline.

## What the release verifies

The primary release target extracts the archive it just built, verifies every
payload hash and executable mode, and gives Cargo a fresh empty home and build
directory. These commands receive only an explicit installed-toolchain/native-SDK
environment, with private home/config directories and no inherited credentials,
compiler wrappers or Cargo flags. Ancestor Cargo configuration above the extracted
workspace and consumer is rejected. Verification uses two build jobs, disables
incremental compilation and debug symbols, and preserves debug assertions. It builds all production libraries with all features, then compiles
and runs an external consumer that passes foundation error, geometry-property
and topology types across crate boundaries and checks an empty measurement.
The committed lockfile must remain unchanged. The archive gets its acceptance
checksum only after these checks pass.

The archive includes tracked test sources and records the test-data gitlink.
It excludes the external dataset contents. Dataset-backed tests still require
the separately provisioned original data at the recorded revision, as in the
repository. The supported-platform test matrix remains a separate release gate.
The SDK check does not claim that all tests run offline.

## Individual crate archives

Changed packages still ship their independently versioned `.crate` files.
Unchanged historical archives keep their original bytes and requirements.
For example, a published requirement `martensite-tkernel = "0.3.6"` excludes
version 0.4.0. An updated path requirement in the current workspace does not
alter that historical archive.

The schema-2 `martensite-<release>-packages.json` catalog names the source
workspace as the supported source-consumption route. Its `source_workspace`
record contains the archive hash, revision, manifest hash and consumer scope.
The `individual_archives` record identifies the exact archive hashes and releases
whose normalized manifests were inspected. Its status is either
`requirements-compatible` or `incompatible`, with each incompatible internal
requirement reported. Missing archives, invalid checksums or malformed
requirements abort packaging. Requirement compatibility alone is not a compiled
consumer result for a collection of individual archives.

## Reproducibility and inputs

The source manifest lists package versions, locked dependency identities, the
unexpanded dataset revision and all payload hashes. It also records committed
input hashes, including the original configuration before the vendor-source
addition. Archive paths are sorted; owners, timestamps and file modes are fixed.
Staging rejects uncommitted changes, symbolic links, unsupported gitlinks,
ordinary files replacing the excluded dataset gitlink, and path escapes. Local caches, ignored worktrees and Git administration files are
excluded.

Vendoring may download locked dependencies during preparation. The acceptance
commands use private Cargo caches and offline Cargo resolution after preparation
has finished. Third-party vendor files retain their distribution notices.
