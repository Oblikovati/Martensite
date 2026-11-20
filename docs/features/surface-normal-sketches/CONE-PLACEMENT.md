# Placed cone profiles

`normal_extrude_cone_profile_on_chart` accepts a solved cone profile,
the existing cone execution selector, and an explicit apex, axis, radial
direction, and chart angle. Its companion with cancellation has the same
contract. The kernel normalizes the supplied axis, projects the radial
direction into its perpendicular plane, and uses their right-handed
frame. The requested chart angle rotates this frame about the axis.

The original profile is borrowed during admission. The kernel captures
the selected curves and reserves world-space error before fitting.
The certified point domain bounds canonical slant by the control hull
and then adds the absolute normal depth and the requested spatial
allowance. The shared frame proof accounts for the stored coefficients,
translation, and floating-point point evaluation. The cone runs with
the resulting downward-rounded canonical allowance.

## Frozen shape and reports

The placed archive binds every authored entity to the same decoded
datum as the root. Stripping that datum must reproduce the canonical
BRep bytes and semantic references. The frame proof certifies a positive
determinant, so an exact canonical intersection certificate also
establishes that the located mathematical surfaces preserve that
intersection result. Earlier execution selectors retain the ordered
world checks with native search for self-intersection. The explicit
`DirectedTransportedPlacementV5` selector requests the same chart
construction and uses the transported certificate in that stage.

`canonical()` exposes the canonical proof receipts and authored history.
`placement()` gives the physical reports, the original world limit,
the effective canonical allocation, and the achieved outward spatial
bound. The cap trace check transports each decoded residual using the
stored stretch and two point-evaluation bounds. It checks the result
against the retained edge and face limit for that occurrence.

The world boundary flux is the canonical conditional flux multiplied by
the certified determinant enclosure. Its field origin is the decoded
translation. The ideal authored and reference volumes use the rigid
target frame. Physical native quadrature is reported separately from
the flux proof. Export decodes new world-space owners with the original
authored history keys.

An oblique frame and nonzero chart angle passed for both normal senses
on a curved profile with a hole. Independent world-coordinate checks
confirmed both depth rows at each authored vertex. The outward result
replayed its complete archive, history, and world report in a separate
process. An excessive translation refused during budget allocation.
The previous canonical entry keeps its original admission rules.

The placed atlas control also exported two holed solids after rotation
and translation. Both retained their own chart receipt and passed the
world-space vertex and native checks.

The first oblique control exposed the remaining constant-weight storage
case tracked by #536. The planned and decoded pole bits agreed, but the
native representation replaced an exactly constant non-unit weight with
unit weights. The directed fit now certifies that native representation
before issuing the transition proof. A canonical-only regression
reproduced the mismatch before the fix and passed after it.

The focused suites passed 135 canonical cone tests and 51 placement tests.
Strict Clippy passed. Across the pre-change and candidate builds, the
canonical legacy, recentered, atlas, and cylinder-placement replay
controls produced byte-identical archives, histories, and reports.

Three alternating pairs on the existing directed outward O. fixture
measured median API times of 5.367 s at `5c30a092` and 5.315 s with the
placement changes. Median peak RSS was 67,196 KiB and 66,868 KiB.
Arithmetic operations, integration cells, and retained spans matched
in all six runs. The 5.315 s median falls below the maintainer's revised
5.4 s threshold for this fixture. [Follow-up source and replay audit](FOLLOWUP-PERFORMANCE-CHECKS-2026-11-19.md#native-replay-and-planar-material)


## Transported self-intersection certificate

Use `DirectedTransportedPlacementV5` with a directed domain from the
common, family, recentered or region-atlas choices. The arithmetic
policy remains independent. The world check keeps the same order.
It checks topology and sampled geometry on the placed member, rebinds
the canonical self-intersection certificate, and checks the placed
tolerances. The result reports `Exact` for the intersection stage under
the same bounded-proxy contract as the canonical certificate.

The placed semantic bindings are indexed once for the family. The
certificate step checks that every bound face shares its member's
datum and that the stored linear coefficients match the certified
positive-determinant frame. It removes that datum without inverse
arithmetic, then rebinds the existing proof to the actual canonical
coefficients, cap materials and incidences. An invalid binding returns
its ordered validation report. The support trim checks and the
independent native validation of an attached feature's Boolean result
still run on their own geometry. After history capture, the captured
solid owns the native bindings it needs. The shared attachment pipeline
releases the Boolean workspace and detached tool before it encodes and
validates the result.

The oblique control produced byte-identical placed archives and
authored history with both validation selectors. The refusal controls
cover a different certified matrix, a mixed face datum and changed
source labels. The identity placement and separate-chart atlas also
rebind their certificates.

### Full text attachment measurement

The same assertion-enabled test binary ran three matched outward
boss pairs on Linux x86_64. Each pair used the same pinned CPU core,
cone support, 239 authored curves, 18 material regions and
8,810,496-operation grant. The baseline used native search on the
placed tool and retained the Boolean workspace through validation.
The candidate rebound the tool's certificate and released that
workspace after composing history.

| CPU | Native search time (s) | Rebound certificate time (s) | Native search peak RSS (KiB) | Rebound certificate peak RSS (KiB) |
| --- | ---: | ---: | ---: | ---: |
| 20 | 155.93 | 106.66 | 526,620 | 315,892 |
| 22 | 154.08 | 106.62 | 527,756 | 317,032 |
| 24 | 144.81 | 98.60 | 527,884 | 317,008 |
| Median | 154.08 | 106.62 | 527,756 | 317,008 |

All three pairs reproduced 2,039,762 bytes exactly across the tool
and result archives, composed history, footprint and final
validation. The placed tool's intersection mode changes by design.
The matched candidate used 31% less API time and 40% less peak RSS.
A separate inward pocket also passed with the new mode and retained
all 18 cap faces. It took 101.92 s and peaked at 343,740 KiB.

Certificate reuse alone had saved time in an earlier paired run,
but its median peak RSS rose by 560 KiB. Releasing the Boolean
workspace before final validation brought peak memory below the
baseline. The executed measurement toggles and temporary replay
files were removed. These timings cover the owned text fixture on
the shared host.

On the final build, the separate standalone O. control kept 148
fitted spans and 994,535 operations. Its median API time across
three pinned runs was 4.85 s. Median peak RSS was 67,228 KiB.
