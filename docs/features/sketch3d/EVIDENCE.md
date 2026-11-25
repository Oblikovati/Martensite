# Sketch3D plan evidence

Measurements and source findings behind the [implementation plan](IMPLEMENTATION-PLAN.md), recorded on 2026-11-25. The
probes ran in temporary clones or Python scripts and close no acceptance gate.

Revisions: Martensite `d4635266` (`origin/main`; local `main` checkout is at `4b76a8239`, 607 commits behind);
application `4d35f6551` and `1f5db9d56` (branch `rust-work`).

## E1. Silhouette candidate on a block and `ContapContour` state

Input: `BRepPrimAPIMakeBox` 60 × 40 × 10 and `BRepPrimAPIMakeCylinder` radius 5, height 10. Exact HLR
`Algo` with `Projector::with_frame`, then `HLRToShape`. The captured silhouette fixture is a 60 × 40 × 10 mm block.

| Input | View direction | `out_line_v_compound_3d` edges | `v_compound` edges | `h_compound` edges |
| --- | --- | ---: | ---: | ---: |
| Block | (0, 0, 1) | 4 | 4 | 4 |
| Block | (1, 0, 0) | null | 4 | 4 |
| Block | (1, 1, 1) | null | 9 | 3 |
| Cylinder | (1, 0, 0) | 2 | 4 | 2 |

- In OCCT HLR, `out_line_v_compound_3d` carries smooth-surface contour edges; sharp polyhedral silhouettes belong to
  `v_compound`. The Z-versus-X inconsistency on the box is still worth filing as an HLR bug, while 3D Sketch Silhouette
  Curve extracts the pull-direction parting boundary (smooth contours, sharp front/back transitions and a declared
  `n·d = 0` band rule).
- In an axis view each side face has `n·d = 0`. A rule selecting only strict front/back face transitions selects nothing
  there. The help text says faces in the pull direction are ignored; both rims of that zero band are candidates, and no
  capture yet identifies which rim the product selects.
- Thread-safety blocker in `martensite-tkhlr` (`d4635266`): `ContapContour::perform`
  (`crates/martensite-tkhlr/src/contap/contour/mod.rs`) locks `std::io::stdout()` by default and calls
  `function.set_surface(surface)` and `arc_function.set_surface(surface)`, mutating process-wide global sampling bounds
  in `ContapHContTool` instead of using an operation-local `ContapSamplingContext`. `martensite-tkhlr/Cargo.toml` splits
  10 test binaries specifically to avoid cross-test races on that global state. Using `ContapContour` without migrating
  it and `ContapTheSearchInside` to `ContapSamplingContext` violates `AGENTS.md`'s no-global-state rule.

## E2. Solver candidate latency

Input: a 3D chain of `n` points with 10-unit segments in X, Y, Z order, a driving length on each segment, Perpendicular on
every second corner, and the first point fixed. Each sample moves the preference target of the last point and starts from
the previous solution with `solve_with_preference`. 100 samples per size and process (30 for 240 variables), three
processes. Release build, AMD Ryzen AI Max+ 395, one-minute load 5.8 before the first process. All solves converged.

| Variables | Rows | Drag p95, range of three processes (ms) | `redundant_sources` with one duplicated row (ms) |
| ---: | ---: | ---: | ---: |
| 15 | 6 | 0.19 to 0.29 | 0.06 to 0.09 |
| 30 | 13 | 0.64 to 1.15 | 0.60 to 0.84 |
| 60 | 28 | 3.95 to 5.56 | 7.60 to 7.81 |
| 120 | 58 | 26.4 to 28.3 | 103 to 106 |
| 240 | 118 | 114 to 116 | 1348 to 1366 |

This probe ran under load 5.8 (not a quiet host), used only 30 samples at 240 variables, and tested a tridiagonal 1D
line chain without arcs, splines or `On Face` surface queries. Phase 0 `X1` repeats the measurement on a quiet host with
at least 100 samples and mixed-curve and surface fixtures.

## E3. Solver candidate interface and architecture

Inspected in `oblikovati-rust` (`crates/solve/src/` and `architecture/decisions/`):

- `ResidualEvaluation` stores a dense row-major Jacobian (`residual.rs`), and `linalg` is dense column-pivoted QR.
  `DualNumber` (`ad.rs`, `residual.rs:121`) also seeds every referenced variable with a global-width dense `Vec<f64>` of
  length `values.len()`, allocating `O(N)` heap vectors per elementary op even when a block touches only 3 to 12 local
  variables.
- In `solve_with_preference` (`nonlinear/preference.rs`), each outer step adds the full undamped tangent step
  (`value + delta`) and runs a complete nested `iteration::run`. If curvature makes `next_distance >= distance` or causes
  non-convergence, it returns `PreferenceTermination::Stalled` immediately with no step halving.
