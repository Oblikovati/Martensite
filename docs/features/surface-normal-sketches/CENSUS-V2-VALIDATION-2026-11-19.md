# Census v2 prototype verification

The supplied exact geometric method reproduced the captured workload. Both
members had no unresolved domains, and Python matched all 148 recorded
source-pair, endpoint-index and point-bit tuples. Exhaustive closed-box tests
matched the sweep. This comparison does not reproduce the old projected-root
multiplicities, solver traces, callbacks or complete material-region result.

Archive: `census-v2-prototype.zip`, SHA256
`e199f9628888cfdf7f9c684d5648be37ffcf4bcaa3614e3688b115917216afbe`.
[Run logs and hashes](references/census-v2-validation-2026-11-19/manifest.json)
retain the executed commands and results. Production kernel code was unchanged.

## Observed standalone costs

| Implementation/profile | O137 | Dot11 |
| --- | ---: | ---: |
| Supplied Rust, development | 0.303 ms | 0.022 ms |
| Supplied Rust, release with assertions and overflow checks enabled | 0.046 ms | 0.003 ms |
| Development caller using Martensite's existing exact orientation predicate | 1.110 ms | 0.079 ms |

These are diagnostic averages from the prototype's 200-iteration loop. They
exclude native construction, source-bound production proof issuance, partition,
contact transport, integration and publication. The prototype's all-pairs
BigRational comparator is not the existing kernel's full Census implementation.
No full-operation speedup or peak-memory acceptance is established by this table.

The supplied development manifest optimizes `num-rational` at level 3;
`num-bigint` and `num-integer` actually use level 0. The program's profile label
does not report this correctly. Its release label also relies on
`cfg!(debug_assertions)`, so the checked release run prints a misleading debug
label. The recorded Cargo command selected release optimization with checks on.
The kernel orientation adapter links the existing Math library built at level 3.

## Numerical regression and predicate reuse

The exact sector and hodograph arguments hold for their stated polynomial,
finite-control, unit-interval inputs. The supplied Rust shared-point branch calls
unscaled adaptive orientation directly, bypassing its rational fallback. With
`h=2^-600`, these finite cubics provide a second contact:

```text
P = (0,0), v = (h,h/2)
A = [P,v,v,(h,h)]
B = [P,P,P,(1,3/4)]
A_y(t) - (3/4) A_x(t) = (h/4)t(t²+3t-3).
```

The last factor is negative at 1/2 and positive at 1, establishing an interior
intersection with B's line segment. The supplied predicate returned zero for an
exact negative determinant and falsely certified endpoint-only contact. The
[regression failed as expected](references/census-v2-validation-2026-11-19/underflow-red.log).

Substituting `martensite_tkmath::predicates::orient2d` in the disposable adapter
made the [same regression pass](references/census-v2-validation-2026-11-19/kernel-predicate-green.log)
and retained the fixture counts with the approximately 1.11 ms O137 timing above.
This reuse needs no new production orientation dependency.
[Exact counterexample inputs](references/census-v2-validation-2026-11-19/finite-range-counterexample.json)
are retained as binary64 bits.

## Production boundaries

The prototype supplies geometric self/pair tests. Rust returns statistics;
Python returns endpoint tuples and an unresolved list. Neither wires a genuine
general-solver fallback or issues the kernel's complete planar-material result.

Current consumers require source- and policy-bound logical pair coverage,
contact parameter/world evidence, native cardinality, regularity, nesting and
history. An endpoint certificate must represent its actual proof instead of
claiming an old algebraic root or multiplicity was computed. The existing
non-exhaustive proof-version selector provides an additive integration seam;
earlier methods and their replay contracts remain available.

The prototype processes self domains before distinct domains. Current Census
uses interleaved logical pair order. Fallback budget, cancellation and first-error
ordering therefore need explicit treatment during integration.

The report-reuse lookup's cold scan count is confirmed as D(D-1)/2 with
D=N(N+1)/2; the captured O case has 44,674,878 comparisons. Contact cardinality
also contains repeated report, endpoint and vertex scans beyond #564's source-row
optimization. These structural facts do not establish an exclusive explanation
for the observed O/dot timing ratio.

The remaining whole-operation costs, checked-release baseline, integrated
before/after timings, memory and all existing kernel gates remain unmeasured for
this implementation. The earlier 13.39-second operation receipt is unchanged.
