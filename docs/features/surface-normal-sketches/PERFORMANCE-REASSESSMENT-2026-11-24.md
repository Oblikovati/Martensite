# Wrapped-sketch performance on v0.14.0

The pre-rebase revision was `5be785fc`. The rebased source was
`3a628a44`, based on release commit `b2a6a5a2`. The follow-up
`2b8d1026` formats one signature. The two revisions use the same input
fixtures and public selections. The attached case is the 239-curve
transported V5 outward boss with 18 material regions.

These are local Linux x86_64 measurements on a Ryzen AI Max+ 395 with
glibc 2.39 and Rust 1.99.0. Both builds use the assertion-enabled Cargo
test profile, including the workspace's TKMath and TKBO optimization
overrides. Each observation used a separate process. The attached and
standalone text tests ran on logical CPUs 0–7. The quieter connected
atlas series ran on CPU 14. Elapsed API time comes from the fixture
where available. RSS is the peak of the entire test process.
The text comparison used one standalone pair and two stock boss pairs.

## Rebase measurements

| Workload and metric | Before rebase | After rebase |
| --- | ---: | ---: |
| 239-curve standalone outward tool, API time | 14.97 s | 11.93 s |
| 239-curve standalone outward tool, peak RSS | 198,836 KiB | 196,448 KiB |
| V7 connected `O.`, median test time, three samples | 7.02 s | 5.68 s |
| V7 connected `O.`, median user-space instructions | 50.87 billion | 39.61 billion |
| V7 connected `O.`, median peak RSS | 66,712 KiB | 64,472 KiB |
| V5 attached boss, default policy, median API time | 61.86 s | 78.76 s |
| V5 attached boss, default policy, median peak RSS | 472,500 KiB | 319,746 KiB |

The standalone text tool retained 3,057,275 arithmetic operations and
1,017 integration subintervals. The V7 median retired 22% fewer
instructions. Its fastest test took 5.41 s, and the measured test
median was 5.68 s against the earlier 5.4 s API target. The larger
common-chart control recorded 989,355 arithmetic operations, 444
integration subintervals and 148 spans in both directions after the
rebase. The pre-rebase outward runs recorded the same counts.
Its wall time moved with SMT sibling activity. One paired run recorded
212.94 billion user-space instructions before the rebase and 186.35
billion afterward.

The old feature branch ran the native self-check on four workers.
The tag restored the serial check to limit allocator retention across
multiple checks. Under the old four-worker policy and with no heap
trimming, the tagged kernel completed the boss in 48.96 s, 21% less time
than the old branch, and peak RSS went from 472,500 to 455,278 KiB.
This comparison captures the combined kernel changes between the revisions.

## PR #582

The runtime change from `5324c600` was applied in an isolated copy
of the rebased source. A temporary selector ran serial, four-worker,
and four-worker-with-trim checks in one build. The same test assertions
ran in each mode. Encoded solids, validation reports and composed
histories matched byte for byte across the three modes.

| Post-rebase self-check policy | Median API time | Median peak RSS |
| --- | ---: | ---: |
| Serial, stock build | 78.76 s | 319,746 KiB |
| Four workers | 48.96 s | 455,278 KiB |
| Four workers with `malloc_trim(0)` | 49.65 s | 315,798 KiB |

The two threaded modes use three timing observations each. Two peaks per
mode are from timers wrapped around the test inside the sandbox. Three
earlier sandbox measurements captured the supervisor's memory and are
excluded from the RSS comparison. The serial selector took 80.30 s median
and its usable peak sample was 318,484 KiB.

Trimming reduced the four-worker peak by 136 MiB. The original problem
therefore persists on Linux/glibc after the object-model migration.
The trim variant took 37% less time than the stock tagged policy with
a peak 1.2% lower. The existing self-check suite passed eight tests,
including the comparison of serial and parallel findings. The native
trim test passed with live allocations retained.

