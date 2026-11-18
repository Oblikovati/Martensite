# Surface sketches and normal extrusion

Feature [#414](https://github.com/Oblikovati/Martensite/issues/414) adds sketches
on cylinders and cones. A feature grows from its sketch along the support's
normal at each point. Existing planar sketches and vector extrusions retain
their interfaces and behaviour.

The primitive entries accept developed cylindrical rectangles and conical
annular sectors, including their admitted seam crossings. Solved-profile
cylindrical extrusion has a separate owned-result API. The current unreleased
candidate also completes the Phase A curved-profile cone workload with a hole
under the restricted contract below. Final Gate A review accepted a phase commit
and ready PR; required hosted checks, including new cone TSan, must pass before
merge. Wider wrapping and attachment capability remain outside that acceptance.

## Existing kernel support

`tkbo::profile_extrusion` accepts solved planar curves, builds bounded regions,
and extrudes a region along a fixed vector. It supplies history, validation and
measured volume. It does not solve a general sketch constraint system.

`tkg3d` supplies analytic cylinders and cones, normal-offset surfaces and the
`Adaptor3dCurveOnSurface` evaluator. These are geometry building blocks. The new
operation must also construct shared topology, account for periodicity, retain
source identity and validate the completed solid.

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

## General wrapped profiles

The [API contract](PUBLIC-API.md) describes `normal_extrude_profile` for cylinders
and the unreleased `normal_extrude_cone_profile` candidate. Both use the shared
executor and keyed incidence builder. Existing cylinder and primitive
interfaces retain their signatures and behavior. The cone work is preserved
above candidate integration `64110e402e131afff2e3fadcbf9e2b9d4c9b245c` plus feature
working changes. It remains unreleased; review acceptance covers the bounded
phase commit and ready PR only. The change
from the earlier `de2b7590` checkpoint only reorders canonical root-module
declarations and changes no behavior.

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

The new cone-only proof default selects `FilteredBernsteinV4` for retained UV
material admission. `BernsteinV3` remains explicitly selectable for replay;
existing cylinder and toolkit defaults are unchanged. Requested limits,
effective leaf policies and actual work/error receipts are retained separately.

### Candidate evidence and performance

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
bounded phase commit and ready PR. Required hosted checks, including new cone
TSan, remain pending and must pass before merge; B/C have not started. Ordinary
cone concurrency passed, and sanitizer-driver/inventory checks passed 46 and 9
tests respectively. The fresh
TSan startup fixture exited 139 before `main`; no instrumented cone control was
built or run.

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

Wider profiles, text-scale cone workloads, multiple charts and seam occurrences,
oblique placement and attachment remain Phase B. Primitive exact-span lowering,
replacement deletion and the broader acceptance corpus remain Phase C.

Earlier checkpoint records remain available: [rational implementation](RATIONAL-IMPLEMENTATION.md),
[profile input](PROFILE-INPUT.md), [storage certificates](STORAGE-CERTIFICATES.md),
[profile certificates](PROFILE-CERTIFICATES.md), [authored history](AUTHORED-HISTORY.md),
[cancellation/isolation](FAMILY-CANCELLATION.md), [replay](FAMILY-REPLAY.md),
[publication budget](PUBLICATION-BUDGET.md), [cap realization](CAP-REALIZATION.md)
and [planar work policy](PLANAR-WORK-POLICY.md). The existing cylinder
[text census](P5-WORKLOAD.md) and [oblique qualification](P5-OBLIQUE-QUALIFICATION.md)
do not qualify those scenarios for the new cone path.

The [revision-bound face attachment](FACE-ATTACHMENT.md) owns a selected cylindrical
face and support snapshot, follows explicit native history, and refuses ambiguous
reattachment. Wrapped boss/pocket and hit-mapping integration under #420 remain open.

The [native boss/pocket controls](BODY-BOOLEAN-QUALIFICATION.md) exercise actual
Fuse/Cut operations, seam-crossing and oblique placements, input preservation,
Boolean correspondence and process/concurrent replay. The unified body-feature
operation and its whole-profile trim contract remain open under #420.

The [rotation-aware frame proof](CHART-ROTATION-PROOF.md) supplies bounded
trigonometric and spatial-allocation evidence for the existing cylinder chart
placement path. Extending accounted placement to cones remains Phase B.
