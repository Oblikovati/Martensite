# Rational wrapped-profile representation

Status: accepted by the maintainer on 2026-11-13 for implementation; geometric
qualification remains open. Related: #414, #418, #420 and #425.

This supersedes the polynomial representation and canonical-repair approach in
`wrapped-profile-tolerance-coherence.md`, preserved on archive branch
`archive/wrapped-polynomial-profiles-2026-11-13` at `775f373c`. The review and
measurements are preserved there in `docs/features/surface-normal-sketches/REVIEW-2026-11-13.md`.

## Decision

Use one rational angular representation for both depth boundaries of each
wrapped piece. For a chart center c, approximate tan((theta-c)/2) by a
polynomial t. Construct homogeneous coordinates proportional to
`(r*(1-t*t), 2*r*t, z*(1+t*t), 1+t*t)`, then apply the support frame.
Both depths share t, weights, subdivisions and endpoint computations. The
ideal rational construction has radial rulings and lies on its analytic
cylindrical caps. Binary64 storage requires separate certified bounds; exact
incidence must never be inferred from rounded poles or shared topology.

The intended result contract is an exact normal extrusion of a profile within
the requested epsilon of the authored profile, with the same topology, subject
to certified storage bounds. Publication must distinguish the ideal realization
from the stored native representation and its achieved errors. Existing public
primitive contracts remain unchanged. No version change is authorized.

Stop extending the seam, collar, inverse-cover and correction-homotopy families.
The archive preserves their research and replay evidence. Reuse established
planar capture, native builders, encoding, history and validation capabilities.
Do not carry the archived proof pipeline into the replacement by default.

## Evidence required

One request owns the error budget: reserve at most epsilon/4 for developed
angular displacement and epsilon/1000 for storage rounding; require certified
nonadjacent clearance of at least 4*epsilon. Record requested, effective and
achieved limits separately. Failure to establish a required bound refuses.

The implementation must establish all six obligations from the approved review:

1. Planar admission, regularity, joins and material selection.
2. Angular position and derivative bounds from stored homogeneous coefficients.
3. Positive weights, radial residual and ruling residual after storage/readback.
4. A topology-preserving planar homotopy, including shared endpoints and joins.
5. Ordered native checks with sampled/native-search evidence labelled honestly.
6. Represented measures with integration error and discrepancy from the ideal.

Endpoint interpolation and tangency are properties to prove against actual
stored operands, not consequences of using Hermite interpolation alone. Near-
equal-weight rationality detection, codec retention, periodic chart choices,
and direct/indirect frames require explicit controls. Analytic developed area
alone cannot validate the stored body's volume.

## Delivery gates

First build one loop with a hole through a private orchestrator: admit,
construct, freeze, validate, measure, retain history and export a detached copy.
The walking skeleton must run within five seconds in debug and must not publish
an accepted body before its required certificates exist. Exercise one boss and
one pocket to expose rational-surface interoperability constraints early.

Then implement the one-dimensional certificates and planar topology proof,
followed by publication, glyph scale and oblique frames. Preserve authored
identity independently of proof exhaustion and capture-local cell ordinals.
Freeze once and prove against the bytes that publication retains. Verify
isolation, deterministic replay, two processes and concurrency before release.

Delete superseded production paths when the replacement gate passes. Audit
retained public proof APIs and historical behavior versions before removal;
their compatibility impact requires a recorded migration and explicit version
approval. The archive is evidence, not an API compatibility exemption.
