# FixedGridAliasedV2

This opt-in behavior specifies a fixed-grid plain-point operation. HardwareQuadV1
remains the default and replay behavior. This version is not a claim of universal
OpenGL compatibility or an AMD driver model. Its software reference is Mesa 26.1.6
`lp_setup_point.c` legacy point setup, with bottom-edge adjustment zero.

Inputs retain the original requested positive finite size and existing device
range validation. No clamp is introduced. Let `R` be nearest-even integer rounding
of the float32 product, `S=256`, `X=R(window_x*S)`, `Y=R(window_y*S)` and
`F=max(S,(R(size*S)+127)&~255)`. The effective integer extent is `W=F/S`.
For odd W, first pixel index is `floor(X/S)-floor(W/2)`; for even W it is
`floor((X+128)/S)-W/2`, independently on each axis. Floor is mathematical floor,
including negative coordinates. Fixed extent and window grid intermediates must
fit signed 32-bit integers, including the even-width adjustment; values outside
that mathematical domain refuse rather than clamp or overflow. Exactly the W×W inclusive integer pixel box is
covered. Half widths round down after fixed-grid snapping; this deliberately
preserves this version's contract rather than inferring a universal tie rule
from the calibration widths.

Window coordinates use the original signed viewport and logical lower-left pixel
lattice. Reject the original clip center before expansion; retain original Z/W,
world clipping position, UV, color, normal and lighting results at every corner.
An inside-center footprint may extend beyond the original viewport. Its synthetic
quad therefore uses a full-attachment raster viewport with the original depth
range and scissor, then restores the actual viewport for subsequent draws.

Selection requires semantic Points, Plain marker policy and actual single-sample
raster configuration. Sprites, polygon-mode points, multisampling and V1 retain
their original paths. Unknown registered point programs refuse before lowering.
No device identity or environment variable selects this behavior automatically.

CPU rectangles are independent closed-form fixtures. Encoding tests must retain
indexed/repeated attribute order, instance rate and count, original-center clip
rejection, viewport orientation and edge intersections, and immutable inputs.
Shader tests require source/binary agreement and reflected push offsets. Shared
fragment shaders and lighting remain unchanged. Owned GPU operations need a
separate authorized validation run; CPU support is not image qualification.

## Implementation checkpoint

This checkpoint has CPU rectangle/admission/encoding/viewport/push controls and
22 compiled vertex variants, with source/binary validation, push offset reflection,
and byte-identical recompilation of the 22 affected V1 vertex modules. The owned
full-operation GPU fixtures remain ignored in routine CPU runs. Explicit runs on
2026-11-02 passed 11 coverage/clipping frames per driver, then 8 lighting and 2
non-full-viewport/restoration frames per driver, on lavapipe and AMD. All required
RGBA/depth assertions passed with zero validation findings. Translucent diagonal
coverage and the complete original image obligations remain unverified. This is
not release or migration evidence; AMD original-image equivalence remains open.

The successor adds independent synthetic-corner area, non-full viewport,
front/back material, mirrored model, depth-range, ordinary draw restoration,
C/UV/normal/weight and fixed-lighting source controls. The three owned GPU
operations were executed explicitly as described above. An unchanged full sphere
original at `7061dc14377ee7d0eb7700b3a1d05096d36c4a31` also passed on lavapipe:
all seven decoded images matched both retained reference repeats exactly, with
zero validation findings. Its twenty point draws retained requested size 1 and
5,106 instances, exercising Unlit, Gouraud and Phong shading. The recorded source
and executable are distinct from the later Main integration revision.

The same executable subsequently passed the unchanged original's assertions and
validation on AMD, but its seven images still differed by 35, 38, 37, 37, 37, 36
and 40 pixels. This software coverage contract therefore does not establish AMD
reference-image equivalence. Its raw evidence is retained in
`~/.cache/martensite/fixed-grid-point-v2-sphere-amd-20261102`.

Raw requests, captures and before/after input checks are retained under
`~/.cache/martensite/fixed-grid-point-v2-opt2-20261102`,
`fixed-grid-point-v2-lit-region-opt2-20261102` and
`fixed-grid-point-v2-sphere-20261102`. These focused runs do not qualify an
original or change the default behavior. Focused CPU verification is:

```sh
cargo test --locked -p martensite-vulkan --lib fixed_grid_ -- --test-threads=1
python3 -B -m unittest discover -s the recording tool -p test_vulkan_fixed_grid_point_shaders.py -v
```

The GPU cases are intentionally not selected for execution by those commands.
The opt-in viewport and size bounds protect the declared signed fixed-grid
integer operations; they do not change V1 admission or introduce a clamp.
