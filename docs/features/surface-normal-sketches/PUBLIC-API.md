# Wrapped-profile API contract

The current candidate exposes cylinder and cone solved-profile operations from
`martensite_tksketch::surface_sketch` (#418). Phase A was integrated by
[PR #549](https://github.com/Oblikovati/Martensite/pull/549) into `release/next`
at `8f2b423103d13b22d9724747946b43ee1ca15ded`. Required hosted checks passed on
PR head `7a2aa0814a3f8c5b049aaec74b5d98bd26b296fe`, including actual new cone
TSan execution. The receipts were captured before merge and retain that tested
head. Phase A's curved outer contour with a hole completes through the public
entry, and the case-specific footprint tradeoff was accepted on 2026-11-18.
This is an unreleased bounded contract, not completion of general cone wrapping
or B/C qualification. Existing cylinder-rectangle and conical-sector APIs retain their
signatures and behavior.

## Entries and owned input

| Support | Public entries | Input and result |
| --- | --- | --- |
| Cylinder | `normal_extrude_profile`, `normal_extrude_profile_with_cancel` | `WrappedNormalRequest`, `WrappedNormalResult` |
| Cylinder with explicit chart placement | `normal_extrude_profile_on_chart`, `normal_extrude_profile_on_chart_with_cancel` | Existing cylinder request and `WrappedChartNormalResult`; see [frame contract](OBLIQUE-FRAMES.md). |
| Cone candidate | `normal_extrude_cone_profile`, `normal_extrude_cone_profile_with_cancel` | `WrappedConeNormalRequest`, `WrappedConeNormalResult` |

A request owns a solved `SketchProfile`, explicit `WrappedRegionSelection`
values, analytic support, increasing signed normal depths and
`WrappedNormalOptions`. Each selection names its authored region, outer loop and
holes; bounded cells are never selected implicitly. Current profile admission
uses lines and polynomial Bézier spans of degrees one through three in the
world-XY developed plane. Curve directions are resolved once; dyadic spans and
split vertices map back to authored parameter directions with integer arithmetic.

The cylinder input uses `[arc_length, height]`. Its existing physical placement
normalizes/projects a right-handed axis/radial frame and applies stored-frame
admission and the world-space budget. The ordinary entry still requires zero
chart origin; the chart-placement entries have their own recorded contract.
Their [oblique bounds](OBLIQUE-FRAMES.md) cover the decoded located graph and
direct location evaluation. A normalized analytic convenience-query copy is a
separate representation requiring correspondence evidence.

The cone input uses developed Cartesian `[x,y]` measured from the apex. Phase A
requires exactly apex `[0,0,0]`, axis `[0,0,1]` and radial `[1,0,0]`.
`admit_frame` returns `Unsupported` before source capture for other frames.
The whole family must fit one admitted strict angular chart/wedge, with positive
radius throughout its filled material and depth interval. The fitted reference
must also satisfy the chart and perturbation margin. Chart admission and finite
proof exhaustion return typed refusals. This scope does not establish wider
wraps, seam occurrences, text-scale families, oblique cone placement or attachment.
Those remain Phase B; exact primitive-span lowering and replacement deletion
remain Phase C.

## Cone request example

Given a solved developed `profile` and explicit `selections` that satisfy that
scope, the request uses the existing support and option types:

```rust
use martensite_tkbo::profile_extrusion::{PlanarProofVersion, PlanarProofWorkLimit};
use martensite_tksketch::surface_sketch::*;

let request = WrappedConeNormalRequest {
    profile,
    selections,
    support: ConeSupport {
        apex: [0.0; 3],
        axis: [0.0, 0.0, 1.0],
        radial: [1.0, 0.0, 0.0],
        semi_angle: 0.7,
        chart_origin: 0.0,
        normal: NormalSense::Outward,
    },
    depths: [-0.1, 0.2],
    options: WrappedNormalOptions {
        spatial_error: 1e-4,
        planar_work: PlanarProofWorkLimit::CurrentDefault,
        maximum_fit_level: 8,
        volume: WrappedVolumePolicy::Report,
    },
    proof: WrappedConeProofOptions::default(),
};
let result = normal_extrude_cone_profile(&request)?;
let detached = result.export_native()?;
assert_eq!(result.depths(), request.depths);
assert_eq!(result.proof_options(), request.proof);

let mut replay_request = request.clone();
replay_request.proof.planar_proof = PlanarProofVersion::BernsteinV3;
let replay = normal_extrude_cone_profile_with_cancel(&replay_request, || false)?;
```

`WrappedConeProofOptions::default()` selects `FilteredBernsteinV4` only for the
new unreleased cone request's retained UV material proof. Explicit
`BernsteinV3` remains available for same-policy replay; V3 and V4 may have
different work/cancellation traces. Existing cylinder and toolkit defaults stay
unchanged. The requested selector is rebound to the actual UV capture receipt
and reported by `reports()[&region].material.planar_proof`.

The cone proof defaults retain 128 fractional arithmetic bits and one finite
aggregate allowance of 1,000,000 charged operations. `integration` uses
`CylindricalFluxOptions::default()` as a shared finite reference-volume and
conditional proxy-flux policy. `WrappedConeAuthoredVolumeOptions` defaults to
subdivision level 6, clearance level 0 and enclosure width `1e-7`.
`maximum_fit_level` admits `0..=8`. These are explicit requested settings;
tighter requests are not silently enlarged. `proof_options()` returns the
original values, while leaf receipts retain actual remaining/effective settings
and consumed work. Zero work can refuse as incomplete; it is not a no-limit mode.

## Reference body and native representation

The [approved rational contract](../../architecture/decisions/rational-wrapped-profile-representation.md)
defines an exact normal extrusion of an epsilon-close profile with the same
topology, subject to separately certified storage bounds. For a cone the chosen
reference is the common-chart ideal q/L profile: q represents
`tan((theta-center)/2)` and L is the rational slant function. Its executed
composition binds original intervals, shared ideal endpoint identities,
monotone join projections, nonadjacent clearance, a continuous angular range
shorter than one turn and positive radius through the full depth interval.
The filled reference-body bound is distinct from the stored proxy's checks.

The native representation retains rational depth edges, common-weight two-row
ruled sides and analytic `GeomConicalSurface` caps. An admitted reference slant
`l0` gives cap radius `l0*sin(alpha)+depth*cos(alpha)`, axial origin
`l0*cos(alpha)-depth*sin(alpha)`, U equal to theta and V equal to `L-l0`.
Actual decoded edge/weight/row and cap-pcurve operands are certified. Shared
native incidence alone does not prove exact geometric closure or embedding of
the rounded patches. The actual UV material domain has its own topology and
filled-domain certificate; its bridge to the q/L reference is reported separately.
See the [analytic-carrier decision](../../architecture/decisions/analytic-wrapped-profile-carriers.md).

The operation enforces the developed displacement allocation at epsilon/4,
decoded depth-edge/full-ruling storage allocation at most epsilon/1000, and
original nonadjacent clearance at least 4*epsilon. The storage limit is enforced
separately before the total spatial comparison. Exact decoded rows and common
weights extend the maximum endpoint storage bound through the full ruling.
Cap traces, native tolerances, reference fill and the reference-to-proxy filled
cap bridge must also satisfy their actual allocations and the original total
spatial request. Passing a total epsilon cannot hide a failed storage allocation.

One shared executor owns source capture, selection, keyed construction,
freeze/decode, certification, native qualification, history and publication.
Cylinder and cone mappings use the common `construct_mapped` incidence builder;
support adapters provide mapping, carriers and mathematical receipts. Publication
moves the completed owned family once and rebinds required evidence before success.

## Result getters and evidence

Both result families own frozen bytes and authored history. For cones:

| Getter or report | Meaning |
| --- | --- |
| `history()`, `support()`, `depths()`, `options()`, `proof_options()` | Authored lineage and original requested input/settings. |
| `encoded_brep()`, `export_native()` | Immutable keyed archive and independently decoded native owners. |
| `reports()[&region]` | `WrappedConeMemberReport`: counted incidence, native validation, boundary/trace/tolerance, actual storage, material and measure evidence. |
| `budget()` | `WrappedConeBudgetReport`: source and aggregate planar work, aggregate arithmetic/integration work, requested/achieved spatial bounds, reference/bridge bounds and family volume comparisons. |
| `reference_body_receipt()` | Actual source-bound reference topology, filled-body and storage composition. |
| `reference_volume_receipt(region)` | `CertifiedConicalReferenceVolume`, with integration enclosure, effective policies and work. |
| `authored_receipt(region)`, `flux_receipt(region)` | Original-source volume and conditional proxy-flux receipts. |
| `boundary_flux_origin()` | Origin of `F(p)=(p-origin)/3`; the canonical cone uses its apex. |

`depth_storage_error[2]`, `ruled_storage_error` and `storage_allowance` report
achieved O3 bounds and the effective allocated limit. Family fields
`reference_developed_error`, `reference_filled_error`, `reference_storage_error`
and `cap_reference_proxy_error` retain separate comparisons.
`WrappedConeMaterialReport` fields `material.planar`, `material.planar_work` and
`material.planar_proof` describe the actual retained UV proof.
`material.chart_admission_tolerance` is the actual
scalar admission resolution in mixed `[V,U]` chart coordinates, not a physical
separation or spatial-error bound. Member arithmetic excludes family source
preparation; the family total includes it. `source_planar_work` remains distinct
from aggregate `planar_work`; integration cells aggregate reference and proxy work.

Incidence and cap/side-pcurve coverage count actual retained occurrences.
Ordered native validation keeps its original check modes, including sampled
geometry and native-search self-intersection coverage. Those modes do not become
exact proofs because reference or storage certification passed. Cylinder
reports retain their existing `.spatial`, `.measures` and placement report layout.

| Cone measure field | Evidence |
| --- | --- |
| `authored_volume` | Certified authored normal-volume enclosure under the admitted normal map. |
| `reference_volume` | Certified volume enclosure of the executed exact q/L reference body. |
| `boundary_flux` | Certified signed proxy patch-occurrence flux; it does not assert literal native patch closure. |
| `native_volume` | Native quadrature estimate and integration error estimate, separate from certified measures. |

`WrappedConeVolumeBounds` is used for both `member.volume` and `budget().volume`.
Its `discrepancy` preserves authored-versus-proxy-flux meaning, and
`integration_error` is the proxy-flux integration uncertainty.
`authored_reference_discrepancy` is the authoritative authored-versus-reference
body comparison; `proxy_reference_discrepancy` and `reference_integration_error`
remain separately labelled. Reference integration uncertainty is already
included in interval comparisons and is not added a second time.
`WrappedVolumePolicy::Refuse(allowance)` rejects an unmet aggregate
`authored_reference_discrepancy` for cones. `Report` retains the achieved bounds;
neither mode widens spatial or work limits. The existing cylinder policy is unchanged.

Inward normals normalize physical depths to `[-last,-first]` while public first
and last history roles remain caller-relative. Directed source spans and split
vertices retain authored identity. Detached export checks keyed correspondence
and duplicates before publishing entity maps. Exports share no native topology
owners; changing a request or one export leaves the frozen result and other
exports isolated. Published metadata has no mutation interface, and public
request/result types have compile-time `Send + Sync` controls.

Failures retain operation, stage, class, actual known authored entities and an
owned cause. An operation refusal is returned before a later cancellation poll
can mask it. Cancellation and exhausted required checks return no successful
partial geometry. Completed results, invalid input, unsupported scope,
incomplete work and cancellation remain distinct outcomes.

## Candidate measurements and remaining qualification

The full Math library passed 825 tests with no failures or ignored cases. Ten test-only
auto-trait controls cover all 63 new public Math declarations, including 47
previously missing direct `Send + Sync` assertions. This follow-up changes no
runtime algorithm. The
full Sketch library passed 441 with no failures and 5 ignored controls in
142.42 seconds; public surface-sketch integration passed 50 with no failures
and 2 ignored controls in 25.90 seconds. Ignored subprocess workers were
exercised by passed parent controls. The explicit four-worker, three-case
family contention stress passed 1/0/0 in 34.97 seconds. Same-policy process and
thread replay remains covered. Strict all-target Sketch Clippy passed before
the declaration-order formatting follow-up.

The library/public checks preceded source `64110e40`; its difference from tested
parent `de2b7590` is only the canonical root-module declaration reorder
`a72032dd`, with no behavior change. The final contention and benchmark runs
used `64110e40` plus the preserved feature working changes. The first #544 full
PR attempt stopped solely on that formatter ordering and ran no later stages.
The corrected local run completed all planned 472 test binaries and 40 doctest
groups, with 88402 passed, 0 failed and 157 ignored. Its final launcher exit
status was not recovered. Component
[PR #548](https://github.com/Oblikovati/Martensite/pull/548) was opened at
`a72032dd` against `release/next`; its component evidence is separate from the
subsequent full local cone gate. That gate passed with captured exit 0 at
18:25:07 UTC on 2026-11-18. The
[receipt](references/cone-canonical-pr-gate-2026-11-18-receipt.json) records 41
affected packages, 473 native targets, 40 doctest crates and 520 outer summaries:
89075 passed, 0 failed, 158 ignored and 0 filtered. The separate source unsafe
check passed 9 with 436 filtered. Eleven nested summaries were excluded from
these outer totals: seven SDK fixtures, including one intentional negative
fixture, and four isolated worker summaries. The
[verification record](VERIFICATION.md#canonical-cone-pr-gate-2026-11-18)
retains the exact scope, source identity and command/profile inventory.
Gate A review 2 ended with memory-only remediation after resolving all four
reference/body correctness findings. The maintainer accepted the quantified
Phase A code/process footprint tradeoff on 2026-11-18. Final
[review 3](references/cone-gate-a-oracle-3-2026-11-18.md) accepted the bounded
phase commit and ready PR. The [dated decision](PRODUCTION-MEMORY-ACCEPTANCE.md#maintainer-decision-2026-11-18)
does not change runtime, heap, tolerance or general performance policy. Required
hosted checks then passed on `7a2aa081`: the
[final receipt](references/phase-a-549-hosted-final-receipt-2026-11-18.json)
records 55 successful and 4 skipped checks, and the
[cone TSan receipt](references/phase-a-549-hosted-tsan-receipt-2026-11-18.json)
records the actual public concurrency control passing 1/0/0 in 167.83 seconds.
PR #549 subsequently merged at `8f2b4231`. The earlier ordinary concurrency and
46 driver/9 inventory controls remain separate evidence. Local startup exit
139 is preserved as a historical failed attempt; hosted cone instrumentation
subsequently ran and passed. This does not complete #414/#418 or B/C.

The final campaign compares the two cone policies in the same executable and
uses a stronger cylinder baseline from accepted source before the shared
executor. Every accepted-workload assertion passed.

| Final workload | Three recorded seconds | Median seconds | Peak RSS range, KiB |
| --- | --- | --- | --- |
| Cone V3, public API timer | 9.177975 / 9.315035 / 9.701001 | 9.315035 | 56080 to 56560 |
| Cone V4, same executable/API timer | 4.741645 / 4.822891 / 4.973667 | 4.822891 | 56404 to 56608 |
| Pre-shared-executor cylinder, process wall | 3.82 / 3.99 / 3.95 | 3.95 | 54736 to 54872 |
| Current cylinder, process wall | 3.98 / 3.90 / 3.84 | 3.90 | 56304 to 56680 |

All three V4 API measurements met the five-second target on this accepted
workload. Cone V3/V4 RSS ranges overlap. Cylinder wall ranges also overlap,
with no observed runtime regression in these runs; three nonexclusive-host
samples per case do not establish a statistically proven speedup. All six
cylinder replays retained 221424 bytes and SHA256
`ca2e7ed51f75ba57caced69ce679d3a0f87afecf9c70205bb1de0cc5dea00841`.
Native bytes, history and achieved bounds were also compared separately.

The CPU-6 campaign included one whole test-process time of 5.01 seconds while
its API call took 4.973667 seconds. A fixed three-run
[whole-fixture guard](references/cone-whole-fixture-guard-2026-11-18.json) repeated
the final binary on the initial campaign's CPU 10. API calls took
4.623892, 4.612031 and 4.749587 seconds; complete test processes took
4.65, 4.64 and 4.78 seconds. All accepted assertions passed. This checks the
stricter five-second fixture target while retaining the slower CPU-6 observation.

Cylinder median GNU-time peak RSS increased from `54860` to `56624` KiB,
by `1764` KiB (about 1.7 MiB). The separate read-only
[memory-composition diagnostic](references/cone-final-workload-benchmark-2026-11-18-memory-composition.json)
sampled the following values:

| Sampled cylinder quantity, KiB | Earlier artifact | Current | Difference |
| --- | --- | --- | --- |
| VmRSS | 55000 | 56632 | +1632 |
| File-backed RSS | 35336 | 36904 | +1568 |
| Anonymous RSS | 19664 | 19728 | +64 |
| Executable `r-xp` mapping | 26932 | 28520 | +1588 |

Most of the sampled RSS increase was mapped executable pages. Sampling every
25 ms and combining non-atomic status/mapping snapshots gives approximate
composition, not exact allocation counts or proof of the cause of every byte.
This diagnosis does not remove the measured footprint increase from review.
`wait4_launch_peak_rss` in that diagnostic can include pre-exec launch context;
it is not a benchmark process peak. Use the GNU-time campaign rows for actual
process peak RSS.

Final [metadata](references/cone-final-workload-benchmark-2026-11-18-metadata.json),
[rows](references/cone-final-workload-benchmark-2026-11-18-rows.json) and
[summary](references/cone-final-workload-benchmark-2026-11-18-summary.json) retain
exact data and source/binary hashes. Runs used CPU affinity 6 on a nonexclusive
host, with the recorded workspace debug assertions and package optimizations,
stripped debug information, two build jobs, no incremental compilation and no
RUSTFLAGS override. The cylinder baseline is source
`7817e8d1f6c8c502fa860d4300afe32f426c603e`, retained executable SHA256
`a3b8033108444b98a0a49fb0f3ae96c61e4cf9d2b6ecc1a2743228a873390bdd`.
This is not a comparison against a previous published cone release.

The initial campaign remains intact in its own
[metadata](references/cone-policy-benchmark-2026-11-18-metadata.json),
[rows](references/cone-policy-benchmark-2026-11-18-rows.json) and
[summary](references/cone-policy-benchmark-2026-11-18-summary.json). It used a
retained pre-#542 cylinder baseline and CPU affinity 10. Its cone medians were
`8.698827` and `4.412819` seconds. Case 7 succeeded before its driver stopped on
a libtest-prefix parser assumption; the replay token came from that same log,
original metrics were retained, and no cone timing was rerun. Both campaigns
are preserved rather than selecting only the faster observations. Neither
campaign grants remaining kernel/application acceptance or a release.

## Uncommitted B1 opt-in path, 2026-11-19

HEAD `8f2b423103d13b22d9724747946b43ee1ca15ded` contains merged Phase A. The B1
implementation is additional uncommitted work, not part of that commit or a
qualified extension of the full API contract. Its explicit entry points are
`normal_extrude_cone_profile_with_domain_policy` and
`normal_extrude_cone_profile_with_domain_policy_and_cancel`.
`WrappedConeDomainPolicy::LegacySectorV1` remains the existing default;
`PositiveXCommonChartV2` selects actual UV-trim and normal-material evidence in
one positive-X common chart. `result.domain_policy()` retains that choice, and
`chart_domain_receipt()` exposes its executed V2 receipt (legacy returns `None`).

The branch's owned polynomial punctuation control explicitly requests composite
UV proof V5 while retaining the original finite limits. Given an admitted owned
`request`, the selection is:

```rust
let mut b1_request = request.clone();
b1_request.proof.planar_proof = PlanarProofVersion::FilteredCompositeBernsteinV5 {
    max_components: 64,
    max_total_subintervals: u64::from(b1_request.proof.integration.max_total_subintervals),
};
let result = normal_extrude_cone_profile_with_domain_policy(
    &b1_request,
    WrappedConeDomainPolicy::PositiveXCommonChartV2,
)?;
assert_eq!(result.domain_policy(), WrappedConeDomainPolicy::PositiveXCommonChartV2);
```

The V2 path explicitly selects `ConicalAuthoredFluxMethod::ConditionedSimpsonV2`
and reuses pi bounds per bounded arithmetic batch. The legacy authored method,
V4/default selectors and earlier replay paths remain selectable and unchanged.
No tolerance or work limit is widened by these selections.

Two complete outward/inward `O.` fixture controls passed with unchanged source
within each run and identical pre-run manifests. API times were `150.110240`
and `114.468438` seconds; both consumed 984593/1000000 arithmetic operations and
501 integration subintervals. The fixture has eight O curves plus four dot
curves, two regions and three loops, at epsilon `1e-4`, fit level 8, authored
level 6/width `1e-7` and 128 bits. Reported `retained_spans: 167` means derived
side faces (`faces - 2` per member), not an independent fitted-span census.
The [compact receipt](references/b1-functional-punctuation-2026-11-19.json)
binds these results to their commands, profile, manifest and unprobed binary.
They are runtime diagnostics, not a matched benchmark or performance pass;
B1 remains incomplete with phase gate 0 of 3. No five-second or full B/C
capability acceptance follows.

## Historical checkpoints

P4's canonical-cylinder checkpoint passed 199 library tests in 30.59 seconds
and 7 public integration controls in 4.22 seconds. Its subprocess worker was
explicitly run by the parent; a separate stress campaign had its own gate.
The later [oblique qualification](P5-OBLIQUE-QUALIFICATION.md) records 274 passing
library tests, 40 public integration controls, final placement checks and strict
Clippy. These are historical cylinder records, not current cone qualification.
The [P4 review](P4-REVIEW-2026-11-14.md) and
[cap-realization composition](CAP-REALIZATION.md#filled-cap-distance-composition)
retain the earlier review and accounting history. The current bounded candidate
review accepted a commit and ready PR; required hosted checks, including actual
new cone TSan execution, passed before the Phase A merge. The local and hosted
evidence is recorded above. No package version or release tag is authorized by
these records, and B/C qualification remains outstanding.
