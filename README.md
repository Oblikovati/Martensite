# Martensite

Martensite is a Geometry Kernel providing services for the 3D surface and solid modeling.

It started as a 1:1 porting of OpenCASCADE APIs to Rust with OpenCASCADE test suite as
validation oracle. Providing a stable and production tested set of toolkits on which
Martensite can build on top and innovate.

Martensite has no commitment to keep API parity with OpenCASCADE.

The improvememt roadmap for this project can be found on [Milestones](https://github.com/Oblikovati/Martensite/milestones)

- [Contributing](CONTRIBUTING.md)
- [Agentic Development Governance](AGENTS.md)

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

The project is licensed GPL-2.0-only (`LICENSE`). Source files carry no per-file
copyright or license header.
