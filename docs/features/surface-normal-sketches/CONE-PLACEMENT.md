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
intersection result. The separate world validation runs the ordered
native checks and retains their reported modes, including native search
for self-intersection.

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
