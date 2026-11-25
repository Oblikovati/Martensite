# Surface sketches and normal extrusion

Feature [#414](https://github.com/Oblikovati/Martensite/issues/414) adds sketches
on cylinders and cones. A feature grows from its sketch along the support's
normal at each point. Existing planar sketches and vector extrusions retain
their interfaces and behaviour.

The primitive entries accept developed cylindrical rectangles and conical
annular sectors, including seam crossings within their supported domains.
Solved profiles use separate owned-result APIs. Martensite v0.14.0 shipped
single-chart cone profiles with holes. [PR #549](https://github.com/Oblikovati/Martensite/pull/549)
passed hosted checks, including a cone ThreadSanitizer control.

## Existing kernel support

`tkbo::profile_extrusion` accepts solved planar curves, builds bounded regions,
and extrudes a region along a fixed vector. It supplies history, validation and
measured volume. It does not solve a general sketch constraint system.

`tkg3d` supplies analytic cylinders and cones, normal-offset surfaces and the
`Adaptor3dCurveOnSurface` evaluator. These are geometry building blocks. The new
operation must also construct shared topology, account for periodicity, retain
source identity and validate the completed solid.

See [geometric planar proof V6](CENSUS-V6.md) for the opt-in Census method and
its remaining integration and performance checks.

See [the geometry contract](DESIGN.md), [the cone sector contract](CONE.md) and
[the evidence plan](VERIFICATION.md).

See the [supplied cylindrical text workflow](VISUAL-WORKFLOW.md) for the
end-to-end application acceptance case and the
[owned text capture contract](TEXT-CAPTURE.md). The primitive operations below are
foundations for that workflow.

Solved-profile cylinder results report centered patch flux of `(x-t)/3` and
expose the decoded field origin through `boundary_flux_origin()`. The
[field-origin contract](../../architecture/decisions/cylindrical-flux-field-origin.md)
explains its comparison with the ideal normal solid and its separate native
volume estimate.
The [native flux comparison note](BUG546-NATIVE-FLUX-CONTROL.md) documents
its raw error-status units and the same-origin numerical control (#546).

## Delivery tasks

| Task | Scope | Depends on |
| --- | --- | --- |
| [#415](https://github.com/Oblikovati/Martensite/issues/415) | Geometry and ownership contract | None |
| [#416](https://github.com/Oblikovati/Martensite/issues/416) | Complete cylindrical rectangular normal extrusion | #415 |
| [#417](https://github.com/Oblikovati/Martensite/issues/417) | Conical support away from its apex | #416 |
| [#418](https://github.com/Oblikovati/Martensite/issues/418) | General curves and holes | #415, #416, #417 |
| [#419](https://github.com/Oblikovati/Martensite/issues/419) | Complete wraps and periodic regions | #418 |
| [#420](https://github.com/Oblikovati/Martensite/issues/420) | Bosses, pockets, support revisions and replay | #416, then applicable support tasks |

A task is complete only after its implementation and evidence are integrated.
No package version or release is authorized by the feature approval.

## Cylindrical rectangle

`surface_sketch::normal_extrude` accepts a rectangle in developed coordinates.
For a radius of 10, a width of 7 wraps through 0.7 radians. The interval
`[-1, 2]` reaches radii 9 and 12 with an outward support normal. A chart origin
of -0.3 makes this example cross the conventional zero-angle seam.

```rust
use martensite_tkbo::profile_extrusion::{DisplayTolerance, Resolution, ToleranceCap};
use martensite_tksketch::surface_sketch::*;

let rectangle = CylinderRectangle {
    support: CylinderSupport {
        origin: [0.0; 3],
        axis: [0.0, 0.0, 1.0],
        radial: [1.0, 0.0, 0.0],
        radius: 10.0,
        chart_origin: -0.3,
        normal: NormalSense::Outward,
    },
    region: SurfaceRegionId(1),
    s: [0.0, 7.0],
    z: [0.0, 4.0],
    corners: [1, 2, 3, 4].map(SurfaceVertexId),
    boundaries: [1, 2, 3, 4].map(SurfaceCurveId),
};
let options = NormalExtrusionOptions {
    resolution: Resolution::new(
        1e-6, 1e-7, 1e-9,
        DisplayTolerance { chordal: 1e-3, angular: 0.1 },
        ToleranceCap::Refuse(1e-5),
    ),
    volume_relative_error: 1e-9,
};
let result = normal_extrude(&rectangle, [-1.0, 2.0], options)?;
let detached = result.export_brep()?;
// `detached.entities` resolves the result's semantic history keys to native shapes.
```

The first and last depth roles follow the requested interval even for an inward
normal. `export_brep` creates fresh native owners each time. The result's metadata
and encoded body remain immutable when a caller changes an export.

Current admission excludes holes, general curves, complete turns and
axis-crossing depth intervals. Small features also need clearance from the
native geometry tolerances. The operation reports typed refusals for unresolved
dimensions or nonfinite arithmetic. Its generic geometry checks are sampled;
volume uncertainty combines separately reported integration and construction
rounding estimates. `volume_error_estimates()` exposes both components, and
`effective_limits()` reports the linear and angular admission clearances.
Consult the returned validation
report and measured value when deciding whether that evidence meets an
application's requirements.

## Conical sector

`normal_extrude_cone` accepts an annular sector of the developed cone. Slant
distances use model units; `developed_angle` uses radians. A 30-degree cone
maps a developed-angle span of 0.35 radians to a physical span of 0.7 radians.
Using the options from the cylindrical example:

```rust
let sector = ConeSector {
    support: ConeSupport {
        apex: [0.0; 3],
        axis: [0.0, 0.0, 1.0],
        radial: [1.0, 0.0, 0.0],
        semi_angle: std::f64::consts::FRAC_PI_6,
        chart_origin: -0.3,
        normal: NormalSense::Outward,
    },
    region: SurfaceRegionId(2),
    slant: [8.0, 12.0],
    developed_angle: [0.0, 0.35],
    corners: [11, 12, 13, 14].map(SurfaceVertexId),
    boundaries: [11, 12, 13, 14].map(SurfaceCurveId),
};
let cone_result = normal_extrude_cone(&sector, [-1.0, 2.0], options)?;
let cone_body = cone_result.export_brep()?;
```

Normal depth changes both radius and axial position. The operation refuses
axis contact, unresolved apex/angle limits and any result that fails required
validation. This primitive entry retains its exact sector contract; the
candidate solved-profile entry below handles admitted polynomial curves. See the
[conical sector contract](CONE.md) for the coordinate map and supported domain.

## Full-turn bands

`normal_extrude_full_turn_band` takes an explicit `FullTurnBand` on a cylinder
or cone with two authored closed rims. It sweeps one full revolution and binds
the depth faces and rim sides to the decoded native occurrences. Each seam
has two opposite uses on its periodic face. Moving the construction seam
preserves the authored region and rim keys. The tests check both material
sides against the analytic volume and verify replay and input isolation.

The periodic aperture entries retain exact source intervals and signed sheet
transitions when a rectangular hole crosses the native cut. They also support
axially or meridionally separated apertures whose combined angular coverage
leaves no clear cut. Moving the native seam changes its split occurrences
while preserving the authored curves and material volume.

The curved cylinder and cone entries accept a simply connected aperture
with line and polynomial quadratic boundaries or exact circular and
elliptic quarters. The rational entries also accept positive-weight
quadratic spline boundaries. An aperture can occupy one clear sheet or
cross the selected cut transversely twice. Each crossing has an ordinal in
the original curve's parameter order. The result retains the authored
curve keys, the crossing intervals and the split images. The decoded
native trims and ruled walls must satisfy the original bounds before the
operation returns a solid.

## General wrapped profiles

An application can pass its named support members to
`CylinderFaceAttachment::bind_feature_sources` and use the returned references
to read the boss or pocket history. The binding checks the frozen body and
selected-face references before resolving any member.

The [API contract](PUBLIC-API.md) describes `normal_extrude_profile` for
cylinders and `normal_extrude_cone_profile` for cones. Both use the shared
executor and keyed incidence builder. The hosted Phase A receipts identify
PR #549 head `7a2aa0814a3f8c5b049aaec74b5d98bd26b296fe`, which landed at
`8f2b423103d13b22d9724747946b43ee1ca15ded` and reached `main` in #597.

For cones, Phase A admits solved world-XY developed polynomial profiles within
one strict family angular chart/wedge, away from the apex and axis throughout
the requested depths. The frame is currently exactly apex `[0,0,0]`, axis
`[0,0,1]`, radial `[1,0,0]`; unsupported placement refuses before construction.
The exact q/L normal reference preserves the authored material topology within
the requested epsilon. Decoded rational edges, ruled sides and analytic caps
form a separately bounded native proxy. Certified reference-body bounds,
decoded storage bounds, filled-cap correspondence and labelled native checks
remain separate in the result. The
[analytic-carrier decision](../../architecture/decisions/analytic-wrapped-profile-carriers.md)
extends the [approved rational contract](../../architecture/decisions/rational-wrapped-profile-representation.md).

The cone proof defaults to `FilteredBernsteinV4` for retained UV material
admission and allows `BernsteinV3` for replay. Results record requested limits,
effective leaf policies, and measured work and error.

### Candidate evidence and performance

`DirectedTransportedAuthoredPlanarV8` uses GeometricCompositeV6 to certify the
authored planar material before fitting. Set its authored component and cell
caps explicitly. The authored program draws cells from the operation-wide
integration allowance before the decoded UV, reference volume and proxy flux
checks. `WrappedConeNormalResult::authored_planar_receipt` reports the request,
effective loan and consumed cells. `DirectedTransportedPlacementV5` retains
CarrierV2 for authored material. Both selectors read the decoded UV proof
policy from `WrappedConeProofOptions::planar_proof`.

The v0.14.0 rebase has a new local performance baseline for the 239-curve
tool and attached boss, and the connected `O.` fixture. The stage profiles
include a matched Linux/glibc comparison of serial and four-worker
validation with and without heap trimming.
[Rebase measurements and profiles](PERFORMANCE-REASSESSMENT-2026-11-24.md).

The full Math library passed 825 tests. The full Sketch library passed 441
with no failures and 5 ignored controls in 142.42 seconds; public surface-sketch
integration passed 50 with no failures and 2 ignored controls in 25.90 seconds.
Ignored subprocess workers were exercised by their passed parent controls. The
explicit four-worker, three-case family contention stress passed in 34.97
seconds. Strict all-target Sketch Clippy passed before the declaration-order
formatting follow-up. Same-policy thread/process replay remains covered.

The earlier #544 full PR attempt stopped solely on root-module declaration order
before later stages ran. The corrected local run completed all planned 472 test
binaries and 40 doctest groups: 88402 passed, 0 failed and 157 ignored. The final
launcher exit status was not recovered. Component
[PR #548](https://github.com/Oblikovati/Martensite/pull/548)
was opened from `a72032dd` to `release/next`; those component results are
separate from the cone gate. The full local cone gate subsequently passed with
captured exit 0 at 18:25:07 UTC on 2026-11-18. Its
[bounded receipt](references/cone-canonical-pr-gate-2026-11-18-receipt.json)
records 41 affected packages, 473 native targets and 40 doctest crates:
89075 passed, 0 failed and 158 ignored, plus 9 separate unsafe-inventory passes.
The [verification record](VERIFICATION.md#canonical-cone-pr-gate-2026-11-18)
defines outer-result counting and exclusions. Gate A review 2 ended with
memory-only remediation; all four reference/body correctness findings were
resolved. The maintainer accepted
the case-specific footprint tradeoff on 2026-11-18. Final
[Gate A review 3](references/cone-gate-a-oracle-3-2026-11-18.md) accepted the
bounded phase commit and ready PR. The
[final hosted receipt](references/phase-a-549-hosted-final-receipt-2026-11-18.json)
records 55 successful and 4 skipped checks on PR head `7a2aa081`, including
successful Linux affected-package verification and Sonar checks. The new
[cone TSan receipt](references/phase-a-549-hosted-tsan-receipt-2026-11-18.json)
records actual execution of the public overlapping-operation/shared-reader
control: 1 passed, 0 failed, 0 ignored in 167.83 seconds. These conditions were
satisfied before merge `8f2b4231`. The local startup exit 139 remains a historical
failed attempt, not that hosted run. B/C and full-feature qualification remain
outstanding; no version, tag or release was authorized.

The [final accepted-workload campaign](references/cone-final-workload-benchmark-2026-11-18-summary.json)
uses source `64110e40` plus the preserved feature changes. In the same executable,
cone V3 API times were `9.177975/9.315035/9.701001` seconds and V4 times were
`4.741645/4.822891/4.973667`: medians `9.315035` and `4.822891` seconds. All three
V4 runs met the five-second target and every accepted assertion passed.
The pre-shared-executor cylinder artifact recorded `3.82/3.99/3.95` process-wall
seconds versus current `3.98/3.90/3.84`, with medians `3.95` and `3.90` seconds.
The ranges overlap and no runtime regression was observed in these runs; this
small nonexclusive-host campaign does not establish a statistical speedup.
All six cylinder runs preserved the accepted 221424-byte replay hash. Native
bytes, history and achieved bounds were also compared separately.

The older cylinder's median peak RSS was `54860` KiB versus current `56624` KiB,
an increase of `1764` KiB (about 1.7 MiB). Cone V3/V4 RSS ranges overlap.
A separate [memory diagnostic](references/cone-final-workload-benchmark-2026-11-18-memory-composition.json)
sampled cylinder VmRSS `55000 → 56632` KiB: file-backed RSS rose by `1568` KiB
and anonymous RSS by `64` KiB. Executable `r-xp` pages rose from `26932` to
`28520` KiB. Most of this sampled increase was mapped executable pages. The
snapshots are approximate and non-atomic, not an exact allocation count or an
exemption from the memory finding. Diagnostic `wait4_launch_peak_rss` values
include pre-exec launch context; actual benchmark peaks come from GNU-time rows.

The later [matched production-consumer comparison](PRODUCTION-MEMORY-ACCEPTANCE.md)
separates libtest footprint from production cost. After the #547 correction,
production median RSS remains `156` KiB above baseline and selected executable
sections remain `124784` bytes larger. Allocation-call counts match in the
separate diagnostic. On 2026-11-18 the maintainer explicitly accepted this
quantified Phase A code/process footprint tradeoff. The
[dated decision](PRODUCTION-MEMORY-ACCEPTANCE.md#maintainer-decision-2026-11-18)
grants no blanket runtime, heap, tolerance or performance waiver.

Final [metadata](references/cone-final-workload-benchmark-2026-11-18-metadata.json)
and [rows](references/cone-final-workload-benchmark-2026-11-18-rows.json) preserve
source/binary hashes, affinity and flags. The retained baseline is accepted
pre-shared-executor cylinder source `7817e8d1`, not a previous published cone
release. The [first campaign summary](references/cone-policy-benchmark-2026-11-18-summary.json),
[metadata](references/cone-policy-benchmark-2026-11-18-metadata.json) and
[rows](references/cone-policy-benchmark-2026-11-18-rows.json) remain unchanged.
That earlier driver stopped on a libtest prefix after successful case 7; token
extraction used the existing log and retained its metrics without rerunning
cone timings. The final campaign is recorded separately, with no discarded
first-campaign observations.

### B1 functional checkpoint, 2026-11-19

Two complete public-operation controls passed for an owned polynomial `O.`:
eight curves for the outer/counter pair and four for the dot, in two regions
and three loops. The outward API run took `150.110240` seconds and the inward
run `114.468438`. Both used 984593 of the original 1000000 arithmetic operations
and 501 integration subintervals, with epsilon `1e-4`, fit level 8, authored
level 6/width `1e-7` and 128-bit arithmetic. The logged `retained_spans: 167`
is a derived side-face count, summed as member `faces - 2`, not an independently
counted fit-span total.

The [compact functional receipt](references/b1-functional-punctuation-2026-11-19.json)
retains commands, environment, matching source-manifest hash and unprobed binary
identity. Source is HEAD `8f2b4231` plus uncommitted B1 code; that merge commit
alone does not contain B1. The branch uses explicit composite UV proof V5,
positive-X domain policy 2 with actual UV-trim/normal-material evidence, and
conditioned authored method 2 with per-batch pi reuse. Defaults and legacy replay
remain unchanged. These are runtime diagnostics, not a matched benchmark or
performance gate. No five-second acceptance was established; B1 was incomplete
at phase-gate attempt 0 of 3 at this initial checkpoint.

Main bug #555 passed its canonical local gate with unchanged source and its
specific RSS tradeoff received maintainer approval. [PR #556](https://github.com/Oblikovati/Martensite/pull/556)
merged tested head `63a60f5253a98c68f8081ef05248d6cf7943db11` into main at
`b7dc518b412036048e4f8582cf600aa040f86d64`, after
[CI run 44470644480](https://github.com/Oblikovati/Martensite/actions/runs/44470644480)
passed. The #557 qualification record landed at `e4e64a85c9ba7b877844197b8e8bdedda89c83ac`.
The #558 standing rule at `b61a031d0bde0e21d283e0f94975f79b9ae15574`
pre-approves up to 10% higher peak RSS for at least 50% lower runtime on matched,
accepted representative workloads, interpreted conservatively as halving runtime.
Accuracy, validation coverage and performance on other accepted workloads remain
required; this rule is separate from the specific #555 approval.

At that earlier checkpoint, F1 forward integration through #558 was clean.
Its canonical retry had built and was testing; that checkpoint recorded neither
a whole-gate pass nor a candidate merge. The [native-stage findings](references/b1-native-stage-findings-2026-11-19.md)
retain the rejected numeric-dependency experiment and its measurements. Issues
#496 and #498 remain open, as do #551, #552, #553 and #523 until validated
integration. The remaining full-capability requirements are unchanged.

### B1 cache/V5 functional checkpoint, 2026-11-19

After private TKBO cache/V5 integration, focused controls passed 19, 10 and 9
cases, followed by both complete public polynomial `O.` controls. Each run ended
with exit 0 and no source changes. Inward API time was `37.925987` seconds and
outward `39.965076`; each consumed the same 984593/1000000 arithmetic operations
and 501 integration cells, with 167 derived side faces. Math/Sketch runtime input
files were preserved. These timings were collected during concurrent F1 QA and
establish no matched performance result or five-second acceptance.

The [checkpoint and cleanup receipt](references/b1-cache-v5-functional-2026-11-19.json)
records commands, log/source hashes and preserved binary `61ed9ad5…`. Runs used
HEAD `1f709afa` plus uncommitted B1 work; the subsequent fast-forward to
`756a585d` changed only the #558 rule in `AGENTS.md`, preserving Rust inputs.
At this cache/V5 checkpoint, B1 review was 0 of 3, with independent actual
self-intersection coverage, performance and the full corpus still required.

The coordinator completed experiment cleanup after retaining the findings:
16 disposable files and the A experimental build cache (1.2 GiB) were removed.
Four immutable fixture/consumer/snapshot assets were verified unchanged. The
receipt preserves their hashes and cleanup scope; the broader EF investigation
continues.

### B1 bounded correctness checkpoint, 2026-11-19

Correctness review 2 accepted the remediation and Standards recheck 2 passed.
[The current findings and receipts](references/b1-directed-correctness-attribution-2026-11-19.md)
record two complete directed O. passes, five selection/replay controls and 218
focused Math controls, all with unchanged source guards. Each operation used
989355/1000000 arithmetic operations, 444 cells and 148 spans, with coverage
148/148/10730/592/4/6. Geometry remains Sampled and SI Exact under the ADR's
bounded proxy contract; no literal closed-native claim follows. Inward/outward
API times were 13.094710/13.385182 seconds; a separate restored stage probe took
13.388177 seconds outward. These are diagnostics, not matched performance or
five-second acceptance. Full canonical, hosted, concurrency/TSan, matched
runtime/peak-memory and the remaining capability matrix are still open. No B1
commit, push or release approval is recorded.

[Captured-input cone work](CONICAL-INPUT-WORK.md) records the original
and physical-angle grants, the 239-curve text qualification, and the
native validation timings and replay results for attached text.

[Recentered cone charts](RECENTERED-CONE-CHART.md) extend the public polynomial
operation past the datum-centered angular cut for families that fit within one
physical revolution. The [cone region atlas](CONE-REGION-ATLAS.md) uses
separate charts for disconnected regions and certifies separation after
wrapping. The [directed connected selector](CONE-REGION-ATLAS.md#directed-connected-validation)
certifies native self-intersection for connected profiles that need
several fit charts. Wider periodic text still needs qualification.
Primitive exact-span lowering,
replacement deletion and the broader acceptance corpus remain Phase C.

Earlier checkpoint records remain available: [rational implementation](RATIONAL-IMPLEMENTATION.md),
[profile input](PROFILE-INPUT.md), [storage certificates](STORAGE-CERTIFICATES.md),
[profile certificates](PROFILE-CERTIFICATES.md), [authored history](AUTHORED-HISTORY.md),
[cancellation/isolation](FAMILY-CANCELLATION.md), [replay](FAMILY-REPLAY.md),
[publication budget](PUBLICATION-BUDGET.md), [cap realization](CAP-REALIZATION.md)
and [planar work policy](PLANAR-WORK-POLICY.md). The existing cylinder
[text census](P5-WORKLOAD.md) and [oblique qualification](P5-OBLIQUE-QUALIFICATION.md)
do not qualify those scenarios for the new cone path.

The [revision-bound face attachments](FACE-ATTACHMENT.md) capture selected
cylinder or cone faces with their support bodies and follow explicit native
history. They check the authored footprint and both retained caps before
publishing a wrapped boss or pocket with composed history. Supported trims
include rectangular bands, bounded inner wires, and shaped outer wires with
certified endpoints in one UV lift. General support trims and full
hit-to-sketch integration under #420 remain open.

The [native boss/pocket controls](BODY-BOOLEAN-QUALIFICATION.md) exercise actual
Fuse/Cut operations, seam-crossing and oblique placements, input preservation,
Boolean correspondence and process/concurrent replay. These controls predate
the public trim-band feature operation.

The [rotation-aware frame proof](CHART-ROTATION-PROOF.md) supplies bounded
trigonometric and spatial-allocation evidence for the existing cylinder chart
placement path. [Placed cone profiles](CONE-PLACEMENT.md) reuse that frame
proof for oblique cone supports and explicit chart angles.
