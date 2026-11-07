# Parallel toolkit tests

Issue #250 addresses serial execution of affected toolkit suites. The
[release preparation run](https://github.com/Oblikovati/Martensite/actions/runs/39981721808)
at `2254483e14b35f82a838a316ef5121d6059de658` took 2h 6m. Test execution ran
from 15:59:10 to 17:56:23 UTC on 2026-10-23, about 117 minutes.
The manifests and lockfile changed, so selecting every package was appropriate.

| Package and test binary | Seconds |
| --- | ---: |
| TKGeomBase toolkit | 1917.87 |
| TKPrim dataset cases | 992.40 |
| TKGeomAlgo toolkit | 830.30 |
| TKPrim toolkit | 725.60 |
| TKHLR algo_numeric | 377.34 |
| TKShHealing dataset cases | 335.95 |
| TKTopAlgo toolkit | 298.68 |
| TKPrim gprop_primitives | 287.73 |

The PR matrix uses the existing changed-package and transitive-dependent
selection. Base dependencies compile where needed but their tests run only
when selected. Every selected package gets one job, with four jobs allowed
concurrently. Shared checks run once. The aggregate PR check rejects failed,
cancelled and unexpectedly skipped jobs.

Release rehearsals and tag runs use the previous final release as their scope
baseline. Each of the four native platforms receives up to four test shards.
The planner puts TKGeomBase, TKPrim, TKGeomAlgo and TKTopAlgo in separate shards
when selected, then distributes other selected packages in sorted order. This
is a scheduling heuristic based on the long suites observed during #250;
it does not change tests or predict equal shard durations. Every selected
package runs once per platform, including doctests. Empty scopes produce no
test jobs. Shards recompute selection before running and reject invalid indexes
or unsupported targets.

The release check jobs run lint, release builds, packaging verification and
compiler inventory alongside the test matrix. Both matrices must succeed for
the aggregate release result to pass and for publication to run. Local release
commands retain the full sequential pipeline. The native setup action is shared
by release checks and tests so Windows and macOS shards receive FreeType.

The motivating [tag run](https://github.com/Oblikovati/Martensite/actions/runs/40091884019)
at `b7b3e962` had run for about 56 minutes at inspection, with all four targets
inside the sequential release command. Setup took under two minutes. Its test
matrix was skipped because the original split applied only to PRs. Running job
logs were unavailable through the API, so this observation does not establish
how much of that elapsed time was compilation, testing or packaging.

Jobs compile independently; release sharding caps this at four test compilations
per platform, plus the shared checks. This can increase runner minutes even
when elapsed time falls. Record the first broad hosted run's wall time and sum
of job durations before choosing a cache or artifact-transfer strategy. A
toolkit matrix alone also leaves the longest individual toolkit as a lower
bound on completion time. These baseline durations do not predict a measured
speedup on a different runner or revision.

## Remaining investigation

The first local sphere run on `c04ee035` passed in 141.04 seconds of test
execution (42.05 seconds of compilation excluded). A standalone repeat passed
in 150.79 seconds wall time, 150.77 seconds user CPU, 0.00 seconds system CPU,
and 14692 KiB peak RSS. The certified-offset reference test passed in 334.75
seconds, with compilation excluded. These were debug builds on a shared Linux
host, with concurrent investigations, so they are not CI speedup estimates.

Five debugger samples per workload, taken roughly four seconds apart near
startup, located these paths:

| Workload | Sampled work |
| --- | --- |
| Sphere volume | Four samples included `metallic` trigonometry or polynomial arithmetic beneath surface evaluation and Gauss-Kronrod volume integration |
| Polar projection | All five included `GenExtPS`: spline evaluation, grid edge/face distances, `ParamGrid` copying or candidate minima search |
| Certified offsets | All five included `num-bigint` division, multiplication or shifts in exact arithmetic |

These short samples identify candidates, not percentages of total runtime.
The polar-family timing probe was stopped after 340 CPU seconds once samples
were collected; that is a lower bound, not a passing result or full duration.
The separate debugger runs were stopped after sampling. The temporary debugger
and sampling script were kept outside the repository and removed afterwards.

Follow-up #252 covers arithmetic dependency build profiles and their correctness
gates. Follow-up #253 covers repeated extrema-grid work. The archived #226 logs identify the long phase as TKShHealing dataset
execution; the 1,733-test TKPrim toolkit completed in seconds (see below). None of these findings authorizes weaker oracles,
looser tolerances or persistent caches on mutable geometry.

Temporary Cargo command-line overrides tested one dependency at a time, without
changing the repository's profiles:

| Existing test | Baseline seconds | Dependency at opt-level 3 | Trial seconds |
| --- | ---: | --- | ---: |
| `volume_properties_sphere` | 150.80 | `metallic` | 121.38 |
| `certified_offset_contacts_match_every_reference_point` | 334.75 | `num-bigint` | 43.77 |

Both trials passed their existing assertions. The baseline sphere row uses
libtest's reported duration; `/usr/bin/time` reported 150.79 seconds. Compilation
is excluded from these rows: the trials required 42.74 and 54.17 seconds
respectively, with the latter including a build-directory lock wait. Kernel
code retained its debug profile. No dependency version or implementation
changed. These single-run results on a shared host support investigating
`num-bigint` first; they do not establish full-corpus or cross-platform
equivalence. The overrides were supplied with
`--config 'profile.dev.package.num-bigint.opt-level=3'` or
`--config 'profile.dev.package.metallic.opt-level=3'` and were not persisted.

TKGeomBase's warnings name twelve corpus-family tests, including
`pl_polar_complete_family` (164 cases), `pl_projected_complete_family`
(130 cases), and `gpl_curve2d_complete_family` (295 cases). The replay helper
scans fixture rows and executes each matching request directly in the test
process. These cases do not use the isolated executable-fingerprint harness.
Profile one family before splitting it: finer test scheduling may reduce tail
latency, but cannot remove computation. Keep the existing corpus counts and
fixture hashes if individual cases are exposed to the test scheduler.

TKGeomAlgo's four warnings name certified curve and offset contact checks.
The offset corpus has 108 cases. Two tests independently call `certify_case`
on that corpus, comparing with frozen reference contacts and a mapped Rust
intersector respectively. Both oracles must remain. Measure certification
against the intersector before considering a single per-case test that checks
both oracles using one operation-local result.

TKPrim's dataset warning names `bugs_modalg_7::bug28402`, which computes volume
properties for a restored assembly and compares its elapsed time with a box.
Its other warnings name sphere-volume and center-of-mass tests. These exercise
surface integration and should be profiled separately from executable hashing.
Do not replace them with analytic sphere shortcuts or loosen their tolerances.

Reproduce the sphere calculation without the dataset subprocess harness:

```sh
cargo test --locked -p martensite-tkprim --test gprop_primitives volume_properties_sphere -- --exact --nocapture
```

Measure the already-built test executable separately from Cargo compilation.
Compare wall time, user/system CPU and peak memory; use stack sampling to locate
the numerical cost before changing the production implementation.

## Windows checkout regressions

The first merged release rehearsal exposed two line-ending failures (#269).
The workflow contract test now checks LF and CRLF text. The healing resources
`MDTV` and `data` retain their recorded bytes through Git attributes; their
expected SHA-256 values are unchanged. The build-driver checkout regression
uses a temporary Git repository with `core.autocrlf=true` to verify both files.
Converting `data` to CRLF reproduced the exact unexpected digest from Windows.

## Stable pull-request comparison

Every PR job reads `MARTENSITE_CI_BASE_SHA` from the event's
`pull_request.base.sha`. Planning, checks and package jobs use that immutable
commit, even if `main` advances while jobs are queued (#273). Missing or malformed
PR base SHAs fail before selection; the driver never falls back to a moving
branch. Local `cargo xtask pr --base` and manual PR dispatch retain their explicit
or default comparison refs. Release selection remains relative to release tags.

The `martensite-vulkan` suite needs the pinned Mesa lavapipe and validation layer, which
`.github/actions/setup-renderer-tests` and `install-linux.sh` provide on Linux only. Its
tests run on the Linux target. The other platforms still build and lint the package.

## Hosted compilation and runner cost

The successful [four-platform release rehearsal](https://github.com/Oblikovati/Martensite/actions/runs/42235311548)
on 2026-11-06 at `6fd6791eedb0b40e88c5810b090afa8f71c0d6c0` provides a baseline for the current shard layout. Its 25 job records contain 23 successful jobs and two
intentionally skipped jobs. From the first job start to the last completion it
took 120.68 minutes. The sum of recorded job durations was 851.27 minutes. These durations measure runner occupancy. CPU time, queue time and platform
billing multipliers are outside this measurement.

The longest test jobs were:

| Job | Whole job, minutes | Cargo test compilation | First test to last test/doctest result, minutes |
| --- | ---: | ---: | ---: |
| Intel macOS shard 0 | 114.97 | 42m 16s | 70.35 |
| Intel macOS shard 2 | 99.95 | 32m 56s | 63.37 |
| Linux x86_64 shard 0 | 94.00 | 26m 10s | 64.68 |

Cargo reports each compilation interval in its `Finished test profile` line.
The execution interval uses log timestamps and includes test startup, subprocess
work and doctests; it is not summed per-case CPU time. Other job time includes
checkout, setup, the build driver and cleanup. GitHub exposes the combined
compilation and tests as one `cargo xtask ci test` step, so step durations alone
cannot separate them.

Every one of the 16 native test shards logged a compilation of
`martensite-tkernel`, as did each platform's check job. This confirms duplicated
compilation across jobs. Some driver tests also deliberately compile temporary
SDK fixtures; counting every compilation line as avoidable duplication would
misclassify those checks.

The earlier single-platform PR-mode preparation run `39981721808` occupied
125.47 summed job-minutes and 125.52 minutes from first start to last completion,
including about 117 minutes of test execution. It used a different revision,
package scope and test profile. The later run includes four native platforms,
ThreadSanitizer and newly added toolkits. Comparing these totals does not
establish either a speedup or a cost regression from sharding.

The later critical path has also changed. On Intel macOS shard 0, the Boolean
script dataset took 2558.89 seconds, STEP's dataset took 698.12 seconds and the
TKGeomBase toolkit took 332.85 seconds. Intel macOS shard 2 spent 1609.79 seconds
in TKBO library tests and 1262.89 seconds in the offset dataset. These elapsed
measurements justify revisiting shard balance and compilation reuse with a
matched revision and package scope; they do not justify removing cases or
changing numerical behavior.

This records the parallel scheduling, failure propagation and cost measurements
requested by #250. Issue #253 still tracks repeated extrema-grid computation.
The measurements do not show that the current schedule minimizes runner cost.

## Corrected attribution of the 1,733-test phase (#226)

The [first attempt](https://github.com/Oblikovati/Martensite/actions/runs/39790837617/job/119866403018)
contains the reported outliner result: 259 tests passed in 12.71 seconds. Its
1,733-test TKPrim toolkit then passed in 6.52 seconds, from 14:10:48 to 14:10:54
UTC. The job was later cancelled. The old issue comment interpreted a pending
API step status as evidence that Cargo had not started; the completed log
contradicts that interpretation.

The [successful retry](https://github.com/Oblikovati/Martensite/actions/runs/39790837617/job/119916479681)
at the same commit, `1e2bb1e`, passed TKPrim in 9.54 seconds. Its 927-case
TKShHealing dataset took 8767.44 seconds, from 15:52:26 to 18:18:33 UTC.
The largest gaps between nonempty log entries were 39.82 seconds in the first
attempt and 40.77 seconds in the retry. Both logged case completions and
long-running warnings, including `bug27015`, `bug30396_2` and `bug333`.

Issue #228 independently reproduced repeated hashing of a 315 MiB test
executable. Commit `f7775a284` optimized that hashing while retaining all three
fingerprint checks; [its measurements](fingerprint-cost.md) establish that
particular cost. They do not attribute the entire healing runtime to hashing.
The current case runner retains CPU and elapsed-time limits and a five-second
watchdog. The recovered logs establish a live, progressing workload rather
than a stalled 1,733-test suite, so #226 does not justify another heartbeat.
