# Owned open quadratic-extrusion proof

`certify_open_quadratic_extrusion` checks injectivity of an owned polynomial
quadratic spline extrusion over its complete finite parameter rectangle.
Both endpoints belong to the curve interval. They are not identified with
each other, even when their stored coordinates coincide.

The supported input is a nonperiodic clamped degree-two spline with constant
positive weights and a finite nonzero extrusion direction. Input capture owns
the original poles, weights, knots and multiplicities. It preserves the supplied
direction and finite extrusion interval. No closing segment, adjusted pole,
endpoint snap or tolerance is introduced.

## Exact sufficient proof

Choose a nonzero direction component `D[j]`. For each other coordinate `i`,
project a point with `P[i](C) = D[j] C[i] - D[i] C[j]`. This exact linear map
has kernel equal to the extrusion direction. If the projected curve is
injective, equality of two surface points first implies equal curve parameters,
then equal extrusion parameters because the direction is nonzero.

The operation retains exact original-coordinate polynomial coefficients for
every nonempty knot span. Each span must have a self-injectivity witness.
Every distinct span pair must have an exact separating-hull witness. Only
consecutive spans can admit their actual shared knot endpoint, which represents
one parameter value. The first/last pair receives no cyclic endpoint permission.
Thus completed span and pair coverage proves injectivity on the entire interval.

An unresolved hull pair remains incomplete. A projected collapsed or
backtracking span prevents this sufficient proof; that observation alone is
not a finite-interval three-dimensional intersection result.

## Limits and retained outcome

`OpenExtrusionPolicy::bounded()` explicitly selects open-domain behavior one.
The input and policy are retained with the complete exact witnesses and observed
work. All span preparation and separating-direction attempts share the requested
proof-step allowance. Span, obligation and retained coefficient-bit limits are
checked without increasing them. Cancellation and exhausted allowances prevent
certificate publication.

The closed input, policy and certificate APIs retain their endpoint quotient.
They use the same deterministic numerical traversal with the closed domain
selected explicitly. Existing closed face policy versions one through three
remain unchanged.

## Geometry boundaries

This certificate covers only the owned extrusion support. It does not validate
face trims, curve/surface metric consistency, vertex positions, declared boundary
incidences, tolerances or an enclosing shape. Those checks need their own bound
inputs and evidence. A support certificate cannot turn a missing face check into
completed validation.

Independent controls cover a proper parabola, adjacent representable endpoint coordinates, an exactly
closed triangle that must refuse open-domain publication, an explicit crossing,
collapsed and backtracking spans, invalid and nonconstant weights, cancellation,
all finite work limits, and owned-input isolation. Complete retained reports are
compared across independent processes and threads. Earlier closed proof reports
and the existing closed fixture corpus remain replay gates.

The required sanitizer runner selects the same open-support module with its
pinned instrumented toolchain. Startup refusal and failed controls stop the
required check. Existing cache, capture, group and owned-face checks remain in
the pipeline.

The geometry adapter `StoredSplineInput::into_open_extrusion_input` retains the stored source alongside the numeric input. It refuses delegated evaluators and unsupported layouts, and checks the actual flat knots and evaluator domain before admission. Differing endpoints do not bypass these checks. Admission alone does not run the proof or validate a face.

`capture_placed_open_support_inputs` binds inputs from an owned face snapshot using the existing exact rectangular trim and incidence checks. It selects the declared VFirst cap, computes a relative-frame direction candidate, and requires the exact original world-vector relation before returning inputs. The report retains the full original snapshot, finite work, carrier occurrence, and unmodified VFirst reference. A carrier map written with that reference differs from the numeric extrusion map only by a constant translation; stored poles are not shifted.

This input binding does not run the two support proofs, validate geometric residuals, establish seam/cap/vertex relations, or certify a face. Those obligations remain separate. Versions one and two of the face policy do not implicitly select this explicit version-three placement binding.
