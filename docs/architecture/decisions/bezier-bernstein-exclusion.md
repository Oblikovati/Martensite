# Exact Bernstein exclusion beside the versioned pair solver

Status: approved for implementation under the maintainer's #539 delegation on
2026-11-18. Scoped Math and Tkbo tests and strict all-target Clippy passed;
independent component review passed; the full PR gate remains pending. See the
[qualification record](../../features/bezier-sign-exclusion/README.md#qualification).

The cone's captured UV generator has these X-control bits:
`[0, 4599676419421066576, 4604180019048437080, 4607182418800017408]`;
its Y controls are zero. It is a genuine monotone cubic after binary64
interpolation. The existing pair solver correctly reports `NonLinearFiber`:
after exact self-diagonal division its nonzero equation is quadratic in the
paired parameter. Increasing the root budget cannot establish the missing
linear-fibre condition.

Add a separate source-bound Bernstein-exclusion API. Its behavior version 1
accepts the existing pair-policy versions 1 and 2 as nested requests. It reuses
the existing owned homogeneous-source capture, denominator certification and
exact self-diagonal division. The existing `CapturedBezierPair::certify`, its
policy versions, defaults, result types and `BezierPairSignStage` remain
unchanged. A separate report avoids extending that public exhaustive enum or
misrepresenting a new witness as a graph, constant equation or common carrier.

## Proof and admission

For rational coordinates `Nk/D`, an intersection requires both cleared
equations `N0k(t) D1(u) - D0(t) N1k(u)` to vanish. Both denominators must first
be certified nonzero on their actual intervals. A self query divides the
source-derived equations by `t-u` exactly; a distinct query never performs
that division.

Restrict each equation to its actual parameter rectangle with exact affine
substitution. Convert the resulting coefficients to a tensor Bernstein basis
using exact rational arithmetic. If every coefficient of either equation has
one strict sign, the equation has that sign throughout the closed rectangle.
It therefore excludes every intersection there, including the canonical
`t<u` self domain. The report owns the source, original policy, denominator
proofs, tested equations, rectangle, basis degrees, rational coefficient grid
and strict sign.

A zero coefficient, mixed signs or a predicate degree above admission produces
`Inconclusive`. This is a sufficient test; it does not decide every disjoint
pair. Each tensor degree must fit the original primary root-degree cap and,
when present, the original explicit sign-degree cap. All policy versions,
intervals, root widths and world-error requests are checked before an
exclusion can succeed. No effective graph-sign metadata is created.

## Selection and work

The additive report distinguishes `Excluded`, `Inconclusive`, `Invalid`,
`Cancelled` and `Incomplete`. Every outcome retains actual denominator-root
subdivision work. Tensor conversion performs no subdivision and polls for
cancellation during its bounded coefficient loops. An incomplete denominator
proof exposes its original cause and cannot supply an exclusion.

The new graph request version 3 selects this attempt explicitly. Only
`Inconclusive` permits its private census caller to execute the genuine
version-2 pair fallback, with the remaining subdivision budget. The caller
charges both actual receipts once and retains both reports. Existing authored
request version 1 and graph request version 2 retain their original routes.
The request version and census proof mode remain readable; existing polynomial,
annular and prism replay encoders are unchanged. This decision adds no transport
format and authorizes no package-version change.
