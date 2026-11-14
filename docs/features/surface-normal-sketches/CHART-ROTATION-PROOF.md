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

## Shared physical-publication pipeline

The private placement plan now accepts either certificate type through static
interfaces. Freeze/decoded-graph correspondence, ordered native validation,
retained cap-trace limits, determinant-scaled flux and world bounds still use one
implementation. The existing public result and placement report keep their
fields and allocation pattern. Construction from resolved inputs was extracted
without changing callback order.

The 28 existing placement controls passed, with their subprocess worker ignored
during the normal run and invoked by its parent test. A separate rotated control
then passed on a profile with a hole and a disconnected member. It uses an exact
quarter-turn stored matrix, the certified binary64 angle reference, the original
modelling epsilon, and an independently expected transformed vertex. This is a
private integration control; the public constructor still refuses nonzero chart
origins. Both independent review axes and strict all-target TKSketch Clippy passed.

A pre/post comparison of the canonical consumer's full replay payload matched
all 57,214 bytes: native graph, authored history, reports, budget and material
receipts. Its SHA-256 is recorded in
[the replay measurement](references/p420-placement-interface-canonical-replay.json).

The unchanged oblique consumer was measured in four alternating pairs after one
warmup per executable, while the other task's heavy jobs were idle. Median wall
time was 2.664s before and 2.614s after; ranges overlapped. Median GNU peak RSS was
45,690KiB before and 45,334KiB after, also with overlapping ranges. This control
shows no resolved regression and does not establish an improvement or whole-feature
performance parity. [Raw trials](references/p420-placement-interface-paired.tsv)
and [binary hashes and settings](references/p420-placement-interface-paired.json)
are retained. The temporary baseline executable was removed after comparison.

Logs: `target/qualification/p420-generic-placement-controls.log`,
`target/qualification/p420-shared-rotated-publication-final.log`, and
`target/qualification/p420-placement-interface-clippy-final.log`. The complete
feature and its full affected-package gate remain unfinished.
