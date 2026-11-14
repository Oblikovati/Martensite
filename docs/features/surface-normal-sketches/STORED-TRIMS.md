# Retained trim checks for rational wrapped profiles (#418)

The private pipeline checks decoded trim representations before reusing its
stored boundary and normal-sweep proofs. This closes a gap between the native
construction guard and later publication: construction checks alone cannot
establish what survived the frozen codec.

For every prepared span, the checks cover its two depth edges on the analytic
cylinder caps and all four boundary incidences on the ruled side. Each pcurve
must be stored, retain the complete `[0,1]` range, have the expected Bezier class,
and retain the expected coefficient bits with unit weights. Cap controls are
the prepared polynomial controls mapped to `(s/R, z)`. Side controls are the four
unit-square boundaries in their actual curve parameter directions. The existing
side-net check separately binds the ruled surface rows to the stored space curves;
shared-vertex and linear-connector checks remain in force.

The operation also inspects edge and face tolerances on every checked incidence.
Raw storage values must be finite, nonnegative and at most the allocated half
of the requested tolerance. The effective values returned by `BRepTool` must
satisfy the same limit, including the tool's minimum tolerance floor. Raw values
are checked first because that floor can mask NaN or negative storage values.
An excessive floor refuses the operation; it does not enlarge the allocation.
The evidence records the largest effective tolerance observed and the number of
checked cap and side incidences. Errors retain the semantic edge and face keys,
the failure class and the original cause.

Seven focused controls cover the complete expected incidence count, a changed
cap control point, a shortened pcurve range, a widened edge tolerance, a rational
weight change that reparameterizes a cap trim, an altered side pcurve with
unchanged space curves/surface, and raw NaN/negative face tolerances hidden by
the legacy getter's floor. Each mutation is made on a detached decoded fixture.
The valid fixture still passes.

These checks establish retained coefficients, domains and tolerance caps. The
full cap residual certificate described below now covers the same-parameter
space/surface discrepancy. The side still needs the final ordered publication
checks for its trim topology and ownership. Counts and native sampled validation
do not establish those missing obligations.

The result remains `Unqualified`. Certified represented measures, disconnected
material classification, complete history/publication binding, isolation,
replay/concurrency, glyph scale and oblique-frame gates remain open. No version
or release tag is authorized by this checkpoint.

Verification passed all 50 sketch library tests and 32 sketch integration tests,
including the seven retained-trim controls. Strict all-target clippy passed.
The original isolated debug fixture took 3.29 seconds with peak RSS 50,896 KiB,
within the unchanged five-second limit. These individual timings do not establish
a performance improvement. The approximate style scan found no inspected file
over 500 lines, function over 20 code lines or nesting above two. No dependency,
version or release tag changed.


## Full cap residual certificate

`certify_cylindrical_trim` takes an actual rational space Bezier curve, the
polynomial Bezier controls of its actual `(U,V)` pcurve and the actual cylinder
radius. Its declared frame is the local +Z cylinder at the origin, and both
representations use their complete `[0,1]` domains. Native admission separately
checks this frame, placement, curve class and domain. The certificate does not
infer topology or a nearby angular lift.

For retained homogeneous coordinates `(X,Y,Z,W)`, the shared outward engine
bounds the numerators `X-rW cos(U)`, `Y-rW sin(U)` and `Z-WV`. Dividing their
absolute Bernstein hull bounds by a certified positive minimum weight gives
an XY L1 bound and a height bound. Their interval sum bounds the Euclidean
same-parameter discrepancy over the complete domain. Fixed proof subdivisions
refine the bound without changing geometry. Only the final report is rounded
outward to binary64. The original finite precision/work policy, achieved
component/total bounds and requested spatial limit remain separate receipts.

The native cap stage first checks that the retained pcurve's coefficient and
unit-weight bits equal the controls supplied to the residual query. The space
curve coefficients were copied from the decoded native edge. The requested
residual cap is the minimum of the actual effective edge and face tolerances;
both have already passed raw/effective tolerance admission. The result must fit
both tolerances. A larger allocated modelling tolerance never substitutes for
a smaller retained topology tolerance. Each span retains both depth certificates
with their source bits, limits and work receipts. This stage follows the existing
storage, depth and original-profile checks, preserving their earlier refusals.

Independent controls include a zero-error generator and the exact dyadic bound
`0.125 + 0.25 = 0.375` for radial/height discrepancy at radius `8`; one binary64
step below that cap refuses. A nonuniform rational weight control checks the
actual weighted height (the midpoint discrepancy is `5/12`). Source binding
refuses changed UV, space, radius and weight bits. Invalid geometry, precision,
work exhaustion and cancellation return no certificate. A valid distance bound
can also cover a space curve on the axis; angular invertibility is not a premise.

In the native negative control, a loose-tolerance curved fixture passes first.
Only one retained edge tolerance is then reduced to `1e-7`, leaving geometry and
pcurves unchanged. The operation refuses with a numeric `Uncertified` cap
residual at that semantic edge and face. The ordinary mixed profile, seam-crossing
and two-turn helical controls remain passing. Represented measure certification
and the remaining ordered publication gates are still open.

Verification passed 505 math library tests, 3,114 math integration tests,
56 math doctests, 52 sketch library tests and 32 sketch integration tests.
All seven math trim controls and the native reduced-tolerance refusal passed
again after the final diagnostic assertions. Strict all-target clippy passed.
The original isolated debug fixture took 3.59 seconds with peak RSS 51,344 KiB,
within the unchanged five-second limit. This timing includes both depth cap
certificates for every piece; it does not establish unchanged runtime relative
to earlier checkpoints. The approximate style scan found no inspected file over
500 lines, function over 20 code lines or nesting above two. No dependency,
version or release tag changed.
