# Closed quadratic spline extrusion self-exclusion

Design scope for Feature #213. This operation will certify an owned extrusion support when its transverse projected closed spline has no contacts except the declared closing seam. It will not decide shape validity, replace intersection engines, or treat completed sampled searches as absence proofs.

The first supported input is a finite, nonperiodic, clamped degree-2 B-spline with constant positive weights, exact endpoint closure, its full finite parameter interval, a nonzero finite extrusion direction, and a finite nonempty extrusion interval. The certificate concerns the continuous represented map. It does not claim injectivity of rounded evaluator outputs.

The input snapshot owns every pole, weight, knot, multiplicity, domain endpoint and direction component. A spline or extrusion evaluation override is unsupported until its actual represented map can be bound independently. Reading a certificate takes no geometry lock. Later source mutation is not covered.

`DESIGN.md` states the exact proof, exclusions, solver reuse and work accounting. `VERIFICATION.md` assigns each claim to the smallest relevant check. No production certificate is implemented by this design commit.
