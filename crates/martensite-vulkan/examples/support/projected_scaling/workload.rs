//! Dyadic repeated indexed segments keep every accepted endpoint exactly representable.
use martensite_vulkan::{LineStyle, PrimitiveDraw, RenderVertex, identity_clip};
/// Build the fixed accepted workload before measuring; `input(256)`.
pub(super) fn input(count: usize) -> PrimitiveDraw {
    let vertices = (0..count)
        .flat_map(|_| [[-0.5, -0.25, 0.5, 1.], [0.5, 0.25, 0.5, 1.]])
        .map(|point| RenderVertex::homogeneous(point, [0.25, 0.5, 0.75, 1.]))
        .collect();
    let indices = (0..u32::try_from(count * 2).unwrap()).collect();
    let style = LineStyle::new(1., 0xffc0, 1).unwrap();
    PrimitiveDraw::segments(vertices, indices, identity_clip(), style).unwrap()
}
/// Hash every complete output field and relation deterministically; `digest(format!("{output:?}"))`.
pub(super) fn digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xcbf29ce484222325, |state, byte| {
            (state ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
        })
}
