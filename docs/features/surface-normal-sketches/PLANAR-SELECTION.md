# Disconnected planar selection and global boundary preparation (#418)

The private wrapped-profile selection stage now qualifies multiple authored
regions against one planar arrangement. It reuses `build_planar_regions` and the
single-region directed-boundary matcher. It then prepares every region and
certifies one boundary relation over all prepared spans, including cross-region
pairs. It returns preparation evidence, not constructed or accepted bodies.

## Selection semantics

Every requested member has one Jordan outer contour and optional holes. The
existing planar builder constructs all bounded arrangement faces; the selection
matches each member by its complete directed authored curve lists. Capture-local
region indices never become member identities. Results retain the caller's
`SurfaceRegionId` values in a sorted map.

This global classification matters for nested members. An island inside an
unselected hole is disconnected material and can be selected alongside the
surrounding ring. A smaller region inside filled material cannot be selected
alongside the containing filled region: the global arrangement gives the latter
an additional hole, so its declared boundary no longer matches. Undeclared
cross-region boundary contacts are refused by the existing graph certificate.

Arrangement certificate counts describe the whole capture, including unselected
hole-interior faces. They are not counts of selected bodies. Member area evidence
retains its existing classification; selection does not upgrade it to exact.

Before capture, admission requires nonempty input, at most 64 selected regions
and 4,096 authored curves, unique region IDs and globally disjoint curve and
vertex IDs. It applies the existing request admission before indexing any poles.
Support radius, depth offsets and spatial tolerance must match by their declared
bits. Per-region fit limits remain explicit. Failures retain the offending
region, curve or vertex and the underlying cause.

## One complete boundary relation

Each original curve keeps its authored ID and exact dyadic source intervals
through fitting. Collection preparation combines every prepared span, checks
prepared region membership and rejects duplicate authored curve names. The
clearance query excludes only consecutive pairs within each contour. It excludes
no cross-region pair. Thus its pair count is `n*(n-1)/2` minus the declared
within-contour adjacent pairs, rather than a sum of isolated regional counts.

The complete nonzero-periodic-image check then runs once for the collection.
All image relations share its existing arithmetic, domain and curve-node
allowances and certified infinite tail. Fitting retains its existing per-authored-
curve planning allowance; it is not described as the global clearance allowance.
The single-region path calls the same capture and clearance implementations with
one member. No geometric retry or separate modelling algorithm was introduced.

## Periodic material classification

Boundary clearance does not classify material for disconnected selections.
The retained regression uses support radius one, a rectangle `A=[0,2]x[0,2]`
and a smaller rectangle `B=[6.5,7]x[0.5,1]`. They are separate planar regions.
The complete boundary proof passes one positive periodic image. Nevertheless,
`B-2*pi` lies strictly inside `A`. The retained true-period enclosure establishes
`5 < 2*pi < 6.5`, which suffices for those strict horizontal bounds. Both vertical
bounds are also strict. A family constructor must refuse this overlap even though
no boundary crossing exists.

The [periodic material gate](PERIODIC-MATERIAL.md) now classifies both directions for every ordered member
pair and each required positive image. Given original regions `A`, `B` and a
positive shift `kT`, it must locate an outer-boundary witness of `A` in `B+kT`
and an outer-boundary witness of `B+kT` in `A`. Testing only the first direction
for positive images misses containment in the reverse direction. With separated
Jordan boundaries, those membership checks distinguish disjoint material from
strict containment, including containment in holes. The global zero-image
arrangement already supplies the unshifted material relation. The tail's
separated slabs exclude both boundaries and material for all remaining images.

Reuse the existing exact `classify_bezier_wire` primitive on the original
polynomial contours. Classify the outer loop first and, when necessary, its
holes. A point outside the outer loop or inside a hole is outside the selected
material. An off-boundary Jordan result must have winding zero or magnitude one;
missing, boundary or inconclusive required results must refuse.

For a rounded literal shift `L` with certified true-period error `sigma`, use
rounded query points `p-L` and `p+L`. Their addition/subtraction error must be
bounded, never assumed zero. The boundary proof establishes literal clearance
at least `delta+sigma`. If the query-point rounding error is strictly less than
`delta`, then its rounding plus `sigma` cannot cross that boundary, so the exact
winding result transfers to the true periodic image. This needs a charged
outward arithmetic receipt bound to the original witness, image and direction.
A horizontal ray toward zero, or toward one when the seed's S coordinate is
zero, supplies a distinct finite endpoint without assuming `seed+1` is
representable. Wire locations must share one explicit query/subdivision budget
across all members, loops and images.

The material gate now performs these checks with original-source snapshots,
complete dyadic covers and aggregate probe/winding budgets. Preparation evidence
alone remains insufficient. Frozen collection construction, global retained-
source binding, history and publication gates remain open. No version or release
tag changed.

## Verification

The Linux x86_64 sketch package passed 92 library tests and 32 integration tests.
Strict all-target clippy passed, and all 15 focused selection controls passed
after the final period-bound test cleanup. Controls cover separated members,
an island in a hole, containment in filled material, cross-member contacts,
selection order and loop-start changes, duplicate identities, common-context
refusal, malformed sources, finite collection limits, fit refusal, insufficient
global clearance and incomplete prepared membership. The evidence and error
types have compile-time `Send` and `Sync` assertions; operation concurrency remains
unverified.

The original isolated debug P1 fixture took 3.42 seconds with peak RSS 52,168 KiB,
excluding compilation. The five-second limit is unchanged; this timing does not
establish a performance improvement or regression. The approximate scan found
no inspected file over 500 lines, function over 20 code lines or nesting over two.
No dependency or public primitive API changed. The full affected-package PR gate
remains required before a push.
