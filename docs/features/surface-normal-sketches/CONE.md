# Conical sector normal extrusion

Task #417 adds one complete cone operation to `martensite-tksketch`. It preserves
`CylinderRectangle`, `normal_extrude` and existing result/history types. It adds
no support for a Cartesian rectangle, arbitrary curves, holes or a complete
turn on the developed cone; those remain #418 and #419.

## Admitted sketch region

`ConeSupport` owns an apex, orthonormal axial/radial frame, semi-angle `alpha`,
physical chart-origin angle and `NormalSense`. `ConeSector` owns increasing
slant-distance and developed-angle intervals, plus the same region, four
corner and four boundary IDs used by the cylinder operation. Slant distance is
in model units; developed angle is explicitly in radians.

The additive interface is `normal_extrude_cone(&ConeSector, depths, options)`.
Support fields are `apex`, `axis`, `radial`, `semi_angle`, `chart_origin` and
`normal`; region fields are `support`, `region`, `slant`, `developed_angle`,
`corners` and `boundaries`. Reuse `NormalExtrusionOptions` and
`NormalExtrusionResult`.

The developed region is an annular sector:

```
DevelopedPoint(l, phi) = [l cos(phi), l sin(phi)]
theta = chart_origin + phi / sin(alpha)
S(l, theta) = A + l sin(alpha) E(theta) + l cos(alpha) Z
N(l, theta) = q [cos(alpha) E(theta) - sin(alpha) Z]
X(l, theta, t) = A + r(l,t) E(theta) + z(l,t) Z
r(l,t) = l sin(alpha) + q t cos(alpha)
z(l,t) = l cos(alpha) - q t sin(alpha)
```

Here `q` is +1 outward and -1 inward. Reversing the normal changes signed-depth
meaning without reversing the chart. Corners follow `(phi0,l0), (phi1,l0),
(phi1,l1), (phi0,l1)` and each boundary joins its corner to the next. The first
and last depth roles follow the requested depth interval under either normal.

A Cartesian rectangle on the unrolled sheet generally has varying slant and
angle along its edges. It must not be routed through this sector operation by
reinterpreting its coordinates or approximating it silently.

## Construction and ownership

For fixed angle, the four `(l,t)` corners form an orthogonal rectangle in the
meridional plane. Revolve that section through the admitted physical angular
span using the existing public native builder. Preserve shared incidences and
bind sweep images to authored identities through the existing history path.

The result has one solid, six faces, twelve edges and eight vertices. Four
faces are conical: the two depth faces and the two constant-slant side faces.
The two angular-end faces are planes. Constant-slant faces are not planar caps.
Orient the section for positive volume under both normal senses, without
redefining requested depth identities by radial sorting.

Keep cone admission and carrier checks explicit. Extract only the private
inputs and operations now shared by the cylinder and cone implementations:
source identities, section construction, sweep images, immutable encoding,
ordered body checking, measured-volume acceptance and result assembly. Keep
support-specific coordinate arithmetic and carrier expectations separate.
Structural extraction and new cone behavior use separate commits. Existing
cylinder fixtures must pass unchanged through the structural extraction.

Publication retains immutable bytes and value metadata. Every native export
has fresh ownership, and the decoded body and semantic references are checked
before success. Reuse the same typed failure stages, achieved-limit reports,
measurement evidence and history contracts. No new public serialization or
generic support extension point is needed for this task. Existing public enum
variants and public struct fields remain unchanged; use the existing generic
failure causes and result limits. Do not encode a cone as a fabricated cylinder
request to reuse internal code.

## Admission and numerical limits

Require finite orthonormal support, `0 < alpha < pi/2`, positive resolvable
slant bounds, ordered nonzero region/depth intervals and distinct authored IDs.
Both alpha and its complement require clearance from native angular floors;
near-cylinder and near-plane limits are refused when unresolved. Do not
silently replace them with another support kind.

The physical span `(phi1-phi0)/sin(alpha)` must remain strictly below one turn,
with native angular and physical end-face clearance. Preserve the unwrapped
interval and admit a chart-seam crossing. Reject conversion overflow or loss
of angular resolution, including excessive uncertainty from small sin(alpha).

