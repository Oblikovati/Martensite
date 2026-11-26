# SSI and 2D Curve Intersection Completeness Receipts (#38, #568)

## Summary

`martensite_tkgeomalgo::completeness` introduces explicit completeness and uncertainty
receipts so callers can distinguish certified complete intersections from budget-exhausted
or inconclusive searches:

- `IntersectionCompleteness::{Complete, Inconclusive { reason, budget_exhausted }}`
- `SsiCurveReceipt { curve_index, fitting_error, near_tangent_uncertainty }`
- `SsiCompletenessReceipt` via `IntSS::completeness_receipt(max_curves_budget)`
- `Curve2dIntersectionReceipt` for finite-domain 2D curve intersection snaps (`#568`)

## Migration

Check `receipt.is_complete()` before publishing a snap or relying on curve completeness.
Inspect `receipt.curves()` for per-curve fitting error and near-tangent uncertainty.