The open PR still needs an update against `main`, including the versioned
compiler inventory and a new Unreleased changelog entry. Its trim runs
across the application's glibc heap. The PR also turns on parallel
validation on platforms where its trim function does nothing. The
Linux result qualifies the attached boss on this allocator. It leaves
long-operation memory on other platforms and the effects on other
glibc clients in the same process open.
[PR #582](https://github.com/Oblikovati/Martensite/pull/582),
[glibc `malloc_trim`](https://man7.org/linux/man-pages/man3/malloc_trim.3.html).

## Profiles

The stage traces used an isolated build with clocks around the public
work and its major subroutines. The traced serial and trimmed runs
produced the same encoded solid, validation and history as the
benchmark runs. The values below are diagnostic runs. The untraced
medians above are the performance reference.

| Attached boss stage | Serial | Four workers with trim |
| --- | ---: | ---: |
| Construct, place and trim tool | 15.89 s | 15.41 s |
| Fuse | 5.70 s | 5.35 s |
| Validate decoded result | 59.40 s | 26.91 s |
| ↳ Native self-intersection | 55.46 s | 22.80 s |
| ↳↳ Edge–face solver, 2,836 tasks | 22.55 s | 8.45 s |
| ↳↳ Face–face solver, 1,236 tasks | 22.62 s | 6.63 s |
| Complete API call | 81.30 s | 47.99 s |

Both worker policies submitted these task counts. In the serial trace
the two solvers accounted for 45.17 of the 55.46 s native
self-check. Stage preparation, storing results and pave blocks
consumed the rest. Topology took 1.24 s and sampled geometry took
2.67 s. Attaching the tool therefore incurs a large second search on
the decoded output after Fuse. The support chart and native tool
export together took under 0.1 s.

The standalone V5 text tool spent 7.87 s in planar selection proofs
and 11.05 s in member checks in a 21.90 s diagnostic call. These
member checks cover decoded geometry and numeric measures, the
family certificate, and ordered native checks. Fitting took 0.83 s.
Cycle sampling found allocation and freeing, movement of memory and
big-integer arithmetic among the busiest leaf routines.

The common-chart `O.` API spent 8.23 s of a 10.92 s diagnostic call
qualifying the outline's decoded geometry and measures. The two
ordered native checks took about 1.40 s combined. Big-integer
division accounted for 13.74% of user-space cycles in a separate
sample. In the faster V7 case the top timed stages were decoded
family binding at 1.03 s and the outline's ordered native check
at 1.35 s, in a 3.56 s diagnostic test.

Cycle samples were collected at 127 Hz from normal builds. Leaf
attribution was available for the sampled routines. Unwinding
resolved only a fraction of the caller stacks, so stage attribution
uses the clocks rather than percentages derived from those stacks.

Formatting and warnings-as-errors library Clippy passed for the
rebased branch. The TKSketch library suite passed 600 tests with
nine ignored. The attached boss and pocket passed their native
validation, 18-cap coverage and material-change assertions.
[Tenfold performance plan](TENFOLD-PERFORMANCE-PLAN.md).

## Optimized latency baseline

Twenty fresh processes per fixture ran on the same eight logical CPUs
with the release profile at `7ad97e2c`. The public API timers start
after the fixture prepares the request. The V7 fixture at this revision
timed the entire test, including its assertions.

| Fixture | Median | Empirical p95 |
| --- | ---: | ---: |
| 239-curve attached outward boss | 19.344 s | 20.323 s |
| 239-curve attached inward pocket | 19.602 s | 20.514 s |
| 239-curve standalone text tool | 5.862 s | 6.025 s |
| Directed common-chart `O.`, outward | 7.099 s | 7.440 s |
| V7 connected `O.`, whole test | 1.135 s | 1.190 s |

The exact depth-edit fixture holds the previous text boss while it
recomputes after a 2% depth reduction. In twenty fresh processes,
the edit took 19.998 s median and 20.348 s at p95. The test also
checks that the old archive stays intact, the result has the same
cap parents, and material volume decreases.

The existing simple cylinder boss and pocket controls ran in under one
second at p95, including setup and independent validation assertions.
Twenty runs of each on one pinned CPU gave 0.850 s and 0.950 s
respectively. The narrow oblique cone boss control, including a
subprocess replay of the same operation, gave 2.250 s on that CPU.

API-only timers at `43a718ae` ran 20 fresh processes per fixture on
two pinned cores. The cylinder boss reached 0.416 s at p95, the cylinder
pocket 0.426 s, and the cone boss 0.519 s. The V7 connected `O.` call
reached 1.320 s at p95 in this run. The timers exclude fixture setup
and the later assertions. All runs still executed the geometry, solid
validity, and history controls.

The caller-owned cylinder chart eliminates a repeated support decode.
Across 256 paired hit measurements, its p95 moved from 44.151 to
18.484 µs on a single-cylinder support, and from 1,029.576 to
29.164 µs on a support with 16 additional disjoint solids. The inputs
were already native UV hits. Each pair matched result bits and trim
state. The independent-export and concurrent-reader controls passed
under ThreadSanitizer in an isolated Linux runtime. Input-to-display
and preview timing still require the desktop workflow and native
presentation evidence.

### Shared ruling checks

The decoded boss contained 339 adjacent Bézier face pairs with an
actual common linear ruling. Outward interval signs put all the
off-ruling poles on opposite sides of a plane through that ruling
for each pair. An experimental substitution of empty face–face
records saved 7.25 s in the native debug checker on this archive
and kept the published validation report and archive bytes identical.
The substitution also changed the derived vertices, split edges and
pave blocks that feed later native stages. It was excluded from
production. A shared-ruling certificate needs a bounded tolerance
collar and a way to retain or separately validate the section payload
those stages consume.

### Optimized projection and checker trials

The point-surface extrema change traverses contiguous grid rows without
changing the search order. A release-build probe queried the 1,577 Bézier
edge–face pairs from the decoded text boss at the start, middle and end of
each edge range. Across 4,731 queries per sample, every ordered extremum,
parameter, point and squared distance matched bit for bit. Median warm
projection time fell from 568 to 361 ms across six warm samples per variant.
Peak RSS was within 0.2%. In five alternating full boss runs, median API
time fell from 23.912 to 22.579 s and retired user-space instructions fell
by 3.8%. The frozen boss, ordered report and composed history were
byte-identical to the saved baseline.

In three fresh-process pairs on four pinned physical cores, the isolated
checker dropped from 13.174 to 6.146 s median when allowed to use four
workers. Peak RSS rose from 226,748 to 234,452 KiB. Ordered checker
observations, native pave state and section curves matched byte for byte.

The same policy exceeded the 10% memory allowance when applied solely to
final validation within the public feature. These are single matched API
call timings from the scratch build. The archive, report and history
matched for each operation.

| Text fixture | Serial API | Four-worker API | Serial peak RSS | Four-worker peak RSS |
| --- | ---: | ---: | ---: | ---: |
| Cold boss | 21.107 s | 15.195 s | 286,956 KiB | 432,180 KiB |
| Cold pocket | 22.736 s | 15.491 s | 314,512 KiB | 451,264 KiB |
| Warm 2% depth edit | 21.943 s | 14.494 s | 318,764 KiB | 453,696 KiB |

On a separate cold boss run, two workers raised peak RSS from 287,052 to
383,896 KiB. Before final validation the resident sizes agreed within
0.2%. Most of the extra memory was anonymous and stayed resident when the
validator returned. A 44×44 face projector retains about 0.48 MiB in four
parameter grids. Three grids hold point-dependent scratch. Across 339
cached face projectors those three grids alone account for about 120 MiB
of capacity. Reusing the grids across point queries is being tested
separately from worker scheduling.

The GeometricCompositeV6 authored selection trial reduced the median
planar stage from 2.658 to 0.379 s in five fresh-process pairs. All 18
selected regions kept identical directed loops, native archives,
validation and area bits. An independent rational Green integral lay
inside every reported area interval. The new route spent zero pair
bisections and the original spent 7,456. It is being qualified as an
explicit execution selection because the two policies can differ in
which requests fit the current work limit.

### Offscreen interaction measurements

A scratch Vulkan fixture on Linux with the Radeon 8060S starts from
native cylinder UV hits and updates a retained ribbon. It measures chart
mapping, preview geometry and submission through GPU completion. It ran
1280×720 and 3840×2560 in both run orders at synthetic 60 and 120 Hz.

| Viewport | Drag p95 at 60 Hz | Drag p95 at 120 Hz | First preview p95 at 60 Hz | First preview p95 at 120 Hz |
| --- | ---: | ---: | ---: | ---: |
| 1280×720 | 0.91–0.96 ms | 0.88 ms | 1.26–2.94 ms | 0.97–1.04 ms |
| 3840×2560 | 1.00–1.01 ms | 0.92–0.97 ms | 1.15–1.79 ms | 0.77–1.00 ms |

Each run used 160 drag updates and 40 fresh preview openings per mode.
Chart opening and support capture were measured separately from the
preview. No update missed its synthetic frame budget. Immediate and full
render captures had identical color and depth. The test ends at the GPU
completion signal. It excludes the application picker, constraint solve
and compositor presentation.

### Application constraint solve

A scratch probe used the current Oblikovati Rust planar solver at
`a30d24618`. It used the Move preview's sketch clone, coordinate update
and selected-point anchors, with 160 targets per connected chain.

| Connected horizontal lines | Solve p95 |
| --- | ---: |
| 2 | 0.0115 ms |
| 8 | 0.986 ms |
| 24 | 30.831 ms |

Clone, transform and anchor setup took at most 0.016 ms at p95. The
24-line solve alone exceeded the 60 and 120 Hz frame budgets. In a
separate alternating comparison, seeding that chain from the previous
solution changed p95 from 28.159 to 27.937 ms. The selected anchors
stayed exact, and the remaining coordinates differed by up to
`5.91e-7` model units. Cycle samples attributed 43% of self time to
`hypot` and 25% to applying Householder reflectors. The planar solver
rebuilds and factors a dense Jacobian for each trial. These timings
cover the model entry point independently of viewport rendering.

### Conical attachment seam

All 339 zero-depth edges in each text tool fit inside one native periodic
sheet. A full-interval homogeneous Bernstein bound places the stored 3D
edges within `1.917e-13` model units of the selected cone. Carrier
transfer and UV rounding add at most `1.494e-14`. The existing pcurves
still fail the `1e-7` sew allowance after affine transfer. One exact
midpoint gives a same-parameter residual of at least `3.2705587e-6`.
A strict refit retained all 3D curves and their parameter ranges. Across
554 cubic segments per sense, the largest certified same-parameter
residual was `9.77024e-8`. The native B-spline reader preserved the
control and knot bits, and adjacent edges shared identical UV endpoints.
Refit and certification took 3.51 s for the boss and 3.62 s for the
pocket in scratch. Direct assembly still needs to certify whole-depth
separation from both rims, seam winding, and any contact with retained
support inside the sewing collar.

### Repeated latency after the exact arithmetic changes

On pinned CPUs, the `b22832a5` release build completed these public
calls in 20 fresh processes per fixture. The connected `O.` run
alternated the old and new binaries on the same CPU. Its old p95 was
1.189 s. Each call still ran the owned test's geometry and history
assertions after the timer stopped.

| Fixture | Median API time | API p95 |
| --- | ---: | ---: |
| Cylinder boss | 0.344 s | 0.360 s |
| Cylinder pocket | 0.347 s | 0.366 s |
| Cone boss | 0.435 s | 0.457 s |
| Connected `O.` | 0.942 s | 0.992 s |

A repeat of connected `O.` on another pinned core gave 1.039 s p95.
After removing redundant exact-ratio operations, a same-core run with
20 alternating samples per revision measured 0.981 s p95 before and
0.906 s after. Median retired instructions fell from 13.76 to 13.56
billion. Both runs retained the full public fixture assertions.

In five alternating standalone text runs, the dyadic changes reduced
median tool time from 6.212 to 4.947 s and retired instructions from
87.6 to 70.7 billion. The work receipt stayed at 3,057,275 arithmetic
operations and 1,017 integration subintervals. The attached boss and
pocket matched the previous B-rep, validation and history bytes.

The opt-in authored planar selection was also compared with legacy V5
in a single binary before the later arithmetic changes. Across five
pairs, the attached boss median fell from 23.049 to 20.515 s and the
pocket from 23.501 to 20.878 s, with effectively flat peak RSS. The
proof receipt records the reduction in authored pair work. The decoded
solid, tool, footprint and histories matched the legacy output.

Twenty fresh processes per V8 text case measured absolute API latency on
separate pinned physical cores. The warm fixture retains the first result and
changes only its depth. It times the edited call.

| 239-curve text fixture | Median | p95 |
| --- | ---: | ---: |
| Cold boss | 18.410 s | 19.782 s |
| Cold pocket | 18.509 s | 19.791 s |
| Warm 2% depth edit | 18.575 s | 21.520 s |

Run the three public-operation fixtures with the release profile. For a
fresh-process p95 sample, select one test by its full function name per run
and keep CPU affinity fixed. `conical_text_attached_api` records the cold
call. `conical_text_depth_edit_api` records the second call of the warm test.
Each process runs material, history and archive assertions.

```sh
cargo test --release --locked -p martensite-tkv3d --test toolkit \
  authored_planar_latency -- --ignored --nocapture --test-threads=1
```

### Projection workspace reuse

The scratch checker shares a bounded number of operation-owned buffers
for the three grids rebuilt per query. Cached projectors retain their
node samples. On the archived boss, two fresh-process samples per
option gave four-worker peak RSS of 234,428 KiB with individual grids
and 112,732 KiB with the pool. Median checker time was 5.692 and 5.736 s,
respectively. All eight serial and parallel checker runs matched the
ordered observations, internal pave state, section geometry and archive.

The full operation already held roughly 286,000 KiB before the boss
checker started. With the pool, four workers still raised the full
operation's peak by 15.7–17.6% across boss, pocket and warm depth edit.
Two workers reduced median cold boss API time from 21.667 to 16.836 s.
Its two peak-RSS samples were 7.47% and 10.34% above their serial
comparators. The approved 10% ceiling therefore still excludes these
worker settings from the feature.

### Direct curved-counter control

A scratch assembly of one frozen curved-counter member passed the
ordinary ordered checker for both senses. It used 54 shared edges
with opposite coedges, retained the original edge tolerances, and
replayed byte-identically after archive decode. Nine labeled material
witnesses per sense agreed with Fuse or Cut. Both ordinary Booleans
removed the zero-depth cap, retained the outer cap and stock end faces,
and produced a remainder and a counter island on the selected stock face.
The direct assembly took about 30 ms once the strict pcurves were ready.
Archive freeze and native validation took about 2.7 s.

For the full fixture, the refitted UV loops recovered the intended 18
regions and two counter disks under the exact V6 planar census. That
census took 2.36–2.41 s across 153,735 pair and self domains and used
zero root bisections. A separate sign proof covered the decoded ruled
side surfaces and terminal caps against the native carrier. Every side
stayed on the proper side of the stock and at least 11.8759 model units
from the stock end planes. The smallest certified normal separation
was `8.468e-14`. A direct publication path needs to bind this separation,
the UV census and the sewing bounds to the decoded shared seam.

### Full direct assembly proof trials

The direct assembler also produced one decoded solid for all 18 text
regions in each sense. Both results matched the independent Fuse or Cut
material witnesses and recovered the two counter islands, 18 removed
zero-depth caps, 18 exposed caps and 339 retained side faces. The solid
replayed byte for byte with 2,057 typed references. Assembly took about
8.5 ms with the strict pcurves already available. Encode, decode and
re-encode took about 36 ms. The full native validator still ran on the
same decoded solid.

The exact UV checker starts from those native oriented loops. It
certifies contact on the residual Bézier pairs, checks the Green sign
of each cycle, and runs exact winding where one loop's control hull
can contain another loop's witness. It matched the dense V6 output.
Across 20 fresh bindings per sense, UV proof p95 was 72.87 ms for the
boss and 71.72 ms for the pocket. The dense comparison on the same
geometry took 2.71 and 2.60 s, respectively.

The combined scratch proof issued UV, carrier and same-parameter sewing
receipts over the actual decoded solid. Each subsequent independent
decode replayed those receipts against exact input bits. This table
reports 20 samples per sense.

| Stage | Boss p95 | Pocket p95 |
| --- | ---: | ---: |
| Issue proof receipts | 1.079 s | 1.168 s |
| Fresh decode, audit and extract | 33.88 ms | 35.81 ms |
| Rebind receipts | 1.58 ms | 1.67 ms |
| Combined | 1.114 s | 1.204 s |

Strict sewing spent 6,166,500 certified operations per sense and kept
at least `2.298e-9` model units of slack under the requested `1e-7`.
Mutations of pcurve bits, seam winding, counter ownership and carrier
identity triggered typed refusals. The native ordered check still took
12.9–13.2 s. Replacing its self-intersection stage requires complete
retained-tool, stock, seam and tolerant contact coverage on the same
frozen body.

### Live seam fitting in the native builder

A private TKSketch builder reconstructed both 239-curve fixtures using
the original public selection and `1e-7` sewing limit. It found
339 shared edges and retained all 554 fitted cubic spans per sense.
The directed binary64 enclosure bounded every retained span over its
full parameter interval. It encloses each arithmetic operation, the
Taylor remainders and the rational curve weight division. For each
sense, all 769 subdivision decisions matched the dyadic proof. High-precision evaluations at 17 parameters per
retained span gave 9,418 additional checks per sense.

| Live direct stage | Boss | Pocket |
| --- | ---: | ---: |
| Fit and certify all seams with dyadic arithmetic | 1.42 s | 1.56 s |
| Fit and certify all seams with directed binary64 | 25.37 ms | 30.46 ms |
| Largest directed sewing upper bound | `9.77030e-8` | `9.77030e-8` |
| Ordinary ordered validation with directed seams | 11.53 s | 11.74 s |

These were separate full runs pinned to physical CPU cores. Both direct
archives and every normalized history record matched the dyadic builder
byte for byte. The directed proof reports its own outward bounds. The
public path still uses the existing Boolean and ordered validator.

The standalone live fitter reproduced all 769 prepared query bit
sequences per sense against the pinned dyadic fitter. Across 20 fresh
processes on one core, its p95 was 69.68 ms for the boss and 68.00 ms
for the pocket, including the original cap proofs. Peak RSS stayed below
6.1 MiB. The fitter refused a changed control, an unrepresentable seam
endpoint and a cap trace whose bound exceeded its retained tolerance.

A separate exact authored-cycle proof took 8.64 ms at p95 over 20 trials.
It recovered all 18 directed region selections and the 20 boundary
orientations from the retained 239 source curves. It checked every logical
pair domain through hull separation or exact residual predicates and
rejected a swapped counter. The comparison against dense selection took
347.6 ms in one run. The timing for the cycle proof excludes native
construction and independent area comparisons.

The retained contact proof must cover stock face interiors as well as
seams. In each sense, an actual interior point was inside the current
face-pair proximity envelope. The stock-carrier pcurve images were outside
the envelope at the same depth. Over the relevant collar, the squared-distance
function for a fixed side parameter is strictly convex in side depth and
stock UV. Jointly reducing depth and moving toward the declared seam
therefore stays within the envelope. If this path first hits a trim
boundary, the bound excludes other loops and the stock rims and limits
the contact to the same edge or a declared adjacent endpoint. A second
enclosure follows that contact to the shared seam. Publication still requires a complete contract for
declared contacts and a comparison with adaptive native classification.

### Single construction and ordered validation

A live V8 construction trial built the 18-member native graph before
intermediate publication. It took 570 ms for the boss and 568 ms for
the pocket in the first paired run. The normal tool calls took 3.51 s
and 3.52 s on the same core. Both live graphs reproduced the
independently qualified canonical and placed archives byte for byte,
with the same 2,088 source keys, typed history and spatial allocation.
The trial covers capture, authored selection, fit, graph construction,
typed history, placement and the attachment handoff. The existing
selection and fit accounted for about 484 ms. The directed cycle proof
has yet to replace that selection step in the live path.

The ordinary validator on the final decoded direct solid spent about
254 ms on topology, 533–543 ms on geometry, 11.27–11.31 s on native
self-intersection and 2.5 ms on tolerance. A faster self-intersection
check alone would still leave approximately 790 ms of ordinary
validation. The final path also needs to reuse the exact incidence and
representation checks across construction and ordered validation.

### Public operation repeat after the arithmetic changes

The release build at `ba5d4076` repeated each public text fixture in
20 fresh processes. The three fixtures ran concurrently on physical
cores 12, 13 and 14. Other activity on the same host varied during the
run, so the values are a separate latency observation from the earlier
20-run sample. Every fixture passed its existing material, history and
validation assertions.

| Public V8 fixture | Median | p95 |
| --- | ---: | ---: |
| Cold text boss | 22.622 s | 23.058 s |
| Cold text pocket | 22.533 s | 23.266 s |
| Warm text depth edit | 21.044 s | 22.971 s |

### Direct graph and full native validation

The exact authored-cycle proof also ran inside the single-pass builder.
In 20 alternating fresh-process pairs on a fixed core per sense, the
placed-graph p95 fell from 719 to 348 ms for the boss and from 781 to
368 ms for the pocket. Every run reproduced both tool archives, the
typed history and the spatial allocation. This timer ends before the
retained-family proof and stock attachment.

The assembled final archive also passed the decoded support-loop
proof. It traced 339 native edge uses through their directed vertex
incidences and recovered 20 loops with 554 cubic pieces. The proof
covered all 153,735 logical pair domains and inspected 1,117 residual
and self domains. Issuance took about 63 ms per sense. A clean decode
and rebind took about 15 ms. Exchanging the counter owners was refused.

The original stock received its own independent proof. Both end
disks, both rims and the periodic generator rebound byte for byte in
the final archives. The original conical band stayed within
`1.009e-14` model units of a closed analytic frustum, under the retained
`1e-7` tolerance. The raw top-rim gap was `6.449e-15`. Five changed
stock inputs were refused.

| Complete direct prototype | Serial p95 | Four-worker p95 |
| --- | ---: | ---: |
| Cold boss | 13.449 s | 6.504 s |
| Cold pocket | 13.955 s | 8.411 s |
| Warm depth edit | | 6.804 s |

The prototype runs all four ordinary validation stages and still uses
the native self-intersection search. The cold measurements use 20
matched pairs per sense. Every final archive, validation report and
history record reproduced across both worker settings. Four workers
added at most 8.83% peak RSS for the boss and 9.86% for the pocket
across the paired samples. The boss workers shared one last-level
cache. The pocket workers spanned both caches. A repeat of 20
four-worker pocket runs on cores sharing one cache measured 5.651 s
at p95 with matching output bytes. The warm p95 uses
20 fresh four-worker processes, each of which validated an initial
solid before timing the depth change. Its peak was 138,120 KiB.
The warm archives and reports matched a separate serial trial, and
every edit preserved the prior archive.

The worker and direct-construction paths are scratch experiments.
The existing public API timing is reported separately. The large
fixtures still exceed the 1-second target.

### Exact tool-family proof before attachment

The direct tool can earn the existing opaque conical self-intersection
certificate without publishing standalone member reports or calculating
their volumes. A scratch path captured the stored span and join inputs,
issued selected cap material through exact directed cycles, and ran the
existing native-family binder and TKMath proof. In three paired tests
per sense, the cap material stage dropped from 503 to 42 ms for the boss
and from 513 to 40 ms for the pocket.

Combining this issuer with the authored-cycle construction path gave the
following p95 measurements from 20 fresh processes per sense.

| Stage | Boss | Pocket |
| --- | ---: | ---: |
| Live tool and tool-family proof | 836 ms | 845 ms |
| Actual span storage and source binding | 112 ms | 110 ms |
| Selected cap material | 42 ms | 40 ms |
| Existing exact family SI issuance | 379 ms | 395 ms |

Each run verified 18 tool members, 339 sides, 36 caps, 40 cap wires and
678 cap edge uses. The mathematical work and archive bytes matched the
ordinary V8 comparison. The independently decoded canonical and placed
archives rebound with their own issued cap material. A changed native
cap pcurve and exchanged side identity both triggered typed refusals.
The time ends before stock attachment and final ordered validation.
The new issuer still needs production policy admission, cumulative work
and refusal receipts. The attachment proof must also cover the final
stock and tool contacts on the same decoded body.

### Live source binding on the attached solid

The hardened cap issuer and the next exact interval arithmetic revision
were measured together in 20 fresh processes per sense. The live tool
and its existing exact family check took 552 ms at p95 for the boss and
557 ms for the pocket. Actual span storage took 68–69 ms, selected cap
material 39–40 ms, and the exact family check 166–167 ms at p95. All
40 runs reproduced the independently constructed tool archives and
authored bindings.

A separate attachment trial used the live tool's own canonical archive
and proved its retained subset on the freshly decoded solid. It
replayed the original stock, all 339 installed seam uses and all 20
changed stock loops. The exact incidence check took about 5.5 ms per
sense. It found a closed shell with 362 faces, 1,020 edges and 680
vertices, and a connected manifold link at each vertex. The stock
loops were issued on that same decoded body. The timed attachment
steps through the loop proof took 160 ms for the boss and 158 ms for
the pocket in one run per sense, after live tool construction and
pre-sew capture.

The ordinary ordered validator then ran on the same body as an
independent comparison. Both senses reproduced the reference
800,459-byte archives, history and validation reports. Contact coverage
and an ordered publication path within one second remain open.

### Checked cold and warm prototypes

The prototype qualified the final topology, literal stored geometry,
contact domains and tolerance order on one decoded archive. Each sense
covered 153,181 trim-piece pairs and 678 directed incident paths. The
ordinary validator accepted the same archives in a separate comparison.

| Fixture | Fresh processes | Median | p95 | Maximum |
| --- | ---: | ---: | ---: | ---: |
| Cold boss, CPU 7 | 20 | 813 ms | 825 ms | 828 ms |
| Cold pocket, CPU 7 | 20 | 818 ms | 843 ms | 1,311 ms |
| Warm depth edit, CPU 7 | 20 | 828 ms | 1,142 ms | 1,256 ms |
| Warm depth edit, CPU 9 | 40 | 759 ms | 841 ms | 960 ms |

The timers start before attachment admission and end after exact
qualification. The warm run preserves the earlier immutable result and
measures a depth change through the same source attachment. The earlier
native report, archive and history matched on the independent warm
comparison. Both timing samples remain part of the assessment. The
application drag and preview goals still require an application run.

### Public certified attachment

The release build measured the public certified frustum entry from the
first admission step through the immutable result. Each fixture ran in
20 fresh processes pinned to CPU 8 on the Linux AMD Ryzen AI Max+ 395
host. The boss, pocket and warm depth edit were interleaved in each round.

| Fixture | Median | p95 | Maximum | Peak process RSS |
| --- | ---: | ---: | ---: | ---: |
| Cold conical text boss | 757 ms | 797 ms | 806 ms | 65.6 MiB |
| Cold conical text pocket | 757 ms | 771 ms | 772 ms | 65.8 MiB |
| Warm conical text depth edit | 747 ms | 779 ms | 817 ms | 69.2 MiB |

Two individual controls on the same release binary took 18.62 s for
the boss and 18.64 s for the pocket through the earlier V8 entry.
Each control passed its native checks.

Each fixture uses 239 authored curves, 18 selected regions and two
counter islands. All samples retained every selected cap and the
expected closed-body census. A separate run passed the ordinary validator
and checked the signed material change for each case. Two independent
processes reproduced each 800,459-byte B-rep, composed history and
validation report byte for byte. In ten fresh stress runs, two concurrent calls on the same input
reproduced the serial boss result and exported independent native owners.

Build the toolkit test binary with the standard release profile.

```sh
cargo test --locked --release -p martensite-tkv3d --test toolkit --no-run
```

Run one of `multiline_cone_certified_boss`,
`multiline_cone_certified_pocket` or
`multiline_cone_certified_depth_edit` per fresh process with its exact test
name and `--include-ignored --nocapture`. The JSON timing ends before
the independent checks. Set `MARTENSITE_CERTIFIED_ATTACHMENT_P95=1` to
omit the extra ordinary validation during repetition. For byte replay,
set `MARTENSITE_CERTIFIED_ATTACHMENT_RECEIPT_DIR` to an existing empty
directory and compare the three files emitted by each process.

ThreadSanitizer's startup control exited with a segmentation fault on the
host. The default Ubuntu container reported an incompatible memory layout
before running the control. The shared-input test ran in the ordinary
release build. The first native desktop measurements are for the
cylindrical rectangle workflow described below.

The offscreen Radeon Vulkan proxy also ran alongside repeated calls to
the public warm edit. At 3840×2560 it missed zero of 400 paced
deadlines. At 60 Hz, p95 input-to-GPU completion was 1.74 ms for drag
and 1.17 ms for the first height preview. At 120 Hz it was 0.91 ms
and 2.02 ms. The proxy supplies synthetic input and ends at GPU completion.


## 2026-11-25 desktop cylindrical rectangle

Oblikovati's native owner loop ran on Linux with an AMD Radeon 8060S
and RADV. The 960×720 test window used Vulkan validation. The monitor
ran at 59.97 Hz. The test injected motion at 60 and 120 Hz and timed
each input through a completed native frame. Physical scanout at
120 Hz still needs a display that supports that rate.

Two release runs each dragged the same corner for 400 frames per input
rate. Drag p95 was 4.78 and 5.16 ms at 60 Hz, and 5.25 and 4.60 ms at
120 Hz. The slower p95 for the first saved-sketch extrusion preview
was 16.53 ms over 30 requests per run. The editor retains the selected
surface chart and updates one viewport buffer during the drag.
The frame loop coalesces captured motion and retains the GUI draw data
while the command queue is idle. A third FIFO run missed the 120 Hz
input budget at 10.38 ms p95. In that run the editor and overlay update
took 0.20 ms p95. Presentation took 14.91 ms p95 in the separate
unpaced stage run. A RADV mailbox diagnostic brought those p95
figures to 4.19 ms for 120 Hz input and 4.05 ms for presentation.

Twenty cold boss commands took 366.31 ms p95 to build, name and commit
the result, and 383.80 ms p95 through the next completed native
frame. Each case began with a fresh part and a saved cylinder
rectangle. The correctness test checked the added volume against the
analytic annular-sector volume and reproduced the result bit for bit
after restoring the part recipe.

Run the native timing from the Oblikovati desktop branch with the
Radeon ICD selected and a mapped Linux display.

```sh
CARGO_PROFILE_RELEASE_DEBUG=0 cargo test --locked --release -p oblikovati-head \
  --lib measure_wrapped_drag_through_native_owner_frames -- --ignored --nocapture --test-threads=1
CARGO_PROFILE_RELEASE_DEBUG=0 cargo test --locked --release -p oblikovati-head \
  --lib measure_exact_wrapped_boss_command_to_native_frame -- --ignored --nocapture --test-threads=1
```

The desktop fixture uses a saved cylinder rectangle and an outward
boss in the Extrude dialog. The ordinary circle extrusion supplying
the cylinder has an indirect native surface frame. The kernel
preserves its angular seam and reflects height into the right-handed
extrusion chart before checking the original trim.

At app revision `7ed030944` with SDK revision `0150ea1f`, the main Linux
window requests mailbox presentation and falls back to FIFO when the
surface lacks mailbox support. Two native runs with the app on cores
separate from the build used the built-in policy and no Mesa mode
override. Each run sampled 400 drags per input rate, 30 first previews
and 120 depth changes at 60 Hz. The slower p95 from the two runs was
5.26 ms at 60 Hz input, 4.52 ms at 120 Hz input, 15.42 ms for the first
preview and 10.03 ms for depth changes. All 1,600 drag samples met
their budget. A separate run of 20 exact commits reached a completed
native frame in 384.49 ms p95.

Drag timings taken during concurrent workspace builds exceeded the
120 Hz budget. The 20-boss run under that load still reached a native
frame in 678.38 ms p95. The desktop measurements cover one cylindrical
rectangle on a part with one body. Native cone, connected sketch group
and text gestures remain outside this fixture.

[Oblikovati #4187](https://github.com/Oblikovati/Oblikovati/issues/4187),
[Martensite #600](https://github.com/Oblikovati/Martensite/issues/600).
