# Parallel self-intersection check on one shared context (#496)

PR #503 made the body self-intersection check run its tasks in parallel and capped
the workers at eight. It was merged into its stacked base branch,
`fix/496-line-on-2s-storage`, twelve seconds after that base had merged into
`main`. Its commits `4b013360` and `e16ee94a` never reached `main` or
`release/next`. #577 restored them and fixed the memory cost of one check. Across
a long operation the parallel check still raised peak memory until free heap pages
were released before each parallel check (#580); see the last section.

## What went wrong with one context per worker

The context-task executor gave each worker its own `IntTools` context, as the
native design does. A context caches per-face services: 2D classifiers, surface
adaptors, localization grids, bounding boxes, hatchers and point projectors.
Workers take tasks one at a time from a shared queue, so each worker touches
almost every face and builds almost every cache entry again.

The fixture is the decoded 239-curve conical text boss of #419 (349 tool faces
on a conical support), checked by `validate_body` in a release build pinned to
eight physical cores. The validation report was identical in every run.

| Run | Wall (s) | CPU (s) | Peak RSS (KiB) |
| --- | ---: | ---: | ---: |
| Serial | 16.0 | 15.9 | 242,580 |
| 8 workers, context per worker | 8.5 | 38.9 | 813,456 |
| 8 workers, shared context | 7.1 | 26.8 | 267,140 |

In the debug build the same sweep showed memory growing by 85 to 170 MB for each
added worker: 255 MB serial, 422 MB with two workers, 626 MB with four and
863 MB with eight. `MALLOC_ARENA_MAX=2` lowered the eight-worker peak only to
745 MB and did not change the time, so allocator arenas were a small part.

## The change

- Every worker uses the caller's context. Its maps and services are already
  behind `SharedState` locks, so one context serves all workers and each service
  is built once. The per-worker slots and the context factories are removed.
- A cache lookup takes the read lock. A missing service is built outside the map
  lock and inserted under the write lock, unless another worker inserted it
  first; every caller then shares the first inserted service. Before, the build
  ran under the write lock, so one slow build stalled every lookup in that cache.
- When the self-check runs in parallel, it uses at most four workers. `Options`
  carry the bound to every pave-filler and self-check stage, so it is part of the
  operation, not a global setting. Boolean operations keep one worker per task,
  as before. Since #580 the check releases free heap pages before it starts its
  workers.

Parallel mode stays off by default. Only callers that enable it are affected: the
self-intersection check and the Boolean case runner.

#503 had capped every parallel stage at eight workers. On the Boolean workloads
below, run on all 32 logical CPUs, that cap made the parallel pave filler of
`bugs_modalg_5::bug24639` 39% slower (0.141 s against a 0.102 s median on `main`),
so a global cap is not used.

## Results

Self-check of the same fixture, release build, pinned to eight cores, two
alternating runs per row (median shown):

| Workers | Wall (s) | CPU (s) | Peak RSS (KiB) |
| ---: | ---: | ---: | ---: |
| 1 | 15.23 | 15.16 | 242,160 |
| 4 | 7.26 | 17.14 | 253,384 |
| 8 | 7.02 | 25.78 | 269,314 |

Four workers cut the wall time by 52% for 13% more CPU and 4.6% more peak
memory, which is inside the approved tradeoff (at least 50% faster for at most
10% more memory). Eight workers save another 3% of wall time for 50% more CPU
and 11% more memory. Unpinned on 32 logical CPUs, four workers took 7.50 s
(+3.4% memory), eight 6.25 s (+9.8%) and 32 took 5.67 s at +44% memory. In the
debug build four workers took 32.0 s against 65.3 s serial, with 5.5% more peak
memory.

Each worker adds about 3 MB here. With `MALLOC_ARENA_MAX=1` the 32-worker peak
fell from 349 MB to 288 MB, so allocator arenas are about half of it; the rest is
data of tasks in flight. The single-arena run spent 316 s of system time in
allocator locks, which shows how often the intersection code allocates.

