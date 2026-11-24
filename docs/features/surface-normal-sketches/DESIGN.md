# Geometry contract

Status: contract reviewed for #415; implementation and review results are tracked
in [VERIFICATION.md](VERIFICATION.md).

## Surface sketches

A surface sketch owns its support definition and solved entities. Its geometry
uses developed coordinates in model units. Authored vertex and curve IDs survive
changes in coordinates and seam placement. An unrestricted spatial wire does
not determine a support or normal field and is outside this interface.

A later face-bound sketch also records the support's stable identity and
revision. An ambiguous face replacement produces a diagnostic. It must never
reattach by nearest geometry or traversal position. The initial operation owns
an analytic cylinder and has no dependency on a mutable body.

## Normal extrusion

Let `S(a,b)` be the support point, `N(a,b)` its chosen unit normal, and
`[d0,d1]` the ordered signed depth interval. The body is the image of

```text
X(a,b,t) = S(a,b) + t N(a,b),  d0 <= t <= d1.
```

Depth is physical distance. The interface distinguishes the chosen normal from
the solid boundary's outward orientation. Negative depth moves against that
normal; a two-sided interval spans zero. Admission checks the whole interval.

A returned solid must be closed and non-self-intersecting under the reported
validation coverage. Failed, cancelled or inconclusive construction returns a
typed failure with its stage, relevant inputs/entities and cause. No partial
body is a successful result. Numeric findings must state their evidence kind.

## Cylinder coordinates

Use an orthonormal frame with origin `O`, axis `Z`, radial direction `E0`, and
`E1 = Z cross E0`. Let `R > 0`, angular chart origin `theta0`, developed arc
length `s`, axial distance `z`, and normal sign `q` equal to `+1` or `-1`.

```text
theta = theta0 + s/R
E(theta) = cos(theta) E0 + sin(theta) E1
S(s,z) = O + R E(theta) + z Z
N(s,z) = q E(theta)
X(s,z,t) = O + (R + q t) E(theta) + z Z
```

The first admitted region is a rectangle with increasing `s` and `z` bounds.
Its four corners and four boundary curves have distinct caller identities
within each entity kind. Width, height and depth must be finite and resolvable
under the requested tolerances. The angular span is strictly below one turn,
with angular clearance greater than both the requested angular tolerance and
`precision::angular()`, plus physically resolvable separation of the end faces
at the smallest result radius. The native revolution builder closes angles
within its angular tolerance of a full turn. Every
radius `R + q t` stays positive and separated from the axis. Arithmetic overflow,
loss of coordinate resolution and ambiguous near-closure are refusals.

Normal reversal changes only `q`; it does not reverse the developed chart.
Physical radii may be reordered for construction, but the requested first and
last depth identities remain distinct.

The angular interval remains unwrapped. Crossing the conventional zero-angle
seam does not divide the authored region. A native chart can place its seam
outside the region; any required topological split retains the original curve
identity. A complete turn and overlapping turns require #419.

For width `w`, height `h`, and endpoint radii `r0`, `r1`, the intended volume is

```text
V = (w/R) h abs(r1-r0) (r1+r0)/2.
```

The factored expression avoids subtracting nearly equal squared radii. Error
accounting must also cover radius addition, coordinate differences and angular
division. This formula is an independent specification; the actual B-rep must
be checked against it.

## Cone coordinates

For a cone with apex `A`, semi-angle `alpha` in `(0, pi/2)`, positive slant
length `l` and circumferential angle `theta`, use

```text
S(l,theta) = A + l sin(alpha) E(theta) + l cos(alpha) Z
N(l,theta) = q (cos(alpha) E(theta) - sin(alpha) Z)
phi = sin(alpha) (theta - theta0)
DevelopedPoint = [l cos(phi), l sin(phi)]
```

The developed sheet is a sector with angular extent `2 pi sin(alpha)` per
physical revolution. Its seam and winding are explicit. A sketch excludes the
apex and any region whose offset reaches the axis. The normal extrusion changes
both radial and axial position. A circle on the developed sheet generally maps
to a spatial curve that is not a circle. The admitted `ConeSector` is an annular region in this developed sheet.
Its additive entry point and validation contract are specified in
[CONE.md](CONE.md); general developed curves remain #418.

## Construction and module placement

Place `surface_sketch` in the downstream `martensite-tksketch` toolkit. Keep the
support definition, admission and normal-extrusion result behind a small
interface. Reuse existing surface evaluators, B-rep builders, topology indexing
and the public TKBO validation, measurement and tolerance types. Keep
operation-specific construction and evidence internal.

Surface-bound sketch operations have their own tests and development cadence.
The downstream dependency leaves existing TKBO source and consumers unchanged;
changes to this toolkit do not select the suites of TKBO consumers. The new
toolkit exposes no upward reexport or dependency from TKBO.

For the first rectangular cylinder region, evaluate the existing rotational
sweep of its radial/axial cross-section as an analytic construction. Map native
sweep images back to the sketch's radial caps, four boundary curves and four
corners. This is a conditioned construction for the admitted region. It does
not establish a general wrapped-curve implementation.

The region generates both depth-offset faces and the solid. Each of the four
authored boundary curves generates its side face and both cap edges. Each
corner generates its normal connector and the two endpoint vertices. These
semantic relations are bound through native sweep images of the radial section,
not guessed from its traversal order. An entity shared by several sources
retains those parent relations when later operations introduce them.

History names every result face, edge and vertex through its source and role.
Source identity never comes from coordinate sorting or proximity. Expose any
result indices only as addresses in a result table; persistent identity comes
from the recorded source relation. Split pieces keep their authored parent.

The published result owns all reachable geometry and is `Send + Sync`.
Read-only metadata must require no lock. Do not expose interior-mutable native
handles that let a caller alter the published body. A native B-rep export must
be detached, and mutation of one export must not affect another or the result.
Use an immutable native B-rep encoding and value metadata if an existing owned
read representation cannot express the cylindrical body. Validate the decoded
body and its history correspondence before publication; serialization must not
silently change accepted geometry or entity bindings. Export decodes into fresh
owners. Measure encoding and export costs separately.

Preserve the planar interface and its execution path. Do not add dependencies,
unsafe code, caches or a generic surface extension point without a concrete
need. General wrapped curves may require analytic evaluators or bounded
approximation; mesh geometry cannot serve as acceptance evidence.

## Validation and limits

Reuse ordered topology, geometry, self-intersection and tolerance checks.
Report their actual modes: the current generic geometry checker is sampled and
its self-intersection checker is a native search. The affine-prism certificate
and area-times-height formula are inapplicable to normal extrusion.

Any analytic injectivity argument must be tied to the admitted domain and the
constructed carriers and trims. Parameter-map injectivity alone does not prove
that a native builder produced the requested body. Keep requested tolerances,
effective construction settings and achieved errors distinct. The operation
options include a separate volume-relative integration target; a spatial
tolerance is not a volume-error setting. Return the effective options and
achieved integration estimate without silently clamping the target. Report the
integration and construction-rounding estimates separately and require their
sum to meet the relative target. The construction estimate comes from the
input arithmetic and analytic boundary area, independently of the observed
volume discrepancy; a modelling tolerance is not an integration-error bound.
Return the effective linear and angular admission clearances, including native
precision floors. Respect explicit
report/refuse/no-limit tolerance caps.

A later face-bound operation rejects sketches outside its trimmed support by
default. Clipping and support extension require explicit policy. Union and cut
are separate non-destructive operations with composed history (#420).