Radius is affine in `(l,t)`. Its minimum over the four section corners must
remain positively separated from the axis; this covers the whole domain.
Require sufficient clearance from the original apex and refuse unresolved
coordinates, inferred carrier apices, collapsed sections or excessive
construction uncertainty. Error estimates use the physical angular span and
both section coordinates, including amplification by `1/sin(alpha)` and
`1/cos(alpha)` in conversion and carrier-apex checks. Keep parameter, angular,
spatial and volume limits
separate. Report the actual native floors and uncertainty; never relax a
requested tolerance to make the operation pass.

The continuous map is injective on an admitted sub-turn sector with positive
radius: angle identifies the meridional plane, and its orthogonal `(l,t)` map
has a unique inverse. This fact does not establish that the native builder
produced the intended body. The decoded carriers, trims and incidence still
need their own checks.

## Independent oracles

Let `dtheta = (phi1-phi0)/sin(alpha)`, `dl = l1-l0`, `dt = t1-t0`, and use midpoint
subscripts for arithmetic means. Positive radius gives:

```
V = dtheta dl dt [l_mid sin(alpha) + q t_mid cos(alpha)]
DepthFaceArea(t) = dtheta dl [l_mid sin(alpha) + q t cos(alpha)]
SlantSideArea(l) = dtheta dt [l sin(alpha) + q t_mid cos(alpha)]
AngularEndArea = dl dt
```

The meridional Jacobian has absolute determinant one, and the three-dimensional
Jacobian has absolute determinant r. Radius is affine, so integrating it over
the parameter box gives the midpoint formula above. Independently check native
integrated volume and every face area, with their error estimates.

For `sin(alpha)=3/5`, `cos(alpha)=4/5`, `l=[8,12]`, `t=[-1,2]` and
`dtheta=7/10`, the positive expected volume is `1344/25` outward and `1176/25`
inward. Jacobian's exact rational determinant operation independently returned
these magnitudes for the scaled midpoint Jacobians. These are algebraic test
controls, not a certificate for native geometry. Binary64 trigonometric inputs
are compared with their numerical uncertainty; the rational values are not
asserted as exact floating-point identities.

Check endpoint positions and `(X(t1)-X(t0)) dot N = dt`, with zero tangential
component. Depth carriers have apex `A - q t/sin(alpha) Z` and semi-angle alpha.
Constant-slant carriers have apex `A + l/cos(alpha) Z`, opposite axial direction,
and semi-angle `pi/2-alpha`. Allow equivalent native axis parameterizations;
compare geometric carriers, not a constructor's incidental orientation.
Check the correct half-cone against its actual trimmed points; an unoriented
axis match alone is insufficient.

## Fixed acceptance values

For the rational-angle control above, outward first/last depth-face areas are
14.56 and 21.28; lower/upper slant-face areas are 10.92 and 15.96. Inward values
are 19.04 and 12.32 for depth faces, and 9.24 and 14.28 for slant faces. Both
angular-end faces have area 12 in either normal sense.

A separate outward control uses depths `[10,14]`: its volume is 174.72,
minimum radius 12.8 and axial range `[-2,3.6]`. It must remain admissible despite
crossing the original apex plane.

## Verification budget

- Implementation owner: prove one ordinary native cone sector, then admission,
  semantic carrier/trim checks, typed failures and honest error reports.
- Independent test owner: closed-form volume/areas, all corners and normal
  connectors, source identity, seam/period shifts, outward/inward depth roles,
  translated oblique frames, narrow/wide/near-full spans and small scales.
  Include a valid outward offset crossing the original apex plane while
  remaining away from the axis; that plane is not an exclusion boundary.
- Independent test owner: reject apex/axis contact, collapsed/invalid intervals,
  bad frames, degenerate semi-angles, overflow and unresolvable requests;
  verify unchanged inputs after success and failure, and detached-export
  mutation isolation.
- Existing replay control: include cylinder and cone actual geometry/binding
  bytes in two processes and parallel execution. Keep the existing typed TSan
  selector so only the sketch control is selected for sketch-only changes.
- Coordinator: unchanged cylinder fixtures and recorded pre-refactor behavior,
  package clippy, affected `cargo xtask pr`, dependency policy only if a
  dependency changes, and hosted validation before integration.

Keep generic body validation labelled sampled/native-search. Keep integration
and construction uncertainty estimated unless a stronger bound is established.
No tessellation-based modeling decision or silent faceted fallback is allowed.
