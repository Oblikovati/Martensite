# Oblique cylindrical publication qualification

This increment connects the frame and point-domain certificates from
`1db0f3fe` to wrapped-profile construction. The operation contract and native
representation limits are recorded in [OBLIQUE-FRAMES.md](OBLIQUE-FRAMES.md).
The full P5 qualification and cylindrical/conical sketch capability remain open.

## Owned inputs and independent expectations

The existing P4 mixed-profile fixture contains a polynomial outer loop with one
hole, plus a disconnected rectangular material region. Their developed areas
are `141/40` and `1`. For radius `10` and physical depths `[-1,2]`, the normal
volume factor is `(12^2-9^2)/20 = 63/20`. The independent authored volumes are
therefore `8883/800 = 11.10375` and `63/20 = 3.15`.

The physical origin is the exact dyadic vector `(1.25,-2.5,3.75)`. Requested axis
`(1,2,3)` and radial vector `(3,-1,2)` define the independent frame columns

```
x = (5,-4,1) / sqrt(42)
y = (1,1,-1) / sqrt(3)
z = (1,2,3) / sqrt(14).
```

Outer and hole vertices are checked against
`origin + (10+depth)*(cos(s/10)*x + sin(s/10)*y) + height*z`, and the difference
between depth endpoints is checked against the corresponding radial normal.
These closed-form point controls use the requested `1e-5` spatial limit. The
certified error claim comes from the outward arithmetic certificates and
decoded correspondence, rather than from those sampled point comparisons.

Inward normal sense with requested depths `[-2,1]` produces the same physical
graph as outward sense with `[-1,2]`, while exchanging requested first/last
depth roles. The independent authored-volume expectations remain unchanged.

## Evidence boundaries

The physical datum must survive checked decoding with its original matrix and
translation bits. Every semantic reference must share that datum. Stripping
only this location must reproduce the complete canonical graph byte for byte.
Negative controls alter coefficients, datum structure, datum identity and stored
geometry; archive failures retain the first differing byte and both lengths.
Reflected stored locations cannot reuse a right-handed certificate.

Physical solids receive fresh ordered native checks and volume estimates.
The reports preserve sampled geometry and native-search self-intersection
coverage. Retained flux is transported by the certified determinant, with the
ideal rigid-frame volume held fixed. A dyadic determinant `63/64` acting on
exact flux `8` gives `7.875`; the discrepancy control requires the resulting
`0.125` contribution even when the original discrepancy was zero.

The canonical frame returns before constructing a placement plan. Oblique
publication records the reduced canonical allowance and achieved world bounds
separately. Large-origin evaluation loss, unresolvable nearly parallel vectors,
nonfinite vectors, invalid spatial limits and arithmetic overflow refuse
explicitly. No tolerance or default work limit is widened.

The #454 gate checks every placed cap trace against its own retained minimum
edge/face tolerance, using `M*r + 2*E`. Its six arithmetic controls cover an
at-limit near-unit stretch, exact equality, both evaluation contributions,
underflow, invalid/overflowing bounds and common-translation behavior. The
oblique profile with a hole and disconnected material still completed after
this gate was added. Details are in
[the cap-trace record](bugs/454-transported-cap-traces.md).

## Replay and isolation

Concurrent calls compare physical bytes, authored history, budgets and placement
reports. Separate-process replay compares the complete reports and encoding in
forward and reversed selection order. That control exposed #453: only the
consumed planar bisection count differed, at `75` versus `57`. The bug fix must
canonicalize actual wrapped proof input while preserving actual accounting;
substituting a counter after computation would not satisfy this control.

Detached-export mutation and source-request mutation must leave published
bytes and another export unchanged. A cancellation control stops during
physical placement, retains the original request, and checks that an earlier
published result remains unchanged. Partial geometry never accompanies the
failure.

## Executed checks

The composed sketch library run passed 274 tests in 91.15 seconds, with three
subprocess workers ignored as entry points. It included #453's separate-process
replay and #454's placed tolerance checks. A final placement run passed 28
tests in 29.57 seconds, with one worker ignored; it also covered the added
world-limit diagnostic control. The public integration target passed 40 tests
in 13.97 seconds, with one worker ignored. The final public oblique control
passed again in 2.53 seconds. Strict all-target sketch clippy passed.

Refusal controls retain their entity-diagnostic assertions using unsupported
nonzero chart origins. Translated frames are now positive inputs. No failing
test was ignored or removed.

## Canonical-path performance

Six interleaved processes used the identical P4 mixed-profile consumer
operation. The ten fixture/test functions match the `2d5238f6` baseline exactly,
with SHA-256
`51291b17d85e221f5ca561c2347c4e5534b857e5a1311f4f75925f569d558167`.
Both binaries used the same compiler/profile settings and one test thread;
no Cargo build or other test campaign ran during measurement. This compares
unreleased checkpoints because the published release lacks this operation.

| Median | Before | After |
| --- | ---: | ---: |
| Wall seconds | 3.81 | 3.77 |
| User CPU seconds | 3.79 | 3.76 |
| GNU-time peak RSS, KiB | 53,544 | 53,784 |
| Sampled peak anonymous memory, KiB | 21,308 | 21,336 |
| Sampled peak file-backed memory, KiB | 33,800 | 34,244 |

All six executions passed. Their overlapping time ranges do not resolve a
latency regression or improvement from three samples per revision. The RSS
increase remains reported. A separate control blocked each executable while
it only listed tests, with no modelling operation: median anonymous memory
was 1,940 KiB before and 1,968 KiB after. The same 28 KiB anonymous increase
there supports an added-harness explanation; the test count grew from 244 to
278. These are separate observations, not a subtraction that proves exact
modelling-heap or process-RSS parity.

The [raw trials](references/p4-oblique-before-after.tsv),
[binary/fixture bindings](references/p4-oblique-performance.json) and
[harness control](references/p4-oblique-harness-memory.json) retain the
measurements. The completed benchmark executable copies were removed.

## Remaining qualification

The full 239-piece text workload passed after #453's proof-input ordering fix.
With the same explicit 8192-bisection request, all 18 material regions completed
in 322.12 seconds wall time and 321.92 seconds user CPU; GNU-time peak RSS was
195,704 KiB. The earlier accepted pilot at `2d5238f6` took 324.77 seconds wall,
324.61 seconds user CPU and 193,840 KiB peak RSS. These single trials do not
resolve a latency regression or improvement. The small RSS increase is still
under qualification, so strict peak-memory parity is not claimed.

The unchanged default-work control also passed: it consumed its actual 4096
bisections and returned the typed incomplete/root-budget refusal without
partial geometry. That control took 213.70 seconds wall, 213.59 seconds user
CPU and 192,952 KiB peak RSS. Compilation was excluded from both measurements.
The [runtime bindings](references/p5-after-proof-ordering.json) retain compiler,
profile, binary and log hashes. The prior full-text executable remains a
temporary active baseline for the outstanding memory investigation.

Full-text oblique placement, persistent body attachment/history, authored cone
profiles, the replacement cleanup gate and the feature ThreadSanitizer campaign
remain part of the broader action plan. This operation's controls do not qualify
them. Normalized analytic copies have the separate representation obligation
described in `OBLIQUE-FRAMES.md`.
