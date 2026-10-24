# Planar classification without topology cache mutation

Bug #242 corrects the planar-face optimization from #207. Constructing a face must
preserve the input wire's curve representations. Curve lookup may calculate a
parameter curve without storing it on shared edges. The face-construction cache
write is removed; projection formulas and public APIs are unchanged.

A fresh face classifier already owns projected curves and, when bounding boxes are
enabled, their bounds. It reuses those bounds for its face envelope only when the
surface is an untrimmed plane and every probe occurrence has a ready, nonvoid box
whose coordinates are finite and within the plane's natural bounds. Occurrences
retain traversal order, including duplicate edges. Each box's gap is expanded before
union, matching the general face-bounds calculation. Missing, unavailable, void,
nonfinite, trimmed, periodic and nonplanar cases use the existing calculation.

Reuse is confined to construction of one classifier operation. It neither changes
shared topology nor survives into a subsequent operation. The public explorer's
explicit bounds calculation retains its general behavior.

The added tests cover transformed planes, reversed wires, repeated projection
lookup, exact parameter ranges, reconstruction of line and circle geometry,
input-curve representation stability, duplicate occurrences, per-box gaps, and
fallback conditions. The prior stored-curve assertion encoded the regression;
it now asserts that lookup leaves representations unstored, backed by the
independent geometry and input-isolation checks.

`release-performance.json` records ten consecutive normal-scheduler release runs
of the unchanged four classification workloads. Every run passed the original
4%, 3%, 15% and 15% speed-up requirements. No acceptance threshold, input, iteration
count, timing source or production numerical tolerance was changed.
