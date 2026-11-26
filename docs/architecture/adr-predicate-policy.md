# ADR: Exact Predicate Policy and Metric Separation (#33)

## Decision

1. **Exact/filtered sign predicates (`martensite_tkmath::predicates`)** govern all topological
   sign decisions: 2D orientation (`orient2d`), 3D orientation (`compare_tetrahedron_orientation_3d`),
   squared-distance ordering (`compare_squared_distances_2d`), and stationary distance jet signs
   (`certify_distance_stationary_jet`).
2. **Metric coincidence vs. construction error:** Floating-point tolerances compare metric
   quantities only, classified by operand origin (same-computation weld, independent-source sew,
   and angular arc-length). Topological sign predicates never use bare epsilons or perturb
   coordinates.
3. **Removed ad-hoc paths:**
   - Constant/displaced-point minimum flags in `ExtPElC2d`/`ExtPElC` (replaced by
     `certify_distance_stationary_jet` in `#406`).
   - Uncertified discrete distance sampling on B-spline/Bézier spans (replaced by Bernstein
     sign-variation isolation in `#462`).
4. **Verification:** Every predicate is gated by the stress corpus (`predicate_stress_corpus`)
   covering underflow, subnormals, exponent extremes, near-degenerate inputs, permutations,
   and FMA-sensitive arithmetic against exact arbitrary-precision `BigInt` evaluation.
