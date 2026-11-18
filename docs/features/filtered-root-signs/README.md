# Filtered generated-root signs

Bug [#544](https://github.com/Oblikovati/Martensite/issues/544) adds an explicit
version 2 to `BezierPairSignOptions`. It applies to regular graph membership in
the existing Bézier pair solver. `BezierPairSignOptions::derived()` continues to
select version 1. The standalone polynomial-root sign API keeps its existing
behavior, as do primary-root isolation and exceptional-fibre evaluation.

```rust
use martensite_tkmath::predicates::roots::bezier_pair::BezierPairSignOptions;
let mut signs = BezierPairSignOptions::derived();
signs.version = 2;
```

The version is independent of the outer pair-policy version. Existing source
capture, degree admission and root isolation run before the filter. Effective
sign settings retain the complete requested policy, including the chosen
version and generated-degree cap. Bernstein-exclusion admission also accepts
sign versions 1 and 2 and retains their exact policy binding.

## Evaluation contract

The all-point/empty isolation path from #542 runs first. For a mixed or nonpoint
isolation, the filter visits every original candidate root, including candidates
that a later predicate rejects. A singleton uses the existing exact point-sign
function. A nonpoint bracket uses exact rational interval Horner evaluation of
the admitted integer power coefficients. A strictly positive lower bound or
strictly negative upper bound establishes the sign on the whole bracket.

Only an exactly empty normalized coefficient vector establishes a globally zero
query. An interval that touches or contains zero is inconclusive. The first
inconclusive bracket discards all provisional signs and sends the entire
original isolation to the existing eager evaluator. No bracket is refined, no
candidate is omitted and no sign is inferred from a midpoint.

Cancellation during the filter returns the original typed `Evaluation` error
immediately. It never triggers fallback or exposes provisional signs. Version 2
has extra cancellation checkpoints, including one per Horner coefficient; its
poll trace is therefore distinct from version 1. Pair work still measures
completed root subdivisions. The filter adds none, and fallback retains all
actual split work. Degree, precision, approximation width, world-error limits
and subdivision limits are unchanged.

## Evidence and limits

The structural extraction was checked before enabling version 2 using three
owned binary64 source pairs from the actual cone workload. Their complete
candidate brackets, multiplicities and all membership decisions stayed equal;
version 1 retained poll counts `694`, `1687`, `694` and completed split counts
`0`, `29`, `0`. The mixed fixture retains both rejected nonpoint roots and its
accepted singleton.

Focused controls cover strict bounds, exact singleton zeros in mixed isolations,
zero queries, zero-touch fallback, a misleading midpoint, common roots and
multiplicity. Cancellation controls visit every observed filter/fallback
checkpoint. Policy tests retain explicit generated-degree and primary-degree
refusals, reject unsupported sign version 3 and check source/policy mutation.
The focused root unit run passed 635 tests; the unchanged standalone public-sign
controls passed 15 tests. The existing public pair/exclusion controls passed
22 tests. No timing assertion or full-operation performance claim follows from
these functional controls.

Commands used with the dedicated target and lock, two build jobs, incremental
compilation disabled and debug information stripped:

```sh
cargo test -p martensite-tkmath --lib predicates::roots::
cargo test -p martensite-tkmath --test toolkit exact_roots::signs::
cargo test -p martensite-tkmath --test toolkit certified_bezier_pair
cargo clippy -p martensite-tkmath --all-targets -- -D warnings
```

The structural checkpoint is `acdb1126`; version-1 actual graph controls passed
both before and after that extraction. Final strict all-target Clippy passed. Component review and the full PR gate
remain pending.

See the [decision](decision.md) for scope and replay requirements. Topology
consumer routing is a separate integration step; this Math change does not
silently migrate an existing planar-proof version.
