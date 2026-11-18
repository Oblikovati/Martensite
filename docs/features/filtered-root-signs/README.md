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

Only explicit sign policy 2 enters the filter. It handles empty, point-only
and mixed isolations through the same all-root attempt, including candidates
that a later predicate rejects. Legacy sign policy 1 and standalone root signs
always retain the original eager query-chain evaluation. A singleton uses the
existing exact point-sign function. A nonpoint bracket uses exact rational
interval Horner evaluation of the admitted integer power coefficients. A strictly positive lower bound or
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

Production checkpoint `9343219091e2fb95bba1f0f128acf46826c90018` restores the
original eager sign-policy-1 evaluator, including empty and point-root
isolations. The rejected #542 default shortcut will not ship. Independent
characterization of original candidate `54e2df1` established callback counts
`1184`, `1687`, `1184` for the three retained cone-source graph pairs. Those pins,
completed split counts `0`, `29`, `0`, candidate brackets, multiplicities and
membership decisions are retained. The mixed fixture includes rejected
nonpoint roots and an accepted singleton.

The original-source V3 fixture expectation was corrected in test-only checkpoint
`1241a8e83536861a1585f98fe43a0e15432b67d2`. Production code stayed at the
checkpoint above. Current compatibility results are:

| Control | Passed / failed | Retained evidence |
| --- | --- | --- |
| Full Math library | 681 / 0 | Complete library suite. |
| Focused Math roots | 637 / 0 | Includes strict bounds, singleton zeros, zero queries, zero-touch fallback, misleading midpoint, common roots and multiplicity. |
| Public standalone signs | 15 / 0 | Existing public sign behavior. |
| Public pair/exclusion | 22 / 0 | Source/policy binding and typed degree/version refusals. |
| Real complete-owned request | 1 / 0 | Original callback pins `8268/2866/3696` unchanged. |
| Selector census | 6 / 0 | Original V3 signature `[1754, 1, 0, 3, 3]` with sign policy 1. |
| Owned selectors | 7 / 0 | Retained owned-request routing. |
| Public legacy selector | 4 / 0 | Default and earlier selector behavior. |
| Public V4 selector | 2 / 0 | Explicit filtered routing. |

Strict all-target Math and TKBO Clippy passed. Cancellation controls visit every
observed filter/fallback checkpoint. Policy controls cover generated and primary
degree refusals, unsupported sign version 3, and source/policy mutation.

The callback counts `694/1687/694` and V3 poll count `1739` came from earlier
#542-shortcut runs. They are not accepted legacy baselines. Original-source
characterization instead pins the restored graph counts above and V3's
`1754` polls, one subdivision, zero reused reports, three exclusions and three
domains. The complete-owned `8268/2866/3696` pins were preserved throughout.

Component review attempt 2 of 3 passed Standards and found no production Spec
defect. The focused legacy controls required by that review now pass, so the
cone port can proceed. Gate A, accepted-workload
performance, the full affected PR gate and hosted checks remain pending; these
functional results establish no end-to-end speedup or five-second result.

Commands used with the dedicated target and lock, two build jobs, incremental
compilation disabled and debug information stripped:

```sh
cargo test -p martensite-tkmath --lib
cargo test -p martensite-tkmath --lib predicates::roots::
cargo test -p martensite-tkmath --test toolkit exact_roots::signs::
cargo test -p martensite-tkmath --test toolkit certified_bezier_pair
cargo clippy -p martensite-tkmath --all-targets -- -D warnings
```

See the [decision](decision.md) for scope and replay requirements. The topology
selector below keeps earlier planar-proof versions selectable.

## Topology selector

`PlanarProofVersion::FilteredBernsteinV4` selects owned graph request version 4.
It uses the same source capture, native partition, contact transport and graph
proof as `BernsteinV3`, with generated-sign policy version 2. The outer pair
policy stays at version 2 and the Bernstein-exclusion policy stays at version 1.
`CarrierV2`, the default builders and `BernsteinV3` retain sign policy version 1.

The census retains the selected sign settings separately from each requested,
effective and source-bound numerical receipt. Graph consumption checks the
owned selector against that mode: request 2 uses carriers, request 3 uses exact
exclusions with sign version 1, and request 4 uses exact exclusions with sign
version 2. Unknown request version 5 and relabeled or mutated receipts refuse.
A binding mismatch returns its typed refusal before a later cancellation
callback can replace it.

Exclusion and fallback keep their actual work receipts. Fallback subtracts only
completed exclusion subdivisions and copies the full remaining policy,
including sign version 2. Native occurrence counts, source-conformance checks,
root and degree limits, world-error limits and ordered publication checks stay
unchanged. This selector does not change the cone request default or establish
a whole-operation performance result.

The restored default-versus-`CarrierV2` controls retain equal polls, work, census
and area. V4 controls exercise native exclusion and solver fallback, remaining
policy/work, receipt mutation, selector relabeling and unknown graph request
version 5. Public tests require the dataset submodule at pinned revision
`34d65afd72cd3ca8ac1f9d17e5ca408b30e9b4b6`.
