# Interactive latency and tenfold reduction in wrapped-sketch processing time

## User-facing acceptance

Measure from input to the presented frame and from extrusion commit to
publication of the validated body. These are proposed acceptance targets.

| Interaction | Target |
| --- | --- |
| Place or drag a sketch entity | Present by the next refresh at p95. The frame budget is 16.7 ms at 60 Hz and 8.3 ms at 120 Hz. |
| Change extrusion depth or invoke extrusion | Show the first preview within 100 ms at p95. Keep showing depth changes at the display refresh rate. |
| Commit a representative extrusion or a wrapped text edit | Publish the validated solid within 1 s at p95. Include cold creation and warm edits in the benchmark set. |

Replay create, drag, release and commit through the application on the
cylinder and cone. Include a seam-crossing drag, a constrained connected
group, text repositioning, and depth-only edits. Time hit development,
constraint solving, preview update and presentation separately. Measure
the full extrusion on the same trace, including sketch material,
tool, Boolean and final validation. The kernel benchmarks below cover
extrusion; the application trace will establish the interaction baseline.

The public certified conical-frustum entry met the kernel commit target on
the cold boss, cold pocket and warm depth edit. Desktop traces met the
drag, preview and commit targets for a cylindrical rectangle, including
creation, re-edit, a seam-crossing boss and an inward pocket. Native
conical authoring, text gestures and connected constraint groups still
need end-to-end measurements.
[Measured kernel results](PERFORMANCE-REASSESSMENT-2026-11-24.md#public-certified-attachment).
[Measured desktop results](PERFORMANCE-REASSESSMENT-2026-11-24.md#2026-11-25-desktop-cylindrical-rectangle).


### Keep mouse movement on the frame path

Retain the revision-bound authoring chart and last solved coordinates for
the edit. On each frame take the newest pointer sample, update the
touched constraint component and refresh only the changed preview
geometry. Reuse the last solution as a starting point for the next
sample. Keep support and seam classification keyed to their revisions.
Use the existing authoring chart API for the selected face.
[Wrapped workflow](VISUAL-WORKFLOW.md).

Run region rebuilding and full proofs on an immutable edit snapshot
off the input and render paths. Cancel superseded work and bind each
result to the edit and support revisions before showing or publishing it.
An approximate preview is display geometry. On release, commit the exact
solved sketch and only publish the feature after ordered validation.
For a depth-only edit, reuse the accepted sketch and tool inputs whose
keys are independent of depth. Warm immutable, revision-bound proofs
during editing so a later extrusion can start with available work.

## Kernel targets

Use the rebased, assertion-enabled `v0.14.0` results as the reference.
Time the same public operations, material selections, proof policies and
geometry. For the V7 test, time the public call alone in both builds when
the gate is run. The current 5.68 s includes the test assertions.
[Rebase measurements and profiles](PERFORMANCE-REASSESSMENT-2026-11-24.md).

| Workload | Reference | Target for a 10× reduction |
| --- | ---: | ---: |
| 239-curve V5 outward boss, stock | 78.76 s | ≤7.87 s |
| Same boss, four workers with #582 trimming | 49.65 s | ≤4.96 s |
| 239-curve standalone V5 outward tool | 11.93 s | ≤1.19 s |
| V7 connected `O.` | 5.68 s | ≤0.56 s, provisional until the API-only reference is pinned |

The first boss row is the initial kernel milestone. The trimmed row
gives the corresponding tenfold target if four-worker validation
becomes the shipping baseline. Keep inward pockets, disconnected
glyphs, counters, inner support trims and periodic seam-crossing in
the qualification set.

## 1. Measure which result pairs can reuse a proof

Start with the decoded result of Fuse or Cut. Join each candidate to
Boolean history and try to bind its carriers, filled trims, location
and incidences to the tool certificate. Record counts and solver time
by interference kind, source class and reason for falling back to
native search. Exercise this with search enabled for every pair.

The existing census classifies ancestry, which establishes an upper
bound on coverage. In particular, many useful pairs touch the seam,
and the current mathematical certificate keeps a separation verdict
without the lower clearance bound needed by a tolerant Boolean.
Bind candidate-specific lower bounds derived from existing proof
domains and charge the work. Compare the bound with both decoded
tolerances and the active fuzzy value. Treat shared incidences and
split or ambiguous images as separate cases. Keep native search
where the result ancestry, exact binding or bound is insufficient.
[Coverage transfer design](TOOL-COVERAGE-TRANSFER-PLAN.md).

## 2. Cut the cost of validating the decoded solid

Apply the proved pair filter before dispatching edge–face and face–face
tasks. Add a conservative analytic broad phase for the remaining
tool–support and seam candidates, using the selected cone, its bounded
trim and conservative interval enclosures. Re-enter the existing
native solver whenever the bounds overlap. Verify that removing a
pair preserves the later pave-block state and findings for all pairs
that still run.

Reuse ordered topology and sampled geometry evidence for unchanged,
exactly rebound entities where the original check remains valid in
the result. Recheck changed incidence neighborhoods and run the
ordinary ordered checks for the remaining entities. Admission and
tolerance checks still use the decoded solid. Report transferred
coverage and native coverage separately.

Measure time by the cost of the eliminated work as well as pair count.
The target is 3.75 s for the entire final validation. Ordinary
four-worker search gives an intermediate speedup. Even with #582,
duplicate narrow-phase queries have to come out of this stage.

## 3. Reuse the work from the Boolean

Profile the pairs and construction work left in Fuse and Cut. Pass
operation-owned, revision-bound carrier and trim captures through the
existing Boolean path. Reuse a recorded pair outcome when its exact
operands, domains, tolerances and search policy match. Dissociate
proofs when a carrier, trim or incidence changes. For the selected
analytic support, use certified cone-domain rejection before calling
the generic intersection solver. Retain ordinary Fuse and Cut for
the overlap and ambiguous cases.

Budget 1.50 s for the material Boolean. Verify the boss and pocket
independently, including new seam edges, intersecting support trims,
crossed fuzzy thresholds and one source mapping to several result
faces.

## 4. Reduce exact proof cost at tool construction

The text tool spends most of its time in planar selection and
member geometry qualification. Measure exact polynomial operations,
maximum operand widths, allocations and repeated decoded-cap checks.
Keep common source and policy captures through selection, fitting and
qualification. Reuse compatible filled-trim proofs across caps only
after rebinding the actual directed rows and their parameter ranges.
Apply a conservative filter before large integer work and keep the
existing exact fallback when the filter is inconclusive. Carry
dyadic or common-denominator accumulations through the stages that
already use them, and convert at the proof boundary.

For the V7 connected case, concentrate on the decoded cap material
binding and the outline's ordered native qualification. Use the same
checked source indexes for rebinds and validation so multiple passes
over a member need fewer allocations and native lookups.

Budget 2.00 s for the attached tool, including placement and footprint
checks. Budget 0.50 s for the remaining archive and history work.
With the Boolean and final validation budgets, this leaves 0.12 s
under the 7.87 s delivery target. The time budgets are acceptance
targets to test against profiles after each change.

## 5. Reach subsecond publication on the interactive workloads

Measure which route still exceeds 1 s after proof reuse. For repeated
depth and placement edits, recompute the changed proof and B-rep
neighborhoods from the last accepted snapshot. Verify the revised
body against a full rebuild using the same input and result contract.

If cold attachment is still dominated by generic Fuse or Cut, qualify
an analytic construction path inside the existing feature operation
for the selected cylinder and cone. Admit it only with a verified
attachment seam, support containment, source-to-result incidence and
an explicit tolerance bound. Use the existing Boolean path for cases
outside those conditions. Compare the conditioned result against the
full Boolean and independent material and history oracles. Measure
cold creation separately so background work during editing cannot
conceal its cost.

### Direct conical attachment

Begin with a single-shell frustum bounded by the selected cone and two
analytic end faces. The frozen tool publishes its side faces and last-depth
caps. A direct assembly replaces the selected support face with its remainder
and a separate positive face for each retained counter. Bind the former
first-depth edges through native topology IDs. The two incident coedges must
use the same edge with opposite orientations after the archive is decoded.

Prove three obligations on the actual decoded geometry before replacing the
native self-intersection search.

1. The support pcurves replay on a single native periodic sheet, match the
   stored 3D edge at the same parameter within `sew`, and form embedded loops
   with the intended material and counter winding. Rebind their exact stored
   knots, controls and endpoints. Qualify a sparse exact planar census to
   reduce the repeated pair work on separated hulls.
2. The retained tool surfaces stay on the correct side of the stock cone over
   their complete parameter domains and clear both end planes. Evaluate the
   signed implicit carrier polynomial on the stored homogeneous controls
   using outward enclosures. Replay unresolved signs with exact arithmetic
   under a bounded policy.
3. The decoded topology binds every retained tool patch and support incidence
   to its proof and history. Use the checked native edge and face tolerances
   in the seam contract. Require the ordered topology, geometry and tolerance
   stages before publishing. Keep the full native checker as an independent
   oracle for the admitted fixtures.

The scratch builder now assembles the full 239-curve fixture in both senses.
It reproduces native topology, inherited geometry and the two counter
islands under the ordinary validator. Its seam proof bounds every fitted
span under the original sewing limit. Certificate coverage for publication
still needs to bind actual face, wire and edge occurrences through freezing.
Surface identity alone is insufficient because two faces on the same
carrier can resolve the same stored pcurve.

Use the retained stock rim, authored boundaries and all decoded side and
cap incidences to enumerate the contact domains. First certify that off-
incidence pairs are separated. For proximity near a declared seam or
endpoint, bind the enclosure to the relevant entity-pair tolerance and
verify the contact stays in the corresponding retained trim neighborhood.
The native checker can grow its working tolerance adaptively. A direct
certificate needs its own versioned contract and a comparison with native
findings. Keep the original Boolean policy selectable. Add controls for
changed side coefficients, a same-endpoint UV loop, exhausted arithmetic
work, a moved or reversed coedge, a contact assigned to the wrong
endpoint, and a rim or periodic-seam contact.

### Build the final geometry once

The current result includes an independently exportable canonical and
placed tool, each with per-member validation and measures. That contract
forces the intermediate publication before the final stock operation.
Prototype a direct result that holds the final solid, achieved limits,
ordered validation and composed authored history. Preserve the existing
result for callers that consume the detached tool.

Use the issued directed cycles as the starting point for boundary fit
and endpoint finalization. Build the shared rails, ruled sides and exposed
caps from those controls in one private owner. Apply the stored placement
once and attach the stock remainder and counters. Bind the retained
patches, the actual fitted stock trim, and the measure inputs during the
same final decode. Reuse certified boundary integrals where the oriented
domains and stored inputs match. Keep the final boundary measure, native
quadrature estimate and authored-reference comparison distinct.

Budget up to 235 ms for frame admission, mapping, fit and endpoint
finalization, 90 ms for native construction and assembly, and 80 ms
each for strict sewing and the decoded UV and carrier proofs. The
remaining 515 ms covers the 20 ms authored cycle and area budget, archive
and ordered validation, measures and headroom. Run both cold senses and
the warm depth edit before claiming the 1-second p95 goal.

## Qualification

- Pair baseline and candidate in fresh processes on the same physical
  cores, with the same compiler, assertions, allocator and fixture.
  Use at least five accepted pairs and retain all timings, peak RSS,
  retired instructions and per-stage work. Report the median speedup
  and stay within the existing 10% peak-RSS allowance.
- Compare full and filtered checks on the same decoded outputs. Keep
  ordered findings, per-face geometry checks, material regions, authored
  history and attributed validation coverage. Exercise changed carrier
  bits, trims, seams, tolerance boundaries and planted interference.
- Require byte-identical replay for existing policies, including across
  processes and worker counts. A new behavior version needs independent
  geometry, measure and history oracles under the existing limits.
- Rerun inward, outward and cancellation controls as coverage expands.
  Measure supported native allocators before enabling a worker policy
  by default. Qualify the feature through the affected package and PR
  gates once the end-to-end targets are met.
- Replay the same recorded gesture at both refresh rates and under a
  concurrent extrusion. Record p95 input-to-present, missed frames,
  first-preview latency and exact commit latency. Verify that a stale
  solve, preview or extrusion can never replace a newer edit.
