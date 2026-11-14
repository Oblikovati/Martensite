# Directed solved-profile input

The private rational pipeline has an input adapter for existing `SketchProfile`
and `ProfileLoop` values (#418). A selection supplies its own surface-region ID,
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

This adapter reads lines and single polynomial Bezier spans of degree one through
three. A spline must have exactly `degree+1` poles, absent or unit weights, and
`degree+1` zero knots followed by `degree+1` unit knots. Selected coordinates must
be finite, and endpoint pole bits must equal their authored vertex bits. The
adapter does not snap endpoints or reinterpret an internal knot as a single span.
Extreme degree values are checked before count arithmetic can overflow.

The developed profile uses the world-XY plane; physical support placement belongs
to the surface operation. Other plane placements refuse at this boundary.
Duplicate curve or vertex IDs, missing references, empty selected loops and
repeated selected curve uses also refuse. An unused curve's representation is
not converted. Errors retain offending source IDs or values and the expected
shape.

General conics, rational and multi-span splines, higher degrees and the remaining
surface-operation scope are still open. The adapter is compiled only with the
private rational tests. It does not expose an accepted-body API or qualify the
remaining publication, sanitizer, glyph-scale, conical-profile and oblique-frame
work. Existing public primitive operations, package versions and release tags
are unchanged.

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
