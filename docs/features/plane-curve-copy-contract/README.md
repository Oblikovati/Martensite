# Plane curve copy contract

Bug #243 exposed an implicit fixture precondition. The plane conversion test
expected its first edge to store a p-curve, but built only a polygon face. Face
construction now preserves the input wire without inserting that representation.

The conversion callback examines stored surface representations. It copies the
3D curve when a stored p-curve lies on a surface selected for conversion. Without
such a representation it returns false and leaves the output parameters unchanged.
A face-aware p-curve callback can still project the line onto the plane without
storing that projection on the input edge.

The fixture now explicitly attaches a p-curve to its own edge before exercising
the true-copy branch. The original surface degree, bounds, reversal and true-copy
assertions remain. Additional tests cover unchanged output parameters when no
p-curve is stored, generated p-curve geometry and independent mutation of the
copied and source 3D curves. Expected points come from the unit square's bottom
edge, `(t, 0, 0)`, at four fixed parameters with a 1e-14 distance limit.

This correction changes tests only. Production code and public APIs are unchanged.
The release candidate must run its full validation again after integration.
