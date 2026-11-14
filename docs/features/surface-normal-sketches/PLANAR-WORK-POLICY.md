# Explicit planar proof work for wrapped profiles

The 239-piece text workload exhausts the existing shared 4096-bisection
budget. The first unfinished pair completes in isolation, but the retained
whole-operation ledger reaches 4096 before that pair starts. Bug #449 fixes
repeated validation cost; it must preserve this refusal and the default budget.
The detailed ledger is investigation evidence, not accepted geometry.

## Compatibility boundary

`build_planar_regions(&SketchProfile, Resolution)` has no work argument.
`Resolution` has public exhaustive fields, so adding a field would break
struct-literal callers. Its tolerance fields also have different units and
purposes from computational work. Keep this API and its defaults unchanged.
Add a separate entry point with explicit proof work, sharing the same general
pipeline. Do not create a second region classifier or a retry path.

The new entry point must accept a finite shared pair-bisection limit. The
underlying root policy uses a u16 limit; admission must reject unsupported
requests rather than clamp them silently. Increasing that requested limit
must not change spatial, angular or parameter tolerances, solver behavior,
validation requirements or acceptance checks. Work exhaustion remains
incomplete, with no successful partial geometry.

## Result and refusal evidence

Return requested, effective and consumed work with the result, and retain the
same information on a refusal. Attribute an unfinished pair to caller curve
identities through the existing piece table. Keep the owned typed cause;
never infer budget exhaustion by parsing the existing rendered reason string.
Preserve the legacy error projection for the existing entry point.

The work receipt must come from the actual shared census ledger, including
reused reports under its existing accounting rules. Do not sum per-domain
copies of reused work. New public types must be immutable, Send and Sync;
retain no mutable native handles in work receipts or failures. The wrapped
request must carry the explicit work choice, and publication/replay must retain
it with its achieved receipt. Existing callers keep their existing limits.

## Implementation and verification order

1. Land #449 independently, including the live-native-alias regression and
   corrected performance measurements. Incorporate that fix before measuring
   the capability again.
2. Extract the existing builder pipeline behind an internal options argument.
   Prove unchanged defaults and refusals with owned fixtures before exposing
   the additive entry point. Keep proof preparation and graph inspection under
   one shared work ledger.
3. Add typed work receipts and failure projection. Test zero/small/exhausted
   limits, unsupported limits, alias/reused-report accounting and the completed
   case. Verify unchanged existing public signatures and struct literals.
4. Thread the explicit request through wrapped planar selection. Compare the
   complete publication and work receipt in process and concurrency replay.
5. Measure the unchanged full 239-piece workload with explicitly selected
   finite work. Retain the default-budget refusal as a control. Require every
   geometry/topology/history/budget check to pass before calling it accepted.
6. Present P4/P5 elapsed-time and peak-memory measurements under identical
   compiler/profile settings for the required P5 budget approval. A larger
   work request alone does not establish an approved performance budget.

This is the next implementation design, not a shipped API or permission to
widen an existing limit. Oblique frames, general conical profiles, persistent
body attachment, composed history and P6 deletion remain separate requirements.

## Implementation checkpoint

The private graph request now accepts an optional explicit pair-bisection
limit. Existing callers supply no override and retain 4096. Four focused tests
passed: unchanged defaults, isolation from other child policies, the independent
lens area, and analytical completion with zero subdivisions. The lens needs no
subdivisions, so it is a zero-work success control rather than an exhaustion
fixture.

Successful graph certificates now carry the actual census work receipt through
to `PlanarRegionSet::proof_work()`. This value-only receipt records the requested
and effective shared limit and the census's consumed work counter. It retains no
native handles. The focused public profile suite passed all 44 tests, including requested,
effective and consumed work assertions. Three selected receipt/accounting controls passed, including unique-report
accounting under alias reuse and owned Send/Sync receipts. Strict library
clippy passed;
the additive explicit-request entry point and public refusal receipts are now
implemented locally, with qualification described below. No default limit or geometric tolerance has been increased.

