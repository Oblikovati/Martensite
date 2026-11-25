# Sketch3D variables and relation equations

Status: proposed contract for Phase 0 (amended on 2026-11-25). It sets the variables, equation rows, rank, singular set
and branch check of each relation that Phases 1 to 3 publish. A relation outside this table is out of scope.

## Why the rows must have full rank

The candidate solver classifies a system from its row count and numerical rank. Any redundant row gives
`OverConstrained`, even when degrees of freedom remain (`crates/solve/src/diagnostics.rs`). A raw vector equation such
as `a × b = 0` has 3 rows and rank 2 at every solution, so one valid `Parallel` relation would be reported as
overconstrained and trigger the redundancy scan on every solve. The same defect occurs for raw point-on-line cross
products (3 rows, rank 2), raw `G2` (6 rows, rank 4), and emitting an endpoint incidence row for two lines that already
share that point identity (app ADR-0034).

Each relation below therefore emits as many rows as its rank on its regular set, and eliminates identity-supplied
incidences before AD and rank analysis.

## Notation and smooth local frame

- `d` is a line vector, `u(v)` is `v / |v|`, and `n` is a fixed unit plane normal.
- Smooth orthonormal frame `(e1(d), e2(d))`: pick a fixed unit reference axis `r` once at solve start so `|r·u(d₀)|` is
  minimal. For two-vector relations with non-collinear initial vectors `d1₀` and `d2₀`, pick `r = u(d1₀ × d2₀)` so `r`
  is orthogonal to the initial rotation plane; otherwise pick the coordinate axis `X`, `Y` or `Z` minimizing
  `|r·u(d₀)|`, giving `|r·u(d₀)| ≤ 1/√3`. Define:
  - `w(d) = r − (r·u(d)) u(d)`
  - `e1(d) = w(d) / |w(d)|`
  - `e2(d) = u(d) × e1(d)`
  Differentiate both `e1(d)` and `e2(d)` through AD as smooth functions of `d` instead of freezing them at `d₀`.
  - Because `{u(d), e1(d), e2(d)}` stays an exact orthonormal basis of `R³` for every `d` with `|r·u(d)| < 1`, equations
    of the form `[e1(d)·v, e2(d)·v] = [0, 0]` have no spurious roots anywhere off the isolated poles `u(d) = ±r` and
    keep full rank 2 through 90° and wider rotations. Freezing `E(d₀)` admits spurious perpendicular roots across the
    equator `u(d) ⊥ d₀` and drops to rank 1 at a 90° rotation (E8).
  - If any iterate enters `|r·u(d)| > 0.9` (within about 25.8° of the pole `±r`), return a retryable domain or frame
    signal so the solver refreshes `r` and continues without leaving a singular pole. Post-solve validation still checks
    the frame-free 3D vector invariants (`|u1 × u2|`, `|(P−A) × u(d)|`).
- Every directional relation normalizes `d` by its current length `|d|`, including axis and plane parallelism.
  Unnormalized `d` components let a line satisfy a directional constraint by collapsing to zero length and mix length
  units with radians.

## Variables

| Entity | Variables | Intrinsic rows | DOF | Refused set |
| --- | --- | --- | ---: | --- |
| Point (`Defining`, `SketchMarker`, `CenterMarker`) | `P` (3) | none | 3 | non-finite values |
| Line | two distinct point identities | none | 6 with own endpoints | same endpoint identity; `\|d\|` below minimum length |
| Circular arc (3-point) | endpoints `S`, `E` and sagitta vector `h` (9) | `u(E−S)·h = 0` (1, length) | 8 | `\|E−S\|` or `\|h\|` below limit |
| Circular arc (center-point) | `S`, `E`, `h` (9) and center `C` (3) | `u(E−S)·h = 0` (1) and `C − C_derived(S,E,h) = 0` (3) | 8 | `\|E−S\|` or `\|h\|` below limit (see semicircle note) |
| Control-pole spline | poles (3 each); degree, knots and weights fixed | none | 3 per pole | coincident end poles; zero end derivative when tangent or G2 attached |
| Fit-point spline | fit points (3 each); knot and parameter vector fixed during solve | none | 3 per point | duplicate adjacent points; ill-conditioned chord-ratio drift (see note) |
| Attached point (`On Face`) | `P` (3) and surface parameters `(u, v)` (2) | `P − F(u, v) = 0` (3, length) | 2 | `\|F_u × F_v\|` below limit; outside or inconclusive trim |

### Arc notes

