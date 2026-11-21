# Field origin for cylindrical boundary flux

Status: correction to the unpublished cylindrical operation, #528. This decision
supersedes the world-origin interpretation of placed flux in the cylindrical
measure documents. The canonical mathematical certificate remains unchanged.

## Decision

The reported `boundary_flux` integrates `F_t(x) = (x-t)/3` over the retained
patch chain, with its declared orientations and parameter multiplicity. Here
`t` is the translation of the actual stored and decoded elementary location.
For the canonical identity graph, `t = 0`.

`WrappedNormalResult::boundary_flux_origin()` and
`WrappedChartNormalResult::boundary_flux_origin()` return this origin. The
normal result reads its placement budget or returns canonical zero. The chart
result reads its own rotation-aware placement budget. Both budgets already own
the stored translation; the implementation stores no second copy of that state.
Frozen correspondence checks require the exact matrix and translation bits and
shared datum identity on the root and every semantic binding. Removing that
datum must reproduce the canonical archive byte for byte.

This removes the incorrect world-origin interpretation of determinant-only
transport. It adds no vector-area integration or alternate geometry engine.
Existing enclosures, integration work, discrepancy budgets and geometry remain
unchanged. The field origin is an additive getter; existing report fields and
method signatures remain available.

## Why the origin matters

Let a canonical patch chain be `Gamma`, with local flux `Q` of `y/3` and
oriented vector area `N = integral_Gamma n dS`. Under `x = A*y+t`, the exact
world-origin flux is

```text
integral_(A*Gamma+t) (x/3) . n dS
    = det(A)*Q + (t/3) . cofactor(A)*N.
```

Topological closure does not imply that the represented cap and side patches
close geometrically. A curve/pcurve gap can leave `N` nonzero. Omitting the
second term therefore cannot certify the world-origin quantity.

For the chosen field, translation cancels before integration:

```text
F_t(A*y+t) = A*y/3
(A*y) . cofactor(A)*(Su cross Sv) = det(A)*y . (Su cross Sv)
Q_t = det(A)*Q.
```

These identities hold for any nonsingular stored `A`, including its coefficient
rounding. They do not require `Gamma` to close. The existing positive determinant
interval transports the local enclosure, representative and integration error.
The placement code describes this as centered-field transport.

## Comparison with the ideal normal solid

The independent target is the closed ideal normal extrusion of the normalized
profile. Since `div(F_t) = 1` for every constant `t`, its outward flux is its
volume, including when `t` differs from the ideal support origin. The ideal rigid
placement preserves that volume. Comparing the transported interval with the
unchanged ideal-volume interval therefore retains the existing discrepancy
bound. The authored-profile displacement error remains separate.

This argument uses closure of the independently certified ideal solid. It does
not infer geometric closure of the native patch chain. A completed patch-flux
comparison is not a certified native-body volume; native volume integration
continues to report a separate estimate.

## Independent controls

For `S(u,v)=(u+v,u*v,1+v/2)` on the unit square, direct symbolic differentiation
gives vector area `(1/4,-1/2,0)` and flux `1/24`. With

```text
A = [[1,1/2,0], [0,1,1/2], [1/2,0,1]]
t = (1048576,-2097152,524288)
```

`det(A)=9/8`, so the centered flux is exactly `3/64`. The world-origin flux is
`102760457/192`. The retained polynomial integrator independently encloses both
quantities from their actual dyadic control points, using zero subdivision
cells. Before the correction, a regression demanding the documented
world-origin value failed: the transported interval enclosed `0.046875`, while
the world-origin interval enclosed about `535210.713542`.

Public-operation controls use curved profiles with a hole, an independent
second region, large translations and admitted oblique and chart frames. They
read the field origin, compare its bits with the decoded root and retained
budget, and check the unchanged ideal-volume comparison. A changed translation
cannot reuse the original correspondence receipt. Point evaluations in these
controls are numerical regression checks; the affine identity and polynomial
integrals supply the mathematical evidence.