- `model_scale` (`nonlinear/metrics.rs`, `nonlinear/step.rs`) takes `max(|v_i|)` over all variables and multiplies every
  variable step by `model_scale`, including non-length `(u, v)` surface parameters. On a small B-spline face
  (`u, v ∈ [0, 1]`) placed far from the origin, trial steps in `(u, v)` and `Unitless` Jacobian rows scale with
  coordinate magnitude, breaking translation invariance and destabilizing `On Face`. `analyze_structure` scales rows
  without column equilibration.
- `ResidualSource::evaluate(&self, ...)` is immutable, `iteration::run` has no mid-solve continuation hook, and no
  variable bounds or inequalities exist in the solve path.
- `DualNumber` has no direct value-plus-gradient constructor, and `sqrt`, `hypot`, `abs` and `atan2` use a zero-gradient
  convention at nonsmooth points (`ad/nonlinear.rs`).
- `status` is `OverConstrained` whenever rows exceed rank. Connected-component handling exists only for 2D corner edits
  (`crates/model/src/sketch/corner_component.rs`), outside `oblikovati-solve`. Accepted app ADRs that constrain Sketch3D
  include:
  - `ADR-0015`: gives solver ownership to `oblikovati-solve`;
  - `ADR-0017`: separates standalone point markers from curve defining points and defines deletion closure;
  - `ADR-0034`: eliminates shared-endpoint incidence rows in `Colinear` before AD and rank analysis;
  - `ADR-0021`, `ADR-0025`, `ADR-0036` and `ADR-0041`: semantic topology keys, typed spline authoring, Martensite
    release catalog verification, and planar profile extrusion lowering.
- `oblikovati-solve` is `GPL-2.0-only`, depends on `num-rational = "=0.4.2"` and `num-traits = "=0.2.19"`, and sits in a
  workspace with `rust-version = "1.99.0"`. Martensite is `GPL-2.0-only` with an Apache-2.0 option for official release
  binaries, sets `rust-version = "1.98.1"`, enables `missing_docs = "warn"`, and requires
  `cargo deny --all-features check` and `cargo audit` on added dependencies. Constants and fixtures carry Go-derived
  provenance (`tests/fixtures/go-*.tsv`, `#3703`).

## E4. Section and projection candidates (`d4635266`)

- `BRepProjProjection::along` (`crates/martensite-tkbool/src/brep_proj/projection.rs`) translates the wire by `−m` and
  sweeps by `2m`, covering both sides; it drops the section object (losing ancestry), gives no source-parameter map, and
  collapses empty and not-done sections into `Ok` with `is_done() == false`.
- Building a dedicated adapter on `BRepSweepPrism` and `BRepAlgoAPISection` still faces three source-verified constraints:
  1. Edges parallel or tangent to the projection direction create degenerate `GeomSurfaceOfLinearExtrusion` faces.
  2. `BRepSweepTranslation::side_surface` (`crates/martensite-tkprim/src/brep_sweep/translation_geometry.rs`) canonizes
     extruded lines and circles into `GeomPlane` and `GeomCylindricalSurface` when `canonize = true`, losing direct
     `u`-parameter correspondence unless `canonize = false` or an explicit remap is kept.
  3. `BRepAlgoAPISection::ancestor_faces` (`crates/martensite-tkbo/src/brep_algo_api/section.rs`) inspects only
     `interf_ff_records()`. Section edges lying on existing face boundaries (common blocks and edge-face results of
     `BOPAlgoSection::build_section`) return `Ok(None)` for face ancestry.

## E5. Helix, Sweep, Loft, profile extrusion and app spatial code

- In `martensite-tkhelix` (`d4635266`), `HelixGeomHelixCurve` and `HelixBRepBuilderHelix` build cylindrical and conical
  (tapered 3D) helices with constant pitch per part, reported tolerance and integer status codes. They cannot build a
  planar Archimedean spiral: `set_parameters_spiral` builds a conical 3D helix (`spiral_diameters` divides by
  `sum(heights)`), `part_geometry` computes `atan(0.5*(d_next - d_this)/height)`, and `part_error`
  (`builder_helix_perform.rs`) explicitly rejects `taper_angle > PI/2 - 1e-7` (status 13), `height < tolerance`
  (status 12) and `pitch < tolerance` (status 11). `HelixBRepBuilderHelix` also has no cancellation support. `tkg3d`
  provides `GeomEvalCircularHelixCurve` (constant radius and pitch).
- `BRepOffsetAPIMakePipeShell` (`crates/martensite-tkoffset/src/brep_offset_api/make_pipe_shell.rs`) has corner
  transition modes, ignores `ProgressRange` (`_range`), uses legacy tolerances `(1e-4, 1e-4, 1e-2)` and boolean
  `is_done()`, and does not return a typed Martensite operation result.
