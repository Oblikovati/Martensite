# Exact periodic material classification (#418)

The private cylindrical selection stage now classifies material for every
required finite periodic image. It follows global planar selection and complete
boundary clearance. A successful result describes original material regions;
frozen family construction and binding to retained profiles remain separate
steps. It does not publish a body.

## Complete containment relation

For each positive image `k`, every ordered pair `(A,B)` gets two queries:

- Locate an outer-boundary witness of `A` in `B+kT`, using `pA-kT` in B's chart.
- Locate an outer-boundary witness of `B+kT` in `A`, using `pB+kT` in A's chart.

Self-pairs are included. Negative images follow by translating the complete
ordered positive relation. With `m` regions and `n` finite images, the stage
requires exactly `2*n*m*m` probes. It does not truncate the relation to fit a cap.
The global planar arrangement already establishes the unshifted material
relation. The certified tail's separated slabs exclude boundaries and material
for every remaining image.

Each target query first classifies its outer contour. Outside that contour
means outside material. Inside a hole also means outside material. If a witness
is inside the outer contour and no hole contains it, the stage refuses with the
source region, target region, original witness curve and periodic probe.

This decision uses the existing exact `classify_bezier_wire` primitive on the
original degree-one through degree-three polynomials. Each curve retains its
caller ID and full original parameter interval. Required locations must be off
the boundary with winding zero or magnitude one. Boundary, inconclusive or
unsupported winding results refuse. Root bracket width is not a modelling
tolerance, and no tessellation supplies membership evidence.

For separated Jordan boundaries, overlapping outer domains force containment.
An inner outer contour inside the other region's filled material proves an
open material overlap near that contour. If it lies in a hole, its bounded
interior lies there too. Testing both directions therefore handles containment
in either direction while allowing islands in holes. This uses the globally
certified outer/hole roles and complete boundary separation; a winding scalar
alone does not certify a region.

## Rounded probes and the true period

The mathematical entry point is
`CertifiedCylindricalPeriodicClearance::certify_probes`. A probe records its
positive image, original boundary point and addition/subtraction sense. For
literal shift `L`, outward dyadic arithmetic bounds the difference between the
rounded query coordinate and the exact operation `pS +/- L`. The Z coordinate is
retained bit for bit. The method refuses nonfinite original or rounded points.

Let `sigma` bound the literal shift's error from the true `k*2*pi*R`, and let
`delta` be the physical developed-plane clearance. The existing boundary
certificate establishes literal clearance at least `delta+sigma`. The new
probe requires rounding error strictly smaller than `delta`. Its total motion
from the true query is therefore smaller than that literal boundary clearance,
so the exact winding classification cannot cross a boundary. Probe rounding
and true-period error remain separate receipts; neither is silently discarded.
The mathematical method certifies rounding, conditional on the witness being
on its declared source boundary. The native stage establishes that incidence
from the original outer curve's exact endpoint.

The ray endpoint has the same Z coordinate and S coordinate zero, or one when
the seed's S coordinate is zero. It is finite and distinct without assuming
that `seed+1` is representable. There is one deterministic ray choice per query,
with no geometric retry ladder.

## Provenance and interval coverage

Planar evidence now owns snapshots of the original curve IDs, exact source
spans, endpoint identities and pole bits. Reusing only a region's directed-ID
signature after a geometry edit is insufficient. Before winding, the stage
checks every member's snapshot, fitted request context and global boundary
source relation. It binds the periodic report to that exact original clearance,
including chart, clearance and exclusions.

Every authored curve must also have a complete, nonoverlapping dyadic cover of
`[0,1]`. Exact integer normalization checks nonuniform subdivisions without
assigning identity from interval order. Missing curves, omitted intervals,
overlaps, invalid levels or indices and changed adjacency exclusions refuse.
This prevents a certificate for a selected subset of spans from standing in for
the whole material boundary. Raw interval numbers and probe positions are local
references; region and curve identities remain authored IDs.

## Work and cancellation

Probe revision one admits 8 through 4,096 fractional bits, at most 65,536 probes
and at most one million charged interval operations. Defaults are 128 bits,
8,192 probes and 100,000 operations. An empty batch can complete with zero work
and a zero probe allowance. Invalid policies and exhausted work return no
partial successful batch.

The material stage separately caps completed wire-location queries at 65,536,
with a default of 8,192. It shares one requested root subdivision allowance
across every member, contour and image; the default is 4,096 subdivisions and a
`1e-6` parameter-bracket width. Supported root policy is version one with a
degree cap from three through 32. Each location records its effective remaining
allowance, actual subdivisions, contour, probe and signed winding. The aggregate
sum is checked before publication of the receipt.

Cancellation is observed before the material stage, during probe arithmetic and
inside winding queries. Nested winding errors retain their typed cause and
scope. Cancelled, incomplete and exhausted results do not become successful
material evidence. These work budgets are separate from the earlier fitting
and global-clearance budgets, whose receipts remain in the prepared selection.

## Controls and publication scope

The earlier regression `A=[0,2]x[0,2]`, `B=[6.5,7]x[0.5,1]`, support radius one,
now refuses with source region 51, target region 50 and witness curve 200. Its
boundaries still pass their independent periodic clearance check. A corresponding
case translated into an unselected hole passes. Other controls cover separated
regions across one turn, self-pairs, every ordered-pair direction, zero explicit
images, caller/loop ordering, stale original geometry, changed context and an
unrelated periodic report.

A curved-source case records nonzero winding subdivisions. Removing one unit
from the aggregate allowance refuses at a later query with earlier work already
spent. Location and probe-count caps refuse without dropping queries.
Cancellation controls stop both before probes and in winding. Exact rational
oracle checks independently enclose addition/subtraction error; large-coordinate
rounding is refused rather than classified as zero. Public probe types and
private selection evidence have compile-time `Send` and `Sync` controls;
concurrent-operation verification remains open.

This gate classifies original cylindrical material. The later
[frozen-family checkpoint](FROZEN-FAMILIES.md) constructs all member solids in
one table and matches their retained O2 sources to the global boundary proofs.
Complete history, the full isolation checks, replay/concurrency and publication
still remain.
General cones, additional authored curve classes, glyph scale, oblique frames
and replacement deletion are also open. No dependency, public primitive API,
version or release tag changed.

## Verification

The affected-package run on Linux x86_64 passed 557 math library tests, 3,114
math integration tests, 56 math doctests, 108 sketch library tests and 32 sketch
integration tests. After the final structural cleanup, all 11 probe controls,
14 material controls and two exact-partition controls passed. Strict all-target
clippy passed for both packages. The additional probe test split work exhaustion
and cancellation into separate controls; no numerical algorithm changed in that
cleanup.

The isolated debug P1 fixture, including its native checks, source snapshots and
measure comparison, took 3.43 seconds with peak RSS 52,576 KiB, excluding
compilation. The five-second limit is unchanged; an individual timing establishes
neither improvement nor regression. The approximate scan found no inspected file
over 500 lines, function over 20 code lines or nesting over two. The full
affected-package PR gate remains required before a push.
