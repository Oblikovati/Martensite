# Conical accumulation behavior2

The conical reference-volume and analytic-cone flux entries accept
`CylindricalFluxOptions.behavior_version = 2` for bounded outward accumulation
(#570). The default remains 1. The parent receipt reports 2 while its exact scalar
term integrator remains algorithm 1. Other integral and cylindrical operations
retain version 1 admission.

Version 2 rounds each already weighted and oriented exact contribution outward
to the requested dyadic precision, then adds on that grid. It charges the seven
bounded arithmetic calls for each contribution against the existing remaining
grant. The published interval includes this rounding allowance and all retained
term uncertainty. The existing final volume cap remains active; there is no
precision increase or fallback after a bounded-sum refusal.

The source test controls cover signed nonbinary sums, distinct denominators,
large cancellation, rounding-width bounds and work/bit refusals. Leaf controls
check source-policy binding and unchanged term-cell accounting. Complete O.
outward/inward controls select version 2 with the original one-million-unit
budget and the existing geometry, self-intersection, history and isolation
checks. These controls have been written; execution and performance
qualification are pending.

The [kernel decision](../../architecture/decisions/conical-volume-accumulation.md)
records the proof, version boundary and unreleased typed-error stabilization.
The [operand-size diagnostic](references/volume-bits-attribution-2026-11-19/summary.json)
established running-sum growth without attributing all integration time to it.
