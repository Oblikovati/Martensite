# Specialist brief: cone normal extrusion and planar preparation

A measured cone extrusion spends about 8.09 of its 13.39 seconds preparing planar
profiles. This brief traces the whole operation so that an improvement to that
stage can be assessed against its geometric, ownership and replay obligations.
It describes the current unreleased B1 candidate, HEAD `2e8ab886` plus reviewed
work, against main `b61a031d`.
Correctness review 2 accepted its bounded remediation; broader qualification is
unfinished. [Receipts and stage measurements](references/b1-directed-correctness-attribution-2026-11-19.json)
and [148 sanitized actual pair fixtures](references/specialist-census-fixtures-2026-11-19.json)
accompany the brief. No new experiment was run to write it.

## Geometry and workload

The input is an owned planar `SketchProfile`: eight polynomial cubic pieces form
an oval O with a hole, and four lines form a dot. Two selected material regions
become two bodies. The fixture uses semi-angle `alpha=0.125`, canonical apex/axis/
radial frame and signed depths `[-0.1,0.2]`. After refinement, its actual cap-UV
boundaries contain 137 O cubics in two cycles of 66/71 and 11 dot cubics. All 148
captured UV curves are polynomial cubics with unit weights and parameter interval
`[0,1]`. These counts describe the measured O., not general text support.
[Fixture](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/tests/narrow/fixture.rs),
[common depth/policy defaults](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/tests.rs#L11),
[classification provenance](references/specialist-census-fixtures-2026-11-19.json).

For apex-relative slant `l`, angle `theta` and outward signed normal depth `d`,
the conceptual reference map is

```
S(theta,l) = (l sin(alpha) cos(theta), l sin(alpha) sin(theta), l cos(alpha))
n(theta)   = (cos(alpha) cos(theta), cos(alpha) sin(theta), -sin(alpha))
X(theta,l,d) = S(theta,l) + d n(theta).
```

The implementation agrees: radius is `l*sin(alpha)+d*cos(alpha)` and Z is
`l*cos(alpha)-d*sin(alpha)`. Authored developed coordinates `(x,y)` give
`l=hypot(x,y)` and `theta=chart_origin+phi/sin(alpha)`, with `phi` selected on the
admitted continuous wedge; plain `atan2` is sufficient only when that branch
agrees. Narrow cones amplify developed angles by approximately eight here,
as a consequence of the angle formula. Inward requests normalize physical depths to
`[-last,-first]`; final history still reports authored First/Last roles.
[Mapping](../../../crates/martensite-tksketch/src/wrapped_profile/cone_mapping.rs),
[endpoint construction](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/native.rs),
[depth normalization](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/input.rs#L155).

The exact logical reference uses the accepted common-chart rational `q/L` fit,
where `q=tan((theta-center)/2)` and `L` is slant; native cap parameters are
`U=theta, V=L-l0`. The native representation is a
separately bounded proxy: rational depth curves, positive common-weight two-row
ruled sides and analytic conical caps with polynomial trims. Directed final rows
receive certified Z corrections before construction. SI Exact concerns the
selected bounded-proxy/allowed-contact contract; it does **not** certify that
rounded literal native patches glue into an exactly closed surface. Geometry
checks remain Sampled, so the whole report is not exact.
[Carrier ADR](../../architecture/decisions/analytic-wrapped-profile-carriers.md),
[directed finalization](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/prepare/directed.rs).

## The complete operation, in source order

1. **Admit and capture.** The selected public entry polls initial cancellation,
   admits support/scalars/proof policy, checks directed eligibility, then reads
   the selected source curves and orientation. An unsupported selection must
   retain its earlier typed refusal rather than be masked by a later cancellation.
   [Entry](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/mod.rs#L106),
   [input](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/input.rs).

2. **Select, fit and finalize.** One operation context captures the original
   planar material, admits the common chart, adaptively fits/subdivides spans,
   and certifies clearance. Directed mode issues family gamma, constructs one
   source-keyed final endpoint table and checked row transitions, then retains
   those final rows for every subsequent consumer. It does not run an alternate
   retry engine or silently enlarge a limit.
   [Shared executor](../../../crates/martensite-tksketch/src/wrapped_profile/operation/execute.rs),
   [selection](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/prepare.rs),
   [fit](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/prepare/fit.rs).

3. **Construct, freeze and decode.** Cone adapters feed the existing shared
   `construct_mapped` incidence builder. The family is assembled in stable
   source-key order, encoded once, then decoded with coefficient-preserving
   surface/rigid-location policies into an isolated native root. The executor
   checks family coverage and builds source history before member certification.
   No timer isolates the serialization/round-trip cost.
   [Cone adapter](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/native.rs),
   [freeze boundary](../../../crates/martensite-tksketch/src/wrapped_profile/operation/frozen.rs),
   [coverage/history order](../../../crates/martensite-tksketch/src/wrapped_profile/operation/execute.rs#L48).

4. **Read actual geometry and certify members.** Storage captures decoded
   curves, vertices, rows, weights, cap carriers and pcurves, checks them against
   source-bound receipts, and enforces storage/trim/join allocations. Member
   geometry then certifies actual filled UV material, reference volume, proxy
   flux and authored-volume evidence, in that order. The expensive reconstructed UV
   preparation occurs here.
   [Storage](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/storage.rs),
   [geometry order](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/measures.rs#L20).

5. **Issue and bind family SI authority.** Directed mode completes all member
   geometry first. TKBO independently captures actual native sources; Math issues
   the source-bound family certificate; TKBO binds it to complete coefficients,
   trims, roles and incidence. Raw equivalent representations remain distinct
   issued authority. Ordered native checks run topology, sampled geometry,
   certified SI and tolerance; qualification also performs native volume/error
   estimation. That entire qualification clock is not formal-SI computation.
   [Directed member path](../../../crates/martensite-tksketch/src/wrapped_profile/operation/members/directed.rs),
   [actual family](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/actual.rs),
   [ordered check](../../../crates/martensite-tkbo/src/body_check/certified_conical/ordered.rs).

6. **Budget and publish.** The directed family is rebound again, non-passing
   native reports are refused, and the global q/L reference and original spatial/
   volume/work limits are checked. Publication projects caller-relative history,
   prepares reports, revalidates material/reference bindings and polls before
   immutable handoff. It retains archive bytes, history and evidence; each native
   export decodes independent owners and checks keyed correspondence. `Clone` or
   `Arc` alone would not prove isolation from mutable native aliases.
   [Budget](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/measures/totals.rs),
   [publication](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/publication.rs),
   [detached export](../../../crates/martensite-tksketch/src/wrapped_profile/api/cone.rs#L135).

The cone and cylinder adapters use the shared operation executor.
Requested settings, effective leaf policy, achieved bounds and consumed work stay
separate. Error allocations include original-to-reference displacement, downward-
rounded epsilon/1000 native storage, cap trace and filled-domain bridges. Reference
volume, authored volume, proxy boundary flux and native quadrature estimates
remain distinct. First errors, bounded work and cancellation checkpoints are
observable contracts. The 989355 metric-arithmetic count is not a count of CPU
instructions across the entire operation: planar/root ledgers separately count
subdivisions, while BigInt normalization/elimination can be costly with few splits.
[Allocations](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/storage/allocation.rs),
[work accounting](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/storage/work.rs).

## Error-bounded volume and flux

For positively oriented developed material A and normalized physical depths,
the parallel-cone Jacobian is `J=1+d*cot(alpha)/l`. Accordingly,

```
V = (d1-d0) Area(A)
  + (d1*d1-d0*d0)/2 * cot(alpha) * integral_A(1/l) dA.
```

This requires the admitted apex/radial/offset clearance; it cannot cross a
singularity. The authored implementation computes signed area and the inverse-
slant moment from contour integrals, enclosing their numerical error. The q/L
reference separately integrates fitted contour terms through bounded rational
one-dimensional quadrature, retaining its own fit/source binding and enclosures.
[Authored composition](../../../crates/martensite-tkmath/src/predicates/roots/metric_bounds/conical_profile/volume.rs#L211),
[reference coefficients](../../../crates/martensite-tkmath/src/predicates/roots/metric_bounds/rational_integrals/conical_reference_volume/coefficients.rs).

Actual cap/side occurrence flux is integrated separately, with a declared field
origin and error enclosure. The 444-cell operation ledger aggregates UV-program,
reference and proxy integration; authored refinement also spends its declared
arithmetic allowance. The volume refusal uses the certified authored/reference
discrepancy, not the proxy flux or native estimate alone. Native Gauss–Kronrod
volume/status remains an independent numerical comparison, not this certificate.
[Flux and integration charge](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/measures/flux.rs),
[refusal policy](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/measures/totals.rs).

## What the clocks establish

The broad outward probe measured `13.388177 s` of API wall time. This is a debug/test workload,
not an optimized release benchmark: ordinary/test code uses optimization 0,
TKBO 1, Math and `num-bigint` 3, with assertions and overflow checks enabled.
[Workspace profiles](../../../Cargo.toml),
[actual binary and run receipts](references/b1-directed-correctness-attribution-2026-11-19.json).

| Broad block from that probe | Seconds |
| --- | ---: |
| Original 12-edge planar selection | 0.111993074 |
| Reconstructed O137 preparation | 7.678697725 |
| Reconstructed dot11 preparation | 0.302435683 |
| All three preparations | 8.093126482 |
| Actual-family capture/queries/inclusive issuance | 0.087119432 |
| Native qualifications, including estimates | 2.006434236 |
| Publication family rebind | 0.055836426 |

`fixture::request` constructs `SketchProfile` values before `accepted` starts its
clock. The 12-edge planar preparation, however, occurs inside the API through
`select → selection::planar`; it must not be excluded as fixture construction.
For O137, Census/partition/contacts took `5.694264686 / 0.364117301 / 1.617544552 s`;
dot11 took `0.278995013 / 0.019900712 / 0.003003285 s`.
Numeric actual-certificate issuance was `0.026390898 s`, nested inside inclusive
issuance `0.032402708 s`, not an additional block. The listed broad blocks account
for about 10.24 seconds. Roughly 3.15 seconds remain unattributed across fitting,
storage, other geometry/measure/budget/history/serialization work and overhead;
this is not an exclusive per-phase breakdown or evidence against freeze/decode.
[Timer boundary](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/tests/narrow/complete/directed.rs#L17),
[original planar selection](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/prepare/selection.rs#L5).

## Why planar preparation becomes general intersection work

`material/domain.rs` reconstructs source-keyed closed loops from actual stored
cap pcurves, validates shared endpoints, and swaps `[U,V]` into `[V,U]` to preserve
winding under the orientation-reversing development. It then calls the general
owned PlanarRegion V5 preparation: source capture, Census, partition, contact
transport/cardinality, history and region assembly. Rechecking actual UV topology
is legitimate: a fitted approximation can change intersections, so authored
connectivity alone cannot authorize its filled material.
[UV reconstruction](../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/material/domain.rs),
[owned preparation](../../../crates/martensite-tkbo/src/sketch_region_prism/owned_request/profile/pipeline.rs#L122).

Census enumerates `N(N+1)/2` domains, including each self domain once. Rational
curves `(X_i/W_i,Y_i/W_i)` yield exact equations
`F=X_i(t)W_j(u)-X_j(u)W_i(t)` and the analogous Y equation G. Self mode divides
the exact diagonal factor rather than sampling around it. Exact Bernstein sign
witnesses can exclude a parameter rectangle; an inconclusive result invokes the
generic solver with its remaining budget.
[Census enumeration](../../../crates/martensite-tkbo/src/sketch_region_certification/census.rs),
[exclusion/fallback](../../../crates/martensite-tkbo/src/sketch_region_certification/bernstein_execution.rs),
[equations](../../../crates/martensite-tkmath/src/predicates/roots/bezier_pair/equations.rs).

The fallback seeks a linear ideal row `alpha(t)u+beta(t)=0`, substitutes
`u=-beta/alpha` into both original equations, checks exceptional leading/fiber
factors, and isolates projected polynomial roots. Each distinct candidate receives
four exact membership checks: alpha nonzero, the other original equation zero,
and the two paired-parameter bounds. It then constructs paired/world enclosures;
unresolved fibers or overlaps remain incomplete. This is a general algebraic
route even when the answer is one shared endpoint.
[Projection](../../../crates/martensite-tkmath/src/predicates/roots/bezier_pair/projection.rs),
[root/candidate flow](../../../crates/martensite-tkmath/src/predicates/roots/bezier_pair/operation.rs),
[membership](../../../crates/martensite-tkmath/src/predicates/roots/bezier_pair/membership.rs).

The separate Census aggregate diagnostic measured O137's 9453 evaluations, zero reuse
hits and 44674878 lookup entries. Lookup cost only `0.100502691 s`; capture
`0.002741331`, exclusion `1.091860104`, fallback `4.305834137`, enumeration
`5.507383695`, refresh `0.146208490`. Dot11's 11 fallbacks took `0.266347393 s`.
These are a different run's nested clocks and must not be added to the broad
probe as an exact decomposition. The later metadata run occurred during a main
compile; its timing fields are intentionally ignored.
[Aggregate receipt](references/b1-directed-correctness-attribution-2026-11-19.json).

The metadata resolves the cases: all 137 O and 11 dot fallbacks are `Distinct`,
Complete, exactly one genuine shared-endpoint contact, error zero, no failures
or nonendpoint contacts. O has 129 last→first and 8 first→last pairs. Reported
projected-univariate root multiplicities are 23×1, 110×2 and 4×3; these are not
proven intrinsic geometric intersection multiplicities. The retained execution
contact graph has degree two and components 66/71; dot is an 11-cycle.
Thus O's 9316 Excluded outcomes comprise 137 self domains and 9179 other distinct
pairs; the remaining 137 distinct adjacent queries genuinely contact. Strict
closed-rectangle exclusion cannot prove a nonzero sign at their common zero.
A shared endpoint alone still does not prove absence of additional crossings.
[Actual inputs, policies and expected results](references/specialist-census-fixtures-2026-11-19.json).

## Reproduction

The fixture JSON retains all 148 actual control-bit pairs, optional weights,
intervals, effective per-record policy, subdivision work, candidate, exact t/u,
projected multiplicity and zero achieved error. Volatile parent/native addresses
are removed; source/occurrence numbers are diagnostic labels, not persistent
identity. Decode scalars with `f64::from_bits`, reading bit integers as `u64`
without an intervening binary64-only JSON parser. The public Math
`CapturedBezierPair::capture_pair` and `.certify` APIs already exist on main
`b61a031d`; the whole unmerged cone API is unnecessary for isolated reproduction.
Use each recorded effective policy: pair version 2, exclusion 1, root version 1,
width 0.001, degree 32, sign version 2/DerivedProjection and the recorded remaining
split cap. Do not replace these caps with fresh global allowances.
[Public capture/solve](../../../crates/martensite-tkmath/src/predicates/roots/bezier_pair/public_query.rs),
[policy shape](../../../crates/martensite-tkmath/src/predicates/roots/bezier_pair/query.rs).

Separate main-lane [#564](https://github.com/Oblikovati/Martensite/issues/564)
implements sparse matching-row iteration: 48 focused controls passed and source
review accepted it; benchmark acceptance remains pending. It does not resolve
Census fallback. [#565](https://github.com/Oblikovati/Martensite/issues/565) targets
repeated primitive/content normalization of the same projected source polynomial
across four membership queries in the generic fallback. Proposed private query-local
reuse is unapplied and savings unknown; it is separate from source/equation
preparation shared by exclusion and fallback.
[Membership calls](../../../crates/martensite-tkmath/src/predicates/roots/bezier_pair/membership.rs),
[normalization boundary](../../../crates/martensite-tkmath/src/predicates/roots/bezier_pair/algebraic.rs#L62).

The accepted operation uses 989355/1000000 arithmetic units, 444 integration cells
and coverage 148 self /148 adjacent /10730 nonincident /592 side-cap /4 cap-self
/6 cap-pair domains. The documented constraints require exact signs, source/revision/options binding,
original tolerances, established APIs/replay and one general pipeline. Unproved
known-join shortcuts and tessellated acceptance are excluded. The
five-second target, matched time/RSS, canonical/hosted/concurrency gates and
broader wide/text/seam/oblique/primitive capability matrix remain open.
