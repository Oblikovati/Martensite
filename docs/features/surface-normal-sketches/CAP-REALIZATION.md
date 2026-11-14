# Cap realization: remaining publication argument

The approved representation keeps analytic cylindrical caps and authored-angle
pcurves while fitting the space edges with rational curves. This note separates
the resulting cap-domain question from coefficient-rounding and trace-residual
checks. It proposes a proof route within the accepted representation; it does
not close publication qualification or authorize changed tolerances.

## Three boundaries

Let A be an original developed boundary, B the normalized ideal boundary used
by the normal-volume certificate, and P the decoded cap pcurve developed back
into spatial units. P has coordinates `(R * stored_angle, stored_height)`.
All products in this definition are real operations on retained binary64 values.
They must not be replaced by a rounded binary64 reconstruction when proving
source correspondence.

`storage_trims::cap_bindings` computes angle controls with binary64 `s / R`.
`native::retained_cap_parameters` requires those controls to survive native
conformance. Consequently P can differ from A even before the angular fit is
considered. The existing cap certificate bounds the space-edge versus decoded
surface-trace residual over the full parameter interval. It does not assert
that the filled cap domain is the ideal domain bounded by B.

## Winding argument for filled domains

Consider two closed directed planar boundaries P and B with matching contour
ownership and a continuous closed homotopy H between them. For any point x
outside the image of H, winding about x is constant throughout H: the normalized
map from each closed boundary to the unit circle stays continuous and its
integer degree cannot change. The material indicator therefore agrees outside
the swept boundary image, using the same winding rule for both domains.

Suppose additionally that H is the pointwise straight homotopy and
`|P(u) - B(u)| <= eta`. Every point of the swept image is within eta of both
corresponding boundary endpoints. A point belonging to only one material domain
must belong to that swept image, so it lies within eta of the other domain's
closed boundary. The closed domains have Hausdorff distance at most eta. This
argument also applies to multiple contours under a fixed winding rule. Domain
topology still requires its separate regularity, clearance and join evidence.

For a cylindrical cap of radius r, the map
`F(s,z) = (r*cos(s/R), r*sin(s/R), z)` is Lipschitz with constant
`max(r/R, 1)` in developed spatial units. For a purely horizontal displacement,
its pointwise chord bound is at most `(r/R) * eta`. These statements use the
positive physical cap radius and R. Periodic images need the existing explicit
source-bound material and clearance conditions; choosing a coordinate branch
by proximity is insufficient.

This provides a route from a proved P-to-B boundary homotopy to a filled-cap
geometric bound. It does not identify P and B, turn fit error into storage error,
or turn the retained patch flux into the volume of an exactly closed native
body.

## Conditions still to bind

Before this route can qualify publication, the implementation must establish:

- A closed source correspondence between decoded P contours and ideal B
  contours, including endpoint division rounding and periodic chart choices.
- A certified P-to-B displacement bound and regularity/join margins for their
  homotopy. Existing A-to-B certificates cannot silently stand in for these.
- Original material ownership and winding consistency for that correspondence.
- Outward composition of the cap-domain bound with actual retained support and
  radius rounding, without charging approximation to the epsilon/1000 rounding
  share.
- A publication statement that identifies the ideal normal body, the native
  representation, and every achieved discrepancy separately under the approved
  contract. The O6 boundary-flux comparison remains a separate certificate.

No seam inverse, collar cover, correction surface or faceted replacement is
required by this argument. The next implementation step is to bind decoded
cap-angle controls into the existing polynomial profile/regularity certificate
inputs, then reject missing or inconclusive correspondence at publication.
The current public API working tree must remain unqualified until those checks
and the other integration gates pass.

## Decoded-angle certificate checkpoint

`storage_cap_profile` now reuses `certify_cylindrical_profile` with support radius
one and the cap's actual angular/height controls as its polynomial source. The
native curve retains its physical radius. This compares angles directly and
avoids constructing rounded `R * stored_angle` inputs. Its domain is the local
full piece interval; the original A-to-B certificate retains its separate
original-curve dyadic interval.

