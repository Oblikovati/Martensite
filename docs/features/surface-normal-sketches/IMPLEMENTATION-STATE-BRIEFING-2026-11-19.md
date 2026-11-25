# Surface wrapped Sketch3D implementation state

Updated: 2026-11-20. Implementation is active. This record includes
the V6 measurements, exact-point optimization and version-2 volume controls.
Remote commit and issue states remain those observed at the original snapshot
unless a later receipt is named.

## Delivery verdict

The bounded cylinder and cone foundations are implemented. Phase A's
single-chart curved cone operation is merged into `release/next`. The newer B1
directed cone operation and geometric planar proof V6 work locally, including
the narrow-cone `O.` fixture with a hole and a disconnected dot in both normal
senses. The V6 checkpoint passed 206 targeted tests; the later volume-policy
round passed 140 partially overlapping tests, detailed below.

The post-lint pass completed 256 selected tests, including eight complete cone
operations. This overlaps earlier coverage. Math, Boolean and Sketch Clippy
checks passed with warnings denied. The [cleanup receipt](references/b1-lint-regressions-2026-11-20/summary.json)
records 20 source paths, commands, source checks and verbatim logs. Failure-only
payload storage changed in the unpublished cone error and a private profile
error; successful operations gained no allocation. The five-second time/RSS
gate and the full affected-package gate remain pending for B1.

