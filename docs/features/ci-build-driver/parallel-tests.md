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
cancelled and unexpectedly skipped jobs. Release validation remains unchanged.

Jobs currently compile independently. This can increase runner minutes even
when elapsed time falls. Record the first broad hosted run's wall time and sum
of job durations before choosing a cache or artifact-transfer strategy. A
toolkit matrix alone also leaves the longest individual toolkit as a lower
bound on completion time. These baseline durations do not predict a measured
speedup on a different runner or revision.

## Remaining investigation

The first local sphere run on `c04ee035` passed in 141.04 seconds of test
execution (42.05 seconds of compilation excluded). The standalone repeat and
the polar-projection and certified-offset reproductions each consumed about
one CPU core while running. These observations establish substantial work
inside the test processes; they do not identify a particular numerical inner
loop. No profiler is installed in this checkout's environment.

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
