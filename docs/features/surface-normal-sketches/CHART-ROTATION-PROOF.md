# Rotation-aware frame evidence (#420)

`certify_frame_transport_at_angle` supplies the numerical prerequisite for an
explicit cylindrical chart origin. It compares actual stored coefficients with
the mathematically normalized reference frame rotated about its ideal axis:

```text
X' = cos(angle) X + sin(angle) Y
Y' = -sin(angle) X + cos(angle) Y
Z' = Z
```

The requested angle is the exact supplied binary64 value. A binary64 value named
`FRAC_PI_2` is close to mathematical π/2, but is not treated as equal to it.
The quarter-turn control therefore retains a small positive coefficient-error
bound even when the stored matrix is an exact signed permutation.

## Evidence and allocation

The proof reuses the existing cylindrical-profile trigonometric enclosure. Its
fixed-grid precision is 128 bits, with a finite allowance of 10,000 bounded
arithmetic operations. Argument reduction uses the shared outward Machin bound.
The certificate retains the requested angle, effective settings, consumed work,
and sine/cosine bounds. Inconclusive reduction or exhausted arithmetic returns
the original typed cause; it never produces a zero-error certificate.

Exact rational interval products rotate the previously enclosed ideal columns.
The existing Frobenius comparison bounds spectral error against that rotated
reference. Stretch and determinant remain properties of the actual stored matrix.
The separate rotated spatial-budget type uses the same allocation arithmetic as
the existing budget: coefficient error times the declared point-domain bound,
plus translation and affine-evaluation error, is reserved before the canonical
allowance is rounded downward. Native admission, decoding correspondence and the
declared point domain remain caller obligations.

The existing unrotated certificate and spatial-budget types keep their fields and
Debug representation. They acquire no angle field or trigonometric allocation.
The new zero-angle path returns the existing frame evidence and compares its
allocation, translation, evaluation, placement and world bounds with the original
path. This is compatibility evidence, not a performance-parity measurement.

## Qualification and integration status

The focused frame campaign passed 41 tests: the existing 32 controls and nine
rotation controls. The new cases cover exact quarter-turn matrices, an incorrect
unrotated map, positive spatial allocation, nonfinite and unresolvable angles,
original reference failures, oblique frames, negative/multi-turn angles, and
public-type `Send + Sync`. The oblique corpus includes an angle of one million
radians. Strict TKMath library Clippy passed. Independent Standards and Spec
reviews found no remaining correctness or standards defect.

Logs: `target/qualification/p420-rotated-frame-qualified.log` and
`target/qualification/p420-rotated-frame-clippy.log`.

This checkpoint does not enable nonzero chart origins in the modelling
constructor. That integration must bind the physical frozen graph to this
rotation-aware evidence, retain the original modelling error budget, and preserve
selectable replay behavior. The complete body-feature API, #456 solid history,
whole-profile support trims, authored conical profiles and wider qualification
remain open. No package version or release tag changed.
