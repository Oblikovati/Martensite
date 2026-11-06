#!/usr/bin/env bash
# Installs or updates the Martensite development dependencies on Linux.
#
# Supported package managers: apt (Ubuntu only), dnf (Fedora), pacman (Arch)
# and zypper (openSUSE). The script is safe to run again: missing packages
# are installed and installed packages are updated.
#
# Validated Vulkan startup needs validation layer 1.4.313 or newer. On apt the
# script stops before changing anything unless the system is Ubuntu, or declares
# ubuntu in ID_LIKE, and has a route to that layer: the checksum-pinned package
# on 24.04 x86_64, or apt's package on 25.10 and later. It then checks the
# installed layer at the end.
#
# What it sets up:
#   - C toolchain, git, pkg-config, gh
#   - Tcl/Tk 8.6, FreeType, fontconfig and X11 development files
#   - Vulkan loader, Mesa lavapipe (software driver), validation layers,
#     vulkaninfo, glslang and SPIRV-Tools
#   - Xvfb and xauth for headless renderer tests
#   - rustup, the toolchain pinned in rust-toolchain.toml, and the cargo
#     tools the gates use (cargo-deny, cargo-audit, cargo-nextest,
#     cargo-llvm-cov)
#
# Usage: ./install-linux.sh [--no-cargo-tools]
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT
readonly INSTALLER_NAME=install-linux.sh
readonly CARGO_TOOLS=(cargo-deny cargo-audit cargo-nextest cargo-llvm-cov)
readonly REQUIRED_COMMANDS=(git cc pkg-config gh glslangValidator spirv-val vulkaninfo Xvfb xauth)
readonly REQUIRED_PKG_CONFIG_MODULES=(tcl8.6 tk8.6 freetype2 fontconfig x11)
readonly REQUIRED_VALIDATION_LAYER=1.4.313
# shellcheck source=scripts/install/unix-common.sh
source "$REPO_ROOT/scripts/install/unix-common.sh"

as_root() {
    if [[ $EUID -eq 0 ]]; then
        "$@"
    else
        sudo "$@"
    fi
}

detect_package_manager() {
    local manager
    for manager in apt-get dnf pacman zypper; do
        if command -v "$manager" >/dev/null 2>&1; then
            echo "$manager"
            return
        fi
    done
    fail "no supported package manager found; expected one of apt-get, dnf, pacman, zypper"
}

# --- Vulkan validation ------------------------------------------------------

# How the apt route gets the layer: pinned or packaged. Empty on other package managers.
LAYER_ROUTE=

os_release_value() {
    [[ -r /etc/os-release ]] || fail "cannot read /etc/os-release; expected it on an apt-based system"
    sed -n "s/^$1=//p" /etc/os-release | tr -d '"'
    return 0
}

# Prints the Ubuntu release behind this system: nothing when it is not based on
# Ubuntu, and "unknown" for a derivative whose base release is not listed here.
ubuntu_release() {
    if [[ "$(os_release_value ID)" == ubuntu ]]; then
        os_release_value VERSION_ID
        return
    fi
    [[ " $(os_release_value ID_LIKE) " == *" ubuntu "* ]] || return 0
    # A derivative numbers its own releases, so its base comes from the codename.
    case "$(os_release_value UBUNTU_CODENAME)" in
        noble) echo 24.04 ;;
        questing) echo 25.10 ;;
        resolute) echo 26.04 ;;
        *) echo unknown ;;
    esac
}

# Runs before the first package change, so a refused system stays as it was.
select_ubuntu_layer_route() {
    local release machine name
    release="$(ubuntu_release)"
    machine="$(uname -m)"
    name="$(os_release_value PRETTY_NAME)"
    if [[ -z $release ]]; then
        fail "$name is not Ubuntu; use Ubuntu with this installer, because a packaged validation layer elsewhere on apt cannot be relied on to meet the required $REQUIRED_VALIDATION_LAYER"
    elif [[ $release == unknown ]]; then
        fail "cannot tell which Ubuntu release $name is based on; expected UBUNTU_CODENAME noble, questing or resolute in /etc/os-release"
    elif [[ $release == 24.04 && $machine == x86_64 ]]; then
        LAYER_ROUTE=pinned
    elif [[ $release == 24.04 ]]; then
        fail "Ubuntu 24.04 on $machine has no route to validation layer $REQUIRED_VALIDATION_LAYER: the pinned package is x86_64 only and apt has 1.3.275; use Ubuntu 26.04"
    elif dpkg --compare-versions "$release" ge 25.10; then
        LAYER_ROUTE=packaged
    else
        fail "Ubuntu $release has no route to validation layer $REQUIRED_VALIDATION_LAYER; use Ubuntu 24.04 on x86_64, or Ubuntu 25.10 or later"
    fi
    log "Ubuntu $release on $machine: $LAYER_ROUTE validation layer"
}

