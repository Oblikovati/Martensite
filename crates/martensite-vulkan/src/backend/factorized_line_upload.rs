//! Plain lines reach the native rounding boundary on a private upload copy, with stock shaders.
//!
//! Native evaluates one vertex expression for every primitive; Mesa rounds it at
//! `(projection * view) * (model * vertex)` (`factorized_projection` for triangles). The stock
//! line stages take one matrix, so the model product is computed here, in the column order and
//! binary32 arithmetic of `clip_position.glsl`. Without fused operations (llvmpipe at SSE2) this
//! is the operation the driver would run; other drivers do not select this version.
use crate::{PrimitiveDraw, RenderProjectionBehavior as Behavior, RenderVertex};

/// World-space vertices under the camera product; `lowered(behavior, draw)`.
pub(super) fn lowered(behavior: Behavior, draw: &PrimitiveDraw) -> Option<PrimitiveDraw> {
    if !behavior.factorizes_lines() || !stock_stage_owns_positions(draw) {
        return None;
    }
    let factors = draw.projection_factors.as_ref()?;
    // Persistent and camera-independent groups draw through their own matrix, not these factors.
    if !same_transform(&factors.model_view_product(), &draw.clip_transform) {
        return None;
    }
    let model = factors.model();
    let world = draw.vertices.iter().map(|v| world_vertex(&model, v));
    let mut selected = draw.with_replacement_geometry(world.collect(), draw.indices.clone());
    selected.clip_transform = factors.camera_product();
    Some(selected)
}

/// Whether two compositions of one transform agree beyond binary32 reordering.
fn same_transform(a: &[f32; 16], b: &[f32; 16]) -> bool {
    a.iter()
        .zip(b)
        .all(|(a, b)| (a - b).abs() <= 1.0e-4 * a.abs().max(b.abs()).max(1.0))
}

/// A fixed-function draw with the driver's own products and plane; `fixed_function(run, draw)`.
pub(super) fn fixed_function(
    behavior: Behavior,
    clip_space: bool,
    draw: &PrimitiveDraw,
) -> Option<PrimitiveDraw> {
    let composes = behavior.composes_model_view();
    if !(composes || clip_space) || !(draw.fixed_function_lighting || draw.fixed_function_mask) {
        return None;
    }
    let mut selected = draw.with_replacement_geometry(draw.vertices.clone(), draw.indices.clone());
    if composes {
        compose_model_view(&mut selected);
    }
    if clip_space {
        selected.clipping = clip_space_clipping(draw);
    }
    Some(selected)
}
fn compose_model_view(draw: &mut PrimitiveDraw) {
    if let Some(factors) = &draw.fixed_function_factors {
        draw.clip_transform = factors.model_view_product();
    }
    if let Some(lighting) = draw.lighting.as_mut() {
        lighting.use_driver_fixed_normal();
    }
}
/// The draw's planes in clip space, when its matrices are known and of a reproduced class.
pub(super) fn clip_space_clipping(draw: &PrimitiveDraw) -> Option<crate::RenderClipping> {
    let clipping = draw.clipping.as_ref()?;
    let [projection, view, _] = draw.fixed_function_factors.as_ref()?.matrices();
    let plane = |plane: &[f32; 4]| {
        crate::fixed_modelview_inverse::clip_space_plane(*plane, view, projection)
    };
    let chains = clipping
        .chains()
        .iter()
        .map(|chain| chain.iter().map(plane).collect())
        .collect::<Option<Vec<Vec<[f32; 4]>>>>()?;
    crate::RenderClipping::new(chains, *clipping.world_from_object()).ok()
}

/// World clipping and registered programs read the object position, so they keep their route.
fn stock_stage_owns_positions(draw: &PrimitiveDraw) -> bool {
    crate::line_guard_band::clippable(draw)
        && !draw.fixed_function_lighting
        && draw.clipping.is_none()
        && draw.user_program.is_none()
}

fn world_vertex(model: &[f32; 16], vertex: &RenderVertex) -> RenderVertex {
    let [x, y, z] = vertex.position;
    let row = |r: usize| {
        ((model[4 + r] * y + model[r] * x) + model[8 + r] * z) + model[12 + r] * vertex.weight
    };
    RenderVertex {
        position: [row(0), row(1), row(2)],
        weight: row(3),
        color: vertex.color,
    }
}

#[cfg(test)]
#[path = "fixed_function_upload_tests.rs"]
mod fixed_function_tests;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LineStyle, RenderProjectionFactors};
    const IDENTITY: [f32; 16] = [
        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
    ];
    const FAR: f32 = 16_777_216.;
    fn translated(x: f32) -> [f32; 16] {
        let mut matrix = IDENTITY;
        matrix[12] = x;
        matrix
    }
    /// A model translation the view cancels: the two products round differently in binary32.
    fn cancelling_line() -> PrimitiveDraw {
        let vertices = vec![RenderVertex::new([1., 0., 0.], [1.; 4]); 2];
        let style = LineStyle::new(1., 0xffff, 1).unwrap();
        let mut draw = PrimitiveDraw::segments(vertices, vec![0, 1], IDENTITY, style).unwrap();
        draw.projection_factors = Some(
            RenderProjectionFactors::new(IDENTITY, translated(-FAR), translated(FAR)).unwrap(),
        );
        draw
    }
    #[test]
    fn the_model_product_rounds_before_the_camera_product() {
        let selected = lowered(Behavior::FactorizedLinesV3, &cancelling_line()).unwrap();
        // Independent IEEE32 example: 1 + 2^24 rounds to 2^24, so the unit is lost in the world.
        assert_eq!(selected.vertices[0].position, [FAR, 0., 0.]);
        assert_eq!(selected.vertices[0].weight, 1.);
        assert_eq!(selected.clip_transform, translated(-FAR));
        assert_eq!(selected.indices, [0, 1]);
    }
    #[test]
    fn a_draw_with_its_own_matrix_keeps_it() {
        let mut persistent = cancelling_line();
        persistent.clip_transform = translated(5.);
        assert_eq!(lowered(Behavior::FactorizedLinesV3, &persistent), None);
    }
    #[test]
    fn earlier_versions_keep_the_precomposed_line() {
        for behavior in [Behavior::PrecomposedV1, Behavior::FactorizedTrianglesV2] {
            assert_eq!(lowered(behavior, &cancelling_line()), None);
        }
    }
    #[test]
    fn draws_without_factors_or_reading_object_positions_keep_their_route() {
        let mut sourceless = cancelling_line();
        sourceless.projection_factors = None;
        assert_eq!(lowered(Behavior::FactorizedLinesV3, &sourceless), None);
        let mut fixed = cancelling_line();
        fixed.fixed_function_lighting = true;
        assert_eq!(lowered(Behavior::FactorizedLinesV3, &fixed), None);
    }
    #[test]
    fn triangles_are_not_lowered_here() {
        let vertices = vec![RenderVertex::new([0.; 3], [1.; 4]); 3];
        let triangle = PrimitiveDraw::new(vertices, vec![0, 1, 2], IDENTITY).unwrap();
        assert_eq!(lowered(Behavior::FactorizedLinesV3, &triangle), None);
    }
}
