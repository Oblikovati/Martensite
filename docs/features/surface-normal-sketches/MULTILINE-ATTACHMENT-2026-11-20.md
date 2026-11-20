# Multiline cylindrical attachment controls

The owned `This thing\nworks!` fixture contains 239 polynomial curves and 18
selected material regions. The new #425 controls compose its wrapped bodies
with an existing cylindrical support through the existing non-destructive
Boolean API. They add no modelling implementation or public interface.

## Workload and checks

The support has radius 10 and extends from z=-4 to z=4. Both operations request
depths `[0, 0.25]`, spatial error `1e-5`, maximum fit level 8 and the existing
explicit 8192-pair-bisection allowance. Default allowances remain unchanged.

- Outward extrusion is fused to the support as a boss.
- Inward extrusion is cut from the support as a pocket.
- Both results must pass the existing ordered body checks and contain one solid.
- Every selected region's last-depth cap must have a surviving **face** in the
  result, found through its authored history key and Boolean history. An edge
  or vertex image cannot satisfy this requirement.
- Every reported tool history image must belong to the output. Source support
  and tool snapshots, the published encoded result and the request must remain
  unchanged.

The exact developed area from independent polynomial Green integration is
`42000445/6291456`. The cylinder's radial Jacobian gives volume factors `81/320`
outward and `79/320` inward. The controls check that the published authored-volume
bounds contain `226802403/134217728` and `663607031/402653184`, respectively.
Native Boolean volume quadrature is used only to check the direction of the
volume change; it is not a certified bound or an exact Boolean-volume oracle.

## Executed evidence

Both final controls passed with the face-only history guard. Their combined
test time was 207.16 seconds on the shared development host. This includes
profile construction, Boolean operations, validation and assertions; it is
neither an isolated API timing nor performance acceptance. Earlier attempts
include a weaker passing cap-image assertion and two compile failures. Those
attempts do not qualify the final source and remain in the execution receipt.
Strict Clippy passed for the TKV3D library/toolkit tests and the build driver
with warnings denied. The [receipt](references/multiline-attachment-2026-11-20/summary.json)
binds source hashes, executed commands, reviews and retained failed attempts.

Five existing controls also passed: standalone complete multiline text,
single-glyph boss and pocket, seam crossing, and request edits/replay. The
standalone multiline control took 47.90 seconds including its assertions; the
other four took 28.03 seconds together. No peak-memory claim follows.

These bounded cylinder cases do not establish cone text, a chart atlas,
persistent face-attachment revision handling, full multiline body edits/replay,
or the five-second debug cone gate. Native validation retains its existing
coverage classifications; these tests do not promote sampled checks to exact.

The controls are in
[multiline.rs](../../../crates/martensite-tkv3d/tests/wrapped_text_regions/body_controls/multiline.rs).
