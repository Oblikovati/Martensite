# Owned surface query certificates

The version-one query policy proves a sufficient embedding property of a
supplied rational tensor map. A successful report covers the declared query
bounds, with only the declared U-period aliases. It does not validate a face's
trim loops, vertex incidences, tolerances, orientation or floating-point
evaluation error. This is a bounded validation operation under #213.

`martensite_tkgeomalgo::surface_query::certify_face_query` captures the actual
face input under simultaneous nonblocking read leases. The report owns the
root occurrence, all captured representation references, the selected support
owner, the requested adaptor bounds and solver settings, and the exact proof
report. Invocation-local owner labels preserve aliases within that capture;
they are not persistent identifiers. No numerical proof runs while a source
lease is held. Later source mutation cannot change the report.

The new `StoredSurfaceQueriesV2` input-capture selector includes stored tensor
surfaces, finite spatial splines and regularity records with both referenced
supports. A null spatial curve remains an explicit `AbsentSpatial` cell on its
representation owner, alongside its actual range and location. It is not an
edge-validity or empty-result certificate. Existing capture entry points retain
`StoredRepresentationsV1`, including its refusal of null spatial curves.
Neither capture selector validates geometry. An unknown representation or
external evaluator refuses capture instead of disappearing from the input.

## Admitted stored layouts

The first query policy supports one complete clamped tensor span of degree one
or two in each direction, or a three-span quadratic U-periodic cycle with one
clamped V span. It checks the actual pole and weight dimensions, all array
origins, distinct knots, multiplicities, expanded knot vectors and canonical
bounds. Every span is nonempty and ordered. Other layouts remain incomplete.

The proof uses the evaluator's effective rational weights. Nonrational storage
uses unit weights. A constant positive rational weight also represents that
polynomial map. Varying rational weights must select the evaluator's rational
branch unambiguously on every patch; near-constant ambiguous patches refuse.
The policy currently refuses transformed root or support locations, avoiding
an unproved replacement of rounded transformed coefficients by ideal affine
ones.

Nonperiodic query bounds must lie inside the stored canonical rectangle.
The periodic layout admits a zero-origin canonical U interval and its one
adjacent positive cycle. Its period subtraction is exact by Sterbenz's lemma;
only those declared aliases are part of the theorem. Nonperiodic V tails are
never clipped or identified with a pole.

## Exact sufficient theorems

The math operation owns Cartesian Bézier poles and positive effective weights.
It converts their binary64 values to exact rational numbers before multiplying
homogeneous coordinates or differentiating. All signs come from complete
Bernstein coefficient hulls on the closed unit square. No samples establish a
sign, and an unsuccessful hull test remains incomplete.

For a nonperiodic patch, the operation tries a fixed order of signed coordinate
projections. With the common positive denominator, it proves both diagonal
Jacobian numerators positive and `4ad - (b+c)^2 > 0`. The symmetric projected
Jacobian is positive definite, so the projected map is strictly monotone on
the convex square and the surface map is injective there.

For a periodic cycle, the operation proves that Z is independent of U and
strictly increases in V. One exact vanishing endpoint factor is permitted in
the V derivative, provided its remaining factor is strictly positive. Each
span has positive XY angular derivative and lies in an axis-excluding open
half-plane. Complete rational endpoint curves glue exactly for every V.
At the first V endpoint, the three endpoint vertices form an origin-containing
positively oriented triangle. The half-plane homotopies and continuity in V
preserve winding one. Positive angular order therefore separates U modulo the
single declared period, and axial order separates V. The proof does not
require or assert a collapsed endpoint loop.

## Limits and verification

The reports retain unchanged finite step and coefficient-bit limits, consumed
work, successful sign obligations, and typed missing relations. Cancellation,
unsupported storage and exhausted work never expose a successful partial
certificate. Earlier closed-extrusion policies and ordinary self-search APIs
are unchanged.

Focused controls use independently constructed affine tensors and an annular
cycle, with reflected, subnormal, collapsed, broken-seam, reversed-winding,
wrong-domain, busy-writer, poisoned-cell, mutation and finite-budget variants.
The required package and dependent-package gates remain separate from those
focused controls. A query certificate alone does not complete an ordered face
validation or authorize a whole-face result.
