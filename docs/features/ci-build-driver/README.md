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
| `cargo xtask ci plan` | CI | Writes `mode`, `version`, `publish`, target `matrix`, `test-matrix` and `has-tests` to `GITHUB_OUTPUT` |
| `cargo xtask ci run --target T` | CI | Runs the planned pipeline for one matrix target |
| `cargo xtask ci run --target T --checks-only` | CI | Runs shared checks and release packaging, delegating tests to the matrix |
| `cargo xtask ci test --package P --target T` | CI | Revalidates affected-package membership and runs all tests for that package |
| `cargo xtask ci test --shard N --target T` | CI | Recomputes a release shard and runs all selected package suites on that target |
| `cargo xtask ci gate --checks success --tests success --has-tests` | CI | Requires successful shared checks and package tests |
| `cargo xtask ci publish --dist DIR` | CI | Checks every asset against its checksum and creates the GitHub release |

`ci run` reads the trigger again instead of getting the mode from the plan job.
The trigger is the same in both jobs, so the plan is the same.

For PRs, the planner emits one job per affected package. Up to four test jobs
run concurrently, alongside shared lint, compiler inventory and build checks.
Each test job runs unit, integration and documentation tests with
`--no-fail-fast`; a failed job does not cancel the others. The existing
`pr (x86_64-unknown-linux-gnu)` check aggregates their results. Documentation-only
changes produce no package jobs; only that empty selection permits skipped tests.
Local `cargo xtask pr` still runs all stages sequentially. Release pipelines
compute package scope since the previous release and run the selected suites
on each supported target. CI partitions that selection into at most four shards
per platform, with up to sixteen release test jobs running concurrently. Each
package appears once per platform. TKGeomBase, TKPrim, TKGeomAlgo and TKTopAlgo
start separate shards when selected; the remaining packages are distributed in
sorted order. The limit bounds repeated dependency compilation across runners.
The shared checks and packaging run alongside tests. A required `release` result
checks both matrices before publication. Local `cargo xtask release` and `ci run`
without `--checks-only` retain the complete sequential pipeline.

When process-policy is selected, the shared test command explicitly enables
`martensite-process-policy/case-runner`. This preserves the unit tests that a
combined toolkit invocation enabled through dependency feature unification.
Host-math and optional external integration features remain disabled.

