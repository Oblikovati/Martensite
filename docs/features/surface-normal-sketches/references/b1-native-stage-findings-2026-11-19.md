# B1 native-stage findings, 2026-11-19

The numeric-dependency OPT3 experiment was rejected. Four offline cases showed
11.87–17.16% lower median API time with small increases in median peak RSS.
This falls below #558's conservatively interpreted 50% runtime reduction
threshold. No production code or build configuration was adopted.

The [compact data receipt](b1-native-stage-optdeps-2026-11-19.json) retains all
24 raw timing/RSS rows, summaries, input hashes, exact consumer link commands,
library identities, source-manifest hashes and native-stage receipts. The
original matched result SHA-256 is
`360ee85e0d295dc145ced13000acfc76330cfce5b0e6df6e0ef66c5027a6dd18`.
The aggregate preserves its JSON values; it is not a byte-identical copy.

## Numeric-dependency comparison

Both consumers used Rust 1.99.0 with optimization level 0, debug assertions and
overflow checks enabled, and debug information disabled. Both configurations
retained the workspace's optimization levels: 3 for `martensite_tkmath` and
`num_bigint`, and 1 for `martensite_tkbo`. Only `metallic` and `fast_polynomial`
changed from level 0 to level 3. OPT0/OPT3 labels refer to those two dependency
settings. The baseline library source was clean Phase A
head `7a2aa0814a3f8c5b049aaec74b5d98bd26b296fe`; the 3766-file core source
manifest hash is retained. These libraries checked two native snapshots from
the earlier B1 diagnostic, rather than constructing a complete cone operation.

Three repeats per configuration/case alternated on CPU 8. The sampled idle
fractions for CPUs 8 and 24 were approximately 0.98 and 0.96. The comparison
matched normalized counts/grades in every row and recorded unchanged artifacts.
It did not establish numerical bit replay or complete kernel acceptance.

| Offline case | OPT0 median seconds | OPT3 median seconds | Median RSS change, KiB |
| --- | ---: | ---: | ---: |
| Self-intersection, region 1 | 17.801376569 | 14.892705887 | +208 |
| Self-intersection, region 2 | 1.460492353 | 1.209858813 | +180 |
| Ordered body check, region 1 | 19.709880348 | 17.003886565 | +188 |
| Ordered body check, region 2 | 1.545718537 | 1.362236431 | +284 |

## Native-stage diagnosis

Separate single-run instrumentation measured region 1's edge/face (EF) stage
at `8.826843890` seconds and face/face (FF) stage at `7.203075216` seconds.
These are diagnostic stage timings with instrumentation, not matched benchmark
results. The receipt records the slot mapping and excludes weighted-stage
completion from the timed section. Source restoration manifests retain the
original, instrumented and restored hashes; the recorded restored source
matches the original bytes.

The standalone self-intersection check returned `Incomplete` because inner face
self-intersection search coverage was unobserved. It reported 158 faces, zero
pairs and no validity claim. Zero pairs therefore supplies no exact
self-intersection proof. Performance issue #496 and coverage issue #498 remain
open. These findings establish neither a five-second operation nor completed
B1 qualification. Temporary implementations and binaries require cleanup by
the investigation owner after retaining the evidence.

## EF seed hypothesis

A further [instrumented EF seed diagnostic](b1-ef-seed-diagnostic-2026-11-19.json)
completed with build, link and run exit 0. It observed 1229 Bean calls,
4701 projections (156 cold) and 4701 solves, with no duplicate UV seeds within
a projection.
The duplicate-seed hypothesis was refuted for this workload; no optimization
was implemented. Outside Bean, another 624 projections performed 624 solves.

Bean time was approximately 8.9777 seconds, including 2.4581 seconds in
projection, 1.96515 in grid work and 0.19784 in root solving. The Bean timer
includes per-projection JSON serialization and logging outside the projection
timer, so their difference does not isolate numerical work. Concurrent
instrumentation makes these diagnostic timings unsuitable for a matched
performance claim. The normal profiles remained `metallic=0`,
`fast_polynomial=0`, Math=3 and TKBO=1. The compact receipt retains every actual
summary value and the build, artifact, source and patch hashes, without the
5 MB raw log or temporary implementation. The two restored source hashes match
their originals at clean Phase A head `7a2aa081`.

The preparation-stage flags are preserved. The manifest's `link_plan_sha256`
identifies the generator script, whose hash matches; the generated JSON receipt
has its own recorded hash. The terminal receipt records actual execution and
restoration. Native self-intersection coverage remained
incomplete: 158 faces, zero pairs, zero diagnostics and no validity claim.
Issues #496 and #498 remain open.

## Bean local-walk hypothesis

