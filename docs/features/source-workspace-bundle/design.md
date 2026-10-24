# Complete source-workspace release bundle

Status: approved Feature [#279](https://github.com/Oblikovati/Martensite/issues/279),
targeting `release/0.4`. See [SDK usage](README.md) for the artifact contract.

## Artifact contract

The release publishes `martensite-<release>-source-workspace.tar.gz` and its
SHA-256 companion on the primary target. Its top directory is
`martensite-<release>-source-workspace/`. The release number identifies the
workspace snapshot. Each crate retains its independently selected package version.

The archive contains the committed tracked workspace tree, vendored locked
dependencies, and a generated bundle manifest. The dataset gitlink is recorded
without expanding its contents. Consumers extract
it and use its crates by path, preserving the coherent workspace dependency graph.
Historical `.crate` assets retain their bytes and original dependency requirements.
The existing package list remains useful for locating those assets, but is not
described as a compatible standalone graph across package minor versions.

## Inputs established from the repository

- All 21 kernel packages declare explicit versions. `scripts/build` is a separate,
  unpublished 0.1.0 package. The root has no inherited package version.
- Root `Cargo.toml` owns build profiles, lints and inherited package metadata.
  `.cargo/config.toml` owns the `xtask` alias. `Cargo.lock` pins registry packages;
  the inspected baseline has no Git dependency sources.
- Crate tests embed tracked fixtures. The build driver's tests also embed crate
  fixtures, workflow YAML, attributes and an unsafe inventory document. Keeping
  the full tracked workspace avoids an incomplete hand-maintained allowlist.
- Dataset tests use `external/martensite-test-data/dataset-7.9.0` unless a caller
  overrides the data root. The baseline gitlink is
  `c58ee236484bce7b385392454f976d001f8443c4`, with 3,393 tracked files totaling
  365,721,568 bytes. These dataset files are excluded from the SDK artifact.
  Record the gitlink revision, preserve existing tracked notices, and document
  separate provisioning for dataset-backed tests. Vendor content retains its
  own distribution notices.
- Existing `archive.rs` writes sorted regular files with fixed ownership,
  permissions and timestamps, and already provides SHA-256 companions.
- `CommandRunner` supports child-specific environment overrides. It can give
  the verification process a private Cargo home and target directory.

## Preparation

1. Require the release candidate to be committed and clean. Record its full revision. Do not follow arbitrary symlinks
   or copy untracked files from the worktree.
2. Enumerate committed regular files and executable modes through Git. Read the
   committed blobs to preserve fixture bytes on every platform. Reject unsafe
   relative paths, symbolic links, unexpected gitlinks and duplicate archive paths.
   Record the supported dataset gitlink at its committed revision without reading
   or shipping its contents. Exclude
   Git administration, build products, secrets from the local environment and
   ignored worktrees by construction.
3. Stage the committed tree in a private temporary directory. Run Cargo's
   existing `vendor --locked --versioned-dirs` capability against that snapshot
   to obtain all locked dependency sources needed for supported targets. This
   preparation step may download locked dependencies. Dependency acquisition
   must finish before the offline consumer gate starts.
4. Preserve the tracked Cargo configuration and add only the generated source
   replacement tables for `vendor/`. Reject conflicting source configuration;
   do not silently override an existing replacement or an external path patch.
   Keep `Cargo.lock` byte-identical. Record both the committed config hash and
   the resulting bundle config hash, since the latter is an intentional addition.
5. Generate a schema-versioned manifest containing the release version, revision,
   every member's name/version/path, unexpanded submodule revision, locked registry package
   identities/checksums, and each payload's relative path, mode and SHA-256.
   The payload list excludes the manifest itself. Record the manifest's hash in
   the external release catalog; the archive checksum covers all archive bytes.
6. Write the archive using the existing deterministic writer. Extract that actual
   archive into a second temporary directory for verification. Check its complete
   member set and hashes before running any build.

No new library dependency is expected: Cargo supplies vendoring; `tar`, `flate2`,
`sha2`, `serde_json`, TOML parsing and process execution already exist in the driver.

## Consumer gate

The gate operates solely on the extracted archive. It has no Git checkout and
uses an empty private `CARGO_HOME`, a fresh target directory and
`CARGO_NET_OFFLINE=true`. Remove inherited Cargo source overrides, Rust flags,
wrapper variables and test-data overrides that could redirect it to developer
files. Keep the installed compiler and declared platform native-library discovery
available. Validate the installed compiler against the bundle's toolchain file;
the gate must not trigger a toolchain download.

Run locked offline Cargo metadata and assert that every workspace dependency
resolves within the extracted tree and every external source resolves to its
vendor directory. Verify the lockfile remains unchanged. Run `cargo build --locked --offline --workspace --lib --all-features` so
every production library is built from the bundle. The unpublished build driver
is included as tracked source, but its binary/tests and kernel test targets are
outside this production-library gate. Dataset-backed tests remain the separate
release-matrix responsibility.

Create a separate small consumer outside the extracted workspace. Its manifest
uses path dependencies on the extracted foundation, geometry, boundary and
topological packages. Supply the bundle's vendor configuration to that consumer,
generate its own lockfile offline, then run a locked offline test that passes actual shared geometry/topology/error
types across these boundaries and checks an elementary measurement. This catches
duplicate incompatible foundation identities that a name-only catalog cannot.
Use an explicit expected value from a closed form or empty-shape convention.
Keep the smoke consumer independent of a particular optional feature such as
the surface-policy API.

The gate proves production-library source completeness, dependency resolution
and representative consumption on the primary platform. Existing affected-package suites and the
four-target release matrix retain responsibility for kernel behavior and native
platform correctness. The bundle does not contain a Rust toolchain or operating
system libraries and does not promise a fully provisioned offline machine.

## Pipeline and catalog changes

Add focused source-bundle modules to the build driver and invoke them only when
`source_packages` is enabled. Build the bundle once on the primary target after
ordinary release checks. Do not include generated outputs in later input selection.
Verification failure aborts packaging and prevents publication; retain useful
diagnostics without accepting a partial archive.

Extend the release package catalog with an explicit schema version and a
`source_workspace` record containing the archive name, manifest identity, revision
and verified consumer scope. Keep existing package name/version/shipped fields.

Validate the exact individual archive graph separately. Obtain historical assets
from their recorded releases, verify their published checksums, and inspect the
normalized manifests inside those bytes together with newly produced archives.
Require each internal dependency requirement to admit its selected package
version, including optional, development and target-specific requirements.
Missing archives, invalid checksums, missing dependency identities or malformed
requirements abort packaging. A semver mismatch produces an explicit
`individual_archives.compatibility = "incompatible"` catalog record with the
package, requirement and selected version. It must never be labeled compatible.
Such a known incompatibility is allowed only when the complete SDK bundle
consumer gate passed and the catalog names that bundle as the supported source
consumption route. The catalog always names the verified SDK as its supported source route.
Preserve the old archives unchanged. A successful requirement check is labeled
`requirements-compatible` and is explicitly separate from an archive consumer build.

Document how to consume the bundle and explain that individual package assets
must satisfy their own unchanged manifest requirements. Stop instructing users
to treat one selected version per package as a verified archive closure.

Keep source-change classification and historical version-immutability guards
unchanged. This repair does not authorize a new crate version merely because
its local path requirement was updated.

## Verification budget

The bundle owner runs focused build-driver tests for path and identity validation,
regular-file extraction, unexpanded dataset identity, manifest/config/lock hashes,
catalog fields and fail-closed consumer errors. Named fake runners establish
command arguments and environment isolation. Two small real fixtures establish
that immutable historical requirements reject an incompatible candidate and
that the complete source workspace resolves and runs with an empty Cargo home.

Run bundle preparation twice from one immutable input and compare complete
archive bytes. Remove or alter one staged vendor file and one fixture separately
to establish that integrity and completeness failures block acceptance.
Cover exact-archive compatibility success, a historical incompatible requirement,
and missing/corrupt assets. Assert that incompatible or incomplete evidence can
never produce a compatible catalog claim. Do not
rewrite expectations or relax tolerances to make those checks pass.

Before pushing the feature, run the required local PR gate and strict build-driver
checks. After the coordinator selects actual release metadata, run one real
complete-bundle consumer gate on that final candidate and retain revisions,
commands, hashes and results. Reuse the existing platform matrix for its separate
claims rather than rerunning the entire kernel suite inside the smoke gate.

## Approved scope and remaining coordination

The coordinator approved the production-library SDK bundle, locked vendoring,
unexpanded dataset identity and existing notices, offline all-library/all-feature
build, and external consumer gate. Full supported-platform tests continue to use
the separately provisioned original datasets. Target `release/0.4` with an
Unreleased Added entry. Feature #279 owns this implementation. Package version selection, tags and
publication remain coordinator work.
