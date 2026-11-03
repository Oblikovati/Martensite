# Proof and operation contract

## Supported represented map

Let `C(u)` be the stored clamped quadratic B-spline on its full knot domain `[a,b]`, with constant positive weights. The weights cancel exactly. Let `D` be the finite nonzero extrusion vector. The support is `F(u,v)=C(u)+vD` for `[a,b] × [c,d]`.

The declared seam identifies only `(a,v)` with `(b,v)` at the same `v`. Admission requires exact equality of the represented endpoint values, not a distance test. Parameter pairs at interior knots are the same parameter and are merged by their exact parameter value. An interior parameter at the same geometric point as the seam is an additional contact and is never excluded.

The first adapter admits finite increasing representable knot spans and finite nonempty extrusion intervals. It rejects inconsistent storage, nonfinite values, zero direction and invalid knot domains. Periodicity, nonconstant weights, higher degrees and trimmed subinterval requests remain explicit unsupported conclusions in this first operation. A failed certificate is not an assertion that the surface intersects itself.

## Quotient projection proof

Choose the first coordinate `k` for which `D[k]` is exactly nonzero. For the other two coordinates define

`Q(X)[i] = D[k] X[i] - D[i] X[k]`.

All coefficients are exact rationals derived from the input binary64 values. No projected pole is rounded to binary64. `Q(D)=0`, and the kernel of `Q` is exactly the line spanned by `D`.

If `Q(C(u))=Q(C(s))` only when `u=s` or `{u,s}={a,b}`, equal extrusion points force one of those parameter relations. When `u=s`, nonzero `D` forces equal `v`. At the declared closing seam, exact endpoint equality again forces equal `v`. Thus `F` is injective on the stated seam quotient.

This is a sufficient certificate. A contact of projected curves need not produce an actual contact inside a finite extrusion interval. Such a contact returns `NotExcluded`, not an invented surface-intersection witness. Other geometry, regularity, trim and topology checks remain separate.

## Exact span coverage

Reuse `predicates::roots::sketch_incidence::spans`: its Cox–de Boor recurrence derives exact rational homogeneous power polynomials from the original poles, weights and knot values. Extend only the necessary internal input seam to accept the exact projected coordinates. Preserve existing sketch-incidence input behavior.

Every nonempty knot span in `[a,b]` must appear once in the owned span manifest. The span key is the original knot interval and source revision within the owned input, not a returned geometric rank or proximity key. The coverage report must show contiguous domain coverage and exact equality at shared knots.

For every span, account for the canonical off-diagonal self domain. For every distinct span pair, account for the complete Cartesian product of the two closed parameter intervals. For `n` spans this is `n` self obligations and `n(n-1)/2` pair obligations. Declarations of these counts are not completion evidence; each obligation retains its actual terminal proof or incomplete cause.

Allowed parameter identifications are only:

- The exact diagonal in a same-span query.
- The exact shared knot parameter of adjacent spans.
- The exact first/last endpoint pair of the declared closing seam.

An outward parameter box containing an endpoint does not establish endpoint equality. Endpoint exclusions need exact equality in the original ideal, using the existing rational parameter/root-sign machinery. Contact order, overlap, repeated spans, backtracking, exceptional fibres and constant pieces must not disappear through adjacency exclusions.

## Implemented exact pair route

The span bridge reuses the existing exact Cox–de Boor and Bernstein conversion. It first obtains the original coordinate polynomials, then forms `Q` with exact rational linear combinations. No converted or projected pole is rounded.

For one quadratic span, `C(t)-C(s)=(t-s)(A(t+s)+B)`. Nonzero `det(A,B)` excludes every off-diagonal equality. In the collinear case a coordinate derivative with one sign over the entire interval proves strict monotonicity. Collapsed and backtracking projected spans prevent certification.

For distinct spans, exact Bernstein hulls contain their complete images. A strict separating linear functional proves disjointness. At an allowed common endpoint `P`, two independent weak separating normals suffice: any point in both hulls must lie on both lines through `P`, whose sole intersection is `P`. Both hulls must contain that actual endpoint control. Individual-span injectivity then proves that the endpoint has only the declared preimage in each span.

These are sufficient predicates, not a general contact solver. A pair not resolved by them remains incomplete, including overlapping hulls whose actual curves might be disjoint. The existing Bézier equation, root, membership and common-carrier behavior is unchanged. A future algebraic route must consume exact spline-span coefficients, preserve source binding and account for all of its numerical work; rounded Bernstein captures are not an acceptable bridge.

## Whole-operation work policy

One invocation owns its requested span, obligation, exact coefficient-height and proof-step caps. A proof step is a started source-span preparation or exact separating-direction attempt, including stages that later fail. The report separately counts entered and finished obligations. The allowance is never reset per pair.

No root isolation or subdivision runs in this version. Its reported root-subdivision count is exactly zero; it does not claim that unobserved algebraic searches completed. Cancellation is checked during preparation, domain verification, every obligation and every separating-direction attempt. Unstarted obligations stay unstarted, and required unproved work prevents publication.

Every retained polynomial and Bernstein coefficient is checked against the requested exact numerator/denominator height bound before pair traversal. Limits are explicit and never widened. Typed refusals retain the actual stage and partial work.

## Owned outcomes

The proposed API separates immutable input capture from execution and returns one owned report:

- `Certified`: complete absence proof on the declared seam quotient.
- `NotExcluded`: a proved projected contact or overlap prevents this certificate; no assertion about finite-interval surface validity follows automatically.
- `Incomplete`: unsupported representation, unresolved algebra, cancellation or exhausted limits, with stage and actual partial work.
- Typed invalid-input errors: offending field/index/value and required shape.

The report retains policy version, requested/effective limits, complete owned source representation, span/obligation manifest, actual work receipts and proof references. No locks or mutable geometry handles are reachable from a published report. Assert `Send + Sync` on its public types. No geometry is constructed by this predicate, so no metric construction error is fabricated.

## Spline and extrusion adapter

The geometry adapter must hold the extrusion state read guard while atomically capturing the stored basis, direction and override flag. Capture the B-spline state under its read guard in the same established surface-to-basis lock order. It returns an owned snapshot before exact computation. Reusing separate unlocked getters does not make a coherent snapshot.

A nonempty override on either object returns explicit incomplete status with the owned admission metadata. Do not clear or ignore an override. Numeric storage and knot-domain admission must be checked even when no certificate is possible.

The result certifies the recorded represented support, not a mutable entity handle. A caller must bind that owned snapshot to the actual operation input through immutable ownership or explicit revision/equality checks. The adapter must not label a later-mutated handle certified.

## Face trim and seam obligations

The support certificate can exclude self-contacts for any proven subset of its domain. A face consumer must separately prove that every used pcurve parameter stays in that domain and that its actual oriented edge uses and closing seam implement only the declared endpoint identification. Face location must be included in the operation binding; an invertible rigid location preserves injectivity, but that fact is not entity identity.

A support-only numeric snapshot does not provide trim-loop or seam-topology proof. Do not consume the new certificate as a valid face until these obligations are accounted for. Do not bypass them based on analytic ancestry, a font label, control-point proximity or successful sampled intersection.

## Deliberate limits

This child operation does not certify general rational or periodic spline surfaces, spherical conversions, approximation error, shape validity, Boolean results or publication readiness. Feature #213 remains open after this scope lands.
