#!/usr/bin/env bash
# Replays the workspace tests twice with the test-only `libm-attribution` feature (#201):
#
#   1. platform_original: the original (534e7d6) test sources and expected fixtures,
#      with every adapter call returning the platform libm (glibc) result;
#   2. portable_candidate: the candidate test sources and fixtures, with every call
#      returning the portable correctly rounded result.
#
# Both runs log every call whose two results differ. Run from the repository root on
# a clean checkout of the candidate; the script restores the candidate tests at exit.
# Usage: docs/features/portable-transcendentals/attribution/replay.sh OUT_DIR
set -euo pipefail

ORIGINAL=534e7d6134c4a43926736d8c13b99c591126a8d9
OUT=$(realpath "$1")
FEATURE=martensite-cmath/libm-attribution
mkdir -p "$OUT/calls-platform_original" "$OUT/calls-portable_candidate"

run_tests() { # name selection
    local name=$1 selection=$2
    MARTENSITE_CMATH_SELECT=$selection MARTENSITE_CMATH_ATTRIBUTION_LOG="$OUT/calls-$name" \
        cargo test --locked --workspace --features "$FEATURE" --no-fail-fast \
        >"$OUT/$name.cargo-test.log" 2>&1 || true
}

original_test_paths() { # status pathspec...: test files changed since the original commit
    local status=$1
    shift
    git diff --name-only --diff-filter="$status" "$ORIGINAL" HEAD -- "$@"
}

restore_candidate() { git checkout HEAD -- crates; git clean -fdq crates; }

if ! git diff --quiet HEAD -- crates; then
    echo "uncommitted changes under crates/; expected a clean candidate checkout" >&2
    exit 1
fi
run_tests portable_candidate portable
trap restore_candidate EXIT
# Inline test modules count as tests; the added frexp tests stay because frexp.rs,
# moved without change, includes them.
original_test_paths M 'crates/*/tests/**' 'crates/*/src/**/tests.rs' 'crates/*/src/**/*_tests.rs' |
    xargs git checkout "$ORIGINAL" --
original_test_paths A 'crates/*/tests/**' | xargs rm --
run_tests platform_original platform
restore_candidate
trap - EXIT

cargo test --locked --workspace --features "$FEATURE" --no-run --message-format json 2>/dev/null |
    python3 -I -c '
import json, sys
binaries = {}
for line in sys.stdin:
    message = json.loads(line)
    if message.get("reason") == "compiler-artifact" and message.get("executable"):
        stem = message["executable"].rsplit("/", 1)[-1]
        source, fragment = message["package_id"].split("#", 1)
        name = fragment.split("@")[0] if "@" in fragment else source.rsplit("/", 1)[-1]
        binaries[stem] = name
print(json.dumps(binaries, indent=1, sort_keys=True))' >"$OUT/binaries.json"
