# shellcheck shell=bash
# Shared steps of install-linux.sh and install-macos.sh. Source this file
# after setting REPO_ROOT, INSTALLER_NAME and the REQUIRED_COMMANDS and
# REQUIRED_PKG_CONFIG_MODULES arrays; it defines functions only.
#
# Optional variable read by the Tcl/Tk steps:
#   TCL_SOURCE_PKG_CONFIG_PATH  where the Tcl/Tk 8.6 .pc files are when they
#                               are not on pkg-config's default path (for
#                               example, Homebrew's keg-only tcl-tk@8)

readonly MARTENSITE_DATA="${XDG_DATA_HOME:-$HOME/.local/share}/martensite"
readonly SHIM_DIR="$MARTENSITE_DATA/pkgconfig"
readonly ENV_FILE="$MARTENSITE_DATA/env.sh"

log() {
    printf '==> %s\n' "$*"
    return 0
}
fail() {
    printf 'error: %s\n' "$*" >&2
    exit 1
}

# --- Rust -------------------------------------------------------------------

# Reads `key = "value"` or `key = ["a", "b"]` from rust-toolchain.toml.
toolchain_setting() {
    local key=$1 file="$REPO_ROOT/rust-toolchain.toml"
    [[ -f $file ]] || fail "missing $file; expected the pinned toolchain file"
    sed -n "s/^$key *= *//p" "$file" | tr -d '"[] '
    return 0
}

# Installs the pinned toolchain by name, so this works with rustup releases
# older than 1.28 that cannot install from rust-toolchain.toml directly.
install_pinned_toolchain() {
    local channel components
    channel="$(toolchain_setting channel)"
    components="$(toolchain_setting components)"
    [[ -n $channel ]] || fail "no channel in $REPO_ROOT/rust-toolchain.toml; expected channel = \"X.Y.Z\""
    log "Installing Rust $channel with $components and llvm-tools-preview"
    # llvm-tools-preview is what cargo-llvm-cov needs for the coverage gate.
    rustup toolchain install "$channel" --profile minimal --component "$components,llvm-tools-preview"
    return 0
}

# --- Tcl/Tk pkg-config names -------------------------------------------------

# True when a .pc file named $1 is on the search path. `--exists` is wrong
# here: Debian's tk8.6.pc requires a `tcl` module that only the kernel's
# build script provides (it writes a `tcl.pc` forwarder at build time).
has_pkg_config_module() {
    # Capture first: under pipefail, `grep -q` exiting early would SIGPIPE the
    # producer and fail the check; one unreadable .pc must not hide the rest.
    local module=$1 modules
    modules="$(pkg-config --list-all 2>/dev/null || true)"
    if awk '{ print $1 }' <<<"$modules" | grep -x "$module" >/dev/null; then
        return 0
    fi
    return 1
}

tcl_pkg_config() {
    PKG_CONFIG_PATH="${TCL_SOURCE_PKG_CONFIG_PATH:-}" pkg-config "$@"
    return $?
}

# The kernel asks pkg-config for `tcl8.6` and `tk8.6`. Only Debian and Ubuntu
# ship those names on the default search path; Fedora, Arch, openSUSE and
# Homebrew ship `tcl`/`tk`, and Homebrew keeps them off the default path.
# For those, write `tcl8.6.pc`/`tk8.6.pc` with the resolved 8.6 flags.
ensure_tcl_module_names() {
    if PKG_CONFIG_PATH='' has_pkg_config_module tcl8.6 && PKG_CONFIG_PATH='' has_pkg_config_module tk8.6; then
        log "pkg-config finds tcl8.6 and tk8.6"
        rm -f "$SHIM_DIR/tcl8.6.pc" "$SHIM_DIR/tk8.6.pc"
        return
    fi
    mkdir -p "$SHIM_DIR"
    write_module_shim tcl8.6 tcl
    write_module_shim tk8.6 tk
}

# Copies the flags (not a `Requires:` line) so the alias cannot form a cycle
# with the `tcl.pc` forwarder that the kernel's build script writes.
write_module_shim() {
    local name=$1 source=$2 version
    if PKG_CONFIG_PATH="${TCL_SOURCE_PKG_CONFIG_PATH:-}" has_pkg_config_module "$name"; then source=$name; fi
    version="$(tcl_pkg_config --modversion "$source" 2>/dev/null || true)"
    [[ $version == 8.6* ]] || fail "pkg-config module '$source' is '${version:-missing}'; expected Tcl/Tk 8.6.x"
    log "Writing $SHIM_DIR/$name.pc (from $source $version)"
    printf 'Name: %s\nDescription: Martensite alias for %s, written by %s\nVersion: %s\nLibs: %s\nCflags: %s\n' \
        "$name" "$source" "$INSTALLER_NAME" "$version" \
        "$(tcl_pkg_config --libs "$source")" "$(tcl_pkg_config --cflags "$source")" >"$SHIM_DIR/$name.pc"
    return 0
}

# --- Environment and verification -------------------------------------------

# Writes the env file from the given `export ...` lines plus the pkg-config
# alias directory, then loads it into this shell.
write_env_file() {
    mkdir -p "$MARTENSITE_DATA"
    {
        echo "# Martensite development environment, written by $INSTALLER_NAME."
        printf '%s\n' "$@"
        if [[ -f $SHIM_DIR/tcl8.6.pc ]]; then
            echo "export PKG_CONFIG_PATH=\"$SHIM_DIR\${PKG_CONFIG_PATH:+:\$PKG_CONFIG_PATH}\""
        fi
    } >"$ENV_FILE"
    # shellcheck source=/dev/null
    source "$ENV_FILE"
    return 0
}

# Fails naming every command and pkg-config module that is still missing.
verify_installation() {
    local missing=() name
    for name in "${REQUIRED_COMMANDS[@]}" rustup cargo; do
        command -v "$name" >/dev/null 2>&1 || missing+=("command $name")
    done
    for name in "${REQUIRED_PKG_CONFIG_MODULES[@]}"; do
        has_pkg_config_module "$name" || missing+=("pkg-config module $name")
    done
    ((${#missing[@]} == 0)) || fail "missing after installation: ${missing[*]}"
    log "Vulkan devices:"
    vulkaninfo --summary 2>/dev/null | grep -E 'deviceName|driverName' ||
        fail "vulkaninfo found no Vulkan device; expected at least one driver"
    return 0
}

# Prints the comment block at the top of the calling script.
print_usage() {
    local script=$1
    sed -n '2,/^set -euo/p' "$script" | sed -e '$d' -e 's/^# \{0,1\}//'
    return 0
}
