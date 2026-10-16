# Planar geometry

`martensite-tkg2d` provides planar points, directions, vectors, transformations,
analytic curves, Bezier and B-spline curves, trims, offsets, evaluation descriptors,
curve adaptors, parameter-grid evaluation and quantized hashes (#105).

## Modules

| Module | Capabilities |
| --- | --- |
| `geom2d` | Shared geometry nodes, analytic and spline curves, mutation and copying. |
| `adaptor2d` | Common curve evaluation interfaces, bounded lines and offsets. |
| `geom2d_adaptor` | Adaptation of geometry curves, parameter ranges and continuity intervals. |
| `geom2d_eval` | Analytic bases, derivative evaluation and parameter-mapping descriptors. |
| `geom2d_grid_eval` | Batched values and derivatives on parameter grids. |
| `geom2d_hash` | Quantized geometry lookup keys and equality predicates. |

Handles share mutable nodes: cloning a handle preserves identity. Copy a geometry
object when subsequent edits must be isolated. Adaptors can retain handles to
geometry and rebuild their local span caches. The additional analytic evaluation
curves reject unsupported reversal and transformation operations with typed
failures. Hashes use a 64-bit native-endian profile; they are lookup keys, not
persistent entity identities or a portable serialization format.

The current arbitrary-order B-spline grid path uses a different span convention
from lower-order grid evaluation. Missing support at the last knot returns a typed
failure. Grid spline evaluation does not apply an attached evaluation descriptor.
These behaviors remain covered by the frozen corpus; callers should select the
evaluation interface appropriate to their derivative and descriptor requirements.

## Verification

Run `cargo test -p martensite-tkg2d`. The suite exercises constructors, derivatives,
transformations, copying, exceptional inputs, spline mutations, adaptors, grids,
hashing and JSON serialization. Independent analytic checks cover 1,320 line and
conic samples across parameters, scales and frame orientations. A separate set
uses line, circle, parabola and quadratic Bernstein closed forms for 17 cases.
Original numeric policies remain unchanged; the supplemental 1,320-sample check
uses an explicit roundoff allowance, not a certified error bound.

The behavior-version-1 corpus retains all 1,787 requests across 25 families.
Tests run directly in Rust with compiled fixtures and request-local buffers.
Finite values, infinities, signed zeros, failure categories, hashes, JSON and
whitespace compare exactly. Only complete numeric words representing NaNs are
compared by classification. These are reviewed frozen expectations, distinct from
the independent mathematical checks. Updating a baseline requires review stating
the reason and behavior version.

This preparation has been executed on Linux x86_64. The combined release candidate
must pass the full supported platform matrix before publication.
