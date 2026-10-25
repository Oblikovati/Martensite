# B-spline span-cache allocation costs

[Bug #309](https://github.com/Oblikovati/Martensite/issues/309) concerns sequential
evaluation of large splines. Each cache miss copied the complete pole and expanded
knot arrays before rebuilding one local polynomial. Walking through N distinct
spans therefore produced quadratic array-copy traffic.

Both planar and spatial adaptors now borrow their existing input arrays through
one private curve-state read lease. The cache constructors, coefficient builders
and evaluator arithmetic keep their previous call order. Public array getters
still return detached copies. The cache owns its coefficients after the input
lease ends, including after the caller edits or retires the curve.

The read lease surrounds internal cache construction only. Its callers pass
metadata from the same guard and do not re-enter geometry methods or dispatch
application callbacks. The spatial path retains its existing cache owner and
mutex. Cache validity, clone ownership and the existing behavior of already-built
polynomials across curve edits are unchanged. No public interface or numerical
behavior version is introduced.

## Owned controls

The cost fixture is a generated degree-one line: N+1 poles `(i,0,0)`, knots
`0..N`, endpoint multiplicities two and interior multiplicities one. The midpoint
of each span has exact point `(u,0,0)`, first derivative `(1,0,0)` and zero higher
derivatives. Both cost modes construct the same adaptor and write every D0..D3
coordinate as an unchanged little-endian binary64 value. The closed-form control
isolates construction and output costs from adaptor evaluation.

Separate controls compare rational derivatives with independently differentiated
quotient equations, and periodic diamond segments with their linear equations.
They include both coordinate domains and first, last and interior boundaries.
Their floating comparisons use a fixed absolute tolerance of `1e-11`; raw-bit
compatibility is checked separately. Private tests inspect actual array addresses
and read-lease scope, detached public copies, owned coefficients, edits, owner
retirement, refusal and unwinding.

```sh
cargo test -p martensite-tkg2d -p martensite-tkg3d --lib cache_inputs_tests
cargo test -p martensite-tkg3d --example bspline_span_cost
cargo run --release -p martensite-tkg3d --example bspline_span_cost -- cached 2048 > cached.bits
cargo run --release -p martensite-tkg3d --example bspline_span_cost -- closed 2048 > closed.bits
```

## Allocation measurements

The Linux x86_64 measurements use Rust 1.99.0 and the workspace's ordinary release
profile. GNU libc's installed allocation profiler observes each executable;
GNU `time` records peak RSS. Construction, output, profiler and process-startup
costs remain visible. Subtracting the identical-construction closed-form control
gives these evaluation-related totals:

| Spans | Before: extra allocated bytes | After: extra allocated bytes | Before / after allocation calls |
| ---: | ---: | ---: | ---: |
| 512 | 9,045,096 | 631,912 | 2,049 / 1,025 |
| 2,048 | 136,840,296 | 2,524,264 | 8,193 / 4,097 |
| 8,192 | 2,157,970,536 | 10,093,672 | 32,769 / 16,385 |
| 32,768 | 34,401,682,536 | 40,371,304 | 131,073 / 65,537 |

Each size was repeated in two processes. All complete output bitstreams agreed
with the corresponding baseline. Remaining allocation traffic grows linearly
and includes the existing local coefficient work buffers.

At 32,768 spans, the observed evaluation wall times were 0.48066/0.46433 seconds
before and 0.03138/0.03222 seconds after. Whole-process peak RSS was 9,428/9,632 KiB
before and 8,912/8,540 KiB after. These are profiler observations, with output I/O
included; no CI timing threshold is inferred from them.

The removed allocation total, 34,361,311,232 bytes at the largest size, equals the
derived payload of repeated complete pole and flat-knot copies. This is an
allocation measurement and a source-derived copy account, not an instrumented
`memcpy` count. Lock acquisition counts are likewise not instrumented: the code
replaces repeated input getter leases with one scoped read lease, and the tests
check its lifetime and exclusion behavior.