The Boolean case runner's parallel workloads, debug builds, measured in the order
`main`, branch, branch, `main`:

| Workload | CPUs | `main` | This change |
| --- | --- | ---: | ---: |
| `bug24639` parallel pave filler (median of 6) | 8 pinned | 0.147 s | 0.140 s |
| `perf_modalg::bug25788` wall | 8 pinned | 88.0 s, 95.6 s | 87.2 s, 87.7 s |
| `perf_modalg::bug25788` peak RSS | 8 pinned | 3.79 GB | 3.79 GB |

The branch rows are from the build without a global cap; the final code differs
from it only by the operation-scoped bound, which Boolean operations do not set.

In one instrumented serial run, edge–face and face–face intersection took 5.8 s
and 7.2 s of 16.1 s (81%). With four workers those two stages took 2.6 s and
2.9 s. The other stages took about 3 s together, mostly the topology and
geometry stages (1.3 s) and pave-block construction (0.8 s).

## Evidence and limits

- The serial, per-worker and shared runs produced the same report hash. On the
  feature branch, the complete attached-text call with a parallel Fuse, the
  shared context and the parallel check produced an archive identical byte for
  byte to the serial archive.
- `parallel_and_serial_checks_report_identical_findings` pins equal findings.
  The context-task tests now pin that the caller and every worker use the
  caller's context and its cached services. A cache test fails against the
  previous lookup, because a slow build then blocks a lookup of another shape.
- The `shared-context` ThreadSanitizer control runs the context-task, cache and
  self-check tests in CI. On the development host (Linux 7.2) ThreadSanitizer
  crashes before `main`, including the driver's own startup control, so the
  hosted job is the sanitizer evidence.
- Face–face tasks still create a context per task, as the native code does.
  Sharing it would also save repeated classifier builds in serial mode; that
  needs its own measurement.
- The timings come from one fixture on a host with other work running. Repeated
  runs of one configuration varied by up to 4%.

## Memory across many checks (#580)

A single check stayed inside the memory tradeoff. The complete attached cone-text
test, which runs about 19 checks (18 tool members and the final solid), did not.
Debug build, CI build settings, all CPUs:

| Build | API time | Peak RSS (KiB) |
| --- | ---: | ---: |
| before #577 (`5161ffc6`) | 98.6 s, 95.3 s | 308,648, 309,896 |
| parallel check | 60.5 s, 62.0 s | 469,968, 473,808 |
| parallel check, `MALLOC_ARENA_MAX=2` | 59.3 s | 419,584 |
| parallel check, `MALLOC_ARENA_MAX=1` | 59.9 s | 316,156 |

With one allocator arena the time is the same and the peak returns to the
baseline, so the growth is freed memory, not live data. The serial work before a
check leaves free pages in the main glibc arena; a serial check reuses them, but
the parallel workers allocate in their own arenas, so their working set adds to
the peak. An isolated check showed only +5.5% because nothing large had been freed
before it.

The check now calls `HeapRelease::release_free_pages` once before it starts its
workers. `NativeHeapRelease` calls glibc's `malloc_trim(0)` on Linux and does
nothing on other platforms, whose allocators were not measured. The call acts on
the whole process heap, including the host application's memory. Matched runs of
the same test, alternating with the baseline under the same host load:

| Build | API time | Peak RSS (KiB) |
| --- | ---: | ---: |
| before #577 (`5161ffc6`) | 109.3 s, 103.7 s | 310,408, 311,864 |
| parallel check with heap release | 65.9 s, 65.8 s | 314,816, 315,848 |

A throwaway preload library that called `malloc_trim(0)` every 100 ms gave the
same result (60.7 s, 309,928 KiB), so the remaining 1.3% is within the spread of
two runs or is the cost of releasing only before each check.
