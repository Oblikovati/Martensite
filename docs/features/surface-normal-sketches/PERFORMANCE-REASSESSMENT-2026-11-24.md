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
