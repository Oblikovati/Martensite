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
