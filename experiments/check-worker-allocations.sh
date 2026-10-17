#!/usr/bin/env bash
# Diagnostic comparator for #87; not a new performance-acceptance policy.
set -euo pipefail

worker_binary=${1:?expected test binary path}
worker_valgrind=${2:?expected Valgrind executable path}
baseline_allocations=${3:?expected measured baseline allocation count}
worker_symbols=${4:?expected debug-symbol prefix}
worker_test=${5:-persistent_range_workload}
worker_report=$(mktemp /tmp/martensite-worker-allocations.XXXXXX)

"$worker_valgrind" --tool=memcheck --extra-debuginfo-path="$worker_symbols" --log-file="$worker_report" \
    "$worker_binary" "$worker_test" --ignored --test-threads=1
measured_allocations=$(awk '/total heap usage:/ {gsub(/,/, "", $5); print $5}' "$worker_report")
if [[ ! "$measured_allocations" =~ ^[0-9]+$ || ! "$baseline_allocations" =~ ^[0-9]+$ ]]; then
    printf 'Invalid allocation counts: measured=%s baseline=%s; expected unsigned integers\n' \
        "$measured_allocations" "$baseline_allocations" >&2
    exit 2
fi
printf '{"allocations":%s,"baseline_allocations":%s,"report":"%s"}\n' \
    "$measured_allocations" "$baseline_allocations" "$worker_report"
test "$measured_allocations" -le "$baseline_allocations"
