#!/usr/bin/env bash
# Installs or updates the Martensite development dependencies on macOS.
#
# Uses Homebrew for everything it can and installs Homebrew when it is
# missing. The script is safe to run again: missing formulae are installed
# and installed formulae are upgraded.
#
# What it sets up:
#   - Xcode Command Line Tools (C toolchain), git, pkgconf, gh
#   - Tcl/Tk 8.6 (tcl-tk@8), FreeType and fontconfig
#   - Vulkan loader, MoltenVK, validation layers, vulkaninfo, glslang and
#     SPIRV-Tools
#   - rustup, the toolchain pinned in rust-toolchain.toml, and the cargo
#     tools the gates use (cargo-deny, cargo-audit, cargo-nextest,
#     cargo-llvm-cov)
#
# Usage: ./install-macos.sh
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT
readonly INSTALLER_NAME=install-macos.sh
readonly BREW_FORMULAE=(
    git pkgconf gh tcl-tk@8 freetype fontconfig
    vulkan-loader vulkan-headers molten-vk vulkan-validationlayers vulkan-tools glslang spirv-tools
    cargo-deny cargo-audit cargo-nextest cargo-llvm-cov
)
readonly REQUIRED_COMMANDS=(git cc pkg-config gh glslangValidator spirv-val vulkaninfo cargo-deny cargo-nextest)
readonly REQUIRED_PKG_CONFIG_MODULES=(tcl8.6 tk8.6 freetype2 fontconfig)
# shellcheck source=scripts/install/unix-common.sh
source "$REPO_ROOT/scripts/install/unix-common.sh"

# `xcode-select --install` opens a GUI installer and returns at once, so the
# script stops and asks for a second run instead of waiting on it.
ensure_command_line_tools() {
    if xcode-select -p >/dev/null 2>&1; then
        log "Xcode Command Line Tools found (updates come from Software Update)"
        return
    fi
    xcode-select --install || true
    fail "finish the Xcode Command Line Tools installer, then run $INSTALLER_NAME again"
}

ensure_homebrew() {
    local candidate
    for candidate in brew /opt/homebrew/bin/brew /usr/local/bin/brew; do
        if command -v "$candidate" >/dev/null 2>&1; then
            eval "$("$candidate" shellenv)"
            log "Homebrew found; updating it"
            brew update
            return
        fi
    done
    log "Installing Homebrew"
    # HTTPS-only even across redirects (#69); the assignment lets `set -e`
    # stop on a failed download instead of running an empty script.
    local installer
    installer="$(curl --proto '=https' --tlsv1.2 -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    NONINTERACTIVE=1 /bin/bash -c "$installer"
    eval "$(/opt/homebrew/bin/brew shellenv 2>/dev/null || /usr/local/bin/brew shellenv)"
}

# Keeps an existing rustup.rs installation instead of adding a second rustup.
wanted_formulae() {
    printf '%s\n' "${BREW_FORMULAE[@]}"
    if [[ ! -x $HOME/.cargo/bin/rustup ]]; then
        echo rustup
    fi
}

sync_formulae() {
    local formula
    local -a missing=() present=()
    while read -r formula; do
        if brew list --formula --versions "$formula" >/dev/null; then present+=("$formula"); else missing+=("$formula"); fi
    done < <(wanted_formulae)
    log "Homebrew formulae: ${#missing[@]} to install, ${#present[@]} to upgrade"
    if ((${#missing[@]})); then brew install "${missing[@]}"; fi
    if ((${#present[@]})); then brew upgrade "${present[@]}"; fi
}

# Homebrew's rustup is keg-only, and the validation layers need
# VK_LAYER_PATH (see `brew info vulkan-validationlayers`).
write_macos_env_file() {
    # shellcheck disable=SC2016 # expanded when the env file is sourced
    local rustup_bin='$HOME/.cargo/bin'
    if [[ ! -x $HOME/.cargo/bin/rustup ]]; then
        rustup_bin="$(brew --prefix rustup)/bin"
    fi
    write_env_file \
        "export PATH=\"$rustup_bin:\$PATH\"" \
        "export VK_LAYER_PATH=\"$HOMEBREW_PREFIX/opt/vulkan-validationlayers/share/vulkan/explicit_layer.d\""
}

main() {
    local option=${1:-}
    case $option in
        '') ;;
        -h | --help) print_usage "${BASH_SOURCE[0]}"; return ;;
        *) fail "unknown option '$option'; expected --help" ;;
    esac
    ensure_command_line_tools
    ensure_homebrew
    sync_formulae
    TCL_SOURCE_PKG_CONFIG_PATH="$(brew --prefix tcl-tk@8)/lib/pkgconfig"
    ensure_tcl_module_names
    write_macos_env_file
    install_pinned_toolchain
    verify_installation
    log "Done. Load the environment in new shells with: source $ENV_FILE"
}

main "$@"
