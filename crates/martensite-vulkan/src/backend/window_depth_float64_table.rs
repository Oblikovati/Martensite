//! Additive V5 raw-GL vertex and direct-depth fragment registry.
#[rustfmt::skip]
pub(super) const VERTICES: &[(&[u8], &[u8])] = &[
    (include_bytes!("../../shaders/unlit.vert.spv"), include_bytes!("../../shaders/window.depth.v5.unlit.vert.spv")),
    (include_bytes!("../../shaders/lighting.vert.spv"), include_bytes!("../../shaders/window.depth.v5.lighting.vert.spv")),
    (include_bytes!("../../shaders/numeric-max-v2.lighting.vert.spv"), include_bytes!("../../shaders/window.depth.v5.numeric-max-v2.lighting.vert.spv")),
    (include_bytes!("../../shaders/factorized-v2.unlit.vert.spv"), include_bytes!("../../shaders/window.depth.v5.factorized-v2.unlit.vert.spv")),
    (include_bytes!("../../shaders/factorized-v2.lighting.vert.spv"), include_bytes!("../../shaders/window.depth.v5.factorized-v2.lighting.vert.spv")),
    (include_bytes!("../../shaders/factorized-v2.numeric-max-v2.lighting.vert.spv"), include_bytes!("../../shaders/window.depth.v5.factorized-v2.numeric-max-v2.lighting.vert.spv")),
];

#[rustfmt::skip]
pub(super) const FRAGMENTS: &[(&[u8], &[u8])] = &[
    (include_bytes!("../../shaders/unlit.frag.spv"), include_bytes!("../../shaders/window.depth.v5.unlit.frag.spv")),
    (include_bytes!("../../shaders/unlit.coordinates-v2.frag.spv"), include_bytes!("../../shaders/window.depth.v5.unlit.coordinates-v2.frag.spv")),
    (include_bytes!("../../shaders/lighting.frag.spv"), include_bytes!("../../shaders/window.depth.v5.lighting.frag.spv")),
    (include_bytes!("../../shaders/numeric-max-v2.lighting.frag.spv"), include_bytes!("../../shaders/window.depth.v5.numeric-max-v2.lighting.frag.spv")),
    (include_bytes!("../../shaders/lighting.coordinates-v2.frag.spv"), include_bytes!("../../shaders/window.depth.v5.lighting.coordinates-v2.frag.spv")),
    (include_bytes!("../../shaders/numeric-max-v2.lighting.coordinates-v2.frag.spv"), include_bytes!("../../shaders/window.depth.v5.numeric-max-v2.lighting.coordinates-v2.frag.spv")),
    (include_bytes!("../../shaders/unlit.native-alpha.frag.spv"), include_bytes!("../../shaders/window.depth.v5.unlit.native-alpha.frag.spv")),
    (include_bytes!("../../shaders/unlit.native-alpha.coordinates-v2.frag.spv"), include_bytes!("../../shaders/window.depth.v5.unlit.native-alpha.coordinates-v2.frag.spv")),
    (include_bytes!("../../shaders/lighting.native-alpha.frag.spv"), include_bytes!("../../shaders/window.depth.v5.lighting.native-alpha.frag.spv")),
    (include_bytes!("../../shaders/numeric-max-v2.lighting.native-alpha.frag.spv"), include_bytes!("../../shaders/window.depth.v5.numeric-max-v2.lighting.native-alpha.frag.spv")),
    (include_bytes!("../../shaders/lighting.native-alpha.coordinates-v2.frag.spv"), include_bytes!("../../shaders/window.depth.v5.lighting.native-alpha.coordinates-v2.frag.spv")),
    (include_bytes!("../../shaders/numeric-max-v2.lighting.native-alpha.coordinates-v2.frag.spv"), include_bytes!("../../shaders/window.depth.v5.numeric-max-v2.lighting.native-alpha.coordinates-v2.frag.spv")),
];