validation_layer_environment() {
    case $LAYER_ROUTE in
        pinned) "$REPO_ROOT/scripts/install/vulkan-validation.sh" "$MARTENSITE_DATA/vulkan-validation" ;;
        packaged) ;; # The loader finds the system package without any environment.
        *) log "Skipping the pinned Vulkan validation layer: validated Vulkan startup needs layer $REQUIRED_VALIDATION_LAYER or newer" >&2 ;;
    esac
}

# Asks the loader, so this reads the layer that validated startup would enable.
require_verified_validation_layer() {
    local summary installed
    summary="$(vulkaninfo --summary 2>/dev/null || true)"
    installed="$(sed -n 's/^VK_LAYER_KHRONOS_validation .* \([0-9][0-9.]*\) *version .*/\1/p' <<<"$summary")"
    [[ -n $installed ]] || fail "the Vulkan loader lists no VK_LAYER_KHRONOS_validation; expected $REQUIRED_VALIDATION_LAYER or newer"
    dpkg --compare-versions "$installed" ge "$REQUIRED_VALIDATION_LAYER" ||
        fail "validation layer is $installed; expected $REQUIRED_VALIDATION_LAYER or newer"
    log "Validation layer $installed"
}

# --- Package lists ----------------------------------------------------------

apt_packages() {
    echo build-essential git curl ca-certificates pkg-config \
        tcl8.6-dev tk8.6-dev libfreetype-dev libfontconfig-dev libx11-dev \
        libvulkan-dev mesa-vulkan-drivers fonts-dejavu fonts-dejavu-extra fonts-droid-fallback fonts-freefont-ttf fonts-liberation fonts-noto-cjk fonts-noto-core vulkan-validationlayers vulkan-tools \
        glslang-tools spirv-tools xvfb xauth fonts-dejavu-core gh
    # Ubuntu's rustup package is left out on purpose: ensure_rustup installs it from rustup.rs.
    # The pinned driver is built on Ubuntu 24.04 and links these; later releases rename some.
    if [[ $LAYER_ROUTE == pinned ]]; then
        echo libdisplay-info1 libsensors5 libdrm-intel1
    fi
}

dnf_packages() {
    # Fedora 42 moved tcl-devel to Tcl 9; the kernel needs the 8.6 compat packages.
    local tcl=(tcl-devel tk-devel)
    if dnf -q info tcl8-devel >/dev/null 2>&1; then
        tcl=(tcl8-devel tk8-devel)
    fi
    echo gcc gcc-c++ make git curl pkgconf-pkg-config "${tcl[@]}" \
        freetype-devel fontconfig-devel libX11-devel \
        vulkan-loader-devel mesa-vulkan-drivers vulkan-validation-layers vulkan-tools \
        glslang spirv-tools xorg-x11-server-Xvfb xorg-x11-xauth dejavu-sans-fonts gh rustup
}

pacman_packages() {
    echo base-devel git curl pkgconf tcl tk freetype2 fontconfig libx11 \
        vulkan-icd-loader vulkan-headers vulkan-swrast vulkan-validation-layers vulkan-tools \
        glslang spirv-tools xorg-server-xvfb xorg-xauth ttf-dejavu github-cli rustup
}

zypper_packages() {
    echo gcc gcc-c++ make git curl pkgconf-pkg-config tcl-devel tk-devel \
        freetype2-devel fontconfig-devel libX11-devel \
        vulkan-devel libvulkan_lvp vulkan-validationlayers vulkan-tools \
        glslang-devel spirv-tools xorg-x11-server-Xvfb xauth dejavu-fonts gh rustup
}

# --- Package manager operations -------------------------------------------

is_installed() {
    local manager=$1 package=$2
    case $manager in
        apt-get) dpkg-query -W -f='${Status}' "$package" 2>/dev/null | grep -q 'ok installed' ;;
        dnf | zypper) rpm -q "$package" >/dev/null 2>&1 ;;
        pacman) pacman -Q "$package" >/dev/null 2>&1 ;;
        *) fail "unsupported package manager '$manager'; expected apt-get, dnf, pacman or zypper" ;;
    esac
}