The full capability is not delivered. The B1/V6 foundation is checkpointed in
`6fca95d2`, with later implementation at `7747b805`. Exact-point measurement
(#569), shared indexes (#566/#567), and scalar owned metadata reuse (#572) are
incorporated locally; none of B1 is pushed. The five-second target remains
unqualified. PR #571 remains green and held for performance acceptance.
Wider charts, periodic topology, full text-on-body acceptance,
integration and final QA remain open. B1 has no integration PR.

## Required product behavior

Feature [#414](https://github.com/Oblikovati/Martensite/issues/414) requires
surface-bound cylinder/cone sketches and normal extrusion
`X(u,v,t) = S(u,v) + t N(u,v)`. Authoring uses developed surface coordinates in
model units, with explicit orientation, seams, winding, authored identity and
support revisions. Results must include history, ordered validation,
diagnostics and achieved error. Singular or unverified results must refuse.

The seven supplied PNGs add a concrete application case: select a cylindrical
lateral face, resize an editable wrapped layout area, place `This thing` and
`works!` on separate lines, and emboss or engrave the glyph material into the
existing body. Counters and disconnected glyph components must survive.
Layout bounds must not become a rectangular plaque unless selected as material.
The application owns gestures and rendering; the kernel supplies the modelling.

| Tracked scope | Current state |
| --- | --- |
| #415 geometry, ownership and result contracts | Closed. |
| #416 complete cylindrical rectangular operation, including seam crossing | Closed; merged foundation. |
| #417 conical primitive away from the apex | Closed; merged foundation. |
| #418 general profile curves, holes and disconnected material | Open; cylinder work and bounded cone Phase A exist, B1/V6 is local, broader rational/conic/spline and cone scope remains incomplete. |
| #419 full bands, periodic holes and seam-independent topology | Open; full/multiple wraps are not qualified by the current common-chart cone operation. |
| #420 attached bosses/pockets, support revisions and replay | Open; attachment and coordinate primitives exist, full body-feature integration remains unqualified. |
| #425 multiline text workflow | Open; owned capture and both bounded 18-region cylinder boss/pocket controls pass. Persistent full multiline body edits/replay and cone text remain outstanding. |

See [visual acceptance](VISUAL-WORKFLOW.md), [owned text capture](TEXT-CAPTURE.md)
and [public API](PUBLIC-API.md). Existing solved-profile input is not a new
general sketch constraint solver.

## Approved design and result meaning

The maintainer approved rational edges, the result contract of exact normal
extrusion of a profile within epsilon of the authored profile with the same
topology, and archiving the earlier unfinished implementation. The archived
checkout remains separate from the replacement.

The replacement uses paired rational depth edges, common parameter subdivisions
and weights, two-row ruled sides, and analytic cylinder/cone depth caps. A shared
executor owns capture, explicit material selection, keyed construction,
freeze/decode, certification, native qualification, history and publication.
Cylinder and cone reuse the `construct_mapped` incidence builder. Surface
adapters supply mappings and mathematical receipts.

For a cylinder, developed coordinates are arc length and height; outward normal
depth changes the radius. For a cone, developed Cartesian coordinates are
measured from the apex. With slant `l`, semi-angle `alpha` and outward signed
depth `d`, the geometric intent has radius `l*sin(alpha)+d*cos(alpha)` and axial
coordinate `l*cos(alpha)-d*sin(alpha)`. Normal sense and authored depth roles
remain explicit.

The cone's certified reference is the ideal common-chart q/L normal body, with
`q = tan((theta-center)/2)` and rational slant L. The decoded native geometry is
a separately bounded proxy. The current evidence does not claim that rounded
analytic-cap traces and all literal stored patches form an exactly closed
embedded surface. The ADR separates reference topology and volume, decoded
storage/trim bounds, filled-domain correspondence and native validation.

In the directed B1 controls, Geometry is **Sampled**, SelfIntersection is
**Exact under the declared contract**, and the whole validation report is
**not exact**. Those labels must remain visible.

The error allocation retains original requests: developed reference displacement
at most epsilon/4, depth-edge/ruling storage allocation at most epsilon/1000,
and nonadjacent clearance at least 4*epsilon, with the additional certified
margins. Spatial, angular, parameter, approximation and integration limits stay
separate. Refusal does not widen tolerances or work grants. Native volume
estimates, reference volume bounds and proxy flux are distinct measurements.

The [carrier ADR](../../architecture/decisions/analytic-wrapped-profile-carriers.md)
and [rational representation](../../architecture/decisions/rational-wrapped-profile-representation.md)
record the contract.

## Implemented components and current limits

| Component | Implemented state and boundary |
| --- | --- |
| Primitive surface operations | `surface_sketch::normal_extrude` for cylindrical rectangles and `normal_extrude_cone` for developed cone sectors; original interfaces remain available. |
| Solved cylinder profiles | `normal_extrude_profile` and cancellation entry; explicit chart-placement entries; selected polynomial line/quadratic/cubic material with holes and disconnected components. Full application acceptance remains separate. |
| Cone Phase A | `normal_extrude_cone_profile` and cancellation entry; bounded curved holed workload. Canonical apex `[0,0,0]`, axis `[0,0,1]`, radial `[1,0,0]`; one strict chart and positive radius over filled material and all depths. |
| B1 domain choice | `WrappedConeDomainPolicy::PositiveXCommonChartV2`; `LegacySectorV1` remains the default. Requires an admitted positive-X common chart; it is not a multi-chart atlas. |
| B1 execution choice | `DirectedCommonChartRequireCertifiedV1` uses directed construction and requires its certified SI evidence. Missing/incomplete certification refuses; native search does not substitute for it. Actual code admits composite V5 and V6. |
| Owned text input | `brep_font::capture_owned_font_text` in TKV3D captures exact owned font bytes, scalar occurrence IDs, glyph prototypes, multiline outlines, counters and disconnected material. It owns no live native font/library handle after publication. Cluster shaping is outside that scalar-occurrence interface. |
| Attachment | `CylinderFaceAttachment` captures/restores owned selected-face records, verifies revisions, exports detached support and follows explicit history with ambiguity refusal. Chart hit-to-developed mapping and persistence are present. |
| Boss/pocket workflow | [Both complete multiline cylinder compositions](MULTILINE-ATTACHMENT-2026-11-20.md) pass through existing Boolean/history APIs. Persistent attachment and full body edits/replay remain unqualified. |

The pipeline reuses TKBO planar regions/contact transport and TKMath exact
predicates/integration. Text capture stays above modelling without a TKSketch
viewer dependency. Display tessellation supplies no modelling or acceptance evidence.

## Geometric planar proof V6

`PlanarProofVersion::GeometricCompositeV6` is an additive explicit selector.
V2 carrier, V3 Bernstein, V4 filtered signs and V5 composite integral behavior
remain selectable. Requested/effective integration grants and actual work are
retained separately.

The new Math report owns its captured source and policy. For supported degree
at most three, full-unit-interval polynomial inputs, including explicit unit
weights, it tries exact control-box exclusion, monotone-hodograph self exclusion,
a sole shared-endpoint sector proof, and supporting-line hull exclusion.
Unsupported or inconclusive cases use the existing general solver with the
remaining split allowance. Logical self/distinct pair order is retained.

Endpoint evidence carries actual endpoint parameters and the shared point.
`ContactProofKey` distinguishes endpoint evidence from a projected root. No
algebraic root bracket or multiplicity is invented. Native parameter measurement
uses the #569 exact-point path for endpoints; nonpoint intervals retain the
existing construction. Earlier projected-root diagnostic formatting is preserved.

V6 still retains the complete logical pair matrix. Sparse sweep coverage from
the specialist prototype has not been integrated. Native partition correspondence,
cardinality, regularity, authored joins, graph orbits, nesting, integral programs,
cone construction and publication checks still execute.

The Rust prototype's finite `2^-600` underflow counterexample and scaled safe
hairpin are regression controls. Production reuses Martensite's exact orientation
and integer predicates. Prototype 0.2 corrected its predicate, order and profile
faults. Standalone reproduction matched 148/148 recorded contacts without
unresolved pairs; that is not a full-kernel performance measurement.

See [V6 implementation and receipts](CENSUS-V6.md) and
[prototype validation](CENSUS-V2-VALIDATION-2026-11-19.md). The supplied ZIPs in
the root checkout are user inputs; disposable extracted implementations and
their build directories were removed.

## Current complete cone witness

The owned `O.` fixture has eight authored cubic curves for the outer contour
and counter plus four dot lines: twelve curves, three loops and two material
regions. Its narrow-cone semi-angle is 0.125, depths are `[-0.1, 0.2]`, requested
spatial error is `1e-4`, and maximum fit level is 8.

| Retained work or coverage | Count |
| --- | ---: |
| Fitted side spans | 148, comprising O137 and dot11 |
| Integration cells | 444 |
| Metric arithmetic operations | 989355 of the original 1000000 |
| Side self domains / adjacent unions | 148 / 148 |
| Nonincident side pairs | 10730 |
| Side/cap products | 592 |
| Cap self domains / cap pairs | 4 / 6 |

Both normal senses pass the current public-operation controls, including native
export, history, independent checks and source/native mutation isolation. This
witness does not establish arbitrary text-scale cones, every chart quadrant,
full turns, oblique cones or attached Boolean features.

## Validation evidence

| Candidate and scope | Executed result | Limit |
| --- | --- | --- |
| Merged Phase A, PR #549 | Hosted qualification on `7a2aa081`; 55 successful/4 skipped checks and actual new-cone TSan passed; merged at `8f2b4231`. | Applies to that bounded source, not later B1/V6. |
| Current B1/V6 targeted qualification | 184 affected TKBO tests, 18 Math tests including 14 new geometric controls, and 4 Sketch tests: 206 passed. | These are scoped Rust tests, not the whole suite or a BDD scenario total. |
| Final B1/V6 public cone controls | Inward 7.920225 s; outward 7.853291 s; work and coverage above retained. | Correctness-run diagnostics, not matched acceptance timings. |
| Initial shared-fix candidate | #567's 8 focused tests; combined native-contact/reuse run 62 passed. | Before the later no-index cold path. |
| Latest shared-fix candidate | 68 focused tests, including five new cold-source controls and an existing complete named-body publication/isolation test. Normal production build passed with zero source changes. Both source-review axes accepted. | New matched benchmark and canonical/hosted gates remain pending. |

The current B1 source receipt is
[final checks](references/census-v6-integration-2026-11-19/final-checks.json), with
[changed Rust source hashes](references/census-v6-integration-2026-11-19/final-source.json).
The 206 count and the separate 68 count must not be added into a claimed unique
whole-feature or BDD total. Older package/sanitizer results remain historical
evidence for their recorded source snapshots.

## Performance evidence and unresolved bottlenecks

The original instrumented outward operation was 13.388177 s. Planar preparation
was inside that API call and took 8.093126482 s: original twelve-curve preparation
0.111993074 s, O137 7.678697725 s and dot11 0.302435683 s. O137 included Census
5.694264686 s, contacts 1.617544552 s and partition 0.364117301 s. Native
qualification/estimates consumed about 2.006434236 s; about 3.145660424 s remained
unattributed. Nested actual-SI timings must not be added twice.

The old O137 Census made 9453 evaluations with no reuse hits and 44,674,878 report
scan visits. Its lookup share was only about 0.1005 s. General endpoint fallbacks
were the larger Census cost. Structural contact rescans were separately confirmed.
These measurements do not attribute the current V6 remainder.

| Comparison | API time | Peak RSS | Interpretation |
| --- | --- | --- | --- |
| Same-binary V5/V6 outward O., two balanced pairs | 13.649724 to 7.633674 s, 44.1% lower | 114702 to 64846 KiB, 43.5% lower | Diagnostic prefix; final sixth sample was refused by a compiler guard. Unmatched third V5 sample is retained separately. No previous-release or final performance acceptance. |
| First combined #566/#567 candidate, 56-curve public planar API, three AB/BA/AB pairs | Median 0.442821750 to 0.350690716 s, 20.8% lower | Median 35636 to 35712 KiB, +76 KiB or 0.213% | All public/evidence/native bytes matched; no pairs flagged; no samples retried/replaced. The memory increase is outside automatic approval because runtime did not halve. |
| Latest cold-source candidate, cold-r2, three declared pairs | Median 0.471320415 to 0.406096010 s, 13.8% lower | Median 35752 to 35668 KiB, 84 KiB lower | All signatures/native bytes matched and all guards passed. Pair 1 exceeded the 5% sibling-busy threshold (5.769%); all six samples remain included. Descriptive result, not clean performance acceptance. The earlier cold-r1 stopped after baseline identity and has zero timed samples. |

The subsequent #567 refinement eliminates the lookup map when reports are empty
and the complete physical-source mapping equals occurrence positions. Every
logical pair/mode key is then unique, so all original lookups would miss. Aliased,
seeded, incomplete or nonidentity inventories retain the ordered first-match
index. It skips no numerical query, work charge or cancellation checkpoint.
The new memory outcome must be measured; it cannot inherit the first campaign's
numbers.

An earlier, narrower Phase A holed-cone comparison recorded V4 median API time
4.822891 s, with all three V4 observations below five seconds. The
[earlier workload receipt](references/cone-final-workload-benchmark-2026-11-18-summary.json)
uses a different bounded workload; it does not qualify the current narrow O.
fixture or its directed/V6 implementation.

The 2026-11-13 review states the walking-skeleton target as **five seconds in a
debug build**. The current narrow-cone result is above that limit. Checked-release
profiling is useful additional evidence, but does not change that criterion
without a reviewed decision. The standing memory pre-approval allows at most
10% higher peak memory when matched accepted workloads show at least 50% lower
runtime; accuracy, coverage and other accepted runtime behavior remain required.

### Current V6 stage attribution

Two new instrumented runs passed the complete outward `O.` operation checks.
The broad pinned run took 12.342 s; the deeper unpinned run took 9.502 s.
These are single diagnostic observations on a shared host, with different
instrumentation and affinity. Concurrent Cargo activity was recorded. They do
not replace the balanced 7.634 s comparison or establish a regression or an
acceptance result.

The deeper run accounts for the operation as follows. Rows are disjoint;
substage details below them must not be added again.

| Stage | Seconds |
| --- | ---: |
| Selection, fitting and initial authored-profile preparation | 0.462 |
| Native construction, encoding, coverage and history | 0.295 |
| Decoded storage and binding | 0.203 |
| Member geometry, material and measures | 5.845 |
| Actual family SI certificate capture/issue | 0.109 |
| Native ordered qualification and numerical volume estimates | 2.443 |
| Family budget and publication, including executor remainder | 0.143 |
| Complete operation | 9.502 |

O137's decoded UV material phase took 4.514 s. Nested within it, Census took
0.355 s, partition 0.466 s and contact transport 1.979 s; another 1.714 s remains
inside the UV material phase outside those three calls. That remaining nested
cost still needs splitting between graph proof, region construction and
material binding. O137's certified reference-volume and native-flux measures
took 0.578 s and 0.541 s. Its ordered native checks took 2.190 s, including
0.724 s topology, 1.387 s sampled geometry and about 0.064 s rebound SI authority;
its separate numerical volume estimate took 0.103 s. These measurements support
prioritizing contact reuse, native qualification and measures over more Census
work for this fixture.

The metric allowance is a separate text-scale gate. Exact charged deltas sum to
989355 units: selection 737114, storage 159434, member geometry 54382, actual SI
35904 and final reference/budget 2521. Within selection, fitted-boundary work
used 688754 units and original-curve radius checks used 6664. Fitted-boundary
work alone consumes 69.6% of the original million-unit grant; it is not a
measures-only budget problem. Measures can take substantial time while charging
few units, so this counter is not a CPU-time model. The observed per-span ratio
is not a proven scaling law for text. The subsequent operand probe observed
O137 reference-volume running sums up to 7115/7108 numerator/denominator bits
after reduction, despite input polynomial maxima of 219/209 bits; see the
[operand receipt](references/volume-bits-attribution-2026-11-19/summary.json).

A later fit-charge pass confirmed 284 exact attempts: 148 successful leaves
charged 433797 units, and 136 refused parents charged 254957. See the
[follow-up checks](FOLLOWUP-PERFORMANCE-CHECKS-2026-11-19.md) and
[fit receipt](references/fit-charge-attribution-2026-11-19/summary.json).
The current Sketch and Math admission policies both cap a request at one million
units; a larger input-sized allowance requires explicit policy work.

Before text acceptance, measure work versus authored and fitted span counts,
including refused fitting candidates, then reduce duplicated certified work or
review an explicit finite input-sized allowance. No allowance was raised in
these passes. A cylinder's developed-to-UV map is linear; its 3D embedding still
uses sine and cosine, so its refinement cost needs its own evidence.

All 346 frozen changed Rust paths matched after restoration. Temporary source,
probe binaries and the instrumented emitted artifacts were removed. Raw logs,
profiles, stage records, work deltas and restoration hashes are retained in the
[current attribution receipt](references/v6-stage-attribution-2026-11-19/summary.json).

The earlier [PhysCAD reference receipts](references/physcad-reference-2026-11-17/README.md)
cover wrapped cylinder/cone text, seams, holes, joins and cuts. They were
reconciled from the previous worktree without new runs. Owned 239-curve cone
text had complete profile selection and native validity; cylinder text lost a
region and failed its exact-volume oracle. These observations do not qualify
Martensite performance or replace the full application acceptance workflow.

## Open defects and integration status

| Issue | Current state |
| --- | --- |
| #560 non-passing native reports admitted by legacy cone publication | Local remediation and controls exist; issue remains open until validated integration. |
| #561 signed-zero cardinal frames rejected by directed gamma admission | Local remediation and controls exist; unmerged. |
| #562 admitted side-boundary source bits lost by native binding | Local snapshot/binding remediation and mutation controls exist; unmerged. |
| #563 later cancellation masking an unsupported proof selection | Local precedence regression passes; unmerged. |
| #566 repeated native report/endpoint/vertex discovery | Implemented and locally tested; commit `2ab520fe` on integration branch, not main. Actual same-snapshot observations determine cardinality, including empty numerical domains. |
| #567 growing report-vector reuse search | Final main-based candidate `9774ac64`; local B1 adaptation `5696a884` retains V3–V6 evidence. PR #571 is green, held for performance acceptance. |
| #564 matching-source iterator | Superseded by #566's production route. Two useful independent assertions were ported. Keep open until #566 lands, then reconcile/close and clean its obsolete lane. |
| #565 repeated exact membership normalization | Open; separate investigation, no qualified production fix or savings claim. |
| #496 and #498 native SI cost/authority | Directed B1 provides a bounded tested alternative; broader scope and integration remain open. |
| #551, #552, #553 and #523 | Open B1 domain, join, trim and failure-reporting items. Reconcile their local remediation and acceptance against the final merged candidate before closure. |

The shared fixes have source-level Standards and Spec acceptance and green
canonical/hosted checks in PR #571. Performance still blocks integration. B1
contains a tested local adaptation; main-first integration remains required.

## Branches and exact local locations

| Role | Branch/revision | Worktree |
| --- | --- | --- |
| Remote patch line | `main`, `b61a031d0bde0e21d283e0f94975f79b9ae15574` | Remote authoritative baseline. |
| Remote feature candidate | `release/next`, `2e8ab886d4755c9c0165d7f59f2a92ead61f3612` | Includes Phase A and main synchronization PR #559. |
| Active capability | `feature/cone-chart-atlas`, model checkpoint `7747b805`, latest tests `3fa45d05`; no remote branch | `/home/vmiguel/git/oblikovati-workspace/Martensite/.slim/worktrees/cone-chart-atlas-b1` |
| Shared-fix integration | `fix/census-contact-indexes`, HEAD `9774ac64`; PR #571 green, performance held | `/home/vmiguel/git/oblikovati-workspace/Martensite/.slim/worktrees/fix567-census-report-index` |
| Original #566 lane | `fix/566-contact-cardinality-index`, HEAD `b61a031d` plus preserved frozen changes | `/home/vmiguel/git/oblikovati-workspace/Martensite/.slim/worktrees/fix566-contact-cardinality-index` |
| Superseded #564 lane | `fix/564-contact-row-iteration`, preserved unfinished source | `/home/vmiguel/git/oblikovati-workspace/Martensite/.slim/worktrees/fix564-contact-row-iteration` |
| Clean benchmark baseline | Detached `b61a031d` | `/home/vmiguel/git/oblikovati-workspace/Martensite/.slim/worktrees/564-main-baseline` |
| Archived earlier approach | `archive/wrapped-polynomial-profiles-2026-11-13` | `/home/vmiguel/git/oblikovati-workspace/Martensite/.slim/worktrees/conical-surface-sketches` |

The shared checkout retains its old directory name for its build cache; its branch
is `fix/census-contact-indexes`.
The root checkout at `/home/vmiguel/git/oblikovati-workspace/Martensite` is still
on local main `4b76a823` and has unrelated user changes, PNGs and supplied ZIPs.
Do not treat that local HEAD as current remote main or discard its contents.
Other unfinished feature worktrees also remain preserved.

## Current optimization implementation

Bug [#569](https://github.com/Oblikovati/Martensite/issues/569) has an isolated
main-based implementation of exact-point native parameter measurement. Its 21
focused Math tests passed, including 13 new controls; standards and spec source
reviews accepted it. Both normal benchmark consumers compiled. All 14 functional
preflight invocations passed, with identical public signatures across seven
workloads. Those comparisons do not expose every private rational; the unit
controls check complete private values against the original formulas.
The same exact-point arithmetic is now present locally in B1 at `93026c2e`,
with its existing internal visibility retained. All 21 Math controls and both
complete cone directions passed; source hashes stayed unchanged. The
[receipt](references/exact-point-metrics-569-2026-11-20/summary.json) preserves
two incorrect test-target attempts, including a zero-test refusal. Runtime/RSS
qualification, the final gate and main-first integration remain pending.

Task [#570](https://github.com/Oblikovati/Martensite/issues/570) is implemented
locally in B1 as explicit parent operation version 2. It rounds only weighted
conical aggregate contributions outward, using the existing fixed-grid arithmetic.
Version 1 and the ordinary scalar term algorithm remain selectable/unchanged.
The unreleased cone-flux failure reports now retain actual work and typed
arithmetic causes; no published toolkit API changed.

The current #570 round passed 140 targeted tests: 133 Math integral controls,
three Sketch accounting/projection controls, and four complete O. operations
(two version-1 controls and both normal senses under version 2). Version 1
retained 989355 operations; version 2 used 994535. Both used 444 integration
cells and 148 spans under the original million-unit grant and original caps.
Source compilation and tests passed on the frozen r3 tree. These functional
runs overlapped other host work; their timers do not qualify performance or
the five-second target. Source and execution receipts are
[retained here](references/conical-accumulation-570-2026-11-19/summary.json).

PR #571 (#566/#567) passed its local canonical gate and required hosted checks.
It remains unmerged because final-source performance acceptance is unresolved.
No version bump or release tag was made.

The [graph-reader counter pass](GRAPH-READER-ATTRIBUTION-2026-11-20.md) found
27,225,464 repeated relation/prefix evaluations on the 137-source owned view.
Main candidate `5578942` uses a scalar first-failure prefix, with no cache-owned
heap storage. B1 adaptation `abde8bd2` passed 54 focused controls and both
complete cone directions; both source reviews accepted it. Native capture and
first-bound eligibility remain required. The [prefix receipt](references/owned-prefix-b1-2026-11-20/terminal.json)
records 994535 operations, 444 cells and 148 spans for both directions.
The [shared-index adaptation](references/shared-indexes-b1-2026-11-20/terminal.json)
passed 88 focused controls plus both complete operations. These counts overlap
older rounds and establish neither unique BDD totals nor performance acceptance.

The latest [native stage attribution](references/native-stage-attribution-2026-11-20/summary.json)
passed the outward operation at `abde8bd2` in 5.573639 s. Across UV and final
checks, 16 native analyzer constructors consumed 2.850048 s. Final qualification
of both bodies took 2.327294 s, including 0.100139 s of native volume estimates.
For the 139-face O body, topology and geometry constructors alone took 0.586323 s
and 1.218910 s. These are nested durations; do not add them together. Independent
oracle checks after the operation boundary are excluded. All six instrumented
files were restored byte for byte and 13 probe artifacts removed. This diagnostic
does not qualify the five-second gate or RSS. The reviewed #574 Handle fix is now
local at `7747b805`: [both complete controls](references/handle-clone-574-b1-2026-11-20/summary.json)
passed at 4.987650/4.967412 s, versus 5.387059/5.394606 s before it. Ten Handle
regressions also passed. These diagnostic runs overlap compilation; matched
time/RSS and main integration remain pending. TSan startup crashed before main.

## Remaining delivery work

1. Attribute current V6 stages before further Census optimization. The two
   earlier disposable passes and the [nested UV-material pass](UV-MATERIAL-ATTRIBUTION-2026-11-20.md)
   are complete. The latter passed the full outward operation at `11884b4c`:
   O137 UV capture took 4.134473 s, including contact transport 1.883531 s,
   graph contact verification 0.483454 s and native validation 0.948080 s.
   These are nested diagnostics under concurrent compilation, without time/RSS
   acceptance. Source restoration and artifact cleanup passed.
2. Finish the latest shared-fix benchmark against the same clean baseline,
   compiler, profiles and accepted output. Preserve previous observations and
   guard refusals. Resolve any memory/runtime result outside the approved policy.
   PR #571 has passed source review, local `cargo xtask pr` and required hosted
   checks; performance acceptance still blocks merge. Reconcile #564 only after
   #566 lands. Include #569 endpoint measurement and #572 occurrence binding
   fixes after their own review and qualification.
3. Forward main into `release/next` and incorporate the tested fixes into B1.
   Preserve V3–V6 unified domain/contact evidence: main currently has a simpler
   paired-report path. Blindly copying native/cardinality or report-index files
   would lose Bernstein/geometric handling. Recheck aliases, source slots,
   cached failures, work, cancellation, native cardinality and replay.
4. Requalify the complete integrated B1 operation with matched time/peak memory.
   Use current stage costs to choose native-qualification and measures work.
   Meet the recorded five-second/debug criterion or obtain an explicit reviewed
   change. Sparse Census coverage is for text-scale growth; it is not the main
   remaining lever for this fixture.
5. The [239-curve cone text case](CONICAL-INPUT-WORK.md) now passes locally
   in both normal senses within one chart. Qualify sparse pair coverage for
   larger inputs, multi-chart/wider profiles, seam and periodic occurrences,
   accounted cone placement and attachment, and full multiline edit replay.
6. Run the affected canonical and hosted gates for the final source, including
   required concurrency/isolation evidence and matched performance. Remove the
   superseded production paths only after their replacement gates pass. This is
   P6 of the six-phase plan and ADR Phase C; those names describe different levels.

No source-breaking safety migration, package bump or release tag is authorized
by this capability goal. Preserve existing APIs and replay until an approved
migration. Reuse the shared executor and existing toolkit builders rather than
adding a second feature engine or faceted modelling fallback.

## Evidence and resumption references

The source areas are `crates/martensite-tksketch/src/wrapped_profile/`,
`crates/martensite-tkbo/src/{profile_extrusion,sketch_region_certification,sketch_region_contact_transport}/`,
`crates/martensite-tkmath/src/predicates/roots/{bezier_pair,metric_bounds}/`,
`crates/martensite-tksketch/src/surface_sketch/attachment/`, and
`crates/martensite-tkv3d/src/brep_font/text_capture/`, relative to the capability
worktree above.

Repository evidence: [V6 receipts](references/census-v6-integration-2026-11-19/manifest.json),
[B1 attribution](references/b1-directed-correctness-attribution-2026-11-19.md),
[original specialist brief](SPECIALIST-BRIEF-2026-11-19.md),
[original implementation review](REVIEW-2026-11-13.md),
[Phase A hosted receipt](references/phase-a-549-hosted-final-receipt-2026-11-18.json)
and [verification record](VERIFICATION.md). Earlier checkpoint prose contains
older timings and phase-attempt numbers; current receipts and source take
precedence. Some API comments still name V5 alone even though actual directed
admission accepts V5/V6; synchronize them before delivery.

Local shared-fix evidence, all under `/tmp/`:

- `martensite-census-contact-indexes-source-freeze.json`: original 17-file candidate.
- `martensite-census-contact-indexes-cold-source-freeze.json`: original cold 18-file candidate.
- `martensite-census-contact-indexes-cold-clippy-r1-source-freeze.json`: final 18-file candidate after two mechanical Clippy fixes.
- `martensite-census-contact-indexes-cold-focused.log`: 68 passing focused tests.
- `martensite-census-contact-indexes-cold-{standards,spec}-review.md`: source acceptance.
- `martensite-census-contact-indexes-public-api/campaign-r1/`: original complete benchmark; its exact source corresponds to committed tree `2ab520fe`.
- `martensite-census-contact-indexes-public-api-cold/normal-build/terminal.json`: current successful production build, exit 0 and unchanged source.
- `martensite-census-contact-indexes-public-api-cold/campaign-cold-r2/`: completed six-sample campaign with identical native/signature bytes; pair 1 is flagged for sibling activity. The earlier partial cold-r1 and all launch refusals remain preserved.
- `martensite-564-vs566-supersession.md`: why #564 is obsolete and which independent assertions were retained.

Capability target: `/tmp/martensite-cone-b1-cached-target`.
Shared-fix target: `/tmp/martensite-567-main-target`.
Clean baseline target: `/tmp/martensite-564-main-baseline-target`.
The obsolete `/tmp/martensite-564-main-target` was cleaned, reclaiming 1.5 GiB.
The original #564 source checkout remains until supersession is integrated.
Dataset-dependent lanes use independent clean `martensite-test-data` checkouts
at `34d65afd72cd3ca8ac1f9d17e5ca408b30e9b4b6`, without object alternates.

Shared-fix PR #571 at `9774ac64` passed its local canonical gate and hosted
checks. Its final-source build and link passed. All three timed pairs were
flagged for sibling activity; later isolation checks refused before consumers
ran. Performance acceptance is held. Allocation and resident-map diagnostics
do not qualify RSS. The #569 consumers are linked; all 14 functional invocations
passed. The first strict campaign stopped after 20 timed executions when a
compiler appeared; short 5–20 ms API blocks also underresolve its CPU-load guard.
All samples remain diagnostic; longer blocks are required for acceptance.
A running-file alone does not establish process liveness.
