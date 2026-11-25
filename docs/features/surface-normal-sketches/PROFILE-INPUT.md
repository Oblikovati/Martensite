# Directed solved-profile input

The wrapped-profile operations read the existing `SketchProfile` and
`ProfileLoop` types (#418). A selection supplies its own surface-region ID,
one directed outer loop and optional directed hole loops. The adapter resolves
curve and vertex IDs once and copies the selected control coordinates into owned
operation input. Material admission, joins, clearance and all geometry proofs
still run through the existing family pipeline.

## Authored parameters

Each `LoopUse` carries its source curve ID and traversal direction. The adapter
orients controls and endpoint IDs for loop traversal and retains the original
direction in a separate owned `CurveDirections` value. Moving the geometry into
the constructor leaves that value available to translate result history.

For a prepared span with dyadic level `l` and index `i`, forward traversal retains
index `i`. Reverse traversal maps to original index `2^l - 1 - i` and records a
reversed direction. The original interval then runs from its upper endpoint to
its lower endpoint. Integer indices preserve that correspondence through level
63; the mapping uses no floating-point parameter arithmetic. An unselected
curve, an out-of-domain index or a level of 64 or above refuses.

## Current admission

The polynomial entry points read lines and single Bézier spans of degree one
through three.
A spline must have exactly `degree+1` poles, positive weights if supplied, and
`degree+1` zero knots followed by `degree+1` unit knots. A weighted span needs
an exact identity with a binary64 polynomial in the same parameter. Both the
planar proof and the wrapped fit use that polynomial. The owned source binding
retains the original poles and weights and checks their bits when evidence is
reused. Selected coordinates must
be finite, and endpoint pole bits must equal their authored vertex bits. The
adapter does not snap endpoints or reinterpret an internal knot as a single span.
Extreme degree values are checked before count arithmetic can overflow.

The developed profile uses the world-XY plane; physical support placement belongs
to the surface operation. Other plane placements refuse at this boundary.
Duplicate curve or vertex IDs, missing references, empty selected loops and
repeated selected curve uses also refuse. An unused curve's representation is
not converted. Errors retain offending source IDs or values and the expected
shape.

The rational entry points handle the additional profile representations below.
Longer analytic sweeps and native knot insertion require certified geometry
transport with the original parameter identity.

## Positive rational reference

The rational normal entries accept lines, polynomial or positive-weight splines
with monotone Bernstein spans, and exact circular or axis-aligned elliptic
quarters, plus closed circles and axis-aligned ellipses with exact coefficients.
Shorter analytic sweeps can use exact affine-carrier endpoints even when their
stereographic weights require rounding.
The outer and hole loops may mix those curves. The entries require increasing
signed depths and one angular chart. The cone also requires the profile to
lie in the positive developed halfplane, clear of the apex. The rational cylinder
uses `GeometricCompositeV6` for its reference planar proof. The tested cone
request selects the same version for its UV cap proof. The reference level splits
each native span into equal dyadic intervals. An authored multipart spline
must already have full Bernstein knot multiplicity and exact dyadic native
intervals. It may close at a shared authored vertex.

The operation verifies the original planar region and integrates its signed
boundary for area. For multipart splines, the planar proof uses the exact
Bernstein partition. The retained proof uses the remaining pair-work budget
and bounds position and derivative error for each cubic reference span. Both curves stay strictly
inside the straight boundary halfplanes except at shared authored endpoints.
At a shared authored vertex, the proof separates curved uses with an exact
halfplane test. It separates nonincident curves using outward bounds on their
reference error tubes. The normal map bounds the error in world coordinates.
History records the original curve and its dyadic parameter intervals. For an analytic quarter it
also records how the stereographic parameter maps to the native angle. Closed
conics record a native-angle key at each quarter, including the seam at zero.
For a shorter sweep, the key records the original angular endpoints. The
position, derivative and Green-area bounds include any coefficient rounding.
The result includes the authored area and sweep-volume enclosures, the retained boundary flux, the
spatial bound, and the ordered native validation report. The volume proof uses
the original area and the signed depth Jacobian. For asymmetric cone depths,
it also bounds the change in the inverse-slant integral as the rational boundary
moves to the polynomial reference. The resulting volume width must fit the
requested cone policy.

`normal_extrude_rational_cone_with_selection` takes an explicit cone proof policy.
`DirectedTransportedAuthoredPlanarV8` with `PositiveXFamilyChartV3` checks the
stored solid's self-intersection independently. The legacy rounded entry retains
its native-search coverage label. The full ellipse with semi-axes `1/16` and
`1/32` at `[4, 0]`, signed depths `[-0.1, 0.1]`, and a `1e-4` spatial limit
passes the directed proof with 16 reference cubics. Subdividing the residual
bound, without changing those cubics, leaves space for the retained geometry
within the same tolerance. Its 32-cubic execution under the same one-million
arithmetic limit returns a typed `WorkLimit` for an actual side pair.

## Verification

Fifteen input controls cover mixed curves and holes, reversed authored traversal,
exact dyadic correspondence through level 63, malformed references and spline
representations, diagnostics, source mutation and the existing budgeted family
constructor. The constructor control also maps retained span keys back to the
original authored intervals after moving the owned geometry.

On Linux x86_64, the sketch package passed 186 library and 32 integration tests
in 39.17 and 2.51 seconds excluding compilation. Two tests are ignored by default:
the subprocess worker was explicitly invoked by its replay parent; the separate
stress campaign retains its earlier result. Strict all-target clippy, formatting
and whitespace checks passed. The isolated walking skeleton passed in 3.56
seconds with peak RSS 53,100 KiB, within its unchanged five-second debug limit.
The approximate scan found no new file over 500 lines or function over 20 lines.
Its third brace level in two checks is an error-struct literal inside a loop and
an `if`; control-flow nesting remains two.

These runs used CI's unoptimized stripped-debug profile, with two build jobs and
incremental compilation disabled. An earlier full local gate was aborted during
its build stage as disk headroom fell. It selected 41 packages and 561 test or
executable targets; the default local debug artifacts were large enough to make
continuing that run costly. Cleaning the worktree target directory reclaimed
166.2 GiB, including earlier cached artifacts. That aborted run is not a passed
gate. Full affected-package validation remains required before a push, using the
same scope and checks under CI's profile settings.
