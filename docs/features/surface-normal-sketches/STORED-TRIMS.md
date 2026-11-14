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
cap's full same-parameter spatial residual against its actual cylindrical
surface still needs a numerical certificate. The side also needs the final
ordered publication checks for its trim topology and ownership. Counts and
native sampled validation do not establish those missing obligations.

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
