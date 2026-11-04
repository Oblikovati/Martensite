//! Dedicated fixed-lighting bytecode keeps ordinary Phong shader binaries unchanged.
use super::pipeline_variants::PipelineVariant;
macro_rules! fixed_pair {
    ($stem:literal) => {
        [
            include_bytes!(concat!("../../shaders/", $stem, ".vert.spv")).as_slice(),
            include_bytes!(concat!("../../shaders/", $stem, ".frag.spv")).as_slice(),
        ]
    };
}
pub(super) fn fixed_lighting_pair(kind: PipelineVariant) -> [&'static [u8]; 2] {
    if kind.surface_texture || kind.surface_alpha || kind.mesh_edges {
        return fixed_surface_pair(kind.clipped, kind.surface_texture);
    }
    match (kind.clipped, kind.point, kind.line) {
        (false, true, _) => fixed_pair!("lighting.fixed.point"),
        (true, true, _) => fixed_pair!("lighting.fixed.point.clipped"),
        (false, false, true) => fixed_pair!("lighting.fixed.line"),
        (true, false, true) => fixed_pair!("lighting.fixed.line.clipped"),
        (false, false, false) => fixed_pair!("lighting.fixed"),
        (true, false, false) => fixed_pair!("lighting.fixed.clipped"),
    }
}
fn fixed_surface_pair(clipped: bool, image: bool) -> [&'static [u8]; 2] {
    match (clipped, image) {
        (false, false) => fixed_pair!("surface.fixed"),
        (false, true) => fixed_pair!("surface.fixed.image"),
        (true, false) => fixed_pair!("surface.fixed.clipped"),
        (true, true) => fixed_pair!("surface.fixed.clipped.image"),
    }
}
macro_rules! sum_order_pair {
    ($stem:literal) => {
        (
            include_bytes!(concat!("../../shaders/", $stem, ".vert.spv")).as_slice(),
            include_bytes!(concat!("../../shaders/", $stem, ".sum-order-v4.vert.spv")).as_slice(),
        )
    };
}
/// The vertex stage of a lit fixed-function draw under `fixed-model-view-v4`.
pub(super) fn fixed_sum_order_vertex(stock: &[u8]) -> Option<&'static [u8]> {
    [
        sum_order_pair!("lighting.fixed"),
        sum_order_pair!("lighting.fixed.clipped"),
        sum_order_pair!("lighting.fixed.line"),
        sum_order_pair!("lighting.fixed.line.clipped"),
        sum_order_pair!("lighting.fixed.point"),
        sum_order_pair!("lighting.fixed.point.clipped"),
        sum_order_pair!("surface.fixed"),
        sum_order_pair!("surface.fixed.clipped"),
        sum_order_pair!("surface.fixed.image"),
        sum_order_pair!("surface.fixed.clipped.image"),
    ]
    .into_iter()
    .find_map(|(before, selected)| (before == stock).then_some(selected))
}
/// The fragment of a textured fixed-function surface under `fixed-model-view-v4`.
pub(super) fn fixed_reciprocal_fragment(stock: &[u8]) -> Option<&'static [u8]> {
    [
        (
            include_bytes!("../../shaders/surface.fixed.image.frag.spv").as_slice(),
            include_bytes!("../../shaders/surface.fixed.image.reciprocal-q-v4.frag.spv").as_slice(),
        ),
        (
            include_bytes!("../../shaders/surface.fixed.clipped.image.frag.spv").as_slice(),
            include_bytes!("../../shaders/surface.fixed.clipped.image.reciprocal-q-v4.frag.spv")
                .as_slice(),
        ),
    ]
    .into_iter()
    .find_map(|(before, selected)| (before == stock).then_some(selected))
}
#[cfg(test)]
#[test]
fn version_four_modules_replace_only_their_stock_stage() {
    let [vertex, fragment] = fixed_surface_pair(false, true);
    assert_ne!(fixed_sum_order_vertex(vertex), Some(vertex));
    assert!(fixed_sum_order_vertex(vertex).is_some());
    assert!(fixed_sum_order_vertex(fragment).is_none());
    assert!(fixed_reciprocal_fragment(fragment).is_some());
    assert!(fixed_reciprocal_fragment(vertex).is_none());
    let untextured = fixed_surface_pair(false, false)[1];
    assert!(fixed_reciprocal_fragment(untextured).is_none());
    let lit = fixed_lighting_pair_for_tests();
    assert!(fixed_sum_order_vertex(lit).is_some());
}
#[cfg(test)]
fn fixed_lighting_pair_for_tests() -> &'static [u8] {
    fixed_pair!("lighting.fixed.clipped")[0]
}
pub(super) fn fixed_mesh_triple(kind: PipelineVariant) -> [&'static [u8]; 3] {
    let fragment = match (kind.clipped, kind.surface_texture) {
        (false, false) => include_bytes!("../../shaders/surface.edges.fixed.frag.spv").as_slice(),
        (false, true) => {
            include_bytes!("../../shaders/surface.edges.fixed.image.frag.spv").as_slice()
        }
        (true, false) => {
            include_bytes!("../../shaders/surface.edges.fixed.clipped.frag.spv").as_slice()
        }
        (true, true) => {
            include_bytes!("../../shaders/surface.edges.fixed.clipped.image.frag.spv").as_slice()
        }
    };
    let geometry = if kind.clipped {
        include_bytes!("../../shaders/surface.edges.fixed.clipped.geom.spv").as_slice()
    } else {
        include_bytes!("../../shaders/surface.edges.fixed.geom.spv").as_slice()
    };
    [fixed_lighting_pair(kind)[0], fragment, geometry]
}

pub(super) fn fixed_unlit_line_pair(clipped: bool) -> [&'static [u8]; 2] {
    if clipped {
        return fixed_pair!("line.fixed.clipped");
    }
    fixed_pair!("line.fixed")
}
pub(super) fn fixed_unlit_point_pair(clipped: bool) -> [&'static [u8]; 2] {
    if clipped {
        return fixed_pair!("point.fixed.clipped");
    }
    fixed_pair!("point.fixed")
}
