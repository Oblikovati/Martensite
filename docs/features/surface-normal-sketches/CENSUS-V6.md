# Geometric planar proof V6

`PlanarProofVersion::GeometricCompositeV6` selects exact geometric pair
certificates inside the existing planar region pipeline (#418). It accepts the
same integral component and cell limits as the composite program API:

```rust
PlanarProofVersion::GeometricCompositeV6 {
    max_components: 64,
    max_total_subintervals: 8192,
}
```

The request retains its selected version and original limits. An operation loan
can reduce the actual cell allowance. The result records both values and the
actual consumed cells. Earlier proof versions remain selectable.

## Pair evidence

A captured pair attempts the following exact leaves for polynomial controls of
degree at most three over the full unit interval. Explicit unit weights are
also supported. Other weights and trims continue to the general solver.

- Strictly separated closed control boxes certify no contacts.
- An open half-plane containing all nonzero hodograph controls certifies
  injectivity of one curve, excluding every off-diagonal self contact.
- For exactly one shared endpoint, an open half-plane containing the first
  curve's offsets and the negated second curve's offsets certifies that the
  shared endpoint is the only parameter-pair contact. Each opposite endpoint
  must differ from that point.
- A strictly separating supporting line between the two control hulls certifies
  no contacts when the boxes overlap.

The predicates reuse Martensite's exact orientation and integer arithmetic.
In particular, subtraction and determinant underflow do not turn uncertain
signs into zero. The finite `2^-600` second-contact counterexample must remain
inconclusive, while the scaled safe hairpin must still certify.

A geometric report owns its captured source and policy. Its endpoint view
reports exact native parameters and the shared point. The contact identity
explicitly distinguishes an endpoint witness from an algebraic candidate root;
no projected root or multiplicity is attributed to an endpoint certificate.
Native parameter measurements reuse the existing exact interval construction.

## Pipeline and remaining acceptance

Self and distinct domains execute in the existing logical pair order.
Inconclusive leaves execute the existing general solver with the remaining
root-split allowance. Every geometric leaf consumes zero root subdivisions;
completed fallback work is charged once. Cancellation cannot turn an
inconclusive report into a complete empty result.

The initial integration retains the complete pair matrix and its source,
policy, alias and work checks. It does not yet implement sparse sweep coverage.
Partition correspondence, native contact cardinality, geometry regularity,
authored incidences, graph orbits, nesting, integral programs and the cone's
subsequent construction and publication checks remain required.

The standalone prototype measurements in
[Census validation](CENSUS-V2-VALIDATION-2026-11-19.md) do not establish the
integrated operation's time or memory. Acceptance requires complete inward and
outward operations, replay and mutation controls, and matched time and peak
memory measurements. The five-second cone target remains open.

## Initial integrated diagnostics, 2026-11-19

The first V6 qualification passed 14 new Math controls, three new Census
controls, two new planar-program controls, and both complete inward/outward
cone controls. The broader name-filtered runs passed 11 TKBO tests, 18 Math
tests and four Sketch tests. The cone controls retained 148 spans, 444
integration cells and 989,355 metric arithmetic operations, including exact
self-intersection coverage, native export, history and mutation/isolation checks.

Two balanced V5/V6 outward pairs from the same test binary gave these medians:

| Measurement | V5 | V6 |
| --- | ---: | ---: |
| Public operation time | 13.649724 s | 7.633674 s |
| Peak RSS of the test process | 114,702 KiB | 64,846 KiB |

The process runs the operation and identical downstream test checks. RSS covers
that whole process; the API timer covers the operation. This is about 44.1%
less API time and 43.5% less peak RSS. A compiler guard stopped the six-run
campaign before its final V6 sample; the unmatched third V5 sample is retained
but excluded from this table. No source changed during the recorded runs.

[Logs, source hashes and guard receipt](references/census-v6-integration-2026-11-19/manifest.json)
retain the evidence. These are development-profile diagnostics, before the two
subsequent mechanical loop splits, identity diagnostic formatting and added
cancellation/owned-graph tests.
They establish neither checked-release performance nor release acceptance.
The five-second target, wider workload matrix and sparse traversal remain open.

The [final source check](references/census-v6-integration-2026-11-19/final-checks.json)
passed 184 affected TKBO tests, 18 Math tests and four Sketch tests after those
changes. This includes cancellation inside the new pair-domain work, old receipt
formatting, owned/live parity and both complete cone directions. The final
correctness-run API diagnostics were 7.920225 s inward and 7.853291 s outward.
Full repository/hosted gates and the wider feature matrix are still pending.
