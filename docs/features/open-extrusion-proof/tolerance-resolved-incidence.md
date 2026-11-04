# Tolerance-resolved incidence decision

Feature #213 authorizes an explicit tolerance-resolved incidence behavior version.
Existing exact face policies and their refusals remain selectable and unchanged.
The new mode must report its semantics and retain the actual represented contacts;
it does not certify an exact embedding of the unmodified representation.

The first operation proves proper components of an owned analytic curve inside a
ball around a *given* vertex. Adjacency and endpoint roles come from the input
incidence, never from distance. Its radius is the minimum of the applicable raw
face, edge and vertex limits. The caller retains those original limits separately.
No geometry, trim, pole or limit is changed.

Exact squared-distance polynomials are formed from original spline coefficients
and stored affine expressions. Complete root isolation over the full trim proves
the number and transitions of sphere-boundary crossings. A proper endpoint
component has a strict interior endpoint, a simple exit or entry and a root-free
outside remainder. A closed seam vertex requires two such components. Whole-curve
containment, extra components, tangencies, missing work and cancellation do not
produce a certificate. Root brackets remain algebraic evidence, including roots
closer to a trim endpoint than one floating-point parameter spacing.

The face wrapper must additionally retain complete actual contact witnesses and
prove each contact belongs to the declared proper component. It must check
disjoint neighborhoods of distinct located vertices, ideal and carrier
injectivity, unchanged metric limits, declared parameter maps, placement and
height relations, cap and vertex binding, and nonincident exclusion. An owned
neighborhood certificate alone cannot publish a face or body. Ordered topology,
geometry, self-intersection, tolerance, history and operation completion checks
remain mandatory.

Open support reports remain support-only. The same neighborhood and contact
operation must discharge open-domain endpoint and seam obligations; a support
injectivity report cannot substitute for them. No closing chord, endpoint snap,
healing or implicit gap bridge is admitted.

Verification uses independent lines and quadratic curves with known sphere roots,
whole-curve and extra-component counterexamples, shared-knot deduplication,
tangencies, invalid inputs, exhausted budgets and cancellation. Published inputs
and witnesses are owned and immutable. Process and thread comparisons must agree,
and the canonical sanitizer group must cover the new operation before merge.
