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
always retain the original eager query-chain evaluation. A singleton uses the existing exact point-sign
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
the initial poll counts were `694`, `1687`, `694` under the then-installed
#542 shortcut, with completed split counts `0`, `29`, `0`. Those callback counts
were not the original legacy baseline. The mixed fixture retains both rejected nonpoint roots and its
accepted singleton.

Focused controls cover strict bounds, exact singleton zeros in mixed isolations,
zero queries, zero-touch fallback, a misleading midpoint, common roots and
multiplicity. Cancellation controls visit every observed filter/fallback
checkpoint. Policy tests retain explicit generated-degree and primary-degree
refusals, reject unsupported sign version 3 and check source/policy mutation.
Before compatibility remediation, the focused root unit run passed 635 tests,
the standalone public-sign controls passed 15 and the public pair/exclusion
controls passed 22. The later full #542 PR gate found a legacy callback
regression: the complete-owned-request control observed `8238` against its
unchanged `8268` expectation. The default shortcut was removed; exact point and
empty shortcuts now require sign policy 2. Original candidate `54e2df1` measured
sign-policy-1 callback counts `1184`, `1687`, `1184` for the three retained pairs.
Those original-source counts are the new fixture pins; subdivisions and all
exact geometric decisions remain equal. No performance claim follows from
these functional controls.

Commands used with the dedicated target and lock, two build jobs, incremental
compilation disabled and debug information stripped:

```sh
cargo test -p martensite-tkmath --lib
cargo test -p martensite-tkmath --lib predicates::roots::
cargo test -p martensite-tkmath --test toolkit exact_roots::signs::
cargo test -p martensite-tkmath --test toolkit certified_bezier_pair
cargo clippy -p martensite-tkmath --all-targets -- -D warnings
```

The structural checkpoint is `acdb1126`; version-1 actual graph controls passed
both before and after that extraction under the then-installed #542 shortcut.
After compatibility remediation, the full Math library passed 681 tests, the
focused root controls passed 637, the public standalone signs passed 15 and the
public pair/exclusion controls passed 22. Strict all-target Math Clippy passed.
Component review and the full PR gate remain pending.

See the [decision](decision.md) for scope and replay requirements. The topology selector below keeps earlier planar-proof versions selectable.


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


The following routing results predate compatibility remediation. The restored
legacy owned-operation callback controls and original-source V3 signature
controls are pending; no whole-body or performance acceptance is claimed.

TKBO routing qualification passed 100 certification controls, 39 owned-profile
controls and 57 public profile-extrusion controls. The three V1 callback/error
and reuse controls also passed, for 199 focused TKBO controls in total. Strict
all-target TKBO Clippy passed. Before compatibility remediation, the native V3 crossing fixture recorded
1,739 cancellation polls, one charged subdivision, no reused reports, three
exclusions and three domains. That measurement included the rejected #542
default shortcut, so it is not an original-source callback baseline. The
restored V3 replay requires original eager evaluation. The existing public default-versus-`CarrierV2` control
also retained equal polls, work, census and area. V4 controls exercise native
exclusion and solver fallback, remaining policy/work, receipt mutation,
selector relabeling and unknown graph request version 5.

The public test executable requires the dataset submodule. The first compile
attempt found an empty worktree dataset directory; initializing its pinned
`34d65afd72cd3ca8ac1f9d17e5ca408b30e9b4b6` revision resolved the missing files.
That attempt did not run tests. The initialized copy has no object alternates.

```sh
cargo test -p martensite-tkbo --lib sketch_region_certification::
cargo test -p martensite-tkbo --lib sketch_region_prism::owned_request::profile::
cargo test -p martensite-tkbo --test toolkit profile_extrusion::
cargo test -p martensite-tkbo --lib sketch_region_prism::owned_request::progress_tests::
cargo clippy -p martensite-tkbo --all-targets -- -D warnings
```
