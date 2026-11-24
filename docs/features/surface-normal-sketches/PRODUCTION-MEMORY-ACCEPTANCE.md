# Production cylinder memory acceptance

Status: Phase A review 2 ended with memory-only remediation; the four
reference/body correctness findings were resolved. On 2026-11-18 the maintainer
explicitly accepted the quantified code/process footprint tradeoff recorded
below for this Phase A candidate. Final review 3 accepted its phase commit and
ready PR. Required hosted checks, including new cone TSan, remain pending and
must pass before merge; B/C have not started.

## Matched production comparison

An identical standalone consumer runs the existing public mixed line,
quadratic and cubic profile with a clockwise hole and disconnected material.
It uses regions 5 and 100, radius 10, depths `[-1, 2]`, spatial error `1e-5`,
maximum fit level 8 and volume refusal `1e-4`. Independent areas are 3.525 and 1;
normal volumes are 11.10375 and 3.15. Standard constructor checks, source-input
isolation and two detached native exports remain enabled.

The consumer uses the existing `api::operation_tests::request()` fixture. Its
source SHA256 is `bca5f0d02f5e0fd074958656b62584e6990510bc1da0a0a8953b212940fc74fe`.
The baseline is accepted commit `7817e8d1f6c8c502fa860d4300afe32f426c603e`.
The corrected feature snapshot is tree
`6e8424f77f1e6556c0660d3ee8107c4368da6c71`. Compiler, debug profile, package
optimizations, dependencies, feature selection and linkage settings match.
Both archives were genuinely compiled from empty owned targets; an earlier
stale Cargo cache result was rejected and retained as invalid provenance.

| Quantity | Baseline | Before #547 correction | Corrected |
| --- | --- | --- | --- |
| Median constructor time, seconds | 3.774414419 | 3.787060020 | 3.683161061 |
| Median process wall time, seconds | 3.78 | 3.80 | 3.69 |
| Peak RSS range, KiB | 51140 to 51360 | 51412 to 51648 | 51300 to 51532 |
| Median peak RSS, KiB | 51328 | 51604 | 51484 |

All nine runs passed and produced the same 45871 native bytes, SHA256
`77958916b25ae0639f71315392109cbb1b279d47a3f381b9ff1663b49929196a`.
Public history, reports, budgets and policies matched byte for byte, SHA256
`560adbd32919b674940036a80d00d39e5b8657084afc4ddf31328282d9b1bdf5`.
These are three samples per version on a nonexclusive host, not a universal
timing guarantee. [Metadata](references/production-cylinder-547-metadata.json),
[raw rows](references/production-cylinder-547-rows.json) and
[summary](references/production-cylinder-547-summary.json) retain every run.

## Allocation and code footprint

The private #547 correction removed concrete cone failure formatting from the
shared cylinder error boundary. Original formatting, source downcasts,
borrowed planar-work diagnostics and cancellation precedence passed 44 focused
controls and strict all-target Sketch Clippy. No geometry, numerical policy or
success-path allocation changed.

A separate glibc `memusage` diagnostic used identical `argv[0]`, equal-length
output paths, disabled timer sampling and traced mmap. It measured:

| Quantity | Baseline | Corrected |
| --- | --- | --- |
| malloc calls | 13187571 | 13187571 |
| realloc calls | 911825 | 911825 |
| calloc calls | 3618612 | 3618612 |
| Peak tracked heap, bytes | 16880805 | 16880804 |
| Peak tracked stack, bytes | 312512 | 311952 |

The one-byte tracked-heap difference is not claimed as a kernel optimization.
Instrumented timings are not used for runtime acceptance. The
[allocation profile](references/production-cylinder-547-allocation-profile.json)
records the diagnostic's scope and commands.

The corrected executable still has 124784 more bytes in the selected sections
than the baseline, including 65536 bytes of text, 44888 bytes of exception tables
and 11496 bytes of unwind frames. GNU `size` reports 128840 more bytes in its
broader text/read-only aggregate. The
[section measurements](references/production-cylinder-547-sections.json) retain
all three executables. Process RSS medians remain 156 KiB higher, with overlapping
ranges. These measurements do not establish an exact allocation leak or a
general exemption for code pages from the no-degradation requirement.

One throwaway trial forced inlining at the 15 cylinder stage forwarders. It
reduced selected sections by only 1316 bytes and was not adopted or benchmarked.
Its implementation, owned target, executable, attribute patch, build script
and analysis script were removed after ownership, locking and live-process checks.
The [cleanup receipt](references/cone-inline-trial-cleanup-2026-11-18.json)
records preserved findings, measured snapshots and unrelated targets.

## Maintainer decision, 2026-11-18

The corrected constructor median is lower in these groups of three samples. The
separate diagnostic recorded equal allocation call counts, no tracked peak-heap
increase and a lower tracked peak stack. Total process RSS and compiled code
remain larger.

On 2026-11-18 the maintainer answered "accepted" to this candidate's measured
median process-RSS increase of 156 KiB (about 0.3%) and associated quantified
code footprint: 124784 bytes across the selected sections and 128840 bytes in
GNU `size`'s broader aggregate. This acceptance applies to the Phase A candidate,
matched consumer and evidence identified above. It does not grant a blanket
runtime, heap, tolerance or performance waiver, authorize API/version/tag
changes, or accept wider wrapping capability. The measurements, source snapshots,
policies and evidence receipts remain unchanged.

Final [Gate A review 3](references/cone-gate-a-oracle-3-2026-11-18.md) accepted
the bounded phase commit and ready PR, with the user decision resolving the
remaining footprint finding. Required hosted checks, including new cone TSan,
must pass before merge. B/C have not started; wider cone charts, text, placement
and primitive unification remain later work. No version, tag, release or
whole-feature completion is authorized.
