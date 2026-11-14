# Oblique frame implementation gate

The wrapped-profile implementation now constructs noncanonical cylinder frames
through the same intrinsic geometry pipeline. P5 qualification requires the
stored placement, decoded graph and physical publication evidence described
below. Arbitrary chart origins and authored conical profiles remain separate
work.

## Existing interfaces to reuse

`surface_sketch/admission.rs::frame_inputs` checks finite coordinates and the
axis/radial Gram residual against 64 machine epsilons. That admission check is
not a certificate of an exactly orthonormal stored matrix.

`martensite-tkmath/src/gp/transform/forms.rs::Transform3::set_values` derives a
scale and orthogonalizes the matrix. The certificate must inspect the resulting
stored coefficients, rather than certify the supplied coefficients and assume
that the setter preserved them. Existing `Location`, native shape movement and
frozen encoding remain the transport interfaces.

The current publication budget already separates profile, storage, cap trace,
clearance and volume evidence. Placement needs an explicit allocation and
achieved bound within the original requested spatial epsilon. It must not be
added outside that epsilon or hidden by enlarging native tolerances.

## Frame meaning

For the new oblique capability, define the ideal right-handed frame by exact
normalization of the requested axis, projection of the requested radial vector
onto its perpendicular plane, and normalization of that projection. The tangent
is their cross product. Record this meaning in the API documentation when the
capability lands. Degenerate, nonfinite and uncertifiable frames refuse with
typed input or incomplete diagnostics.

The canonical frame keeps its existing path, coefficients and result bytes.
Other frames use the same geometry and publication pipeline, with a certified
placement step; they do not dispatch to the legacy rectangle operation.

## Required placement evidence

Let `R` be the ideal orthogonal frame, `A` the actual stored linear transform,
and `t` its stored translation. For canonical ideal and retained points `x` and
`y`, use the bound

```
||A y + t - (R x + o)||
    <= ||A|| ||y - x|| + ||A - R|| ||x|| + ||t - o||.
```

The certificate must establish each term from retained coefficients and
certified input bounds. An interval enclosure of the exact normalized frame
can use the existing exact-ratio arithmetic and outward square-root bounds.
A certified Gram-residual bound can establish upper and positive lower stretch
bounds for `A`. No sampled matrix or point checks may stand in for these bounds.

The input radius, physical depths and full authored profile bounds supply the
position bound. Include cap realization, storage and decoded-location rounding;
account for large origins and refuse when storage cannot meet the requested
epsilon. Report the requested epsilon, canonical allocations, placement
allocation, actual transform coefficients and achieved errors separately.

A positive lower stretch bound is required before transporting separation and
topology evidence. Propagate the upper stretch bound to tolerance and distance
bounds. Re-run the ordered native checks on the placed result before publication.
The original canonical validation report alone does not validate the placed body.

Use a certified determinant interval to transport retained flux and its error.
The authored ideal normal-volume oracle remains invariant under the ideal rigid
frame. Keep the native volume estimate distinct and recompute or transport it
with its numerical error explicitly. Do not label it a certified body volume.

## Complete-operation verification

Implement one obliquely placed profile with a hole through publication, detached
export and history before extending the glyph controls. Required evidence is:

- independent transformed-point and normal-direction oracles, including inward
  depths;
- exact canonical replay, plus deterministic oblique bytes, history and receipts
  in two processes and concurrent calls;
- per-face trims, shared incidences, cap/side correspondence and ordered native
  validation in the physical frame;
- independent ideal volumes and determinant-scaled retained-flux enclosures;
- near-degenerate, reflected, nonfinite and large-origin refusal controls;
- source mutation, cancellation and detached-export isolation;
- identical accepted P4 workloads measured before and after the change, with
  elapsed time, CPU and process peak memory.

Only after these checks pass should admission accept oblique frames. Arbitrary
chart-origin shifts and conical authored profiles remain separate parts of the
full capability; this gate does not qualify either one.

## Exact frame and stretch construction

For finite requested axis `a` and radial `r`, form the exact dyadic vectors
`k = a cross r` and `u = k cross a`. Provided `a` and `k` are nonzero, the ideal
frame columns are

```
x = u / sqrt((a dot a) (k dot k))
y = k / sqrt(k dot k)
z = a / sqrt(a dot a).
```

