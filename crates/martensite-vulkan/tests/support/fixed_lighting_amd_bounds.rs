//! Fixed-lighting fixtures graded by the D2 cross-driver references on the proven AMD profile.
//! Measured 2026-10-27 on AMD Radeon 8060S, RADV and radeonsi Mesa 26.1.6 (NativeAmdMesa26),
//! identically in all four raster/coordinate cells; Vulkan lavapipe replays its native llvmpipe
//! fixtures byte-exactly and is never graded. Observed: the first five round lit colours one
//! level apart on a few pixels; lit-rotation and the three lit-adjacent scenes (a surface rotated
//! 0.7 rad) also differ in depth by up to 1.10e-6 and 5.20e-6, at most half the snapping bound.
pub const FIXED_LIGHTING_AMD_CROSS_DRIVER_CASES: [&str; 8] = [
    "lit-colored",
    "lit-normal-updated",
    "lit-light-updated",
    "lit-alpha-opacity-updated",
    "lit-rotation",
    "lit-adjacent-basic",
    "lit-adjacent-point",
    "lit-adjacent-spot",
];
