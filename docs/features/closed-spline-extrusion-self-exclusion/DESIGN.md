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

## Existing solver seam

Reuse the exact equations, projection, algebraic root, membership, common-carrier and work-accounting code under `predicates::roots::bezier_pair`. Its public input currently owns binary64 Bernstein controls. Converting exact spline spans into rounded Bernstein controls would certify a different map and is prohibited.

Introduce a crate-private source-bound exact-span path to the existing homogeneous equation pipeline. It must retain the original spline capture plus exact derivation, not accept caller-supplied resultants or arbitrary equations as proof. Existing public Bézier policies, reports, coefficient normalization and world-bound behavior stay unchanged. If a structural extraction is necessary, land it as a separate behavior-preserving commit with existing owned regression fixtures unchanged.

The new operation publishes an absence certificate only after every obligation finishes and all remaining contacts are exactly one of the allowed parameter identifications. Unresolved common-carrier cases and exceptional fibres remain incomplete. Known projected contacts can stop certification with `NotExcluded`; a complete contact enumeration is claimed only if all obligations actually finish.

## Whole-operation work policy

One invocation owns the work policy. Requested span and pair caps, polynomial degree, coefficient-height limits and subdivision limits are explicit. Admission reports effective limits without silently increasing them.

The subdivision allowance is global across all obligations and all root/sign/refinement stages. Existing pair receipts expose requested and completed split events, including partial failures. Either share their operation-local budget or pass the exact remaining allowance and charge the actual returned completion count before the next obligation. Never reset the original allowance for each pair. Zero remaining work may still permit a genuine zero-split exclusion; it cannot permit an unobserved search.

Poll cancellation during capture, span extraction, pair scheduling and existing numerical calls. Preserve failed/cancelled/exhausted work and the actual completed obligations. Unstarted work remains unstarted. No successful certificate may contain unaccounted required work.

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

The current support-only observations do not provide trim-loop or seam-topology proof. Do not consume the new certificate as a valid face until these obligations are accounted for. Do not bypass them based on analytic ancestry, a font label, control-point proximity or successful sampled intersection.

## Deliberate limits

This child operation does not certify general rational or periodic spline surfaces, spherical conversions, approximation error, shape validity, Boolean results or publication readiness. Feature #213 remains open after this scope lands.
