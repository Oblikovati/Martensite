# Follow-up checks before selecting the next optimization

The current-V6 complete outward O. operation passed a disposable fit-charge
probe. Its 12 authored curves generated 284 exact boundary-certificate calls:
148 successful leaves and 136 rejected parent spans. Successful calls consumed
433797 arithmetic units; rejected calls consumed 254957. These sum to 688754
fitted-boundary units. The separate 6664 original-radius units bring regional
preparation to the previously observed 695418. Rejected parents consume 37.0%
of fitting units and 25.8% of the 989355 whole-operation units.

`prepare/fit.rs::plan` invokes `candidate` before deciding whether to subdivide.
`candidate` constructs the approximate fit and native edges, then invokes the
exact boundary certificate. `certify` charges both success and failure using
that result's actual operation count. Only a spatial Budget proof refusal is
eligible for subdivision; cancellation, exhausted arithmetic, invalid source
and other failures do not become fit retries. A refused certificate can stop
early; it does not necessarily cost as much as a successful one.

A deterministic floating subdivision predictor could avoid many refused
certificates while leaving exact certification authoritative for every kept
span. The measured 254957 units are the cost of the current refused attempts,
not a guaranteed replacement saving. A predictor could change the leaf set,
its own cost or downstream work. Fitting currently occupies only a few tenths
of a second, so this addresses budget headroom more directly than the
five-second operation target. Preserve the existing fit route for replay;
record the predictor policy and its effective dyadic schedule. Fixed operation
order and avoiding FMA are necessary controls, but the existing fit also calls
platform sin, tan, atan2 and hypot: cross-platform replay needs evidence for
those decisions as well. Approximate samples must never establish acceptance,
topology or proof coverage.

The current request has a finite arithmetic allowance before `Work` is created.
`operation/cone/input/policy.rs` rejects values above 1000000;
Math's `metric_bounds/cylindrical_profile/admission.rs::policy` independently
applies the same ceiling. An input-sized allowance would need explicit
admission/policy work; merely passing a larger existing request is refused.
Compute any new finite grant from captured authored inputs, fit-level and
precision policy before proofs start; bind its requested/effective values and
hard ceiling into replay. Refusal must not increase it. No grant changed in
this investigation.

Current charged reference-volume/flux hooks do not count every BigInt product
or gcd in their rational integrator. The earlier approximate 134500 figure is
not the current disjoint stage ledger. Current member geometry charges 54382,
including material rebuild, reference volume, flux and authored volume; its
reference-volume/flux coefficient hooks charge 2858 in total and authored
volume charges 36692. Unit counts cannot establish CPU cost.

Reference-volume and actual-flux integrators use integer polynomials and
`ExactRatio`, whose arithmetic normalizes with gcd. Their bounds accumulate as
exact rational sums. Authored-volume and final family-reference checks instead
use fixed-grid dyadic intervals and round outward after products/divisions.
The targeted operand probe passed the same complete operation, retaining
989355 arithmetic units, 444 integration cells and 148 fitted spans. It observed
the following bit-length maxima; numerator and denominator maxima are reported
independently.

| Leaf | Input polynomial numerator / denominator | Raw running-sum numerator / denominator before gcd | Reduced running-sum numerator / denominator |
| --- | ---: | ---: | ---: |
| O137 reference volume | 219 / 209 | 7500 / 7493 | 7115 / 7108 |
| O137 actual flux | 336 / 332 | 4947 / 4943 | 4662 / 4656 |
| dot11 reference volume | 207 / 199 | 1660 / 1660 | 1286 / 1285 |
| dot11 actual flux | 313 / 310 | 1195 / 1195 | 917 / 917 |

O137 shifted polynomials reached 227/215 bits for reference volume and 362/359
for flux; exact Taylor remainders reached 297 and 432 bits. The much larger
values occur in the accumulated rational bounds. This confirms sum growth,
not its share of runtime. The probe observed the already required numerator
cross-sum and denominator product immediately before unchanged gcd reduction,
and observed the reduced result. It did not add integer products, change work
charges or use global/TLS counters. Exact bounds were still produced by the
existing arithmetic. Timings from this instrumented run are not acceptance
measurements.

