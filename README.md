# Martensite

Martensite is a Geometry Kernel providing services for the 3D surface and solid modeling.

It started as a 1:1 porting of OpenCASCADE APIs to Rust with OpenCASCADE test suite as
validation oracle. Providing a stable and production tested set of toolkits on which
Martensite can build on top and innovate.

Martensite has no commitment to keep API parity with OpenCASCADE.

The improvememt roadmap for this project can be found on [Milestones](https://github.com/Oblikovati/Martensite/milestones)

- [Contributing](CONTRIBUTING.md)
- [Agentic Development Governance](AGENTS.md)

## Toolkits

| Crate | Capabilities |
| --- | --- |
| `martensite-cmath` | Platform C math functions behind a typed Rust interface. |
| `martensite-tkernel` | Errors, memory, collections, strings, operating-system services, quantities and units. |
| `martensite-tkexpress` | Symbolic expressions, differentiation, relations and a text interpreter. |
| `martensite-tkg2d` | Planar curves, splines, offsets, adaptors, evaluation and hashing. |
| `martensite-tkmath` | Geometry, linear algebra, solvers, splines, bounding volumes, meshes and exact predicates. |

See the [foundation guide](docs/features/martensite-tkernel/README.md) and
[math guide](docs/features/martensite-tkmath/README.md), and
[symbolic expression guide](docs/features/martensite-tkexpress/README.md), and
[planar geometry guide](docs/features/martensite-tkg2d/README.md) for module and test details.

## Getting started

Install the development dependencies with the installer for your OS. Each
installer can run again later to update them.

```bash
./install-linux.sh                          # Debian, Ubuntu, Fedora, Arch, openSUSE
./install-macos.sh                          # macOS (Homebrew)
source ~/.local/share/martensite/env.sh     # Linux and macOS, in each new shell
```

```powershell
powershell -ExecutionPolicy Bypass -File .\install-windows.ps1   # Windows (winget)
```

Build and test with cargo. Before you push, run the same gate that CI runs:

```bash
cargo build
cargo test
cargo xtask pr
```

CI has one workflow, `.github/workflows/martensite.yml`, and all its checks are
`cargo xtask` commands. [CONTRIBUTING](CONTRIBUTING.md) explains the gates and
the release process.

PR validation checks toolkit Rust source tokens against
`docs/features/safe-toolkits/unsafe-source-baseline.json` when source, build
tooling or inventory inputs change. Unsafe removals must update that inventory;
increases require review against the temporary exception policy in `AGENTS.md`.
The check includes inactive platform code and macro definitions, but does not
expand macros or inspect dependencies. Compiler checks remain necessary (#81).

The source code is licensed GPL-2.0-only (`LICENSE-GPL`). Official release
binaries downloaded from this repository whose files match a published hash are
also offered under the Apache License 2.0 (`LICENSE-APACHE`). Each binary
archive contains `LICENSE-APACHE`, and each source package contains
`LICENSE-GPL`. Source files carry no per-file copyright or license header.