The trim check first establishes bit equality of both decoded cap pcurves with
these controls. The added certificate records position and local derivative
bounds against the normalized retained curve. Reports expose angular position
error in radians and height error in model units. The mixed-unit certificate
limit is conservatively chosen as `(epsilon/2)/max(R,1)`, rounded downward;
limits are refused by the existing certificate machinery when unsupported.

Two controls verify exact source matching, rejection of a one-ULP changed angle,
the local parameter interval and typed cancellation. Both passed. Ten API
controls also passed. The isolated walking skeleton passed in 3.01 seconds
under CI's unoptimized stripped-debug profile, within its unchanged five-second
target. This single timing is not a performance-equivalence benchmark.

The new receipt supplies boundary evidence only. Source-bound homotopy,
material composition and a complete cap-domain publication gate remain open.

## Shared reference binding

`CertifiedCylindricalProfile::matches_reference_curve` compares the retained
physical radius, chart origin, pole count, pole bits and weight representation
across certificates whose authored units can differ. It deliberately leaves
source-polynomial and support-radius equality to their own source matchers.
The publication budget now requires the original and cap-angle certificates
to agree on that native reference. Matching region or span counts cannot
replace this check.

The math controls cover different authored degree and unit charts on an
identical reference, and rejection of changed pole, weight, radius and origin
operands. A publication regression swaps in a foreign span's cap certificate.
These controls qualify reference identity only. The next stage must bind
cap-pcurve sources into regularity and join certificates, then compose the
material-domain argument.

## Cap regularity and joins

The cone stage now also certifies each decoded-angle polynomial piece and each
consecutive pair against the cap-profile certificate. These cones use angular
and height coordinates with support radius one. Their local intervals are
`[0,1]`; dyadic authored intervals remain in the separate original-profile
checks.

The budget binding checks complete cap report counts, unique span keys and
source-matched certificates for every prepared piece and join. Reordered reports
and rotated loop starts pass. Foreign regularity or join certificates, changed
retained cap controls and missing reports refuse. Cancellation returns no cap
cone evidence. All fourteen cone controls passed in 0.36 seconds, including the
existing original-profile controls.

These are local homotopy checks. Nonadjacent cap-domain separation, closed
endpoint correspondence and material composition must still be established
before claiming the complete filled-cap result.

After integrating cap-cone binding, the complete sketch library suite passed
208 tests in 32.66 seconds on Linux x86_64, with two default exclusions and no
failures. A new publication control removes a cap join while leaving original
cones complete and requires refusal. Cancellation controls count trim and cap
profile phases separately and assert their original typed causes. This result
does not include the full affected-package gate or sanitizer qualification.

## Closed decoded cap contours

Cap source construction and budget rebinding now check every directed join,
including the last-to-first join. Adjacent pieces must name the same authored
corner and have equal finite angle/height endpoints after the actual division
by R. Endpoint equality is exact real equality of binary64 operands; signed
zero is accepted. Missing coordinates and empty contours refuse with scoped
values and the expected closure condition.

The retained space-edge endpoint checks separately bind native vertices to
curve endpoints. Source-matched cap-profile certificates establish a unique
nearby angular lift. Shared native endpoints and equal cap endpoints therefore
supply the endpoint correspondence required by the local straight homotopies.
This uses the existing source and incidence checks; it does not derive corner
identity from coordinates.

Four closure controls passed alongside the fourteen existing cone controls.
A publication regression breaks the closing cap join while keeping cone counts
complete and requires the typed cap-join refusal. Strict all-target sketch
clippy passed. The complete sketch package suite passed 213 library tests and
39 integration tests, excluding two library workers/campaigns and one
integration subprocess worker from default execution. Replay parents explicitly
execute their subprocess workers. Nonadjacent homotopy separation and material
composition remain open.

## Common angular lift and nonadjacent separation

