# Development environment installers

Three scripts in the repository root install the Martensite development
dependencies:

| Script | Platform | Package manager |
| --- | --- | --- |
| `install-linux.sh` | Ubuntu, Fedora, Arch, openSUSE | apt (Ubuntu only), dnf, pacman, zypper |
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
| Vulkan validation layer 1.4.313 or newer | checksum-pinned package on Ubuntu 24.04 x86_64 (`scripts/install/vulkan-validation.sh`); apt's `vulkan-validationlayers` on Ubuntu 25.10 and later; the distribution's package on Fedora, Arch and openSUSE | Homebrew's `vulkan-validationlayers` | Vulkan SDK |
| Headless display | Xvfb, xauth | not needed | not needed |
| Rust | rustup.rs on Ubuntu; the distribution's `rustup` elsewhere, else rustup.rs | Homebrew `rustup`, or an existing rustup.rs install | winget `Rustlang.Rustup`, or an existing install |
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
error when it finds no Vulkan device. On Ubuntu, the Linux installer also asks
the Vulkan loader for the validation layer's version, and stops with both
versions when it is older than 1.4.313.

The Linux installer was tested in containers of Ubuntu 24.04 and Ubuntu 26.04,
with `--no-cargo-tools`. Each test ran the installer twice: the first run
installs, the second run updates. They ended with layers 1.4.313 and 1.4.341. A
Debian 13 container was refused on both runs without changing any package. The
routes for other Ubuntu releases and for derivatives ran only as far as the route
choice, with a replaced `/etc/os-release` and `uname`, so Ubuntu 25.10 has no full
install test. The Fedora, Arch and openSUSE routes were not run against this
version. The macOS installer was checked
with shellcheck only. The Windows installer was checked with the PowerShell
parser, PSScriptAnalyzer, and runs of its pure functions under PowerShell on
Linux. Neither was run on its target OS yet.

## Known limits

- On apt, only Ubuntu is supported, along with systems that declare `ubuntu` in
  `ID_LIKE`. Other apt-based systems stop before any package changes, and so do
  Ubuntu releases before 25.10 other than 24.04, and 24.04 on anything but x86_64.
  Use Ubuntu 26.04 on those machines.
- A derivative needs `UBUNTU_CODENAME` set to noble, questing or resolute, because
  it numbers its own releases.
- Ubuntu 25.10 and later use the system's Mesa and get no pinned lavapipe, so
  `MARTENSITE_LAVAPIPE_ICD` is not set there. The recorded images are exact only on
  the pinned build, so image tests can differ on those releases.
- The installer does not check the layer version on Fedora, Arch or openSUSE.
  openSUSE Leap packages an older layer than validated startup accepts: 1.4.309 on
  16.0 and 1.3.275 on 15.6.
- RHEL-family distributions need EPEL for some packages; the installer does
  not configure extra repositories.
- Alpine (apk) and other package managers are not supported.
- On Windows, when Visual Studio Build Tools are installed without the C++
  workload, the installer does not add the workload. It stops and names the
  workload to add.
