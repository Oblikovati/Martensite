# Carrier directions in relative frames

`ClosedFacePolicy::placed_piecewise_boundary()` selects version 3. It retains
the piecewise represented-face contract and finite budgets of version 2.
Versions 1 and 2 remain selectable with their existing direction handling.

A support direction and a spatial boundary curve may use different local
frames. The operation forms a candidate direction in the boundary frame with
the existing numerical matrix inverse. This candidate does not establish any
geometric relation. The exact world-vector comparison must still establish
equality from the original placement coefficients before publication.

An unavailable inverse, a nonfinite candidate, or a zero candidate returns
`Incomplete` with the source owner and relation. A finite candidate that fails
the exact world-vector comparison also remains incomplete. No stored poles,
parameter intervals, placements, or tolerances are changed. Both independent
support proofs continue to consume the same remaining work allowances.

The extension does not admit open spline supports. An auxiliary closure would
need its own complete span and pair proof, together with independent exact
relations for the actual seams, cap curves, and vertices. Proximity to a closed
curve is insufficient.
