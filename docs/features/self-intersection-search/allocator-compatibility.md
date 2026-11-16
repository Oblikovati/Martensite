# Walking-line allocator compatibility (#496)

Bug #507 tracks the allocator and signed-count regressions introduced by PR #502.
The broader performance target remains in #496.

`IntSurfLineOn2S::new(None)` uses contiguous points. An explicit allocator keeps
the original `Sequence` storage, including allocator object ownership, virtual
policy dispatch, node release, reset exclusion and split-tail handling. Box
rebuilds dispatch once per traversal and use slice iteration on the default path.
The public methods and index errors are unchanged.
Point counts retain `Sequence::length`'s signed cast, including wrapping beyond
`i32::MAX`. Scalar tests cover zero, `i32::MAX`, the next count and `usize::MAX`
without allocating large lines.

The original PR #502 head, `28292f63fb16ed25aae45fabab01cb403d36119d`, discarded
an explicit allocator. A live probe observed no allocation in a supplied
incremental arena and allowed its reset while a line point remained alive.
The original allocator-backed sequence allocated one arena block and refused
that reset with `Standard_ProgramError`.

Five regression controls cover allocator handle ownership, incremental allocation
and reset exclusion, default split tails, accumulating allocation/release counts,
and edit/order/box parity on both storage paths. Three ownership/allocation
controls failed against the unpatched PR's existing release artifacts. The edit
parity control passed there. The accumulating control was added afterward.

A disposable Linux x86_64 release microbenchmark compared the unpatched PR's
existing release library with the corrected line module compiled at optimization
level 3 against the same dependencies. Each sample constructed 20 default lines,
appended 20,000 points per line, then read every point by rank. After one warm-up,
nine pairs alternated execution order. Median time was 8,570 microseconds for
the unpatched vector and 8,063 microseconds for the compatibility implementation.
The source and binaries were removed after measurement. These shared-machine
measurements cover point storage only; they do not qualify whole-body time or
peak memory. The holed body-check comparison below uses the complete native validator.

Focused verification uses:

```sh
cargo test --locked -p martensite-tkgeomalgo --test toolkit int_surf_behavior
cargo test --locked -p martensite-tkgeomalgo --test toolkit
cargo clippy --locked -p martensite-tkgeomalgo --lib --test toolkit -- -D warnings
```


## Holed body check against prior main storage

A release-build comparison on Linux x86_64 used Rust 1.99.0 and the same immutable
feature dependency graph, frozen holed conical input, strict stored-geometry
reader and serial native checker. The baseline used the allocator-backed line
from main at `bef25f47`; the correction used the runtime sources at `5d09d8bb`.
Only walking-line storage differed. Loading was outside the body wall timer;
process CPU and peak RSS include loading. Both variants had one warm-up.
The subsequent count correction preserves the measured workload's conversion:
its point counts fit in `i32`, where the tested conversion and signed cast agree.
The measurements belong to `5d09d8bb`; they were not rerun after that correction.

Five alternating pairs were attempted. Continuous one-second `vmstat` samples
required at least 65% idle CPU, at most 10% I/O wait, at most 8 runnable tasks and
no swap-out. The first pair was discarded for swap activity and excess runnable
tasks. The other four pairs met those conditions.

| Median of four accepted pairs | Prior main | Corrected |
| --- | ---: | ---: |
| Native body wall time | 6.08414 s | 5.44437 s |
| Process user CPU | 6.065 s | 5.430 s |
| Process peak RSS | 54,646 KiB | 50,930 KiB |

Median system CPU was 0.01 s in both variants. Wall and user CPU decreased by
about 10.5%; peak RSS decreased by about 6.8%. Every run passed native validation
and retained decoded-geometry fingerprint `c18ae14299948770` and report fingerprint
`db1c30e775788fa9`. The report remained 540 characters. The
[measurement record](references/502-body-benchmark.json) retains the input hash,
reader policies, resolution, accepted and discarded pairs, and their resource
samples. These results cover this feature workload in release mode; they make
no debug/P1 or glyph-scale performance claim. Disposable implementations and
binaries were removed after measurement.
