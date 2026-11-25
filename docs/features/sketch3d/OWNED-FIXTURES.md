# Sketch3D behavior and owned fixtures

On 2026-11-25 the maintainer chose to define Oblikovati behavior when the application tooling corpus lacks a
committed reference result. The cases below specify the behavior and the expected geometry. Implement the kernel
fixtures in Martensite and the command, undo and persistence scenarios in Oblikovati. Each fixture starts from
authored values in millimetres and converts them at the app boundary. Preview and cancel preserve the accepted
revision. Apply creates one undoable edit.

## Parting silhouette

Include shared edges across opposite face signs and trimmed smooth contours. A face whose normal is perpendicular to
the pull direction throughout is in the zero band. Include both rims of that band where it meets a nonzero face.
Adjacent zero-band faces do not contribute their shared edge. Emit each located edge occurrence once and preserve
both participating face parents.

- A closed box with sides 60, 40 and 10 has eight rim edges along any principal pull direction. Along `(1,1,1)`
  it has six edges, each between faces with opposite signs. Check endpoints against the defining box planes.
- A cylinder of radius 5 and height 10 has two straight generators for an X pull. They run from `z=0` to `z=10`
  at `x=0, y=±5`. A Z pull returns the two radius-5 rims.
- **Exclude straight faces** removes contributions requiring a planar face. The box then returns a completed empty
  result. The cylinder still returns the two smooth-face generators on an X pull.
- **Exclude internal faces** removes contributions from enclosed cavity shells. For a solid box with a strictly
  enclosed box-shaped void, a Z pull returns eight exterior and eight cavity edges by default. With the option
  enabled it returns the eight exterior edges only. A through-hole belongs to the exterior shell. Explicit face
  exclusions apply to any contribution that requires the excluded face.

## G2 continuity

Apply to two splines or a spline and a line at a shared endpoint. Hold the first curve fixed during application and
move the second curve's eligible, unlocked controls. Preserve the shared endpoint and branch orientation. Refuse
singular endpoint derivatives, already incompatible hard constraints, and operand pairs outside this contract.

Use a straight first segment ending at the origin along +X and a cubic Bézier with poles `(0,0,0)`, `(1,0,0)`,
`(2,0,0)`, `(3,1,0)`. The spline's start has tangent +X and zero curvature. Changing its third pole to `(2,1,0)`
preserves the tangent and gives curvature vector `(0,2/3,0)` before the solver restores continuity. In a separate
spline/spline fixture, use left poles `(-3,0,0)`, `(-2,1,0)`, `(-1,0,0)`, `(0,0,0)` and the changed right spline.
Both curvature vectors equal `(0,2/3,0)` at the join. Derive the checks from cubic endpoint derivatives.

## Split and rigid transform

Split at interior transverse intersections with a selected face, work plane or surface. A finite face admits only
hits in its trim. Keep fragments in source parameter order and record their shared source and intervals. A touch
without crossing, or a hit at the existing endpoint, leaves the input unchanged. Refuse an overlap with no unique
cut points.

- A segment from `(-2,0,0)` to `(2,0,0)` splits at `(0,0,0)` against `x=0`. It produces two length-2 segments.
  Repeat with a planar face enclosing the hit, then with a face trimmed to exclude it.
- A segment from `(-3,0,1)` to `(3,0,1)` splits at `x=-1` and `x=1` against a unit Z-axis cylinder. All three
  fragments have length 2.
- On an unconstrained segment from `(1,0,0)` to `(2,0,0)`, translation `(3,-2,1)` gives `(4,-2,1)` and
  `(5,-2,1)`. A separate +90° rotation about Z through the origin gives `(0,1,0)` and `(0,2,0)`. A driving
  length of 1 survives either edit. Fixing the original endpoint makes either edit fail atomically.

## Project to Surface

Expose three named outputs. Directional projection follows the selected positive ray by default. Two-sided mode
chooses the first admissible root by absolute travel. Closest Point chooses the unique global minimizer over the
trim. Surface Normal returns all admissible regular normal feet on the selected faces. Preserve source intervals
and target face occurrences. Refuse unresolved root ties where a mode requires a single branch.

