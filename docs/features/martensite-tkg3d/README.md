# Spatial geometry

`martensite-tkg3d` provides spatial curves and surfaces, geometry evaluation,
adaptors, parameter grids, geometry hashes and one-parameter approximation (#192).

| Module | Capabilities |
| --- | --- |
| `geom` | Analytic and spline curves/surfaces, transforms, trims, offsets and copying. |
| `adaptor3d` | Curve/surface interfaces, restricted curves and sampling tools. |
| `geom_adaptor` | Geometry adaptation, parameter bounds, continuity and span caches. |
| `geom_eval` | Analytic and polynomial evaluation descriptors and parameter maps. |
| `geom_grid_eval` | Batched curve/surface values and derivatives. |
| `geom_hash` | Quantized geometry lookup keys and equality predicates. |
| `adv_approx` | Shared-knot B-spline approximation of one-parameter functions. |
| `top_abs` | Shape categories, orientation composition and formatted labels. |

## Ownership and evaluation boundaries

Handles share mutable geometry: cloning a handle retains identity. Copy geometry
when later edits must be isolated. Adaptor value copies and shallow copies have
distinct contracts; nested geometry and spline caches can remain shared. Existing
copy/cache tests preserve those distinctions. Neither `Clone` nor a shared handle
promises input isolation or an immutable modelling result.

Geometry and adaptor traits inherit their shared-handle thread bounds. Evaluation
callbacks keep caller-owned mutable state; a callback and its borrowed inputs
must satisfy the caller's thread requirements before moving between threads.
Borrowed evaluation contexts cannot be reused concurrently. These low-level
interfaces do not perform modelling validation or return certified completion.

Some analytic evaluation types reject reversal or transformation with typed
failures. Error categories, descriptor identities and structured dump labels are
stable behavior-version-1 interfaces. Their spelling is retained for replay and
consumer diagnostics. Failures describe evaluation; an enclosing modelling
operation must attach entity, operation and validation context.

Hashing uses a native-endian 64-bit profile. Hashes are lookup keys, not persistent
entity identities or a portable serialization format. Spline grids and descriptor
application retain their documented evaluation-path differences. Callers should
select the evaluation interface appropriate to their derivative requirements.

## Verification

Run `cargo test -p martensite-tkg3d`. Tests cover construction, derivatives,
transformations, copying, mutations, adaptors, exceptional inputs, geometry
hashing and serialization. Independent Bernstein and quotient-rule calculations
check rational curves; bilinear surface and transform formulas check values and
mixed derivatives. Additional controls cover axis placement, cache identity,
request isolation and exact comparison policies.

The behavior-version-1 corpus contains all 4,695 request/result pairs across
71 families. Compressed fixtures are compiled into the test program and decoded
through a small owned interface over the pure Rust gzip backend. Each request
uses an explicit output buffer and parameter context. No interpreter, external
geometry engine, subprocess or fixture-file read supplies those test results.

Finite values, signed zeros, infinities, failure labels, hashes, JSON and
whitespace compare exactly. Complete decimal NaN-bit tokens compare by
classification, including the established word-boundary policy. The fixture
protocol is ASCII. These frozen expectations are reviewed baselines, distinct
from independently derived mathematical results. Baseline changes require review
stating the reason and behavior version.

Renderer, resource-policy and higher-algorithm integration tests remain with their
actual integration hosts; the standalone crate does not acquire their upward
dependencies. Product acceptance and the final supported platform matrix are
recorded separately. No performance, certified error-bound, immutable result or
new concurrency guarantee is implied by fixture retention.