- `BRepOffsetAPIThruSections` (`crates/martensite-tkoffset/src/brep_offset_api/thru_sections/mod.rs`, `fb7dd57f`)
  takes ordered wires and supports ruled or smoothed shells and solids. It checks section compatibility and reports
  generated shapes for section edges and vertices. It defaults to mutable input and ignores `ProgressRange` in `build`.
  Set `set_mutable_input(false)` and verify source isolation before adapting it to published Sketch3D wires.
- `tkbo::profile_extrusion::ProfileCurve` takes 2D `[f64; 2]` plane coordinates with caller IDs and enforces strict
  admission (clamped B-splines ≤ degree 16, explicit sew tolerance).
- `tkgeomalgo::geom_api::Interpolate::with_parameters` supports fixed fit-point parameters, without exposing a
  pole-to-fit-point Jacobian for solver derivative injection.
- `oblikovati-rust` stores spatial points and lines in `crates/model/src/spatial_sketch.rs`, which allows
  `endpoints[0] == endpoints[1]` in the inspected snapshots. The Corliss reader and document projection helpers in
  `crates/persistence/src/inventor_spatial.rs` have persistence-test callers at `4617d26df`. Desktop Inventor interop
  is pending and will connect them to the native Sketch3D recipe (`#4092`).
- `oblikovati-rust` has no Sweep feature at `4d35f6551` or `1f5db9d56`. These snapshots pin Martensite `bef25f47` and
  `0e72f866`, respectively, and neither depends on `martensite-tksketch`. No DWG/DXF importer exists in Martensite.

## E6. Corpus stability

The corpus share has no revision control and changed during this assessment (`line/FIXTURES.md`,
`shared-tool-lifecycle/FIXTURES.md`, and two SVGs). [CORPUS.sha256](CORPUS.sha256) was refreshed at 2026-11-25 19:58
and verified clean against all 191 files. Its SHA-256 is
`08e13ddf9e26906d47b328dd434c7ccfd8b1ccd678115f963c4e3f8f9573965c`. A later check found differences in four files in
`shared-tool-lifecycle` (`BLOCKERS.md`, `FIXTURES.md`, `FUNCTIONAL.md`, and `shared-tool-lifecycle.feature`). The manifest
identifies the assessment inputs. Kernel acceptance uses versioned fixtures derived from the application scenarios.
Run these commands from `features/sketch3d` to list current inputs and compare them with the assessment manifest.

```sh
find . -type f \( -name '*.feature' -o -name README.md -o -name FUNCTIONAL.md -o -name FIXTURES.md \
  -o -name VISUAL.md -o -name BLOCKERS.md \) -printf '%P\n' | LC_ALL=C sort | xargs -d '\n' sha256sum
sha256sum --check --quiet /path/to/CORPUS.sha256
```

## E7. Related kernel and legacy issues