The [Bean walk diagnostic](b1-bean-walk-diagnostic-2026-11-19.json) completed
with build, link and run exit 0 and restored source hashes matching the originals.
Its summary totals 1902 walks and 5644 local calls: 5623 completed locally and
21 used fallback. Measured local calls took about 0.165759 seconds; all walk
clocks summed to about 0.185725 seconds. This refutes local-walk dominance for
this workload. Inclusive localization and extremum clocks were approximately
2.438892 and 1.915332 seconds. Stage clocks include nested logging, while walk
clocks exclude their own final log output; subtracting them would not isolate
numerical work. No optimization or matched performance result follows.
Native self-intersection coverage remained explicitly incomplete.

## Actual-cap endpoint barrier feasibility

The [actual-cap feasibility receipt](b1-cap-barrier-feasibility-2026-11-19.json)
records verified stored UV rectangles and native `edge.is_same` incidence for
624 corners on 156 ruled sides. At 42 corners the proposed sufficient closed-slab
endpoint condition was refuted; the other 582 were not established by this
point test. This supplies neither a trimmed-face intersection nor a kernel
invalidity finding. The receipt retains one exact integer witness, both cap
datums and trigonometric bound bit patterns, with source/library/snapshot hashes
and commands. Independent integer interval replay reproduced the witness bounds
and their signs; the prototype's arithmetic self-checks passed before capture.

The first diagnostic compile failed on an incorrect receipt getter. After
correcting it to `receipt.result().proof().operations()`, compile and run both
exited 0. Both source identities and the failure-log hash are retained. No strict
self-intersection gate was implemented: the public B1 controls remain two
functional passes, while the stronger proof required by #498 is unfinished.
The [functional receipt](b1-cache-v5-functional-2026-11-19.json) now also records
a successful, source-unchanged normal public build, separately from those tests.

## Original-side hull and storage preflights

The [actual-side hull preflight](b1-side-hull-preflight-2026-11-19.json)
compiled and ran with exit 0 on the two original bodies. Its 167 sides give
13861 unordered pairs: 167 native adjacent pairs remain unproved, while strict
AABB separation excludes 13694 nonincident pairs, including all 1716 pairs
between members. No nonincident residual pair remained. The receipt retains
all actual side hulls, individual control-inventory fingerprints and native
shared-ruling admissions. Positive common weights, stored exact rectangles and
native edge equality were admitted; the Z pad conservatively exceeds the
original O3 upper bound of `1e-7`. Self-domains, caps and adjacent-union
injectivity remain unproved, so this census supplies no full SI result.

The [storage/primitive preflight](b1-storage-primitive-preflight-2026-11-19.json)
completed the original outward/inward operation pair and restored source
exactly. It captured 334 actual storage-error records: each sense has 167 unique
spans, comprising 156 O spans and 11 dot spans. The receipt retains each group's
maximum exact dyadics and upward bit patterns, original requested policy,
source hashes and raw-data hashes. Primitive bound forecasts remain offline
conditioning estimates, with no production selection or acceptance change.

## Temporary half-split and contraction experiments

The [half-split/contraction receipt](b1-half-split-contraction-preflight-2026-11-19.json)
records five independent controls and both full O controls passing. The temporary
half-split used 941311 operations and 444 integration cells, with 148 derived
side faces. Inward/outward API times were `33.249426`/`33.626658` seconds during
concurrent F1 QA. These are diagnostic observations, with no matched performance
or five-second acceptance. Source was restored exactly; this behavior was neither
adopted in production nor selected for replay. Current B1 remains at 984593
operations and 167 derived side faces. The changed 148-side geometry requires its own capture and bound evidence;
the original hull/C1 receipts cannot be reused. Successful capture is recorded
below.

Direct projected contraction found 334 old-geometry checks feasible. Its adjacent
unions alone require 177020 logical arithmetic calls, in addition to 88510 for
self checks; the full diagnostic counted 403330 calls including candidate work.
These are modelled calls, without issued production work receipts. This cost
rejects direct native contraction as the budget plan. At that checkpoint,
Markov transfer had no implementation or feasible-cost result; the later frozen
plan preflight is recorded below.

Initial P1 plan review attempt 1 returned REMEDIATE, leaving two plan rereviews; the B1
source-phase gate remains 0 of 3. A combined 148-side archive/profiler probe
subsequently failed to compile because its diagnostic Drop guard caused E0502.
It restored source, ran no model and captured no geometry artifacts. The corrected capture subsequently succeeded, as recorded below; the initial
probe failure establishes no feature defect.
The earlier F1 gate stopped only on a local 25 GiB disk watchdog, with source
and dataset unchanged. Its warm full-mode retry is live with the same base/scope
and a 15 GiB reserve. No PR, version or tag follows from these preflights.