- With `d = E − S`, `(S + E) / 2 + h` lies on the arc. The radius is `(|d|²/4 + |h|²) / (2|h|)` and the derived center is
  `C_derived = (S + E) / 2 + h (|h|² − |d|²/4) / (2|h|²)`.
- End tangents are parallel to `(|d|² − 4|h|²) d ± 4|d|² h` (`+` at `S`, `−` at `E`), with magnitude
  `|d| (|d|² + 4|h|²)`, nonzero for every arc with distinct endpoints.
- Semicircles (`|h| = |d|/2`) and major arcs (`|h| > |d|/2`) are regular in the `(S, E, h)` layout and the sweep is
  unambiguous.
- Semicircle singularities:
  - With fixed `S` and `E`, radius has its minimum at `|h| = |d|/2`, where `∂radius/∂h = 0`. A `Radius` dimension is
    singular there, and no solution exists when `|E − S| > 2r`. Free endpoints remove the singularity.
  - On a center-point arc, `C_derived = (S + E)/2` is independent of the orientation of `h` around chord `SE` at a
    semicircle, so `∂C_derived/∂h` drops from rank 3 to rank 1 (the 4-row intrinsic rank w.r.t. `(S, E, h, C)` stays 4).
    If `C`, `S` and `E` are all simultaneously fixed at a semicircle, the 13 equations on 12 variables have rank 11: one
    real rotational DOF around diameter `SE` remains alongside 2 redundant rows (`|C−S|=|C−E|` and the midpoint lock).
    Callers cannot assume `Fix(C) + Fix(S) + Fix(E)` fully constrains a semicircular arc.
- A closed circle needs a separate layout (center, unit normal, radius). No captured scenario creates one.

### Fit-point spline notes

- Keeping parameter values `t_i` fixed during a solve makes the B-spline poles a fixed linear combination of the fit
  points (`M P = Q` for fixed `M`), so derivatives `∂C'/∂Q_j` and `∂C''/∂Q_j` are constant linear operators. Because
  `GeomAPI_Interpolate::with_parameters` does not expose `M⁻¹`, the sketch adapter builds or factors the banded
  interpolation matrix directly once per fixed parameter vector and verifies agreement with `GeomAPI_Interpolate`.
- Recipe parameters stay fixed across solve, save and reopen so solved Tangent, G2 and length constraints do not drift on
  reload. If a drag or edit changes adjacent chord-to-parameter ratios beyond a configured distortion bound (set in D8),
  return a diagnostic warning or refuse the edit instead of reparameterizing and invalidating solved constraints.

## Relations

