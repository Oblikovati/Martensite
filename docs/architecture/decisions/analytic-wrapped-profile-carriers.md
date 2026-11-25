# Analytic carriers for one wrapped-profile operation

Status: implementation decision, 2026-11-17, updated for the 2026-11-18
Phase A candidate. The curved holed-cone public-entry workload completes.
Gate A review 2 ended with memory-only remediation after resolving the four
reference/body correctness findings. On 2026-11-18 the maintainer accepted this
candidate's quantified code/process footprint tradeoff. Final review 3 accepted
the phase commit and ready PR. Required hosted checks, including actual new cone
TSan execution, passed on `7a2aa081` before PR #549 merged into `release/next`
at `8f2b4231`. B/C qualification remains outstanding. Related:
#414 and #418. The tracked
[2026-11-13 review](../../features/surface-normal-sketches/REVIEW-2026-11-13.md#verdict)
recommends rational edges and one complete operation before further proof work.

## Decision

Use analytic cylindrical and conical surfaces as native depth caps, with the
approved paired rational depth edges and two-row ruled sides. Extend the
[accepted rational representation](rational-wrapped-profile-representation.md)
without changing its exact-reference, epsilon-close, same-topology contract or
existing primitive interfaces.

For the cone, the exact reference is the common-chart ideal q/L normal body.
Here q represents `tan((theta-center)/2)` and L is the rational slant function.
Its topology composition consumes original source intervals, shared ideal
endpoint identities, executed monotone joins, nonadjacent clearance, a strict
continuous angular range shorter than one turn and full-depth positive radius.
The reference profile is within the admitted epsilon of the authored material
with the same topology. This is the body whose represented volume is certified.

The decoded native representation is a separately bounded proxy. Actual
rational edges, weights, two ruled control rows and analytic cap pcurves are
captured and checked. Its polynomial UV material domain has independently
executed topology and filled-domain evidence. The reference-to-proxy filled-cap
bridge compares both certified domains through the same authored material.
Reference topology, native storage/trim bounds and native validation remain
separate evidence. Rounded analytic-cap traces are not claimed to make the
literal stored patches an exactly closed embedded surface.

One shared executor owns capture, selection, keyed construction, freeze/decode,
certification, native qualification, history and publication. Cylinder and cone
mappings use the same `construct_mapped` incidence builder. Support adapters
provide coordinates, fits, carriers and mathematical receipts; they do not
return completed operation results. Existing cylinder report owners and public
interfaces remain unchanged. Cone Phase A admits the canonical frame only;
accounted physical placement for cones remains Phase B.

For an admitted reference slant l0 and signed depth delta, the analytic cone
radius is `l0*sin(alpha)+delta*cos(alpha)` and its axial origin is
`l0*cos(alpha)-delta*sin(alpha)`. Parameters are `U=theta` and `V=L-l0`.
This reuses `GeomConicalSurface` and native CONE encoding. The reference slant
comes from admitted source input.

## Budget and publication obligations

The original request owns all limits. The current composition separately
enforces:

| Quantity | Required allocation or bound |
| --- | --- |
| Original-to-reference developed displacement | At most epsilon/4. |
| Decoded depth-edge and full-depth ruling storage error | At most epsilon/1000, using the effective downward-rounded allocation. |
| Original nonadjacent clearance | At least 4*epsilon and sufficient for both homotopy tubes. |
| Filled reference, actual proxy and their cap bridge | Each required comparison must pass its actual certificate and the original total spatial request. |

Exact decoded rows and common weights make ruling interpolation affine, so the
maximum endpoint storage bound covers the full ruling. That O3 check is enforced
before trimming; passing a total spatial limit cannot hide an exceeded storage
allocation. Cap-trace bounds and actual native tolerances remain separate.

`WrappedConeNormalResult` retains original settings through `options()` and
`proof_options()`, effective leaf policies through borrowed receipts, and actual
achieved bounds/work through member and family reports. The new unreleased
cone default selects retained-UV `FilteredBernsteinV4`; explicit `BernsteinV3`
replay remains available. Existing cylinder/toolkit defaults are unchanged.
Neither work limits nor tolerances are widened after a refusal.

Authored normal volume, q/L reference volume, conditional native patch-occurrence
flux and native quadrature estimates have distinct fields. Cone
`Refuse(allowance)` applies to the aggregate authored/reference certified
comparison. Reference integration uncertainty is already included in that
interval comparison; proxy flux is not relabelled as body volume. Ordered
native checks preserve sampled-geometry and native-search coverage modes.
Pcurve coverage counts actual retained uses. History keeps caller-relative
first/last roles for inward depths, and detached export verifies keyed identity
and duplicates before returning independent native owners.

The [public contract](../../features/surface-normal-sketches/PUBLIC-API.md) lists
the exact entries, getters, scope and refusal behavior.

## Evidence and limitations

The Phase A family is restricted to one admitted strict angular chart/wedge and
canonical apex/axis/radial frame. Positive radius must hold over filled material
and all requested depths. The successful curved outer contour with one hole
is a workload witness, not qualification of arbitrary text, wide wraps, seam
occurrences or oblique attachment.

The [final complete-workload comparison](../../features/surface-normal-sketches/references/cone-final-workload-benchmark-2026-11-18-summary.json)
uses source `64110e402e131afff2e3fadcbf9e2b9d4c9b245c` plus preserved feature
working changes and one executable for cone V3/V4. Median API times were
`9.315035` and `4.822891` seconds; all three V4 measurements met five seconds.
The pre-shared-executor cylinder comparison had overlapping wall ranges and
medians `3.95` and `3.90` seconds, with all six accepted 221424-byte replays
unchanged. These samples show no observed runtime regression, not a statistical
speedup or a comparison against a previous published cone release.

Cylinder median peak RSS increased from `54860` to `56624` KiB, by `1764` KiB
(about 1.7 MiB). Cone policy RSS ranges overlap. A separate approximate,
non-atomic [memory diagnostic](../../features/surface-normal-sketches/references/cone-final-workload-benchmark-2026-11-18-memory-composition.json)
found the sampled RSS increase mostly in mapped executable pages: file-backed
RSS rose `1568` KiB and anonymous RSS `64` KiB. It establishes no exact allocation
count and does not waive the footprint finding. Its wait4 launch values include
pre-exec context; GNU-time campaign rows supply benchmark process peaks.

Final [metadata](../../features/surface-normal-sketches/references/cone-final-workload-benchmark-2026-11-18-metadata.json)
and [rows](../../features/surface-normal-sketches/references/cone-final-workload-benchmark-2026-11-18-rows.json)
retain source/binary hashes and nonexclusive-host/build conditions. The
[first campaign](../../features/surface-normal-sketches/references/cone-policy-benchmark-2026-11-18-summary.json)
and its [parser-correction provenance](../../features/surface-normal-sketches/references/cone-policy-benchmark-2026-11-18-metadata.json)
remain unchanged alongside the final campaign.

Local checks passed: Math 825, Sketch 441, public integration 50 and explicit
four-worker/three-case contention stress. Parent controls exercised ignored
subprocess workers. Strict Sketch Clippy passed before the formatter-only
root-module reorder. The corrected local #544 run completed all planned 472
test binaries and 40 doctest groups (88402 passed, 0 failed, 157 ignored), but
its final launcher exit status was not recovered. Component
[PR #548](https://github.com/Oblikovati/Martensite/pull/548) is open at `a72032dd`
against `release/next` as a separate component PR. The full local cone gate
subsequently passed with captured exit 0 on 2026-11-18. Its
[bounded receipt](../../features/surface-normal-sketches/references/cone-canonical-pr-gate-2026-11-18-receipt.json)
records 41 affected packages, 473 native targets and 40 doctest crates:
89075 passed, 0 failed and 158 ignored, with 9 separate unsafe-inventory passes.
Outer totals exclude intentional nested SDK/worker summaries; the
[verification record](../../features/surface-normal-sketches/VERIFICATION.md#canonical-cone-pr-gate-2026-11-18)
retains exact source, command and exclusion scope. Those local checks preceded
the hosted pass recorded below.
The focused ordinary
[public cone concurrency control](../../features/surface-normal-sketches/VERIFICATION.md#ordinary-public-cone-concurrency-control)
passed 1/0/0 in 9.71 seconds, covering overlapping operations and shared readers
with isolated native exports. Ordinary sanitizer-driver and inventory checks
passed 46 and 9 tests; their [receipt](../../features/surface-normal-sketches/references/cone-tsan-driver-2026-11-18-receipt.json)
retains the frozen source hashes. Local TSan startup exited 139 before either
main marker; that failed attempt built no instrumented cone control. The later
hosted cone control actually ran and passed 1/0/0 in 167.83 seconds, as retained
in its [receipt](../../features/surface-normal-sketches/references/phase-a-549-hosted-tsan-receipt-2026-11-18.json).
The maintainer's 2026-11-18 "accepted" decision covers the matched production
consumer's +156 KiB (about 0.3%) median RSS and associated quantified code
footprint in the [memory record](../../features/surface-normal-sketches/PRODUCTION-MEMORY-ACCEPTANCE.md#maintainer-decision-2026-11-18).
It is case-specific; runtime, heap, tolerance and general performance policy
remain unchanged. Final
[review 3](../../features/surface-normal-sketches/references/cone-gate-a-oracle-3-2026-11-18.md)
accepted delivery of this bounded Phase A result as a commit and ready PR.
Required hosted checks passed on PR head
`7a2aa0814a3f8c5b049aaec74b5d98bd26b296fe`, with 55 successful and 4 skipped
checks in the [final receipt](../../features/surface-normal-sketches/references/phase-a-549-hosted-final-receipt-2026-11-18.json).
Those receipts were captured before PR #549 merged at
`8f2b423103d13b22d9724747946b43ee1ca15ded`; merge identity is separate from the
tested head. B/C qualification remains outstanding.
These results do not complete the broader capability or authorize a release.

The earlier [cap microbenchmark](../../features/surface-normal-sketches/references/cap-representation-microbenchmark-2026-11-17.json)
measured analytic construction near 2.1 microseconds versus 8.4 for rational
caps, with archives of 289 versus 375 bytes and successful strict byte replays.
Its sampled closed-form comparison was only a smoke check. It supplies no
certified modelling bound or complete-operation performance result.

## Remaining phases and deletion

Phase B covers multiple charts, wider/text-scale profiles, explicit seam and
period-branch occurrences, and accounted cone placement/attachment. Preserve
source identity across chart choices; do not infer it from proof traversal or
coordinate proximity.

Phase C preserves the primitive sector's exact contract through analytic-span
lowering before removing its separate construction path. Replacement deletion,
the broader acceptance corpus and application integration follow their own
gates. Keep findings and measurements, then remove temporary investigation
implementations and binaries after their investigation concludes. No release,
version bump or whole-feature completion is authorized by this decision.
