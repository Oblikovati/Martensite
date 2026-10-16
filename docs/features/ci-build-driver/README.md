# CI and release driver

Martensite has one CI workflow, `.github/workflows/martensite.yml`. The
workflow only prepares the runner: it checks out the code, installs the
toolchain pinned in `rust-toolchain.toml`, and moves release files between
jobs. Every decision and every check is in the Rust program in
`scripts/build` (package `martensite-build`). The `cargo xtask` alias in
`.cargo/config.toml` runs it.

Before you run the driver locally, install the development dependencies with
`install-linux.sh`, `install-macos.sh` or `install-windows.ps1` (see
[dev-environment-installers](../dev-environment-installers/README.md)).

This gives two properties:

- A contributor runs the same pipeline as CI, with the same command.
- The rules that select the pipeline (trigger, matrix, publish or not) are
  ordinary Rust code with unit tests, not YAML expressions.

## Commands

| Command | Who runs it | What it does |
| --- | --- | --- |
| `cargo xtask pr [--base origin/main]` | contributor, CI | PR validation of the current branch |
| `cargo xtask release --version X.Y.Z [--target T]` | contributor | Release rehearsal for one target; never publishes |
| `cargo xtask ci plan` | CI | Reads the trigger and writes `mode`, `version`, `publish` and `matrix` to `GITHUB_OUTPUT` |
| `cargo xtask ci run --target T` | CI | Runs the planned pipeline for one matrix target |
| `cargo xtask ci publish --dist DIR` | CI | Checks every asset against its checksum and creates the GitHub release |

`ci run` reads the trigger again instead of getting the mode from the plan job.
The trigger is the same in both jobs, so the plan is the same.

## Triggers

| Trigger | Pipeline | Targets | Publishes |
| --- | --- | --- | --- |
| `pull_request` | PR validation against `origin/<base branch>` | primary | no |
| push of a `v*` tag | release of the tag's version | all | yes |
| `workflow_dispatch`, mode `pr` | PR validation against `origin/main` | primary | no |
| `workflow_dispatch`, mode `release` | release rehearsal of the `version` input | all | no |

Any other trigger fails in `ci plan` and names the trigger. Use the dispatch
release mode to run the full matrix on a release candidate before you push the
tag.

The release targets are in `scripts/build/src/targets.rs`. Each one builds on a
native runner, so the tests run on the target platform:

- `x86_64-unknown-linux-gnu` on `ubuntu-24.04` (primary)
- `aarch64-unknown-linux-gnu` on `ubuntu-24.04-arm`
- `aarch64-apple-darwin` on `macos-15`
- `x86_64-pc-windows-msvc` on `windows-2025`

## PR validation

1. Find the merge base of `HEAD` and the base branch. The change set is every
   file that differs between the merge base and the working tree, plus
   untracked files. A local run checks what the pushed PR will contain.
2. Check whitespace (`git diff --check`) and `rustfmt --check` on the changed
   files only. Both checks run before the step fails, so one run shows all
   problems.
3. Find the affected packages: the package that owns each changed file (the
   longest package directory that contains it), plus every workspace package
   that depends on an affected package. A change to `Cargo.toml`,
   `Cargo.lock`, `rust-toolchain.toml`, `rustfmt.toml`, `clippy.toml` or
   `.cargo/` affects the whole workspace. A change that touches no package
   (for example, documentation only) stops here.
4. Run clippy with `-D warnings`, a build of all targets, and the tests for
   the affected packages.

Whitespace checks retain Git diagnostics and fail on unreadable inputs or
unexpected results. Untracked regular-file symlinks use Git's link semantics;
directory symlinks that Git cannot compare are rejected with the command and
diagnostic. The check never changes a symlink or its target ([#75]).

[#75]: https://github.com/Oblikovati/Martensite/issues/75

## Release pipeline

1. Semver gate:
   - The tag is `vMAJOR.MINOR.PATCH[-PRERELEASE]`, without build metadata.
   - Every releasable package (all members except `martensite-build`) has
     exactly the release version.
   - `CHANGELOG.md` has a `## [X.Y.Z]` section with at least one heading.
   - The bump from the previous final release (the highest earlier `v*` tag
     that is not a pre-release) is at least what the headings require:
     `Fixed`/`Security` need a patch, `Added`/`Changed`/`Deprecated` need a
     minor, and `Removed` or a `BREAKING` marker needs a major. Before 1.0.0,
     a breaking change needs a minor bump, as cargo's compatibility rules
     expect. The first release has no baseline, so only the section is
     checked.
2. `cargo fmt --all --check`, then clippy and the full test suite (with doc
   tests) for the whole workspace on the target.
3. Release build of the releasable packages. The driver reads cargo's JSON
   messages to find executables and native libraries (`cdylib`, `staticlib`,
   `dylib`).
4. Packaging into `target/dist/v<version>/`:
   - `martensite-<version>-<target>.tar.gz` with `bin/`, `lib/`, `LICENSE`,
     `README.md` and `CHANGELOG.md`. The archive is reproducible: entries are
     sorted, and owners, timestamps and modes are fixed. A target without
     executables or native libraries gets no archive.
   - On the primary target only: one `.crate` source package per releasable
     package from `cargo package`. Cargo builds each package from its
     packaged sources to verify it. A kernel crate that depends on another
     unreleased kernel crate resolves it through cargo's local overlay
     registry, which cargo uses only for publishable packages, so kernel
     crates must not set `publish = false`.
   - A `<file>.sha256` file in `sha256sum` format next to each asset.

The publish job downloads the files from all targets, checks every asset
against its `.sha256`, and runs `gh release create v<version> --verify-tag`.
The release notes are the version's `CHANGELOG.md` section. Pre-release
versions are marked as pre-releases.

## Known limits

- The workflow installs only the Rust toolchain. The kernel crates also need
  Tcl/Tk, FreeType, fontconfig and Vulkan. When the first kernel crate lands,
  the workflow's setup step must run the matching installer
  (`install-linux.sh --no-cargo-tools` and so on) on each runner.
- `cargo deny` and `cargo audit` are not in the driver yet.
- No build cache in CI. Adding one (for example, `actions/cache` keyed on
  `Cargo.lock` and the target) is a change to the workflow only.
- The workspace member list is `["scripts/build"]` until the first kernel
  crate lands. Then add `"crates/*"` (cargo rejects a glob that matches no
  directory).
