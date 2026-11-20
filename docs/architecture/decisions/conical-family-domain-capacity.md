# Explicit source capacity for a conical sketch family

Status: implementation decision, 2026-11-20. The capacity design passed a Spec
review. Complete-operation and performance qualification remain pending.
Related: #414, #418 and #425.

## Evidence

The complete public cone operation was attempted with the owned 239-curve,
18-region multiline text, developed by `[x,y] -> [100+y,-x]`. It requested
semi-angle 0.125, depths `[0,0.25]`, spatial error `1e-5`, maximum fit level 8,
8192 planar pair bisections, geometric planar proof V6, aggregate version 2 and
the original million-unit metric grant.

The domain refused `SpanCount(239)` with zero domain arithmetic operations.
Its admission reused the legacy sector's 128-source cap across the entire
selected family. This establishes an input-capacity blocker, not metric-grant
exhaustion. The temporary probe and its emitted artifacts were removed.
The [capacity receipt](../../features/surface-normal-sketches/references/cone-text-239-capacity-2026-11-20/summary.json)
retains the refusal, input recipe and restoration evidence.

## Decision

Keep the original chart behavior and default at 1 through 128 source spans.
Add an explicitly selected family-domain version with a fixed cap of 4096
original source spans. Check that complete input count before proof work or
source receipt allocation. This matches the existing sketch-selection source
capacity; it does not increase any geometric, arithmetic or integration limit.

Both versions use one admission/proof pipeline. Factor policy, source-count
and geometry checks without changing the original error order. The family
version retains the same positive-X requirement, equal datum/fit-center bits,
zero lifts, strict common-chart angular conditions, precision and
million-unit cap for the domain primitive. There is no retry, partial publication, count
growth or grant increase after refusal. Existing per-contour and member caps
remain in force.

The issued authority binds its version, fixed cap, actual count, complete raw
source, chart and arithmetic policy. Boundary/reference bindings retain that
authority. Legacy `matches` accepts only the original version; explicitly
versioned matching is required for the family authority. The operation records
its selected behavior for replay. The original directed selection stays on the
128-source behavior; the new family selection uses the same executor and
required certified self-intersection route.

This changes the explicit feature capacity. Package versions, release tags,
wider charts and periodic topology are outside the decision. The later
[input-work policy](../../features/surface-normal-sketches/CONICAL-INPUT-WORK.md)
describes the separately selected aggregate allowance and FamilyV3 trim
position bound.

## Exit checks

Pin the original 128-source admission, errors, work and cancellation behavior.
Test the new count boundaries, admission precedence, source/version mutations,
budget refusal and cancellation without partial receipts. Check version binding
through boundary/reference composition. Then attempt the complete 239-curve,
18-region operation under the original numerical and work limits. If another
obligation refuses, retain its actual cause and address it without substituting
a detached proof or a smaller success witness. Measure time and peak memory
only after functional completion on the final source.
