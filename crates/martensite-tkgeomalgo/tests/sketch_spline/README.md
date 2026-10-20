# Sketch spline reference fixtures

These fixtures pin the sketch spline constructor against frozen reference values
computed outside this crate. They are retained test resources; no external program
runs during the tests.

## Fit and control recipes

`fit-reference.csv` holds six fit recipes of the application's planar spline
behavior. Cases 0–2 are open and 3–5 are closed, with centripetal, chord-length and
uniform spacing in that order. The source points are
`(0,0), (0.2,1), (2,1.5), (3,-0.5), (1,-2)`. Rows keep the degree and counts, poles,
full knots, stations and all 135 analytic position and first-derivative samples. The
expected values do not come from the Rust constructor.

`control-reference.csv` holds the open and closed five-point control curves of the
same application behavior: 34 sample rows. The open knot vector is the five-point
control policy.

Each row is `case,kind,index,values...`. The `shape` row values are the degree, pole
count, full-knot count and fit-station count. `pole` rows contain x/y; `knot` and
`station` rows contain one scalar. `sample` rows contain the parameter, x/y and the
first-derivative x/y. The fit-station count is zero for control intent.

`fit-evaluation.csv` and `control-evaluation.csv` evaluate those recipes as B-spline
curves with an independent evaluator (position, first and second derivative). Rows
are the case, the parameter, x/y, the first-derivative x/y and the second-derivative
x/y. They check the analytic evaluation of the intended recipe; they do not claim
that another interpolation policy matches it.

The Rust tests compare these independent values with a bound of
`1024 * f64::EPSILON * max(abs(expected), 1)`. Runtime acceptance uses separate
caller-supplied fit and seam limits; the fixture comparison bound is not a production
tolerance policy. The constructor uses the existing LU solver, so bitwise agreement
with another solver is not claimed.

## Active handle corpus

`handle-reference.csv` contains six handle cases (three station policies, open and
closed), 33 station rows and 219 sampled positions. Handles at source indices 0 and 2
change direction and weight; a coincident endpoint at index 4 keeps its natural
derivative. Station rows are `case,station,index,u,x,y,dx,dy`; sample rows are
`case,sample,index,u,x,y`. The sampled points are observations only; production
construction and evaluation stay analytic.

`handle-evaluation.csv` evaluates separate cubic Bezier spans built from those source
positions and effective derivatives, without the constructor's output poles: 243 rows
of `case,span,u,x,y,dx,dy,ddx,ddy`, including both sides of each join. Derivatives
are converted from the local Bezier parameter to the normalized global parameter.
The same floating-point fixture bound applies.

```sh
cargo test -p martensite-tkgeomalgo --test toolkit sketch_spline
```