The next useful arithmetic comparison is a bounded outward accumulator against
the exact running sum, with its rounding error included in the original volume
limit. Reuse the existing dyadic arithmetic. That would change numerical
behavior and therefore needs its own selectable behavior version, independent
volume oracles and unchanged refusal limits; it is not implemented here.

Exact geometric endpoint certificates carry 0/1 parameters, but B1 still routes
them through `native_parameter_construction.rs::measure_captured_axes`, then
general interval map and distance construction. Point intervals multiply all
four equal corner products; equal distance endpoints repeat subtraction.
This is interval metric construction, not repeated root isolation. #566 leaves
these Math/measurement paths unchanged. A conditioned exact-point branch can
reuse the current admission, both-map-endpoint validation, trim checks, owned
measurement and threshold decision. It must preserve raw signed-zero fields,
reversed and extreme maps, subnormals and first-error order. No timing or
end-to-end saving is established for that candidate.


The fit and operand probes each passed the complete outward operation checks.
All 346 frozen changed Rust paths matched after restoration. Probe source and
instrumented artifacts were discarded; the operand pass removed 576594744 bytes
of emitted probe artifacts. These passes changed no production behavior,
request grant or version.

Evidence: [fit charges](references/fit-charge-attribution-2026-11-19/summary.json),
[operand sizes](references/volume-bits-attribution-2026-11-19/summary.json), and
[current stage attribution](references/v6-stage-attribution-2026-11-19/summary.json).
The raw logs preserve every observed attempt and sum measurement. These are
Martensite diagnostics, not a PhysCAD comparison or final performance approval.

## Source and incidence audit, 2026-11-20

The wrapped cylinder and cone pipelines were checked at input capture, fitting,
stored trims, periodic material, native qualification, history and publication.
The review followed snapshot and lookup calls inside the region, span, edge
and image loops. A proof that validates new geometry can still require a full
pass over that geometry. A span that only needs to identify an already captured
source can use its immutable coefficient record or an indexed reference.

| Path | Repeated work found | Change |
| --- | --- | --- |
| Cone chart, boundary, trim and material proofs | Each fitted span copied a chart's complete coefficient list. Material rebinding repeatedly built the serialized list. | Each chart owns one immutable ordered source list. Receipts share it. Material retains one chart binding and span-local inputs. Independent captures compare every coefficient bit. |
| Cone directed transitions and cap ownership | Every span rebuilt the full side-ID list. Cap and side ownership were repeatedly searched by ID. | Reuse the issued side index and build one cap index. Serialize the full lists only to report a mismatch. |
| Cone source reconstruction | Clearance scanned the complete set of original curves for each fitted span. Storage and material searched each member's curves again. | Index authored curve ownership once for clearance. Reuse positions into each original member for retained proof queries. |
| Native cone faces, cap edges and rulings | Each face or edge searched the native member. Shared rulings were found by comparing all pairs of sides. | Index exact native identity and incidence. Visit side pairs that actually share an edge in the original pair order. |
| Cylinder periodic clearance | Every translated image retained its own copy of the same authored curves and intervals. | Share the source snapshot. Keep separate geometric proof, translation and work receipts for every image. |
| Shared storage and cylinder placement and material | Repeated linear region and shape lookups ran inside member or witness loops. | Index the member owners and use the existing sorted archive keys and sorted region IDs for lookup. |

This work leaves the arithmetic and geometry policies intact. Native graph
round trips and body validation still run at the ownership boundaries. The
geometric pair census and bounded intersection proofs keep their own search
and work policies. Wider cone charts and general attachment trims are separate
requirements.

348 selected tests passed, including inward and outward 239-curve cone
text and the cylinder's periodic material checks. An independent before-and-after
replay of the curved, holed cone matched its BRep, history and budget byte for
byte. Three local pairs on the outward text case measured median API time at
14.94 s before and 14.63 s after. Median process peak RSS rose from 200,360 to
201,716 KiB. In a separate sampled run, resident file pages accounted for
1,060 KiB of the 1,080 KiB difference at the observed peaks. The paired result
still needs to clear the peak-memory gate. [Audit receipt](references/source-incidence-audit-2026-11-20.json)