## Successful half-split capture and owned-path profile

The [corrected capture/profile receipt](b1-half-split-capture-profile-2026-11-19.json)
records both complete O controls passing with 941311 operations and 444 cells.
The captured boundary records now establish 148 unique actual spans per sense:
137 O spans and 11 dot spans. Both archive hashes and all 296 boundary-record
fingerprints are retained. Eight temporary source edits were restored exactly.
This was an instrumented experiment; current production WIP still uses the
original 984593 operations and 167 derived side faces, with half-split unadopted.

For the actual 137-occurrence owned path, inward/outward preparation clocks were
`7.930743827`/`8.467879681` seconds. Metadata clocks were
`0.859222748`/`0.891646614` seconds, and six native source scans took
`0.006474069`/`0.006291303` seconds. This refutes dominant lookup/freshness cost
for that recorded path. Bound/query timers overlap and graph timers include
logging overhead; these clocks provide no matched performance result or SI
acceptance. At that capture checkpoint, offline Markov findings were preliminary,
with full adjacency coverage and charged receipt totals missing. The subsequent
plan reconciliation is recorded below.

The [new native-incidence census](b1-148-native-incidence-2026-11-19.json)
separately checked both actual archives. Each has 137 O sides, 11 dot sides and
four caps, identified through native component membership and `edge.is_same`.
Of 10878 distinct side pairs, 148 adjacent pairs remain unproved; strict padded
AABB separation excludes 10730 nonincident pairs, including all 1507 cross-member
pairs, with no nonincident residual. Source, library, snapshot and consumer
guards were unchanged. This is new geometry-bound evidence, with no reuse of the
old 167-side census. Self-domains, adjacent unions and caps remain unproved;
there is no whole filled-cap, SI, production-arithmetic-budget or five-second
acceptance claim.

## P1 bounded implementation approval

[Plan review 2 and the final feasibility receipts](b1-p1-final-plan-feasibility-2026-11-19.json)
record GO for bounded implementation of one coherent outward/inward O. operation.
The final Math manifest is
`121981176c26ef486a896210d574a9bdbd438c44cc9f4cc3b43c99dd82afe137`;
every listed artifact hash was verified. The stronger separate-family-extrema
gamma retains the same final rounded XYZ bits as the preceding model.
Numeric feasibility covers all 148 adjacent unions, all 10730 nonincident pairs,
592 side/cap-carrier products, four cap self-domains and six cap pairs. Actual
selected bounded UV material, roles, orientation and common lift still require
independent production binding; native wire labels provide no such authority.

The prospective per-sense total is 996304: the earlier measured 941311, plus
53477 executed numeric-model calls, 148 source-derived derivative imports that
have not executed, and 1368 proposed work steps. The spare 3696 is conditional,
not demonstrated remaining work. Real shared checked-transition issuance, actual
final native/UV rebinding and all changed first-pass costs must run within the
original one-million limit and every original error cap, or refuse. The issued
composition capture and actual cap-UV run are retained separately from these
prospective translation calculations.

This approves implementation, with B1 source-phase review still 0 of 3. The
functional source remains old NativeSearch at 984593 operations/167 derived side
faces; half-split 941311/148 remains a reverted diagnostic. Ordered actual SI,
source/mode/refinement mutations, cancellation/exhaustion, legacy replay and
normals/area/volume/history/export/isolation still need the complete operation
matrix. Matched runtime/peak memory, the five-second target, and full B/C scope
remain open. No merge, version, tag or capability completion is authorized.

## Census and observed-row measurements

The [preparation and unprobed sampling receipt](b1-preparation-unprobed-perf-2026-11-19.json)
keeps the instrumented 148-span attribution separate from the preserved original
167-span executable. On the 137-edge UV profile, inward/outward Census checkpoint
intervals were `5.694622095`/`5.799079550` seconds with 2579676 callbacks each;
actual delegated callback bodies took `0.064174713`/`0.064526960` seconds.
Observed-row discovery took `1.185206330`/`1.282426512` seconds. Its 9453 first
visits had no visited hits; missed searches encountered 44674878 entries.
Intervals include existing polling and nested logging, and cannot be summed or
used to justify removing polls. Existing evidence does not yet separate Census
reuse lookup from exact exclusion work and inconclusive fallback.

The single unprobed outward run passed with unchanged source/binary and original
984593/501/167 work/cells/side-face diagnostics. Of 3547 samples, none were lost,
but 94.50% of weighted cycles had zero or one recovered DWARF frame. Inclusive
caller attribution is unreliable; whole-operation bigint self samples cannot
locate that cost specifically inside Census. Observed-row discovery is a measured
bounded target, with output/order/refusal controls and measurement required before
any optimization. These observations establish no speedup or matched benchmark.