| Relation | Rows (and residual domain) | Rank | Singular or refused set | Branch or acceptance check |
| --- | --- | ---: | --- | --- |
| Coincident, point/point | `P − Q = 0` (3, length) | 3 | both operands have one identity | none |
| Fix, point | `P − P₀ = 0` or hard constant (3, length) | 3 | none | exact equality with stored coordinates |
| Fix, line | hold both endpoints (6, length) | 6 | zero-length line | exact equality |
| Fix, arc | `S − S₀`, `E − E₀` (6, length) and `[e1(E−S)·(h−h₀), e2(E−S)·(h−h₀)]` (2, length) | 8 | zero chord; pole `u(E−S) = ±r` | exact equality; combined with intrinsic row gives rank 9/9 from any start pose |
| Point on line | `[e1(d)·(P−A), e2(d)·(P−A)]` (2, length) | 2 | zero-length line; pole `u(d) = ±r`; `P` shares identity with `A` or `B` | full check `\|(P−A) × u(d)\|` within length tolerance |
| Colinear, 0 shared endpoints (ADR-0034) | `[e1(d1)·u(d2), e2(d1)·u(d2)]` (2, unitless) and `[e1(d1)·(A2−A1), e2(d1)·(A2−A1)]` (2, length) | 4 | zero-length line; pole `u(d1) = ±r` | stored sign of `d1·d2`; full vector parallel and offset check |
| Colinear, 1 shared endpoint (ADR-0034) | `[e1(d1)·u(d2), e2(d1)·u(d2)]` (2, unitless) | 2 | zero-length line; pole `u(d1) = ±r` | stored sign of `d1·d2`; shared identity supplies incidence |
| Colinear, 2 shared endpoints (ADR-0034) | none (0) | 0 | zero-length line | direction validity still checked |
| Parallel, two lines | `[e1(d1)·u(d2), e2(d1)·u(d2)]` (2, unitless) | 2 | zero-length line; pole `u(d1) = ±r` | stored sign of `d1·d2`; full check `\|u(d1) × u(d2)\|` |
| Parallel to X, Y or Z | the other two components of `u(d)` (2, unitless) | 2 | zero-length line | stored sign of the free component of `u(d)` |
| Parallel to a plane | `n·u(d)` (1, unitless) | 1 | zero-length line | plane offset stays free |
| Perpendicular | `u(d1)·u(d2)` (1, unitless) | 1 | zero-length line | none; lines need not intersect |
| Length | `\|d\| − L` (1, length) | 1 | zero-length line, `L ≤ 0` | none |
| Angle | `(u(d1)·u(d2) − cos θ) / sin θ` (1, unitless/rad) | 1 | target `θ ∉ (0, π)`; iterate `\|u1 × u2\| ≤ sin(ang_tol)` | open interval `(0, π)`; smooth everywhere with unit radian slope at solution; applying to initially parallel lines requires an explicit seed plane or named refusal |
| Radius | `radius − r` (1, length) | 1 | `r ≤ 0`; near-straight arc; fixed-endpoint semicircle | stored minor or major arc |
| Equal length | `\|d1\| − \|d2\|` (1, length) | 1 | zero-length line | none |
| Equal radius | `radius1 − radius2` (1, length) | 1 | near-straight arc; both fixed-endpoint semicircles | stored minor or major arc of each operand |
| Tangent at a shared endpoint | `[e1(t1)·u(t2), e2(t1)·u(t2)]` (2, unitless) | 2 | zero end derivative; pole `u(t1) = ±r` | stored sign of `t1·t2` under one endpoint convention; full check `\|u(t1) × u(t2)\|` |
| G2 at a shared endpoint (blocked on capture) | Tangent (2, unitless) and `[e1(t1)·(κ1 − κ2), e2(t1)·(κ1 − κ2)] · L_ref` (2, unitless) | 4 | zero end derivative; pole `u(t1) = ±r`; inadmissible pairs `line/line` (identically zero curvature rows) and `arc/line` (`1/R ≠ 0` infeasible) | requires at least one spline (or curvature-compatible pair); zero curvature on spline/line is regular |
| On Face | attached-point rows `P − F(u, v) = 0` (3, length; 2 new variables) | 3 | singular surface point (`\|F_u × F_v\|` below limit, e.g. sphere pole) | trim classification after solve; requires variable-domain scaling for `(u, v)` |
| Driven measurement | none (0) | 0 | as driving form | not applicable |

Curvature vector: `κ = ((C' × C'') × C') / |C'|⁴`.

## On Face, variable scaling and trims

The candidate solver has no variable bounds and currently applies one global `model_scale` to every variable step (E3).
Before `On Face` ships in Phase 3:

1. Separate variable-domain scaling (length coordinates and surface `(u, v)` parameters) and add column-equilibrated
   rank diagnostics so a small or large NURBS parameter domain far from the origin neither explodes `(u, v)` trial steps
   nor triggers false rank deficiency.
2. Solve with unbounded `(u, v)` on a continuous periodic lift.
3. Classify the solved point against the full trim (outer loop and holes) with explicit uncertainty.
4. Refuse the edit with a named error when the point is outside or classification is inconclusive; keep the last
   accepted snapshot.

An active-set or trim-boundary-sliding rule requires an accepted capture (Capture backlog #8).

At a sphere pole the 3 rows `P − F(u, v) = 0` have Jacobian rank 3 (suggesting 2 DOF), even though `F_u = 0` restricts
first-order surface motion to 1 direction. Rank alone is not a freedom certificate in a refused set.

## Verification performed

All relations in the table above were checked numerically (central-difference Jacobian `eps = 1e-7`, pivoted elimination
rank tolerance `1e-9`; 33 configurations in E8): Coincident, Fix point, Fix line, Arc intrinsic, Fix arc (at solution
and from a 90°-displaced start), Center-point arc (minor, semicircle, major), Parallel (90°-rotated solution and
perpendicular non-root), Point on line (90°-rotated), Colinear (0 shared endpoints with short 0.01 segment, and 1 shared
endpoint), normalized Parallel-X and Parallel-plane, Perpendicular, Length, Equal length, Angle (`θ = 0.01, π/4, π/2, π − 0.01`),
Radius (minor and free-endpoint semicircle), Equal radius, Tangent (control-pole spline/line at 0° and 90°, fit-point
spline/line, arc/line), G2 (spline/line, spline/spline), and On Face (cylinder and sphere). Phase 2 repeats these checks
across generated configurations and production AD or injected residuals.