These columns express the normalization/projection contract above. Exact cross
product identities give mutual orthogonality, unit lengths and positive
orientation. Each component can be enclosed by exact rational arithmetic and
outward square-root bounds; no trigonometric normalization is needed.
Unrepresentable or zero denominator bounds must refuse.

The owned fixture in `references/oblique-frame-oracle.tsv` uses `a=(1,2,3)` and
`r=(3,-1,2)`. Jacobian's exact rational matrix multiplication returned Gram
matrix `diag(2058,147,14)` for columns `u,k,a`, and its exact determinant operation
returned `2058`. Dividing by the positive column norms gives determinant +1.
This is an independent fixture oracle, not verification of an implementation
or a proof obtained by testing arbitrary frames.

For the actual stored linear matrix `A`, compute `E = transpose(A) A - I`
exactly. Let `e` be an outward bound on its maximum absolute row sum. Symmetry
and `2 abs(v_i v_j) <= v_i^2 + v_j^2` give

```
abs(transpose(v) E v) <= e ||v||^2.
```

If `e < 1`, outward square roots of `1-e` and `1+e` provide a positive lower
stretch bound and an upper stretch bound. Check the exact determinant sign
separately; a reflected stored transform must not silently satisfy a requested
right-handed placement.

Enclose `R` componentwise and bound `||A-R||` by its Frobenius norm using exact
squared endpoint differences. Translation error uses the corresponding exact
squared differences. For the physical cylinder, the maximum absolute radius at
the requested depth endpoints bounds the radial coordinate. The authored pole
hull, enlarged by the declared canonical profile error, bounds height. These
bounds supply the position term in the placement inequality.

Before constructing canonical geometry, subtract the certified placement-only
error from the original requested epsilon and divide the positive remainder
by the upper stretch bound, rounding the canonical allocation downward.
Refuse a nonpositive remainder; do not retry with a looser request. Preserve the
identity placement's current allocation exactly. Recheck the final achieved
world bound against the original epsilon after decoding the frozen result.

For volume, recompute the placed retained-flux enclosure using the certified
positive determinant. The target ideal volume remains the rigid-frame oracle.
The discrepancy includes both the canonical retained/ideal difference and the
`(det(A)-1) * ideal_volume` term; scaling only the old discrepancy would omit
that term.

## Stored-location dependency

Bug #451 tracks the checked, opt-in rigid-location decoder required by this
path. The existing readers deliberately call `Transform3::set_values`, and the
owned probe confirmed repeatable coefficient changes of about one ulp for an
ordinary oblique frame. Raw binary serialization preserved all twelve values.
The new path must preserve admitted rigid coefficients while leaving legacy
normalization as the default. Current canonical wrapped storage still requires
identity locations, so this is a boundary needed for oblique placement rather
than evidence that the canonical publication is invalid.

## Numerical helper checkpoint

`metric_bounds::certify_frame_transport` now implements the coefficient-only
part of this gate. Its owned input binds the requested axis/radial vectors and
the actual stored matrix. The result reports a certified operator-error upper
bound, positive stretch bounds, an outward determinant interval and the Gram
residual. It does not admit a native transform or certify translation,
point-domain extent or a completed placed body.

The reference enclosure evaluates each signed component through
`sqrt(q*q / squared_norm)` in exact-ratio arithmetic. This avoids rounding a
normalization denominator to binary64 before division. Tests cover both maximum
finite and minimum subnormal oblique reference scales.

All 16 focused numerical tests passed, including the independent integer-frame
fixture, exact dyadic inequalities over 125 generated vectors, invalid and
inconclusive inputs, owned binding and concurrent/sequential equality. Strict
all-target TKMath clippy also passed. Independent numerical review found no
correctness issue. Standards review identified missing compile-time assertions
for three public types; those assertions were added and compiled successfully.
The physical placement step consumes this evidence before constructing its
canonical geometry.

## Point-domain budget checkpoint

`CertifiedFrameTransport::spatial_budget` now reserves placement error before
allocating canonical geometry error. Its owned input records the requested
origin, actual stored translation, original spatial limit, and a bound on the
Euclidean norms of both ideal and retained local points. Establishing that
domain bound remains the operation's responsibility.

