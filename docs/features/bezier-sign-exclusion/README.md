# Source-bound Bernstein exclusion

Issue #539 adds an explicit exact exclusion attempt for planar Bézier pair
and self queries. It addresses captured monotone cubic UV generators that the
existing linear-fibre solver declines. The implementation decision is in
[the kernel decision](../../architecture/decisions/bezier-bernstein-exclusion.md).
Scoped Math and Tkbo checks and independent component review passed.

Call `CapturedBezierPair::certify_bernstein_exclusion` with
`BezierPairExclusionPolicy { version: 1, pair_policy }`. The nested policy is
an unchanged `BezierPairPolicy` version 1 or 2. Existing `certify` calls and
`BezierPairSignStage` retain their previous behavior.

The report exposes the original source and policy, actual root-subdivision
work, a completion class and an optional original failure. `matches_source`
compares all source, domain and policy bits, including signed zeros and rejected
NaN payloads. A complete witness supplies the actual equation axis, strict sign,
parameter rectangle, diagonal-division flag, tensor degrees, exact rational
Bernstein coefficients and both denominator certificates.

Only `Excluded` proves the queried intersection domain empty. `Inconclusive`
allows the caller's existing pair solver to continue under its remaining
budget. Invalid inputs, cancellation and incomplete denominator proofs retain
their typed causes. The attempt proves no curve regularity, join identity,
region nesting or body validity.

The proof tests the full actual rectangle. It uses exact affine substitution
and exact rational basis conversion; no rounded control points or modelling
tolerance decide a sign. Both tensor degrees must fit the existing primary
root-degree cap and any explicit sign-degree cap. A zero or mixed coefficient
sign remains inconclusive, even when a stronger method could establish
positivity.

The Tkbo integration selects graph request version 3 explicitly and retains a
private union of the genuine exclusion and optional legacy fallback reports.
Authored request version 1 and graph request version 2 keep their original
selection. Leaf and fallback subdivision work are both charged once, including
failed work; reused census aliases do not repeat execution. No new replay
transport is introduced.

Controls cover the exact captured #539 cubic bits, the unchanged version-1/2
refusal, independent tensor coefficients, extrapolated and unequal intervals,
negative and subnormal witnesses, crossing/backtracking counterexamples,
denominator failures, policy admission, cancellation, source ownership and
immutable `Send`/`Sync` reports.

## Qualification

On 2026-11-18, the scoped runs passed 33 Math exclusion tests, 7 Tkbo Bernstein
library tests, 8 existing and new owned-work controls, and 4 public proof-version
integration tests. Each run reported zero failures and zero ignored tests.
Strict all-target Clippy for Math and Tkbo also exited successfully.

The local logs are `/tmp/martensite-539-exclusion-tests.log`,
`/tmp/martensite-539-tkbo-tests.log`,
`/tmp/martensite-539-owned-tests-corrected.log`,
`/tmp/martensite-539-public-tests-corrected.log` and
`/tmp/martensite-539-clippy.log`; each accompanying `.exit` file records zero.

Independent component review found no remaining material blocker after the
native-cardinality correction. The full PR gate remains pending. These checks
establish neither a completed cone operation nor a performance result.

A subsequent native-cardinality fix used the unified certificate when checking
native partition observations. Two real native-partition regressions passed
with zero failures and zero ignored tests: an excluded domain accepted zero
actual native contacts, and an excluded domain expecting zero contacts refused
extra observed partition vertices as incomplete. Strict all-target Clippy
passed again after the fix. The receipts are
`/tmp/martensite-539-native-cardinality-tests.log` and
`/tmp/martensite-539-native-cardinality-clippy.log`; both accompanying `.exit`
files record zero. Independent component review, the full PR gate and the
whole-cone check remain pending.
