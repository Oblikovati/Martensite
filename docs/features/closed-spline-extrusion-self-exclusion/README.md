# Closed quadratic spline extrusion self-exclusion

Feature #213. This operation certifies an owned extrusion support when its transverse projected closed spline has no contacts except the declared closing seam. It does not decide shape validity, replace intersection engines, or treat completed sampled searches as absence proofs.

The first supported input is a finite, nonperiodic, clamped degree-2 B-spline with constant positive weights, exact endpoint closure, its full finite parameter interval, a nonzero finite extrusion direction, and a finite nonempty extrusion interval. The certificate concerns the continuous represented map. It does not claim injectivity of rounded evaluator outputs.

The input snapshot owns every pole, weight, knot, multiplicity, domain endpoint and direction component. A spline or extrusion evaluation override is unsupported until its actual represented map can be bound independently. Reading a certificate takes no geometry lock. Later source mutation is not covered.

`DESIGN.md` states the exact proof, exclusions, solver reuse and work accounting. `VERIFICATION.md` assigns each claim to the smallest relevant check. The version-one implementation is a sufficient exact hull/monotonicity certificate. Unseparated pairs remain explicitly incomplete; it performs no numerical root search.

The geometry adapter holds the extrusion read guard while capturing the actual spline basis. It records the evaluator layout and checks that stored flat knots agree exactly with the captured distinct knots and multiplicities. Delegated evaluators, periodic or higher-degree bases, and open endpoint seams return explicit inconclusive outcomes. The exact calculation and cancellation callbacks run after both geometry guards are released. The snapshot covers its owned support and requested parameter interval; face trims, entity locations and seam topology need separate validation.
