# Kernel decision: explicit surface policy, schema 1

Feature #277 adds integration and representation controls to one existing
surface-property pipeline. The default remains adaptive geometry at requested
relative target `1e-5`, with the earlier public methods retained for replay.
Fixed-order geometry is now selectable together with preference for existing
meshes. Face traversal, accumulation order, geometry evaluators and triangle
integration remain shared.

The new API publishes an owned result only after the pipeline returns and its
property values are finite. It rejects missing face representations and the
adaptive engine's known incomplete-observation signal and exhausted required
intervals. It does not add modelling
validation or a new integration engine. The earlier methods retain their error
and mutation-of-output behavior; callers needing them can still select them.

No current path supplies a certified surface-area bound. The result therefore
has only estimate and unknown classifications. Adaptive requests enter the
existing refinement range `0 < target < 0.001`; requested options are retained.
The engine's effective global target is unknown rather than inferred from the
request or from a small estimate.

## Existing numerical limits

The engine caps Gauss orders at `math::gauss_points_max()` (currently 61). Its U
and boundary interval limits are computed as
`min(32*initial_intervals+1, 32*GaussPointsMax+1)`, with integer-overflow saturation.
It stops at sufficiently small parameter intervals (threshold `1e-12`), and
clamps pcurve coordinates to face parameter bounds. Boundary-order selection
also caps its span ratio at 1, and boundary samples with weighted parameter
derivative magnitude below `1e-12` are screened by the existing rule. Exhausted refinement can raise internal absolute
error budgets; those budgets are not published as a final global relative
target. These limits remain unchanged and are not convergence guarantees.

## Accuracy boundary

Closed forms and triangle moment formulas check the selected representations.
The exact-rational ellipse enclosure checks the mathematical reference
independently of either integration mode. Existing adaptive output can disagree
with that enclosure by far more than its estimate. Numerical correction and a
reviewed behavior migration are separate work. This interface does not silently
replace that behavior, widen a tolerance or claim that the requested target was
satisfied.

The new public option, evidence and owned result types are structurally
`Send + Sync`. Results contain only numeric values and options. Tests compare
one/four workers and two processes, and retain results across source mutation;
these checks do not extend thread-safety claims to concurrent mutation of input
geometry.

## Completion bookkeeping and compatibility

Inputs without faces are completed empty queries. Their area and inertia are
zero; the centroid convention is the input's located origin and carries no
physical-center claim. `is_empty()` inspects completed face contributions,
not the mass alone. Missing sources, nonfinite properties, invalid adaptive estimates, an incomplete
adaptive signal or an exhausted required interval are errors in the owned API.

A private flag lives in each adaptive run and records existing interval-limit
stop branches. Private observed Gauss/Sinert calls carry it to the same surface
pipeline. It changes no arithmetic, face order, interval decisions, public
struct fields or earlier method behavior. The earlier methods discard this
metadata and retain their values. This is completion bookkeeping, not a new
integration engine or a numerical repair.

An owned bilinear unit square with U parameter span `1e-13` distinguishes a real
nonempty area from a stopped computation. Fixed evaluation gives area 1; the
earlier adaptive entry point retains zero properties/error 0. The new owned
adaptive query refuses that exhausted result. Its complete earlier property
payload agrees byte for byte before and after the metadata change. Empty
compound, located empty, missing-source and incomplete-signal controls are
separate tests.