The profile and cap-angle certificates now require a separate lift receipt.
`certify_cylindrical_lift` reads the retained source operands and restricts each
original polynomial with the existing bounded dyadic machinery. At the common
local start it bounds chart separation plus both angular-position errors. A
bound strictly below six radians rules out a nonzero whole-turn difference,
since six is below `2*pi`. Identical native curve bits alone do not establish
that fact. Continuity of the source-certified normalized curve extends the
endpoint lift agreement across the piece.

The receipt retains both profile sources, requested arithmetic settings and
charged work. The budget checks those sources before composing cap bounds.
Three math controls cover successful alignment, a whole-turn alias with the
same native curve, changed native reference and cancellation. A sketch regression
substitutes a foreign lift receipt and requires refusal. Storage cancellation
controls now also exercise the lift phase after cap-profile completion.

For each source-matched piece, let a be its original developed-profile bound
and let `b = max(R,1) * cap_chart_bound`, evaluated outward. The straight
homotopy from decoded P to normalized B stays within `a+b` of original A:
`H-A = (B-A) + (1-t)*(P-B)`. The budget records the maximum such envelope
across the selection. It requires both within-chart and periodic authored
clearance to exceed twice that envelope. The existing original-clearance
certificate supplies those lower bounds; no new geometric search runs.

Positive products and sums round outward; invalid, underflow-sensitive or
unrepresentable bounds refuse when a finite enclosure cannot be retained.
The separation threshold is strictly above twice the envelope even for an
exact product or a zero envelope. Arithmetic controls include subnormal
multiplication, overflow and strict thresholds.

This closes the nonadjacent-separation step for the bound straight homotopy,
in combination with cap closure, local regularity and joins. The filled-domain
winding argument still needs its physical-cap distance bound and explicit
publication composition. The complete sketch package passed 219 library and
39 integration tests after this checkpoint; default exclusions remain two
library tests and one integration subprocess worker. This is not sanitizer or
full affected-package qualification.

## Filled-cap distance composition

The budget now computes a physical distance bound for each cap. Let r be its
stored radius, d a certified bound on the difference from the exact requested
radius, and q the cap-profile position bound in angular/height coordinates.
The exact requested radius is at most `r+d`. The cylindrical map has Lipschitz
factor at most `max(r+d,1)` for that coordinate L1 metric. The outward bound is
therefore `d + max(r+d,1)*q`.

For material coordinates common to P and B, use the same coordinate on both
caps; radius rounding contributes at most d. For a point belonging to only one
material domain, the winding argument places it in the straight boundary
homotopy image. It lies within q of a corresponding endpoint on the other
boundary. Mapping that endpoint to the other cap contributes at most the
Lipschitz term plus d. The argument works in both directions and establishes a
Hausdorff bound for the closed filled cap sets.

The original selection's winding rule and authored region/contour owners are
retained. Closure, strict local homotopy cones, nonadjacent and periodic
separation prevent contour crossings throughout the homotopy. The native wire
coverage check binds every declared oriented contour to its actual cap wire.
These conditions transport material ownership and hole membership; no new
sampled point-in-polygon verdict is used for acceptance.

Depth radius operands must match the original request and retained storage
certificate. Both cap bounds must fit the original spatial epsilon. The
existing epsilon/4 profile, epsilon/1000 storage, epsilon/2 trace and 4*epsilon
clearance checks remain in force. The result reports `cap_realization_error`
separately from trace residual and coefficient rounding. The native patch flux
continues to have its own measure certificate and interpretation.

Closed-form controls cover radii above and below one, radius rounding, zero
error and overflow. A source/limit control changes requested depth operands and
requires refusal, and separately refuses a zero cap-domain allowance. The
consumer test checks the reported cap-domain bound against requested epsilon.
The complete sketch package passed 221 library and 39 integration tests; strict
all-target clippy passed. Default exclusions are unchanged.

These checks implement the geometric accounting missing at the earlier
checkpoint. Independent review, full affected-package validation, sanitizer
qualification and application-scale acceptance remain required before
integration. The public API working tree remains uncommitted.
