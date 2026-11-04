#!/usr/bin/env bash
# Installs the checksum-pinned Vulkan validation layer that the renderer tests require.
# Ubuntu 24.04 packages layer 1.3.275, which reports a false dynamic-rendering depth-resolve
# hazard and crashes under parallel instances; `martensite-vulkan` refuses to run its
# validated tests on anything older than 1.4.313.
#
# Usage: scripts/install/vulkan-validation.sh DIRECTORY [--github-env FILE]
# Prints the `export` lines of the layer; with --github-env it appends them to FILE instead.
set -euo pipefail

readonly VERSION="1.4.313.0~rc2-1lunarg24.04-1"
readonly URL="https://packages.lunarg.com/vulkan/pool/main/v/vulkan-validationlayers/vulkan-validationlayers_${VERSION}_amd64.deb"
readonly ARCHIVE_SHA256="b100bfafac3df98c5d8ef5a572b423bcd9ebf3fab963e2e68957f4bf6f2423f0"
readonly LIBRARY_SHA256="89e618526d6d1532096dbbd62ee1dc710dda647061cd3b7b66b542dc7dbce104"
readonly MANIFEST_SHA256="f328caf95d75607075575cf5849b4e765f9eabe111ea5be7f971c92a9a7d1a58"

fail() {
    printf 'error: %s\n' "$*" >&2
    exit 1
}

require_digest() {
    local path=$1 expected=$2 actual
    actual="$(sha256sum "$path" | cut -d' ' -f1)"
    [[ $actual == "$expected" ]] || fail "$path has SHA-256 $actual; expected $expected"
    return 0
}

prepare_layer() {
    local directory=$1 archive="$1/validation-layer.deb" extracted="$1/extracted"
    local manifest="$extracted/usr/share/vulkan/explicit_layer.d/VkLayer_khronos_validation.json"
    local library="$extracted/usr/lib/x86_64-linux-gnu/libVkLayer_khronos_validation.so"
    if [[ ! -f $manifest || ! -f $library ]]; then
        mkdir -p "$directory"
        [[ -f $archive ]] || curl --fail --location --silent --show-error --output "$archive" "$URL"
        require_digest "$archive" "$ARCHIVE_SHA256"
        dpkg-deb -x "$archive" "$extracted"
    fi
    require_digest "$library" "$LIBRARY_SHA256"
    require_digest "$manifest" "$MANIFEST_SHA256"
    return 0
}

environment_lines() {
    local extracted="$1/extracted/usr"
    printf 'export VK_LAYER_PATH="%s"\n' "$extracted/share/vulkan/explicit_layer.d"
    printf 'export LD_LIBRARY_PATH="%s${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"\n' "$extracted/lib/x86_64-linux-gnu"
    # Every validated process must report zero errors, warnings and performance warnings.
    printf 'export VK_KHRONOS_VALIDATION_DEBUG_ACTION="VK_DBG_LAYER_ACTION_LOG_MSG"\n'
    printf 'export VK_KHRONOS_VALIDATION_REPORT_FLAGS="error,warn,perf"\n'
    printf 'export VK_KHRONOS_VALIDATION_VALIDATE_SYNC="true"\n'
    return 0
}

github_lines() {
    local extracted="$1/extracted/usr"
    printf 'VK_LAYER_PATH=%s\n' "$extracted/share/vulkan/explicit_layer.d"
    printf 'LD_LIBRARY_PATH=%s\n' "$extracted/lib/x86_64-linux-gnu"
    printf 'VK_KHRONOS_VALIDATION_DEBUG_ACTION=VK_DBG_LAYER_ACTION_LOG_MSG\n'
    printf 'VK_KHRONOS_VALIDATION_REPORT_FLAGS=error,warn,perf\n'
    printf 'VK_KHRONOS_VALIDATION_VALIDATE_SYNC=true\n'
    return 0
}

main() {
    [[ $# -ge 1 ]] || fail "missing DIRECTORY; expected a directory for the extracted layer"
    [[ "$(uname -sm)" == "Linux x86_64" ]] || fail "got $(uname -sm); expected Linux x86_64"
    prepare_layer "$1"
    if [[ ${2:-} == --github-env ]]; then
        [[ -n ${3:-} ]] || fail "--github-env needs a FILE argument"
        github_lines "$1" >>"$3"
        return 0
    fi
    environment_lines "$1"
}

main "$@"
