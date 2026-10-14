# Contributing to ai-memory

## Dev setup

```bash
gh repo clone Oblikovati/Martensite
cd Martensite
cargo build
cargo test
```

Rust 1.98.1 is required (pinned in `Cargo.toml`). The build is
self-contained:. No system libraries need installing beyond a standard C toolchain.

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

All of these must pass; CI enforces them and so does `bin/release`. The build
is self-contained, so none of them needs an environment variable.

```bash
cargo fmt --all -- --check
git diff --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test                            # every test (alias: cargo nextest run -P full)
cargo deny check                    # dependency policy
```

```bash
cargo install cargo-deny cargo-audit
```

### The everyday loop

```bash
cargo test
```

Every **user-facing** change must add a `CHANGELOG.md` entry under
`## [Unreleased]` in the same PR. User-facing means: a new CLI flag or
subcommand, env var, HTTP/admin endpoint, MCP tool or tool-response field,
`.ai-memory.toml` marker key, any changed behaviour or default, or an
observable bug fix. Internal refactors, dead-code removal, and test-only
churn are exempt.

This has been the most often forgotten obligation across review batches,
so reviewers treat a missing entry as **blocking**, and the PR template has a
checkbox for it. Follow the existing entry style (past-tense summary,
trailing `([#NNN])` PR/issue reference) and place it under the right
`### Added` / `### Changed` / `### Fixed` heading.

`CHANGELOG.md` holds the current major line only. Older lines are archived per
major version in [`docs/changelog/`](docs/changelog/); `bin/release` moves the
previous major there when it cuts an X.0.0. Released sections are frozen in
both places, and CI rejects an edit to any of them.

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