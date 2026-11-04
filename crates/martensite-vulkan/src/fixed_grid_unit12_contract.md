# FixedGridUnit12V3

`RenderPointRasterBehavior::FixedGridUnit12V3` is an explicit replay choice for
plain, one-pixel, single-sample points. The worker selector is
`MARTENSITE_CASE_POINT_RASTER_BEHAVIOR=fixed-grid-unit12-v3`.

The point center is transformed through the existing point pipeline and rounded
to the nearest even coordinate on a 1/4096-pixel lattice. The one-pixel square
uses that center's integer cell; the bottom edge subtracts one lattice unit from
the Y center before finding its cell. Signed coordinates use Euclidean floor.
The viewport is rejected when its 12-bit fixed coordinate domain can overflow.

Only requested size `1.0` is admitted by this mode. A plain single-sample point
with another requested size returns a typed refusal. Sprite, polygon-point,
non-point, and multisample draws retain the existing V2 admission boundary;
`HardwareQuadV1` remains the default. V2 shader modules and replay behavior are
unchanged.

At frozen `568f56179388c1c`, the complete unchanged sphere original passed on
AMD with this mode. Six images matched their retained references exactly; the
last image retained four cyan highlight-line pixel differences. Those pixels
belong to a separate dashed-line draw. This is not whole-case qualification.
The source-bound request, raw output and comparisons are retained under
`~/.cache/martensite/fixed-grid-point-unit12-v3-amd-20261102/r3`.
The earlier non-executable-copy launch failed before test execution and remains
preserved under `r2`; no failed execution was relabelled as successful.

```rust
use martensite_vulkan::RenderPointRasterBehavior;

assert_eq!(
    RenderPointRasterBehavior::FixedGridUnit12V3.behavior_version(),
    3,
);
```