Private preparation failures now retain the last observed census receipt, and
graph refusals carry it alongside their existing typed cause. A missing receipt
means no work observation is available; it is not converted to a zero-work
claim. Cancellation controls distinguish stopping before census from stopping
after the completed zero-work census. Crossing-graph refusal and malformed
admission controls cover graph-level receipt retention. All 33 focused
owned-profile tests, 44 public profile tests and strict library clippy passed.
Existing public decline projection is unchanged. The additive public work-request API now exposes this evidence.

## Additive public entry point

`build_planar_regions_with_work` accepts `PlanarProofWorkLimit::CurrentDefault`
or an explicit finite `PairBisections(u16)`. The existing builder delegates to
the same pipeline and projects failures back to its original return type.
No field was added to the public `Resolution` structure.

`PlanarRegionWorkFailure` retains the requested work choice, the existing typed
decline and an optional observed receipt. No observed receipt is not a
zero-work assertion. Failed operations return no partial regions. The new
request and failure types pass compile-time Send/Sync controls.

All 50 public profile tests passed, including existing defaults and declines,
explicit default equivalence, zero-work analytical completion, unobserved
admission failure and observed crossing failure. The existing tilted-ellipse
fixture consumed its one permitted bisection and refused. With an explicit
8192-bisection cap, the same profile completed with nonzero consumed work and
its independent area of π inside the reported enclosure. Its resolution stayed
unchanged. Full glyph acceptance and P5 performance-budget qualification
remain open. Wrapped-request plumbing and kernel replay of these receipts are
covered by the checkpoint below. The failure
retains the legacy typed decline. Original pair root-isolation causes and
authored parent curves are now exposed as typed values, as described below.
No default or geometric limit was increased.


## Wrapped request checkpoint

The unpublished wrapped API now accepts the planar work choice in
`WrappedNormalOptions::planar_work`. It passes that choice through selection
and family construction to the same planar builder. Successful budget reports
retain the actual work receipt, and `WrappedNormalFailure::planar_work_failure`
exposes a retained planar refusal without parsing its display text. Existing
call sites explicitly select `CurrentDefault`.

The full sketch library suite passed: 240 tests, no failures and two existing
ignored tests. Its public failure-projection control retained the exact planar
decline and the requested and observed work values. An explicit 8192-bisection
request replayed with identical budget evidence and frozen BRep bytes. Its two material
regions still enclose the independently expected volumes 11.10375 and 3.15.
All 39 sketch integration tests passed, with one existing ignored worker test.
Strict all-target sketch clippy and compilation of the V3d toolkit test target
passed. This is kernel request replay; persistent document and body replay
remain open.
The full glyph fixture has not yet passed, and these controls do not establish
its performance budget.

## Typed pair root refusals

`PlanarRegionWorkFailure::pair_root_failure()` now exposes the actual retained
`RootIsolationError` and both authored parent curve IDs when the unavailable
pair failed in root isolation. Proof-piece IDs are mapped through the existing
source table; output order and coordinate proximity supply no identity.
The accessor returns no diagnostic for admission errors, crossing failures or
other kinds of pair noncompletion. Reaching a work cap alone does not manufacture
an exhaustion cause. Other incomplete proof causes retain their existing decline
projection.

The tilted-ellipse exhaustion control retained `SubdivisionLimit` and the same
authored curve ID for both proof pieces. Admission and crossing controls retained
no root failure. The optional diagnostic is boxed only on failure to keep the
public result error small; successful builds allocate no diagnostic.

The public planar suite passed all 50 tests before the final helper extraction.
All 28 wrapped API tests and all six focused planar work controls passed
against that extraction. Strict library clippy passed for both affected crates.
The tests cover work-receipt replay and exact refusal retention; they do not qualify the full text
workload or the remaining general surfaces.
