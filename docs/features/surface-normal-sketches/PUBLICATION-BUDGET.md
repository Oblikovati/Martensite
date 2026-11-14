# Family publication error budget

The private rational pipeline now has one refusing error-budget check over the
whole frozen cylindrical family (#418). Its receipt remains separate from an
accepted body. The gate reads the existing source-bound certificates; it does
not reconstruct geometry or replace their provenance checks.

## Requested, effective and achieved values

The caller explicitly selects `Report` or `Refuse(volume_allowance)` for the
retained-versus-ideal volume comparison. Spatial epsilon comes from the original
requests. Every original region must match its retained source snapshot and
construction context. Duplicate, missing or extra region owners refuse.

The receipt records epsilon, the downward-rounded epsilon/4 profile allowance,
the downward-rounded epsilon/1000 storage allowance, the epsilon/2 trace
allowance, the required 4*epsilon clearance and the selected volume policy.
Achieved profile displacement, requested-depth normal-storage discrepancy,
cap-trace residual and retained trace tolerance are reported separately per
member. Distinct quantities retain their own units and meanings.

Every retained span needs storage, regularity and join evidence, plus its
measure correspondence. The gate checks the recorded family binding and history
census. Each native report must contain the four ordered passing stages, no
findings, one solid census and the original refusing tolerance cap. Topology
keeps its exact label, geometry its sampled label and self-intersection its
native-search label. The actual tolerance-stage mode is retained. A missing,
failed, unrun, inconclusive or relabelled required native stage refuses.

## Aggregate volume allowance

Per-member absolute discrepancy bounds are added outward in authored region-ID
order. The receipt has separate totals for retained flux versus ideal normalized
profile volume, the flux representative versus that ideal volume, flux
integration uncertainty, and authored versus normalized-profile volume error.
The last two are reported separately; they are not added again to a comparison
that already contains its input uncertainty.

`Refuse` checks both aggregate retained/ideal and representative/ideal bounds.
It does not reset the allowance for each disconnected member. `Report` retains
the bounds without asserting a volume tolerance. Invalid, negative or nonfinite
allowances refuse; no no-limit default or clamp is introduced.

For two positive bounds, aggregation takes the binary64 sum followed by its
upward successor. This conservatively encloses the exact sum. Zero contributions
are exact. A nonfinite input or an unrepresentable upper envelope refuses rather
than publishing infinity or lowering the bound. This rule can overestimate a
representable exact sum; the receipt reports that conservative bound.

The [Jacobian reference](references/family-volume-sum-jacobian.json) records
independent exact rational sums for a half-ULP case and the actual binary64 values
of 0.1 and 0.2. Those examples support the arithmetic controls; they do not prove
the complete geometry contract.

## Controls and limits

Seven controls cover the mixed contour with a hole and a disconnected square,
aggregate refusal at an allowance sufficient for each member alone, all four
native stages under failed/inconclusive/unrun verdicts, changed source context,
missing span evidence, promoted coverage modes, selection-order independence,
invalid policies, overflow, half-ULP loss and subnormal addition.

On Linux x86_64, the affected sketch package passed 159 library and 32 integration
tests in 25.52 and 1.96 seconds excluding compilation. The ignored subprocess
worker was explicitly executed by its parent replay control. Strict all-target
clippy, formatting and whitespace checks passed. The approximate scan found no
new function over 20 lines, file over 500 lines or nesting above two. The full
affected-package PR gate remains required before a push.

The diagnostic constructor returns `UnqualifiedFamily`; the composed constructor
below returns the private `BudgetedFamily` state. Final publication qualification,
sanitizer qualification, broader concurrency stress, general conical profiles,
additional curve classes, glyph scale, oblique frames and replacement deletion
remain open. No public API, dependency, package version or release tag changed.

## Composed constructor

`family::construct_budgeted` now declares the budget before geometry work and
returns the frozen family together with its successful budget receipt. Its
cancellable form shares one operation control across selection, construction,
proofs, budget validation and final handoff. There is one completion checkpoint,
after the budget passes. Failure or cancellation returns no `BudgetedFamily`.

Budget allocation fields are private to the budget module. Other stages receive
the canonical allocation as a value; they cannot construct or alter individual
shares. The original unqualified walking-skeleton entry remains available for
its diagnostic controls. The composed constructor reuses its construction and
proof stages and encodes the geometry once.

Six additional controls cover the successful combined result, invalid policy
before construction, an unmet volume allowance, cancellation before and after
budget validation, cancellation at final handoff, and reordered selections.
Process and concurrent replay now compare the budget receipt as well as frozen
bytes, history, native reports and measure evidence. A family storage-cancellation
control also exercises the composed entry point.

The affected sketch package passed 165 library and 32 integration tests on Linux
x86_64, taking 35.41 and 2.30 seconds excluding compilation. The ignored worker
was explicitly invoked by its parent process-replay test. Strict all-target
clippy passed. The six focused composed-constructor controls were rerun after
test cleanup. The approximate scan found no new function over 20 lines, file over
500 lines or nesting above two. The full affected-package PR gate is still
required before a push.

`BudgetedFamily` is a private pre-publication state. It is not an accepted-body
API and does not waive sanitizer qualification or any remaining geometric and
application gates. No package version, dependency or release tag changed.

## Cone provenance at the budget boundary

The budget check binds each regularity certificate and each directed adjacent
join certificate to its retained O2 sources. It verifies authored polynomial
bits, source intervals, chart, retained homogeneous coefficients and derivative
error bounds through the certificate's source matcher. Reports are indexed by
authored span keys. Report order and a rotated loop start carry no identity.
Missing, duplicate, relabelled or foreign certificates refuse with the region
owner and the cone failure. The check reads retained evidence without native
recapture or geometric recertification.

Five binding controls cover reordered reports and loop starts, foreign span and
join certificates, duplicate and reversed keys, changed authored coefficients
and missing reports. A sixth control exercises the budget entry point with an
unchanged report census and a foreign certificate.

The affected sketch package passed 171 library and 32 integration tests on Linux
x86_64, taking 39.64 and 2.65 seconds excluding compilation. Its ignored worker
was explicitly invoked by the process-replay parent. Strict all-target clippy
passed. After helper extraction, all ten cone tests passed again. Formatting
and whitespace checks passed. The affected-package PR gate remains required
before a push.

## Cap-trace accounting still needs publication qualification

The forward family replay reports these certified upper bounds at epsilon
`1e-5`:

| Region | Profile displacement | Normal-storage discrepancy | Cap-trace residual |
| --- | ---: | ---: | ---: |
| 5 | `5.189805592157191e-7` | `7.0667041093387305e-15` | `6.227766385274778e-7` |
| 100 | `1.0293297251086026e-8` | `4.66301573741315e-15` | `1.235195619910975e-8` |

Both members retain a trace tolerance of `5e-6`. These are bounds from different
certificates, not measured errors or interchangeable budget shares. The native
cap pcurves retain the authored polynomial angle, whereas the rational space
edges use the fitted half-angle representation. Their discrepancy includes
approximation; it cannot be described as binary64 storage rounding. The ADR's
epsilon/1000 allocation applies to storage rounding. The larger trace residual
requires an explicit argument about the native trimmed-cap realization and its
relationship to the ideal extruded profile before publication. The current
separate epsilon/2 trace check alone does not supply that argument.

Reproduce the receipt with the existing process-replay worker using
`MARTENSITE_RATIONAL_FAMILY_REPLAY=forward cargo test -p martensite-tksketch --lib surface_sketch::wrapped_profile::family::replay_tests::family_replay_worker -- --exact --ignored --nocapture`.
This checkpoint changes no geometry, tolerance allocation or public API.