- Martensite: [#20](https://github.com/Oblikovati/Martensite/issues/20) (persistent-reference history),
  [#21](https://github.com/Oblikovati/Martensite/issues/21) (typed tolerances),
  [#65](https://github.com/Oblikovati/Martensite/issues/65) (immutable published geometry),
  [#294](https://github.com/Oblikovati/Martensite/issues/294) (planar-spline section 161 to 164 ms vs 100 ms limit),
  [#390](https://github.com/Oblikovati/Martensite/issues/390) and
  [#494](https://github.com/Oblikovati/Martensite/issues/494) (sketch-region extrusion and source uncertainty),
  [#406](https://github.com/Oblikovati/Martensite/issues/406) (conic point extrema minimum classification),
  [#457](https://github.com/Oblikovati/Martensite/issues/457) (non-destructive Boolean changes after source vertex mutation),
  [#495](https://github.com/Oblikovati/Martensite/issues/495) (identity-preserving B-rep persistence),
  [#550](https://github.com/Oblikovati/Martensite/issues/550) (`Direction3` zero-after-overflow),
  [#583](https://github.com/Oblikovati/Martensite/issues/583) to #590 (`tkernel` 3x to 137x overhead vs OCCT).
- Oblikovati: [#4004](https://github.com/Oblikovati/Oblikovati/issues/4004) (refuse unsupported Go `model.sketches3D`),
  [#3317](https://github.com/Oblikovati/Oblikovati/issues/3317) (Go intersection curve silent marching-squares fallback).

## E8. Numerical equation checks

Finite-difference Jacobian checks (central difference `eps = 1e-7`, pivoted elimination rank tolerance `1e-9`) verified:

- Frozen-frame baseline (rejected): `E(d₀)·(u(d1)×u(d2))` had exact `[0, 0]` residual on perpendicular lines
  `d1 = Z, d2 = Y` (`d1₀ = X`) and dropped to rank 1 at the true 90°-rotated solution; `E(d₀)·((P−A)×u(d))` had `[0, 0]`
  residual for a point 10 mm off a rotated line; 4-row `Colinear` dropped to rank 2 on connected segments;
  `atan2(|u1×u2|, u1·u2) − π/2` had zero central-difference Jacobian and a `+1/-1` kink at `u1 ∥ u2`.
- Smooth-frame formulation ([RELATION-EQUATIONS.md](RELATION-EQUATIONS.md)): all 33 tested regular, 90°-rotated,
  short-segment, minor/semicircle/major arc, spline and surface cases reached exact declared row counts and full rank,
  including `Fix arc` from a 90°-displaced start (rank 9/9), `Colinear` with 0 shared endpoints (rank 4/4) and 1 shared
  endpoint (rank 2/2), normalized `Parallel-X` and `Parallel-plane`, `(u1·u2 − cos θ)/sin θ` across
  `θ ∈ [0.01, π − 0.01]` (unit slope `1.000`), center-point arc (rank 4/4, with `∂C/∂h` rank drop from 3 to 1
  documented at fixed-endpoint semicircles), fit-point spline tangent, arc/line tangent, and `G2` spline/line and
  spline/spline.

## E9. Phase 0 application baseline

On Linux x86_64 with Rust `1.99.0`, Oblikovati `76f5e59a6` passed 5 spatial model tests, 6 Corliss reader and
projection tests, and 20 sketch package tests. These checks cover shared endpoint identity, stable document IDs,
native save/reopen and refusal of unsupported Go spatial sections. `Sketch.Create3D` is still unavailable in the
native ribbon. The test results establish the model and persistence baseline before desktop integration in
[Oblikovati #4190](https://github.com/Oblikovati/Oblikovati/issues/4190).

The app branch then added same-ID endpoint refusal and corrected two stale recipe diagnostic assertions. At
`5458bc91c`, the complete `oblikovati-model` and `oblikovati-persistence` run passed 2,466 tests and left four
existing tests ignored. The solver experiment's unmodified app baseline and local-AD adapter each passed 1,954
tests, failed the same two diagnostic assertions, and ignored the same four. Both affected targets passed when
run with the diagnostic-only corrections. [Oblikovati #4191](https://github.com/Oblikovati/Oblikovati/issues/4191)

```sh
cargo test --locked -p oblikovati-model -p oblikovati-persistence \
  --test spatial_sketch --test inventor_spatial --test package_sketches
```

The live application tooling corpus now has 243 scenario definitions. The assessed scenario `Identify a point from
another sketch` was split into `Identify a point from another 3D sketch` and `Identify a planar sketch point from a
3D sketch`. The 27 shifted references and both revised names belong to the app's scenario inventory. Preserve the
historical map and use the revised names when tracing new fixtures. [Oblikovati #4038](https://github.com/Oblikovati/Oblikovati/issues/4038)

## E10. Sweep and Loft builder baseline

On Martensite `fb7dd57f`, Linux x86_64 and Rust `1.99.0`, four loft and two pipe-shell regression tests passed. A
separate five-test probe used line-based wires like those Sketch3D will supply. A three-segment path through
`(0,0,0)`, `(4,0,0)`, `(4,4,0)` and `(4,4,4)` produced a valid round-corner shell with the end sections centered
at the path endpoints. A ruled loft of side-2 squares at `z=0` and `z=3` had volume 12. Rebuilding with the second
square at `z=5` had volume 20. The measured volumes agreed with `4 × height` within the integration error allowance.

The resulting shape archives in those fixtures stayed identical after a source vertex edit. A separate closed-spine
control exposed an input mutation. Given a wire that starts and ends at the same vertex with its `Closed` flag unset,
the pipe-shell constructor set that flag on the caller's wire. Giving the builder a topology and geometry copy
kept the original flag unchanged. Materialize new wires from the immutable sketch recipe, or copy externally owned
inputs at the adapter boundary. Compose the copy history with the builder history. The controls exercise kernel
builders directly. App references, transaction adoption and desktop timing need their own tests.

```sh
cargo test --locked -p martensite-tkoffset --test toolkit brep_offset_api_thru_sections::
cargo test --locked -p martensite-tkoffset --test toolkit legacy_brep_offset_api_make_pipe_shell::
```

## E11. X3 source revisions and publication

The X3 experiment used actual `Point3` and `Line3` values. It resolved two app recipe points with terminal positions
`(0,20,30)` and `(100,25,50)` mm, then applied independent synthetic source edits. The independent fixture check read
both point keys and their model-space frames from the native scenario part. The app resolves frames, units and document
identities. The kernel consumes copied values with source keys and revision stamps.

All 15 optimized tests passed at `fb7dd57f` on Linux. Two worker completion orders accepted only the latest request.
Equal local IDs from another document, recreated sources, superseded destination recipes and stale source revisions
were refused. Failure and cancellation preserved the last accepted line. Four fresh AB/BA processes produced the same
receipt, SHA-256 `8d839ff5cdf66c2ce1e86acd20faee57a79f060943f2b545e864a81b5f8f762d`. An independent review reran both checks.

In production the document owner issues source authority and incarnation values and versions changes in any input
that affects the resolved point, including frame and units. One owner checks the source stamps, destination generation
and captured tolerance policy before adopting the prepared result. Historic results retain their original stamps.

## E12. X2 section and projection

The X2 experiment used the radius-4 periodic face trimmed to `0 ≤ z ≤ 6`, with a hole around +Y over `2 < z < 4`.
A transverse section returned +Y intervals `[0,2]` and `[4,6]` and the intact −Y generator. Forward projection
returned source intervals `[1,3]` and `[5,7]`. Shared degree-1 knots allowed the actual 3D curve and two pcurves
to be checked over every active span. The outward-rounded bound on the fixture's carrier, source and travel
residuals was `1.147e-13`. The native edge tolerance was `5e-6`. Source parameters ran backward along the result
edge. Tangency, empty intersection, multiple roots, invalid input and a ray direction parallel to its source
line each had a separate control.

The boundary circle and tangent seam lacked public face ancestry. Native interference data recovered the unique
parents using the original edge's face incidence and the common block's other face. The boundary control verified
both operand orders. [#613](https://github.com/Oblikovati/Martensite/issues/613)

The #613 fix is on the Sketch3D branch through `d09d603f`. It binds the face roots to the completed operation.
Eight focused ancestry tests and six ranged API tests passed on the integration branch. The reviewed patch also
passed 147 legacy section and Boolean controls. Pending input edits retain the old result's ancestry. A cancelled
or failed rebuild clears it. The API reports an unresolved pair when common-block lineage admits multiple parents.

Native non-destructive mode preserved input archives during construction. Some output curves still shared pcurve
carriers with the source after construction. Copying the operands with their geometry fixed that input alias.
Copying the standalone output dropped its detached pcurves. One binary shape table with the result and both
parents retained those associations through byte-identical encode/decode. Mutating one decoded export left its
sibling, the sealed archive and the original sources intact. Two processes reproduced all seven BRep exports
and the evidence text, SHA-256 `84a06950b0e6a79c3312c208cdde980e7c70f3f7396265eeb284b31ea2b78ead` for the latter.

The independent review accepted this bounded fixture. The production adapter still needs to certify admitted
carrier and trim intervals, recover all required parents, and refuse unsupported chart, rank and branch cases.
Keep numeric bounds in TKMath, captured support and trim evaluation in TKGeomAlgo, and topology incidence and
traversal in TKTopAlgo. Section common-block ancestry belongs to TKBO. Compose the TKPrim projection sweep and
TKBO section above them so the lower toolkits stay acyclic.

## E13. X4 parting silhouette and Contap isolation

At `fb7dd57f`, a 60 × 40 × 10 block returned eight parting edges for each principal axis and six for `(1,1,1)`.
A radius-5, height-10 cylinder returned two analytic generators for X and both circular rims for Z. Generator
endpoints matched the closed form within `2e-14`. Twelve concurrent rounds reproduced serial endpoint bits and
source ancestry. The fixture admitted complete cylinders and orthogonal blocks. It refused partial-turn or tilted
cylinders and did not implement the owned internal-face and planar-face exclusions.

The reusable Contap change is `9759e0b9` on the Sketch3D branch. It uses operation-local sampling for contour setup,
interior search and walking. Default calls discard diagnostic text, and the explicit writer still receives the
historical message. The native library and scratch suites passed 17 and 5 tests respectively. The same targets
passed under the pinned `nightly-2026-09-25` ThreadSanitizer in a clean Linux container. The companion native checks
passed 20,258 compatibility tests, five dataset tests, 59 doctests, strict TKHLR Clippy and formatting.

Exact HLR remains separate. On the original and migrated code, the visible 3D outline of the same block was null
for X, two edges for Y, four for Z and null for the skew direction. The typed bug tracks those visibility results.
[Martensite #612](https://github.com/Oblikovati/Martensite/issues/612)

## E14. X1 warm solver results

The X1 prototype used the app solver from `76f5e59a6`, with block-local automatic differentiation, a sparse
component factorization, and scale-aware backtracking. Three release processes ran 100 matched updates per fixture
on CPU 3, with 3.7% observed load on its SMT sibling and CPU pressure `avg10=0.00`. The Rust `1.98.1` measurements
include solving and lightweight changed-curve evaluation. Layout construction, target preparation and independent
post-validation were outside the timer.

| Fixture | Dense p95 (ms) | Sparse p95 (ms) |
| --- | ---: | ---: |
| 60-variable chain | 4.280 to 4.726 | 0.229 to 0.233 |
| 120-variable chain | 28.132 to 28.637 | 0.504 to 0.507 |
| 240-variable chain | 120.158 to 133.846 | 0.997 to 1.033 |
| Near-origin arc, spline and UV | 0.295 to 0.309 | 0.074 to 0.102 |

The matched results passed geometry checks and agreed on rank. All 300 far-origin sparse updates passed the
independent geometry check, versus 12 dense updates. The dense model-length scale grows with absolute coordinate
magnitude, so the far-origin comparison qualifies acceptance behavior. The timing comparison uses only the matched
accepted controls. The planar app adapter preserved the existing dense iteration and tested local AD. The static
assembly proof used synthetic joints. The current application assembly code has no numerical solver caller.

The follow-up replaced the preview fallback with capped work and typed deferral. Nine independent rank and refusal
groups passed, including arc singularities, lifted UV and G2. Four focused preview tests covered structural limits,
work exhaustion, cancellation during sparse construction and explicit noninteractive rank recovery. Three further
100-tick release runs for each fixture accepted all 2,100 preview updates. They used no dense factorizations and
reported no missed solver targets. Preview p95 was `0.233–0.368 ms` at 60 variables, `0.511–0.814 ms` at 120 and
`1.010–1.130 ms` at 240. The corresponding dense p95 ranges were `4.374–5.735`, `28.761–33.931` and
`127.070–141.799 ms`. The far-origin acceptance difference persisted.

The tested preview policy admitted up to 240 coordinates, 118 blocks and equations, 12 inputs per block,
and three soft targets. Per update it capped Gram terms at 8,000, factor updates at 2,200, and total charged work
at 250,000. The 240-variable fixture peaked at 5,540, 1,436 and 198,785 respectively. These limits bound the tested
numerical work. Each admitted residual evaluator also needs a work contract. The observations establish a warm
headless latency target. They leave desktop timing, model-space and parameter tolerances, fit-point distortion,
and the app solver migration for their own verification. Bind all equation inputs and tolerance policy to the
immutable request and validate final geometry before adoption.

The extracted `martensite-tksolve` converted the math call sites to Martensite CMath. In three more paired runs
against the pinned app solver, the extracted `Prepared` route accepted all 2,100 updates. The 120-variable
preview was `0.523–0.570 ms` p95 versus `27.835–29.530 ms` in the app baseline. At 240 variables it was
`1.009–1.389 ms` versus `121.823–144.576 ms`. The candidate used no wide dense factorization. Its peak process
RSS was 6,036–6,292 KiB, versus 7,684–7,780 KiB in the dense baseline. The measured run used CPU 3. Its SMT
sibling was about 7.4% busy. The Linux CPU pressure counter increased by 0.139 s over roughly 46 s.

Rust `1.98.1` tests, package verification, dependency checks and strict Clippy passed. The isolated app re-export
of the extracted default API reproduced the 1,960 model outcomes after correcting the two baseline assertions
in #4191. Nine diagnostic rank groups and the bounded preview controls are separate evidence. Sketch3D still needs
a typed evaluator with bounded curve and trim work, revision-bound publication and native desktop timings.

## E15. Native point and line adoption

The `martensite-tksketch3d` value layer landed locally at `55184872`. It holds authored points and lines in
immutable snapshots and validates one atomic batch. The line admission threshold uses the exact squared distance
of the authored binary64 endpoints. Length estimates and the nominal support direction are separate. Unchanged
lines reuse admitted geometry only when their directed IDs and endpoint coordinate bits match. The app owns
identity retirement across revisions and undo. The kernel checks current IDs and reuse within a batch.

The 21 toolkit tests, 12 exact-comparison controls and six triple-product regressions passed under Rust `1.98.1`.
The full TKMath suite and both-package strict Clippy passed. The same toolkit tests passed under pinned TSan in
a clean Linux container. Four AB/BA processes produced identical ordered native receipts. The SHA-256 was
`59037d02840a2f540ff11d755904de3c445071769c32fe88608fba5e0407a315`.

Two optimized benchmark runs per version measured 512 whole-snapshot adopts after warm-up, with one changed point
and all other lines unchanged. At 1,024 lines, p95 fell from `416–423 µs` to `98 µs`. At 256 lines it fell from
`106–109 µs` to `23–24 µs`. The timer excluded app edits, rendering and retiring the previous snapshot.

## E16. Frozen source points

The station-to-station fixture captures `(0,20,30)` mm and connects it to `(100,25,50)` mm in the destination.
After moving and deleting the source, the destination line retains both endpoint IDs, the captured coordinates,
and the source document, sketch, incarnation, revision and point IDs. A direct move of the frozen endpoint
refuses the entire edit batch. Detaching it preserves its coordinate bits and incident lines. Deletion and
orphan pruning clear the captured provenance. The kernel takes the resolved capture from the app, which owns
source resolution and checks the source revision before capture.

The four focused cases passed with the existing 21 toolkit tests on Rust `1.98.1`. Strict toolkit Clippy
and an independent review passed.

## E17. Native constraints and bounded drag

The native point and line constraint slice at `dad1a659` supports Fix, release, coincidence, same- and
opposite-direction parallelism, and driving length. The checked transaction captures Fix anchors from the
current sketch and verifies the final geometry and hard relations before publishing. A frozen point
contributes its exact coordinate bits to a solve and keeps its source stamp. Preview uses a prepared
program with explicit size and residual work limits. A point drag on a sphere has only two reachable
coordinate directions. The soft target solve uses those directions while the hard length equation stays
in the hard system. An extreme finite-coordinate test covers the rounding bounds used by the parallel check.

Three independent warm release processes on Linux x86-64 and Rust `1.98.1` ran 100 updates per size on a
pinned Ryzen AI Max+ 395 core. The solver, whole-snapshot publication and independent relation checks
were inside the timer. All 1,500 updates passed.

| Coordinates | p95 range in ms |
| ---: | ---: |
| 15 | 0.041–0.059 |
| 30 | 0.076–0.077 |
| 60 | 0.140–0.143 |
| 120 | 0.266–0.278 |
| 240 | 0.545–0.587 |

Peak RSS was 7,352–7,536 KiB. At 240 coordinates, the conservative residual bound peaked at 30,692,352
of the allowed 64,000,000. There were 34 native tests and 103 solver tests, with three opt-in tests excluded
from those counts. The focused benchmark ran separately. Strict Clippy and dependency checks passed.
This fixture measures a native line chain. Desktop input, GPU completion and presentation have their own gates.

## E18. Ordered native line wires

The line-wire export at `dc13eeda` resolves directed occurrences by line ID, joins them by point ID and
checks authored segment contact with exact arithmetic within a caller-supplied pair budget. Native readback
verifies the occurrence order, orientation, vertex identities, trim ranges and stored `1e-7` tolerance. It
reports an outward bound on the whole affine trim relative to the authored segment. Authored incidence and
native trim deviation have separate checks. Native self-incidence and minimum clearance require their own proof.

The E10 nonplanar Sweep produced a valid shell at the expected stations and traced a face to its profile edge.
The E10 ruled Loft rebuilt after the second closed section moved. The two valid native solids had volumes 12
and 20 under the fixture's volume check. Both sections traced to output side faces, and later edits or native
vertex mutations left captured inputs and earlier exports intact. These controls use the production builders.
Full consumer validation, curved wires, incompatible sections and application-driven rebuild remain open.

All 35 toolkit tests passed on the wire lane. Independent re-review covered the late cancellation checks during
native point binding publication. Two fresh processes emitted identical 692-byte wire archives with SHA-256
`1ed6890c71ebe742d8e5b7e993d85dbccb6fbe3d73e311a5d8bbc62ae1fdba18`.

## E19. Authored circular arc placement

`731a1487` adds directed three-point and center-point arcs to the immutable spatial snapshot. The three-point
case checks the chord and witness altitude with exact predicates, constructs the nominal circle in scaled
local coordinates, and verifies that the witness is inside its trim. Center-point placement uses a supplied
normal and exact radial orientation. At a zero orientation, exact projected opposition admits a semicircle,
including an oblique frame whose rounded angle would otherwise move by one ULP. For each admitted arc, the
rounded carrier's placement samples fit within the caller's spatial limit. Authored coordinates remain
available independently, and the measured point deviation is a sampled estimate.

The toolkit fixtures cover both corpus placements, major and minor choices, non-XY and oblique semicircles,
very small and large coordinate scales, unrepresentable carriers, over-limit point fit, edit and deletion
closure, cancellation and signed-zero replay. The independent review found the oblique counterexample and
verified exact projected opposition on the corrected code. The point and line solver gives a typed refusal
for an arc layout. Arc equations, spline interaction, native curved wires and desktop authoring have separate
acceptance cases.

## E20. Construction participation

`0842c375` keeps spatial construction roles separate from the point and curve recipes. Changing a role
preserves geometry and identity. A line with the construction role can drive a length in the native solver.
New wire recipes refuse it with its line ID. A previously captured recipe still materializes its own
inputs. The tests also cover atomic cancellation, typed lookup, point-deletion closure and construction
arcs. Independent review, 64 Sketch3D tests and strict toolkit Clippy passed on Rust `1.98.1`.

## E21. Analytic planar spiral

The TKG3d spiral at `f54c64ac7` evaluates the finite radial law from the owned fixture. The radial pitch is
signed and the rotation sense is independent. Admission compares the exact authored turn law and the
rounded radian parameter law at their endpoints. Both have to keep a positive radius through the entire
interval. The curve evaluates native D0 through D3 and DN. For DN the four derivative phases repeat and
bound the work independently of derivative order.

The fixture checks the one-turn cardinal points, initial derivative, reversed rotation, oblique world
placement and an inward radius limit. Public JSON and failed-edit inputs verify that bad finite frames
refuse without changing valid oblique coefficients. All 16 focused tests passed on Rust `1.98.1` and under
pinned ThreadSanitizer. Strict Clippy passed. Two independent processes emitted the same evaluator bytes,
with SHA-256 `0732f1788cfa5599f2083429914deb02597ba94c7e8e1034fa1aca25a6563a40`.
The earlier full TKG3d run passed all 985 tests before the frame admission correction. BRep storage and
the desktop spiral command remain separate checks.

## E22. Checked rigid motion

`11a91acd` adds a checked edit for translating or rotating owned points in a constrained sketch. A shared
point moves once. Geometry admission and the hard-relation checks run on the full draft before an updated
sketch is published. The edit preserves anchors and captured source points, and prepares a new reference
frame for subsequent direction-constrained drag.

The rotation acts on offsets relative to its pivot. Exact axis points and zero-angle rotations keep their
original coordinates. The point-placement check compares the stored output with the exact translation or
with the native binary64 rotation matrix acting on exact offsets. It refuses error beyond the constraint
policy's length limit. Independent controls cover the owned translation and quarter turn, fixed and frozen
refusal, a large pivot, a lost translation unit at `2^53`, wrong selection and cancellation. The native
Parallel fixture also drags after a 90-degree rotation. The toolkit passed 74 tests on Rust `1.98.1`, plus
strict toolkit and TKMath Clippy. A separate package run exercised 1,269 TKMath library and 3,118 toolkit tests.

## E23. Checked relation replay

`b114f6ed` prepares stored geometry and typed relation values for warm drag. It checks relation identities,
operand types, dimensional limits and physical feasibility, then returns the original accepted coordinates.
The supplied Fix anchors are checked bit by bit. Restore uses no numerical solve. The preparation limit
applies before staging the relation list, and cancellation is checked before publication.

A frozen source can retain either a source incarnation and revision or a historical session generation.
The latter preserves the evidence available in an Oblikovati version 18 recipe. The restore fixture keeps
its frozen coordinate bits through a later warm drag. The other cases cover a signed-zero anchor mismatch,
contradictory saved length, malformed relations, size refusal and late cancellation. Independent review,
78 toolkit tests and strict Clippy passed on Rust `1.98.1`. Package and desktop adoption have their own gates.

## E24. Native parting silhouettes

`bb1b2a3a2` returns detached curves with caller-owned source occurrence IDs. The query checks that the
native root is a solid. It uses the original placed plane tangents for oriented face signs and recognizes
cylindrical walls only after checking their stored full UV trim and seam. Along a cylinder's pull axis it
returns both rims. For a radial pull, each returned line has an outward position bound over its whole
trim against the exact placed cylinder. The query compares that bound with the original face tolerance.
It rejects an unsupported or unproved contribution and distinguishes completed empty results.

The fixtures cover principal and diagonal box pulls, the cylinder seam, located geometry, planar and
cavity exclusions, a through-hole, invalid topology, cancellation and source isolation. Native validation
observes cancellation at call boundaries. Two processes produced identical receipt bytes with SHA-256
`7a1d8a15701e4793177cb929d4e92d1aed0ef3aa6ec518eda3434ae373726538`. On Rust `1.98.1`, all 20,361
TKHLR tests and 1,270 TKMath library tests passed. The 22 focused cases also passed pinned ThreadSanitizer.
Independent review and strict TKHLR and TKMath library Clippy passed. App command wiring remains open.

## E25. Ordinary extrusion from a spatial line wire

`tksketch::spatial_prism::extrude_wire` accepts one closed straight-line Sketch3D loop. Exact source
coplanarity precedes projection, and actual native geometry is checked against source XYZ. The ordinary
TKBO prism validates its result in the existing four stages. The adapter binds retained cap and side
trims and computes an outward volume enclosure from their oriented planar flux. Detaching the solid
also detaches the 26 history images in the oblique rectangle fixture.

The review regression requests a displacement halfway between stored coordinates at world X `2^27`.
The operation refuses the resulting cap-vertex trace under the original `1e-8` source limit. At the adjacent
exact displacement, the independent volume 45 lies in the enclosure. Both signs of the oblique sweep and
an oblique non-normal sweep also enclose their independent GProp masses. A cap-only mutation is refused
without a corresponding vertex or edge change.

At exact source `3b55f9ed`, 1,024 optimized kernel runs on Linux x86-64 and Rust `1.98.1` included the
four-line recipe, ordinary extrusion, native trim checks, volume, and detached history. Warm p50/p95 were
`10.038/11.862 ms`, with peak RSS `21,108 KiB`. For 40 independent processes the first operation p95 was
`11.982 ms` and process wall p95 `16.344 ms`. No samples refused. The host ran other builds. The same
fixture had boundary trace at most `2.032e-14` model units and an actual volume enclosure radius of
`7.532e-13` around 75. All 10 focused native cases, two volume unit tests, strict Clippy and the concurrent
detachment ThreadSanitizer control passed. Curved profiles and desktop timing have their own gates.
