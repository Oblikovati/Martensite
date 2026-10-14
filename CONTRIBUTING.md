# Contributing to Martensite

## Dev setup

Clone the repository, then run the installer for your OS from the repository
root. Each installer is safe to run again: it installs what is missing and
updates what is already installed.

```bash
git clone https://github.com/Oblikovati/Martensite.git
cd Martensite
./install-linux.sh        # apt, dnf, pacman or zypper
./install-macos.sh        # Homebrew
```

```powershell
powershell -ExecutionPolicy Bypass -File .\install-windows.ps1   # winget
```

The installers set up the C toolchain, Tcl/Tk 8.6, FreeType, fontconfig, the
Vulkan loader and tools (with Mesa lavapipe on Linux and MoltenVK on macOS),
rustup with the toolchain pinned in `rust-toolchain.toml`, and the cargo tools
the gates use. `--no-cargo-tools` (Linux) and `-NoCargoTools` (Windows) skip
the cargo tools, which take several minutes to compile. On Linux and macOS,
load the environment the installer writes before you build:

```bash
source ~/.local/share/martensite/env.sh
cargo build
cargo test
```

See [`docs/features/dev-environment-installers/`](docs/features/dev-environment-installers/README.md)
for what each installer does on each platform.

## Commit attribution

GitHub associates commits with accounts through the author email stored in
each commit. Before pushing a branch, inspect every commit that the pull request
will add:

```bash
git log --format='%h %an <%ae>' "$(git merge-base HEAD origin/main)"..HEAD
```

Use an email verified by your GitHub account, or its GitHub-provided `noreply`
address. Set it for this checkout when your global Git identity belongs to a
different project or employer:

```bash
git config --local user.name "Your Name"
git config --local user.email "your-verified-address@example.com"
```

Correct attribution mistakes on the pull-request branch before it is merged.
The project does not rewrite shared `main` history or published release tags
solely to change attribution because doing so invalidates commit hashes and
breaks existing clones and forks. Maintainers use [`.mailmap`](.mailmap) to
canonicalize accidental aliases without changing published commits.

## Required gates before push/merge

CI runs one workflow, `.github/workflows/martensite.yml`. It only installs the
toolchain; every check is the `scripts/build` driver, so you can run the same
gates locally before you push:

```bash
cargo xtask pr                        # what CI runs on a pull request
cargo xtask release --version 0.3.0   # release rehearsal for the host target
```

`cargo xtask pr` diffs your branch, including uncommitted and untracked work,
against `origin/main` (`--base` changes this). It checks whitespace and
`rustfmt` on the changed files, then runs clippy (`-D warnings`), a build and
the tests for the changed packages and every package that depends on them. A
change to `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `rustfmt.toml`,
`clippy.toml` or `.cargo/` checks the whole workspace.

`cargo deny check` (dependency policy) and `cargo audit` (known
vulnerabilities) are not part of the driver yet. The installers install both
tools; run them by hand when you add or update a dependency.

See [`docs/features/ci-build-driver/`](docs/features/ci-build-driver/README.md)
for the release pipeline and the CI trigger rules.

### The everyday loop

```bash
cargo build
cargo test                 # every test, including doc tests
cargo test -p <package>    # one package
cargo xtask pr             # the full PR gate, before you push
```

Every **user-facing** change must add a `CHANGELOG.md` entry under
`## [Unreleased]` in the same PR. User-facing means: a new CLI flag or
subcommand, env var, HTTP/admin endpoint, MCP tool or tool-response field,
`.ai-memory.toml` marker key, any changed behaviour or default, or an
observable bug fix. Internal refactors, dead-code removal, and test-only
churn are exempt.

This has been the most often forgotten obligation across review batches,
so reviewers treat a missing entry as **blocking**. Follow the existing entry style (past-tense summary,
trailing `([#NNN])` PR/issue reference) and place it under the right
`### Added` / `### Changed` / `### Fixed` heading.

The headings matter for releases: `cargo xtask release` reads the
`## [X.Y.Z]` section and refuses a version bump smaller than its headings
require (see [Releasing](#releasing)). Released sections are frozen; do not
edit them.

## Releasing

Releases need explicit maintainer approval for the version number and the tag.

1. Set the new version on every kernel crate, and rename `## [Unreleased]` in
   `CHANGELOG.md` to `## [X.Y.Z] - YYYY-MM-DD` with a new empty
   `## [Unreleased]` above it.
2. Rehearse the release locally for your host target:

   ```bash
   cargo xtask release --version X.Y.Z
   ```

   This checks the semver rules, runs fmt, clippy and the full test suite,
   builds in release mode, and writes the packages to `target/dist/vX.Y.Z/`.
   It never publishes.
3. Run the full matrix on the release-candidate commit: start the `Martensite`
   workflow manually (Actions tab, or `gh workflow run martensite.yml -f
   mode=release -f version=X.Y.Z --ref <branch>`) and wait until all targets
   are green. A manual run never publishes.
4. Push the tag `vX.Y.Z` on that commit. The workflow runs the release on
   every target again, then creates the GitHub release with the packages and
   the changelog section as notes.

[`docs/features/ci-build-driver/`](docs/features/ci-build-driver/README.md)
describes the semver rules, the targets and the packages.

## Workflow rules (condensed from AGENTS.md)

The full rules are in [`AGENTS.md`](AGENTS.md), the single
canonical agent/contributor rules file (`CLAUDE.md` is just a pointer to
it). Short version:

1. Work milestone by milestone. Do not start M(n+1) until every "Done when"
   bullet in M(n) passes (see `docs/design-decisions.md`).
2. No dead code, no half-built features. Stubs are documented with
   `// M<n> TODO` in the module doc-comment.
3. Write tests before claiming done. Parsers, ID derivation, and
   retention/decay math especially.
4. Do not refactor outside the milestone. Only touch what the current
   milestone requires.
5. Comments explain *why*, never *what*. No comments that restate the line
   above them.

## Cross-cutting invariants

Never violate any of the invariants in [`AGENTS.md`](AGENTS.md) (see the
"Rust Engineering Rules" and "Project Maintenance Rules" sections).
Highlights for contributors:

## Versioning and deprecation policy

This project follows [Semantic Versioning](https://semver.org/):

- **Patch** (`x.y.Z`): bug fixes that do not change public API or
  on-disk format.
- **Minor** (`x.Y.0`): additive changes; new CLI subcommands, new MCP
  tools, new config keys. Existing behaviour is preserved.
- **Major** (`X.0.0`): breaking changes. This includes on-disk format
  changes that are not handled by a migration, removal of CLI subcommands,
  or changes to the MCP tool schema that would break existing agents.

Breaking changes only ship in major releases. Deprecated items are
documented in the CHANGELOG under `### Deprecated` and removed no sooner
than the following major release.

### How this affects your PR

- Put your CHANGELOG entry under the heading that matches its semver
  impact: `### Fixed` for bug fixes, `### Added` for new capabilities,
  `### Changed` for altered behaviour. The maintainer reads the
  `[Unreleased]` section to pick the next version number, so a fix filed
  under `Added` (or vice versa) can bump the wrong release.
- If your change is **breaking** (on-disk format, removed/renamed
  surface, changed MCP schema), say so explicitly in the PR description
  so it gets the `breaking-change` label and is scheduled for the next
  major instead of blocking patch/minor releases.
- Bug fixes ship in the next **patch** release, usually promptly, and
  are not held for feature releases. Small additive features (a new
  agent harness, LLM provider, install client) ship in the next
  **minor**.
