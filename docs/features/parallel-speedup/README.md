# Parallel speedup qualification — #480

The 0.13.1 rehearsal at `fa9e6e5979c2423240b356130ec89cd216a2ab55` failed `bugs_modalg_5::bug24639` on macOS ARM. It printed serial/parallel times of 0.26/0.21 seconds. The helper returns the displayed centiseconds, so the test rejected 0.21 > 0.8 × 0.26. The original raw times are unavailable; these printed values cannot establish whether the raw ratio exceeded 0.8.

The same case passed the v0.13.0 rehearsal at `d051ff699982faba4ec2df3ee9a47e14f38dedbd`, run 42334254227, ARM job 128314679019. Both jobs used macOS 26.7.2, image 20261102.0473.1 and runner 2.338.2. The case, timing helper and OSD parallel sources were unchanged. Successful historical output contains no timings, so it does not establish a performance delta.

## Bounded ARM diagnostic

Run 43709024633 at disposable commit `7daee52c8c45c4fba991f10ba78355b8255e31fe` retained the original rounded guard and added raw timing output. It used three physical/logical CPUs and 7,516,192,768 bytes of RAM. All three isolated controls and the ordinary family harness passed:

| Control | Serial seconds | Parallel seconds | Raw ratio |
| --- | ---: | ---: | ---: |
| Isolated 1 | 0.210195 | 0.086838 | 0.4131 |
| Isolated 2 | 0.189654 | 0.092797 | 0.4893 |
| Isolated 3 | 0.178340 | 0.113427 | 0.6360 |
| Family harness | 0.397320 | 0.260390 | 0.6554 |

The family harness passed 58 tests and ignored its existing seven optimized-Linux cases in 121.64 seconds. The times show substantial variation under harness load. They do not reproduce the failing ratio or prove that contention caused it. This diagnostic is not release validation, a benchmark against the previous release, or proof of universal performance parity. The log's resource fields are cumulative child usage, including compilation; they are not operation peak-memory measurements. Raw streams, context and the complete compressed log are retained in `references/`.

Three unchanged Linux controls also passed at 0.14/0.03, 0.15/0.03 and 0.14/0.03 seconds on 32 online CPUs during driver compilation. These are mechanism checks, not ARM qualification.

## Correction and scheduling

Only this timing case moves into the `parallel_speedup` integration-test binary. Its test identity, 75-edge input, argument order, process budget and 20% speedup requirement remain unchanged. Its private timer returns raw seconds while preserving the displayed `Tps` lines and reporting both raw measurements. `ScriptBop::bfillds_timed` keeps its public, rounded return contract.

The new binary contains one test. Precision controls run in the ordinary dataset binary. Local/release Cargo runs test executables sequentially; the xtask optimization groups are synchronous and `ProcessRunner` waits for each command. Split PR parts iterate targets and filter chunks synchronously; stable test ownership selects this test in exactly one part. Hosted jobs use separate VMs. This prevents overlap with ordinary cases within the job; it cannot remove provider or unrelated local-host load. The existing machine gate alone is insufficient because Linux heap controls spawn Boolean workers outside it.

Independent precision controls cover exact threshold equality, its next representable float, raw timer bit preservation, and centisecond rounding that can either falsely reject (0.264/0.207) or falsely accept (0.254/0.204) the same 0.8 bound. A failed bound still emits the original fatal line and uses the existing whole-stream failure contract. No geometry algorithm, package version, tolerance or search budget changes.
