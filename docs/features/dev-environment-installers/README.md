# Development environment installers

Three scripts in the repository root install the Martensite development
dependencies:

| Script | Platform | Package manager |
| --- | --- | --- |
| `install-linux.sh` | Debian, Ubuntu, Fedora, Arch, openSUSE | apt, dnf, pacman, zypper |
| `install-macos.sh` | macOS | Homebrew (installed when missing) |
| `install-windows.ps1` | Windows x64 and ARM64 | winget |

Each script is safe to run again. For each package it checks whether the
package is installed. It installs the missing packages and updates the
installed ones. On Arch, pacman upgrades the whole system (`pacman -Syu`),
because Arch does not support partial upgrades.

## What gets installed

| Dependency | Linux | macOS | Windows |
| --- | --- | --- | --- |
| C toolchain | distribution compiler packages | Xcode Command Line Tools | Visual Studio Build Tools, C++ workload |
| git, gh | packages | formulae | winget |
| pkg-config | package | `pkgconf` | `bloodrock.pkg-config-lite` |
| Tcl/Tk 8.6 | packages | `tcl-tk@8` | built from pinned source with MSVC |
| FreeType, fontconfig | packages | formulae | FreeType built from pinned source |
| Vulkan | loader, Mesa lavapipe, validation layers, vulkan-tools, glslang, SPIRV-Tools | loader, MoltenVK, validation layers, vulkan-tools, glslang, SPIRV-Tools | Vulkan SDK |
| Headless display | Xvfb, xauth | not needed | not needed |
| Rust | distribution `rustup`, else rustup.rs | Homebrew `rustup`, or an existing rustup.rs install | winget `Rustlang.Rustup`, or an existing install |
| Cargo tools | `cargo install --locked` | formulae | `cargo install --locked` |

The cargo tools are cargo-deny, cargo-audit, cargo-nextest and
cargo-llvm-cov. Each script also installs the toolchain from
`rust-toolchain.toml` with the `llvm-tools-preview` component, which
cargo-llvm-cov needs.

## Tcl/Tk module names

The kernel's Tcl build script asks pkg-config for the modules `tcl8.6` and
`tk8.6`. Only Debian and Ubuntu install `.pc` files with these names.
Fedora, Arch, openSUSE and Homebrew install `tcl.pc` and `tk.pc`, and
Homebrew's `tcl-tk@8` is keg-only, so it is not on pkg-config's default path.
On those platforms, the Unix installers write `tcl8.6.pc` and `tk8.6.pc` to
`~/.local/share/martensite/pkgconfig`. These files contain the flags of the
8.6 modules that the system has. They do not use a `Requires:` line, so they
cannot make a loop with the `tcl.pc` forwarder that the kernel's build script
writes. The installer stops with an error when the system Tcl is not 8.6.x.

On Fedora 42 and later, `tcl-devel` is Tcl 9. The installer selects the
`tcl8-devel` and `tk8-devel` compatibility packages when they exist.

## Environment

The Unix installers write `~/.local/share/martensite/env.sh` (or the same
path under `$XDG_DATA_HOME`). It puts rustup on `PATH` and, when the aliases
exist, adds them to `PKG_CONFIG_PATH`. On macOS it also sets `VK_LAYER_PATH`
for the Homebrew validation layers. Source it in your shell profile.

The Windows installer sets these variables for the current user:
`PKG_CONFIG_PATH`, `TCL_LIBRARY`, `TK_LIBRARY`, and `PATH` entries for the
Tcl/Tk and FreeType DLLs. It edits the registry value directly, so `%VAR%`
entries in your `PATH` stay unexpanded.

## Windows source builds

No Windows package ships Tcl/Tk or FreeType built with the MSVC ABI that Rust
links against. The installer builds them from the same commits as the kernel
CI:

| Library | Version | Commit |
| --- | --- | --- |
| Tcl | 8.6.18 | `515167c897a7c8066abfbd71e8fe7bd8bf3569de` |
| Tk | 8.6.18 | `6dbe68819aac4a22f6f7a143133ce26c5e8d2b05` |
| FreeType | 2.14.3 | `0a0221a1347e2f1e07c395263540026e9a0aa7c7` |

The output goes to `%LOCALAPPDATA%\Martensite\deps`. A `<name>.commit` marker
records the commit of each build, and the installer rebuilds a library only
when its pinned commit changes. FreeType is built without zlib, bzip2, PNG,
HarfBuzz and Brotli, as in the kernel CI.

## After installation

Load the environment (Linux and macOS), then check the setup with the same
gate that CI runs:

```bash
source ~/.local/share/martensite/env.sh
cargo xtask pr
```

See [ci-build-driver](../ci-build-driver/README.md) for the `cargo xtask`
commands.

## Verification

At the end, each installer checks that the required commands and pkg-config
modules are present. It also runs `vulkaninfo --summary` and stops with an
error when it finds no Vulkan device.

The Linux installer was tested in containers of Ubuntu 24.04, Debian 12,
Fedora, Arch and openSUSE Tumbleweed. Each test ran the installer twice: the
first run installs, the second run updates. The macOS installer was checked
with shellcheck only. The Windows installer was checked with the PowerShell
parser, PSScriptAnalyzer, and runs of its pure functions under PowerShell on
Linux. Neither was run on its target OS yet.

## Known limits

- RHEL-family distributions need EPEL for some packages; the installer does
  not configure extra repositories.
- Alpine (apk) and other package managers are not supported.
- On Windows, when Visual Studio Build Tools are installed without the C++
  workload, the installer does not add the workload. It stops and names the
  workload to add.