Each package job compiles its own dependencies. The four-job limit bounds
concurrency, not total runner minutes. Compare job compilation time, elapsed
workflow time and summed runner time before claiming a cost improvement.
The [timing investigation](parallel-tests.md) records the baseline and remaining
slow cases (#250).

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
- `x86_64-apple-darwin` on `macos-15-intel`
- `aarch64-apple-darwin` on `macos-15`
- `x86_64-pc-windows-msvc` on `windows-2025`

ARM64 release support is limited to macOS. Both macOS architectures run their
own native tests; Linux and Windows release support is x86_64 only ([#198]).
The [GitHub runner reference] lists `macos-15-intel` for Intel macOS hosts.

The current kernel packages are Rust source crates. Supporting both macOS
architectures does not produce a universal Mach-O binary. If native binary
assets are added, the current packager keeps each target separate; a universal
binary would need an explicit combine-and-verify step.

The macOS Intel compiler inventory must be captured on its native runner and
reviewed before release approval. A missing target baseline fails the release
gate; the ARM baseline must not be substituted.

[#198]: https://github.com/Oblikovati/Martensite/issues/198
[GitHub runner reference]: https://docs.github.com/en/actions/reference/runners/github-hosted-runners

## PR validation

1. Find the merge base of `HEAD` and the base branch. The change set is every
   file that differs between the merge base and the working tree, plus
   untracked files. A local run checks what the pushed PR will contain.
2. Check whitespace (`git diff --check`) and `rustfmt --check` on the changed
   files only. Both checks run before the step fails, so one run shows all
   problems.
3. For changes to crate Rust sources, the build driver, safe-toolkit evidence,
   Cargo manifests or lockfile, the pinned toolchain, `.cargo/`, or the workflow,
   collect expanded unsafe diagnostics and compare the host-target baseline.
   This runs even when no package is affected, including baseline-only changes.
4. Find the affected packages: the package that owns each changed file (the
   longest package directory that contains it), plus every workspace package
   that depends on an affected package. Manifest version bookkeeping does not
   select unchanged packages. Lockfile changes follow exact package identities
   and changed workspace dependency edges; package-specific optimization changes
   follow their dependency consumers. Global build configuration, toolchain or
   `.cargo/` changes affect the whole workspace. Formatting and lint settings do
   not select behavioral suites. A change that affects no package stops here
   after any required inventory check.
5. Run clippy with `-D warnings`, a build of all targets, and the tests for
   the affected packages.

Whitespace checks retain Git diagnostics and fail on unreadable inputs or
unexpected results. Untracked regular-file symlinks use Git's link semantics;
directory symlinks that Git cannot compare are rejected with the command and
diagnostic. The check never changes a symlink or its target ([#75]).

[#75]: https://github.com/Oblikovati/Martensite/issues/75

Frozen fixture paths use byte-preserving Git attributes so automatic line-ending
conversion cannot alter embedded inputs or expected answers. A real checkout
regression exercises this policy with `core.autocrlf=true` ([#197]).

[#197]: https://github.com/Oblikovati/Martensite/issues/197

## Release pipeline

1. Semver gate:
   - The tag is `vMAJOR.MINOR.PATCH[-PRERELEASE]`, without build metadata.
   - Each package is compared with the previous final release. Changed package
     source needs a newer version; unchanged source must keep its version.
     Own-version and local dependency requirement edits are bookkeeping, so they
     cannot justify a version bump by themselves (#256).
   - `CHANGELOG.md` has a `## [X.Y.Z]` section with at least one heading.
   - The bump from the previous final release (the highest earlier final GitHub release
     with a valid version tag) is at least what the headings require:
     `Fixed`/`Security` need a patch, `Added`/`Changed`/`Deprecated` need a
     minor, and `Removed` or a `BREAKING` marker needs a major. Before 1.0.0
     every level moves down one, as cargo's compatibility rules expect: an
     additive change (`Added`, `Deprecated`, a non-breaking `Changed`) needs
     only a patch bump, and a breaking change (`Removed` or a `BREAKING`
     marker) needs a minor bump. The first release has no baseline, so only
     the section is checked.
2. `cargo fmt --all --check` and workspace clippy, then the changed and affected
   package suites, including doctests, since the previous release. Tests include
   transitive dependents, without selecting base dependencies merely because
   they need compiling. Lockfile changes select consumers of changed external
   dependencies; package-specific optimization changes select their consumers;
   global build settings select all packages. The four supported targets remain
   mandatory. The first release tests the whole workspace.
3. Release build of the releasable packages. The driver reads cargo's JSON
   messages to find executables and native libraries (`cdylib`, `staticlib`,
   `dylib`).
4. Packaging into `target/dist/v<version>/`:
   - `martensite-<version>-<target>.tar.gz` with `bin/`, `lib/`,
     `LICENSE-APACHE`, `README.md` and `CHANGELOG.md`. The archive is
     reproducible: entries are sorted, and owners, timestamps and modes are
     fixed. A target without executables or native libraries gets no
     archive.
   - On the primary target only: changed-source and never-published `.crate`
     identities, named with each package's own version (#283). Complete paginated
     GitHub release and asset history establishes publication; a Git tag alone
     does not. Already-published identities reuse their verified immutable bytes.
     The schema-2 catalog records each publication reason, archive digest and
     historical origins separately from affected tests. Incomplete history,
     missing assets or inconsistent receipts stop the release. Cargo verifies all workspace
     source packages together so unchanged dependencies remain available in the
     local overlay registry. Cargo builds each package from its
     packaged sources to verify it. A kernel crate that depends on another
     unreleased kernel crate resolves it through cargo's local overlay
     registry, which cargo uses only for publishable packages, so kernel
     crates must not set `publish = false`. Each `.crate` must contain
     `LICENSE-GPL`, which cargo copies from the workspace `license-file`; the
     release stops when a crate does not set `license-file.workspace = true`.
     Test dependencies also declare versions. Keep them acyclic: a reciprocal
     development dependency can pass local tests but cannot be ordered for the
     temporary package registry (#241).
   - `martensite-<version>-packages.json` on the primary target records every
     package version, which source packages this release ships, the previous
     release baseline, and the selected test suites.
   - A `<file>.sha256` file in `sha256sum` format next to each asset.
5. Collect expanded unsafe diagnostics for the selected target and compare its
   compiler baseline. Collection cleans only its dedicated build cache and
   retains workspace and dependency diagnostics separately. CI uploads the
   report and raw compiler streams, including fresh evidence from failed checks.

The publish job requires the aggregate release result to pass. Failed, cancelled
or unexpectedly skipped tests block it. It downloads the files from all targets, checks every asset
against its `.sha256`, and runs `gh release create v<version> --verify-tag`.
The release notes are the version's `CHANGELOG.md` section. Pre-release
versions are marked as pre-releases.

## Known limits

- The shared target setup installs Rust and the FreeType test dependency on macOS and Windows. The kernel crates also need
  Tcl/Tk, FreeType, fontconfig and Vulkan. When the first kernel crate lands,
  the workflow's setup step must run the matching installer
  (`install-linux.sh --no-cargo-tools` and so on) on each runner.
- `cargo deny` and `cargo audit` are not in the driver yet.
- No build cache in CI. Adding one (for example, `actions/cache` keyed on
  `Cargo.lock` and the target) is a change to the workflow only.
- The workspace member list is `["scripts/build"]` until the first kernel
  crate lands. Then add `"crates/*"` (cargo rejects a glob that matches no
  directory).