Translation error comes from exact squared coefficient differences and an
outward square root. The placement-only bound adds that error, the stored-frame
operator error times the point-domain bound, and affine evaluation rounding.
The remaining original epsilon is divided by upper stretch in exact arithmetic;
the positive canonical allowance is rounded downward. Missing finite bounds or
a nonpositive representable allowance produce typed refusals.

The evaluation bound covers three binary64 products, two left-associated sums,
and translation addition, with unit transform scale. Each intermediate carries
an absolute magnitude bound and accumulated error. A full upward ulp of the
outward magnitude endpoint bounds each nontrivial operation's rounding error,
including gradual underflow. Rounded intermediate magnitudes include this
error before the next operation. The sum of the three row-error bounds encloses
Euclidean evaluation error. Multiplication by zero or signed one, and addition
of an identically zero term, remain exact. Identity and signed-permutation
placements without translation preserve the original allocation exactly.

Large origins can consume epsilon through evaluation rounding even when stored
and requested translations agree. The helper refuses those cases without
widening the limit. It does not cover a separate scale multiplication, another
evaluation order, native representation admission or frozen-body correspondence.
The physical operation binds these conditions to its decoded transform and
validates each placed solid before returning a result.

## Frozen physical publication

The operation creates a native frame by normalizing and projecting the supplied
axis/radial vectors, then admits the resulting coefficients through the checked
rigid constructor from #451. The certificate compares these actual coefficients
with exact normalization of the original vectors. The canonical-frame path
returns before allocating a placement plan and keeps its original epsilon.

For a placed request, the point-domain bound is the support radius plus maximum
absolute requested depth, maximum absolute selected authored pole height and
the original epsilon, summed outward. The radial/height sum bounds the ideal
cylindrical point norm. The canonical spatial contract bounds retained points
within the additional epsilon. Unselected source curves do not enlarge this
domain. The resulting positive canonical allowance is used by the existing
family construction and proof pipeline.

After canonical qualification, one elementary location datum is applied to the
compound root and every semantic binding. The physical encoding is decoded
with both coefficient-preserving policies. Publication requires unit scale,
the exact admitted matrix/translation bits, and the same elementary datum on
the root and all bindings. Removing that datum, without inverse arithmetic,
must reproduce the complete canonical graph encoding byte for byte. This binds
the retained incidences, trims, poles, weights, tolerances and authored history
to the placed graph. Mismatches report actual coefficients, datum structure,
binding identity, or the first differing archive byte and both lengths.

For every retained cap trace, publication also checks `M*r + 2*E` against that
cap's original minimum edge/face tolerance. Here `M` is upper stretch, `r` the
canonical full-interval residual bound, and `E` the point-domain affine
evaluation bound. The common actual translation cancels in the exact trace
difference, while its rounding contribution remains in `E`. The two independent
evaluations require two error contributions. This same-frame comparison does
not include ideal-frame or ideal-origin mismatch. Excess refuses with the
region, authored span, depth, achieved bound and unchanged retained limit
(#454). The reported physical cap-trace bound is the maximum of these checked
per-cap bounds.

Every decoded physical solid receives the ordered native validation again.
Sampled geometry and native-search self-intersection checks keep their coverage
labels. Native volume estimates are recomputed from those physical solids.
The retained boundary-flux and representative intervals are multiplied by the
positive determinant enclosure and compared with unchanged rigid-frame ideal
volumes. The original volume refusal policy is enforced on these transported
comparisons.

`result.reports()` retains canonical chart-frame spatial quantities, alongside
the physical native validation and transported measures.
`result.placement()` exposes the original world epsilon, reduced canonical
allowance, actual frame/translation, placement errors and achieved per-member
world-space bounds. `result.encoded_brep()` and detached exports return the
physical graph. The canonical proof archive remains private and immutable.

## Native query representation

The placement certificate describes the decoded located graph `A*y+t` and its
specified direct binary64 location evaluation. `BRepAdaptorSurface` evaluates
the original surface and then applies that location, matching this model.

`BRepTool::surface` constructs a transformed analytic copy when a location is
present. Its cylindrical-surface transformation normalizes axis directions
again. That copy is a different floating representation; this certificate does
not establish its extra reconstruction error. Consumers that replace the
located representation with such a copy need correspondence evidence for that
replacement. The placed graph's proof must not be presented as a certificate
for every reconstructed native query result.