Project `(2,0,t)` for `1 ≤ t ≤ 3` onto a radius-4 Z-axis cylinder trimmed to `0 ≤ z ≤ 6`. A −X direction returns
`(-4,0,t)` on the forward ray. Two-sided mode returns `(4,0,t)`, at travel 2 versus 6. Closest Point returns
`(4,0,t)`. Surface Normal returns both generators. Verify the two stationary roots and interval mapping against
the cylinder equation, then trim one branch with a hole and verify its remaining intervals. Keep the transverse,
boundary and tangent controls in X2.

## Helix, spiral and equation curves

Use a right-handed local frame with signed rotation about +Z. Pitch is axial rise per turn for helices and radial
change per turn for planar spirals. For a planar spiral use `r(t)=r0+pitch*t/(2π)` and angle `rotationSign*t`. Refuse
an interval that reaches a nonpositive radius.

- For one positive turn from radius 4 with radial pitch 2, the cardinal points are `(4,0,0)`, `(0,4.5,0)`,
  `(-5,0,0)`, `(0,-5.5,0)` and `(6,0,0)`. Reversing rotation flips Y at the quarter turns. The initial derivative
  is `(1/π,4,0)` for the positive case.
- A radius-4 cylindrical helix of pitch 2 checks the same angular samples with constant radius and elevations
  0, 0.5, 1, 1.5 and 2. On a tapered helix with radius changing from 4 to 3 in one turn, the midpoint is
  `(-3.5,0,1)`.
- For a flat end, hold axial position over the flat angle. Ramp the axial derivative between zero and the natural
  value over the transition angle using `3u²−2u³` for normalized transition parameter `u`. Reverse the rule at
  the end. Verify elevation, endpoint derivative and continuity at the joins. A 90° ramp advances by one eighth
  of the natural pitch. Refuse overlapping start/end zones. Table rows define ordered constant-pitch spans
  joined at a common radius and elevation. For radius 4, one revolution at pitch 2 followed by half a revolution
  at pitch 4 leaves the intermediate point at `(4,0,2)` and the final point at `(-4,0,4)`. Preserve the derivative
  discontinuity where the pitch changes.
- Equation curves use Cartesian XYZ, cylindrical radius/azimuth/Z and spherical radius/inclination/azimuth.
  Angles are radians at the kernel boundary, with inclination measured from +Z. Use `(t,t²,0)` on `[-1,1]` for
  Cartesian, `(2,t,t)` on `[0,π/2]` for cylindrical, and `(2,π/2,t)` on the same interval for spherical.
  Check endpoint positions, derivatives and achieved error with analytic or interval evaluation. Reject
  singularities, nonfinite results and reversed or empty parameter intervals.

## Imports, attachment drag and cross-sketch references

- Import a three-row XYZ input with `(0,0,0)`, `(4,0,0)` and `(4,3,0)` in millimetres. Point and polyline
  modes retain those coordinates and order. The length of the polyline is 7. A generated drawing in millimetres
  supplies the same two line segments through the app exchange adapter. Refuse nonfinite or unresolved-unit
  input before any entities are published.
- Bind an endpoint to the `z=0` square face spanning `[-4,4]²` with a square hole `(-1,1)²`. Drag from
  `(-2,0,0)` to `(-1,0,0)` and then to `(0,0,0)`. The hole boundary is admissible and the hole interior is
  refused. A later valid request at `(2,0,0)` succeeds. The accepted position survives a refused request
  and a cancelled preview. Include an outer trim check and a curved periodic equivalent.
- Use two source sketches supplying `(0,20,30)` and `(100,25,50)`. Change each source independently as in X3.
  Repeat with one planar source resolved through its placement frame and one spatial source. Check source-local
  invalidation, stale-result refusal, shared endpoint identity where intended, and selection and reference
  creation through the app. Picking priority remains an app assertion.