refresh_package_index() {
    local manager=$1
    case $manager in
        apt-get) as_root apt-get update ;;
        dnf) as_root dnf -y makecache ;;
        zypper) as_root zypper --non-interactive refresh ;;
        pacman) ;; # `pacman -Syu` below refreshes and upgrades in one step.
        *) fail "unsupported package manager '$manager'; expected apt-get, dnf, pacman or zypper" ;;
    esac
}

install_packages() {
    local manager=$1
    shift
    case $manager in
        apt-get) as_root env DEBIAN_FRONTEND=noninteractive apt-get install -y "$@" ;;
        dnf) as_root dnf install -y "$@" ;;
        zypper) as_root zypper --non-interactive install "$@" ;;
        *) fail "unsupported package manager '$manager'; expected apt-get, dnf, pacman or zypper" ;;
    esac
}

update_packages() {
    local manager=$1
    shift
    case $manager in
        apt-get) as_root env DEBIAN_FRONTEND=noninteractive apt-get install -y --only-upgrade "$@" ;;
        dnf) as_root dnf upgrade -y "$@" ;;
        zypper) as_root zypper --non-interactive update "$@" ;;
        *) fail "unsupported package manager '$manager'; expected apt-get, dnf, pacman or zypper" ;;
    esac
}

# Splits the package list into missing and installed, then installs the
# missing ones and updates the rest. Arch does not support partial upgrades,
# so pacman upgrades the whole system together with the new packages.
sync_system_packages() {
    local manager=$1 package
    local -a wanted missing=() present=()
    read -r -a wanted <<<"$("${manager%-get}_packages" | tr '\n' ' ')"
    for package in "${wanted[@]}"; do
        if is_installed "$manager" "$package"; then present+=("$package"); else missing+=("$package"); fi
    done
    log "System packages: ${#missing[@]} to install, ${#present[@]} to update"
    if [[ $manager == pacman ]]; then
        as_root pacman -Syu --needed --noconfirm "${wanted[@]}"
        return
    fi
    if ((${#missing[@]})); then install_packages "$manager" "${missing[@]}"; fi
    if ((${#present[@]})); then update_packages "$manager" "${present[@]}"; fi
}

# --- Rust -------------------------------------------------------------------

# Prefers the distribution's rustup (installed above when it exists) and
# falls back to the official rustup.rs installer.
ensure_rustup() {
    export PATH="$HOME/.cargo/bin:$PATH"
    if command -v rustup >/dev/null 2>&1; then
        log "rustup found; updating it"
        # A distribution-built rustup disables self-update; its package manager updated it above.
        rustup self update 2>/dev/null || log "rustup is managed by the system package manager"
        return
    fi
    log "Installing rustup from rustup.rs"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none
    command -v rustup >/dev/null 2>&1 || fail "rustup is not on PATH after installation; expected it in ~/.cargo/bin"
}

sync_cargo_tools() {
    local tool
    for tool in "${CARGO_TOOLS[@]}"; do
        if cargo install --list | grep -q "^$tool v"; then
            log "Updating $tool"
        else
            log "Installing $tool"
        fi
        # `cargo install` rebuilds only when crates.io has a newer version.
        (cd "$REPO_ROOT" && cargo install --locked "$tool")
    done
}

main() {
    local option=${1:-} cargo_tools=true manager
    case $option in
        '') ;;
        --no-cargo-tools) cargo_tools=false ;;
        -h | --help) print_usage "${BASH_SOURCE[0]}"; return ;;
        *) fail "unknown option '$option'; expected --no-cargo-tools or --help" ;;
    esac
    manager="$(detect_package_manager)"
    log "Using $manager"
    if [[ $manager == apt-get ]]; then select_ubuntu_layer_route; fi
    refresh_package_index "$manager"
    sync_system_packages "$manager"
    ensure_tcl_module_names
    # shellcheck disable=SC2016 # expanded when the env file is sourced
    write_env_file 'export PATH="$HOME/.cargo/bin:$PATH"' "$(validation_layer_environment)"
    ensure_rustup
    install_pinned_toolchain
    if $cargo_tools; then sync_cargo_tools; fi
    verify_installation
    if [[ $manager == apt-get ]]; then require_verified_validation_layer; fi
    log "Done. Load the environment in new shells with: source $ENV_FILE"
}

main "$@"
