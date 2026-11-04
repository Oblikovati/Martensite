#!/usr/bin/env bash
# Installs the checksum-pinned Kache compiler cache for local builds on Linux x86_64.
# It stays opt-in: nothing uses it until `RUSTC_WRAPPER=kache` is set for a build.
#
# It does not run `kache init`, which sets the wrapper for every Cargo project on the machine.
# It also turns off the daemon's target cleanup, which deletes and replaces files in target
# directories, including ones that belong to other work on the machine.
#
# Usage: scripts/install/kache.sh [BIN_DIRECTORY]
set -euo pipefail

readonly VERSION="v2.0.0"
readonly ARCHIVE="kache-x86_64-unknown-linux-musl.tar.gz"
readonly URL="https://github.com/kunobi-ninja/kache/releases/download/$VERSION/$ARCHIVE"
readonly ARCHIVE_SHA256="6c59665faf1b14d0944391e6f2a1b510195e6be92c63060a9d1770ed00676824"

fail() {
    printf 'error: %s\n' "$*" >&2
    exit 1
}

main() {
    local bin_directory=${1:-"$HOME/.cargo/bin"} temporary actual
    [[ "$(uname -sm)" == "Linux x86_64" ]] || fail "got $(uname -sm); expected Linux x86_64"
    temporary="$(mktemp -d)"
    # shellcheck disable=SC2064 # The local path must expand now; the trap runs after main returns.
    trap "rm -rf '$temporary'" EXIT
    curl --proto "=https" --proto-redir "=https" --fail --location --silent --show-error \
        --output "$temporary/$ARCHIVE" "$URL"
    actual="$(sha256sum "$temporary/$ARCHIVE" | cut -d' ' -f1)"
    [[ $actual == "$ARCHIVE_SHA256" ]] || fail "$ARCHIVE has SHA-256 $actual; expected $ARCHIVE_SHA256"
    tar -xzf "$temporary/$ARCHIVE" -C "$temporary" kache kache-engine
    mkdir -p "$bin_directory"
    install -m 755 "$temporary/kache" "$temporary/kache-engine" "$bin_directory/"
    local setting
    for setting in cache.auto_clean_orphaned_targets cache.auto_share_target_files cache.auto_recover_targets; do
        "$bin_directory/kache" config set --global "$setting" false >&2
    done
    "$bin_directory/kache" config set --global cache.auto_clean_unused_units_days 0 >&2
    "$bin_directory/kache" config set --global cache.auto_clean_idle_targets_days 0 >&2
}

main "$@"
