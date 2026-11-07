# Kernel decision: adaptive surface integration, numerical behavior 2

Issue [#407](https://github.com/Oblikovati/Martensite/issues/407) blocks the
application's independent per-face area checks. This decision supersedes the
accuracy boundary of [surface policy schema 1](../surface-property-policy/decision.md).
Public option and result shapes remain unchanged.

The general nested Gauss integrator must refine both parameter directions.
Each inner integral must receive a relative error budget before refinement.
Its estimate must be updated after every subdivision. Both outer quadrature
orders must use the same inner convergence rule. Inner errors must survive the
second outer pass and accumulate with the absolute quadrature weights. Errors
from separate intervals and boundary contributions add; selecting only the
largest contribution does not estimate their total.

Remove the uninitialized inner-budget stop rule, stale inner error, overwritten
inner evidence and maximum-only aggregation. Keep one shared integration path
for bounded parametric surfaces. Add no elliptic-surface dispatch, retry engine,
mesh replacement or tolerance widening. Preserve fixed-order integration.

The reported error remains an estimate, not a certified enclosure. Exhausted
refinement remains incomplete and must not be hidden by increasing a budget.
Independent analytic enclosures test numerical behavior 2. Historical behavior-1
artifacts remain under surface-property-policy/legacy-replay for diagnosis;
their inaccurate values are no longer acceptance criteria for adaptive results.

The first corpus case is a half-turn elliptic extrusion with axes 1 and 0.6,
height 2 and area 4 E(16/25). Its area is approximately
5.1053997726796256932357324009980582783918997697584373114570326975.
The earlier result at requested relative error 1e-12 was
5.105596882761290 with reported error 3.4792344170335926e-16.
