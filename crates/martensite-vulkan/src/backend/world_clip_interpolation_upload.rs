//! Admit grid-based fragment clipping only with its explicit triangle raster contract.
use crate::{
    PrimitiveDraw, PrimitiveTopology, RenderError,
    RenderWorldClipInterpolationBehavior as Behavior,
    RenderWorldClipInterpolationFailure as Failure,
};

pub(super) fn admit(
    behavior: Behavior,
    world_clipping: crate::RenderWorldClippingBehavior,
    triangle_coverage: crate::RenderTriangleCoverageBehavior,
    phong_interpolation: crate::RenderPhongInterpolationBehavior,
    draw: &PrimitiveDraw,
) -> Result<bool, RenderError> {
    if behavior == Behavior::HardwareV1
        || !has_world_planes(draw)
        || draw.topology() != PrimitiveTopology::Triangles
    {
        return Ok(false);
    }
    require_composition(
        behavior,
        world_clipping,
        triangle_coverage,
        phong_interpolation,
    )?;
    require_draw(draw)?;
    require_finite_world_clip_domain(draw)?;
    Ok(true)
}

pub(super) fn selects_grid12_fragment(draw: &PrimitiveDraw, behavior: Behavior) -> bool {
    matches!(
        behavior,
        Behavior::ProjectedGrid12V1 | Behavior::ProjectedGrid12V2
    ) && has_world_planes(draw)
        && draw.topology() == PrimitiveTopology::Triangles
}

pub(super) fn permits_unclipped_grid12_v3_coverage(
    draw: &PrimitiveDraw,
    behavior: Behavior,
    world_clipping: crate::RenderWorldClippingBehavior,
    triangle_coverage: crate::RenderTriangleCoverageBehavior,
    phong_interpolation: crate::RenderPhongInterpolationBehavior,
) -> bool {
    !has_world_planes(draw)
        && draw.topology() == PrimitiveTopology::Triangles
        && super::triangle_grid_coverage_upload::selected_phong(draw)
        && behavior == Behavior::ProjectedGrid12V2
        && world_clipping == crate::RenderWorldClippingBehavior::FragmentV1
        && matches!(
            triangle_coverage,
            crate::RenderTriangleCoverageBehavior::FixedGrid12V3 { .. }
                | crate::RenderTriangleCoverageBehavior::NativeGrid12V4 { .. }
        )
        && phong_interpolation == crate::RenderPhongInterpolationBehavior::UnsnappedV3
}

fn require_finite_world_clip_domain(draw: &PrimitiveDraw) -> Result<(), RenderError> {
    let clipping = draw.clipping.as_ref().expect("world planes were admitted");
    let mut world_positions = Vec::with_capacity(draw.vertices.len());
    for (index, vertex) in draw.vertices.iter().copied().enumerate() {
        let world = validate_world_vertex(draw, clipping, index as u32, vertex)?;
        world_positions.push(world);
    }
    require_bounded_projective_products(draw, &world_positions)?;
    Ok(())
}

fn validate_world_vertex(
    draw: &PrimitiveDraw,
    clipping: &crate::RenderClipping,
    index: u32,
    vertex: crate::RenderVertex,
) -> Result<BoundedWorldPosition, RenderError> {
    let world = bounded_homogeneous_world_position(clipping.world_from_object(), vertex)
        .ok_or_else(|| {
            unsupported_vertex(
                index,
                "world transform",
                (vertex, clipping.world_from_object()),
            )
        })?;
    require_normal_clip_w(draw, index, vertex)?;
    let point_bounds = divided_world_coordinate_bounds(index, world)?;
    require_finite_plane_dots(index, point_bounds, clipping.chains())?;
    Ok(world)
}

#[derive(Clone, Copy, Debug)]
struct BoundedWorldPosition {
    value: [f64; 4],
    magnitude: [f64; 4],
}

fn bounded_homogeneous_world_position(
    matrix: &[f32; 16],
    vertex: crate::RenderVertex,
) -> Option<BoundedWorldPosition> {
    let input = homogeneous_position(vertex)?;
    let world = bounded_matrix_product(matrix, input)?;
    (world.value[3] >= f64::from(MIN_WORLD_W)).then_some(world)
}

fn homogeneous_position(vertex: crate::RenderVertex) -> Option<[f32; 4]> {
    (vertex.weight.is_finite() && vertex.weight >= MIN_WORLD_W).then_some([
        vertex.position[0],
        vertex.position[1],
        vertex.position[2],
        vertex.weight,
    ])
}

fn bounded_matrix_product(matrix: &[f32; 16], input: [f32; 4]) -> Option<BoundedWorldPosition> {
    let contributions = std::array::from_fn::<_, 4, _>(|row| {
        std::array::from_fn::<_, 4, _>(|column| {
            f64::from(matrix[column * 4 + row]) * f64::from(input[column])
        })
    });
    let magnitude =
        contributions.map(|row| f32_operation_upper_bound(row.into_iter().map(f64::abs).sum()));
    if magnitude
        .iter()
        .any(|value| !value.is_finite() || *value > SAFE_WORLD_COMPONENT)
    {
        return None;
    }
    let world = contributions.map(|row| row.into_iter().sum::<f64>());
    world
        .iter()
        .all(|value| value.is_finite())
        .then_some(BoundedWorldPosition {
            value: world,
            magnitude,
        })
}

fn divided_world_coordinate_bounds(
    index: u32,
    world: BoundedWorldPosition,
) -> Result<[f64; 3], RenderError> {
    let point = std::array::from_fn(|axis| {
        f32_division_upper_bound(world.magnitude[axis] / world.value[3])
    });
    if point
        .iter()
        .all(|value| value.is_finite() && *value <= SAFE_WORLD_COMPONENT)
    {
        return Ok(point);
    }
    Err(unsupported_vertex(
        index,
        "world coordinate perspective divide",
        (world, point),
    ))
}

fn require_normal_clip_w(
    draw: &PrimitiveDraw,
    index: u32,
    vertex: crate::RenderVertex,
) -> Result<(), RenderError> {
    let clip = super::phong_interpolation_upload::clip_position(draw.clip_transform, vertex);
    let reciprocal = 1.0 / clip[3];
    let error =
        super::triangle_grid_coverage_upload::clip_rounding_bounds(draw.clip_transform, vertex)
            .map(|bounds| bounds[3]);
    let interval = error.map(|error| (f64::from(clip[3]) - error, f64::from(clip[3]) + error));
    if clip[3].is_finite()
        && interval.is_some_and(|(minimum, maximum)| {
            minimum >= f64::from(MIN_CLIP_W)
                && maximum <= f64::from(MAX_CLIP_W)
                && f32_division_lower_bound(1.0 / maximum) >= f64::from(MIN_RECIPROCAL_W)
        })
        && reciprocal.is_finite()
        && reciprocal >= MIN_RECIPROCAL_W
    {
        return Ok(());
    }
    Err(unsupported_vertex(
        index,
        "positive normal clip W and finite reciprocal",
        (clip, reciprocal),
    ))
}

const SAFE_WORLD_COMPONENT: f64 = f32::MAX as f64 / 64.0;
const MIN_CLIP_W: f32 = f32::MIN_POSITIVE * 4.0;
const MIN_RECIPROCAL_W: f32 = f32::MIN_POSITIVE * 16.0;
const MAX_CLIP_W: f32 = 5.316_912e36;
const MIN_WORLD_W: f32 = f32::MIN_POSITIVE * 16.0;
const MIN_WORLD_W_OVER_CLIP_W: f64 = f32::MIN_POSITIVE as f64 * 8.0;

fn f32_operation_upper_bound(magnitude: f64) -> f64 {
    magnitude * (1.0 + 8.0 * f64::from(f32::EPSILON)) + 8.0 * f64::from(f32::MIN_POSITIVE)
}

fn f32_division_upper_bound(magnitude: f64) -> f64 {
    magnitude * (1.0 + 4.0 * f64::from(f32::EPSILON)) + 4.0 * f64::from(f32::MIN_POSITIVE)
}

fn f32_division_lower_bound(value: f64) -> f64 {
    (value * (1.0 - 4.0 * f64::from(f32::EPSILON)) - 4.0 * f64::from(f32::MIN_POSITIVE)).max(0.0)
}

fn require_bounded_projective_products(
    draw: &PrimitiveDraw,
    world: &[BoundedWorldPosition],
) -> Result<(), RenderError> {
    for indices in draw.indices.as_chunks::<3>().0.iter() {
        require_triangle_products(draw, world, [indices[0], indices[1], indices[2]])?;
    }
    Ok(())
}

fn require_triangle_products(
    draw: &PrimitiveDraw,
    world: &[BoundedWorldPosition],
    indices: [u32; 3],
) -> Result<(), RenderError> {
    let bounds = triangle_product_bounds(draw, world, indices);
    if bounds.within_finite_shader_domain() {
        return Ok(());
    }
    Err(unsupported_vertex(
        indices[0],
        "Q12 projective interpolation products",
        bounds,
    ))
}

#[derive(Debug)]
struct ProjectiveProductBounds {
    clip_w: [f32; 3],
    reciprocal_sum: f64,
    weighted_components: [f64; 4],
    minimum_world_w_over_clip_w: f64,
}

impl ProjectiveProductBounds {
    fn within_finite_shader_domain(&self) -> bool {
        self.reciprocal_sum * 2.0 <= SAFE_WORLD_COMPONENT
            && self.minimum_world_w_over_clip_w >= MIN_WORLD_W_OVER_CLIP_W
            && self
                .weighted_components
                .iter()
                .all(|value| value * 2.0 <= SAFE_WORLD_COMPONENT)
    }
}

fn triangle_product_bounds(
    draw: &PrimitiveDraw,
    world: &[BoundedWorldPosition],
    indices: [u32; 3],
) -> ProjectiveProductBounds {
    let mut bounds = ProjectiveProductBounds {
        clip_w: [0.0; 3],
        reciprocal_sum: 0.0,
        weighted_components: [0.0; 4],
        minimum_world_w_over_clip_w: f64::INFINITY,
    };
    for (slot, index) in indices.into_iter().enumerate() {
        add_vertex_projective_bound(draw, world, index, slot, &mut bounds);
    }
    bounds
}

fn add_vertex_projective_bound(
    draw: &PrimitiveDraw,
    world: &[BoundedWorldPosition],
    index: u32,
    slot: usize,
    bounds: &mut ProjectiveProductBounds,
) {
    let clip = super::phong_interpolation_upload::clip_position(
        draw.clip_transform,
        draw.vertices[index as usize],
    );
    let clip_error = super::triangle_grid_coverage_upload::clip_rounding_bounds(
        draw.clip_transform,
        draw.vertices[index as usize],
    )
    .map(|bounds| bounds[3]);
    let clip_minimum = clip_error.map(|error| f64::from(clip[3]) - error);
    let clip_maximum = clip_error.map(|error| f64::from(clip[3]) + error);
    let reciprocal = clip_minimum.map_or(f64::INFINITY, |minimum| 1.0 / minimum);
    bounds.clip_w[slot] = clip[3];
    bounds.reciprocal_sum += reciprocal;
    let world_w_over_clip_w = clip_maximum.map_or(0.0, |maximum| {
        f64::from(draw.vertices[index as usize].weight) / maximum
    });
    bounds.minimum_world_w_over_clip_w = bounds
        .minimum_world_w_over_clip_w
        .min(f32_division_lower_bound(world_w_over_clip_w));
    for axis in 0..4 {
        let quotient = clip_minimum.map_or(f64::INFINITY, |minimum| {
            world[index as usize].magnitude[axis] / minimum
        });
        bounds.weighted_components[axis] += f32_division_upper_bound(quotient);
    }
}

fn require_finite_plane_dots(
    vertex: u32,
    point: [f64; 3],
    chains: &[Vec<[f32; 4]>],
) -> Result<(), RenderError> {
    for plane in chains.iter().flatten() {
        let magnitude = f32_operation_upper_bound(
            (0..3)
                .map(|axis| f64::from(plane[axis]).abs() * point[axis])
                .sum::<f64>()
                + f64::from(plane[3]).abs(),
        );
        if !magnitude.is_finite() || magnitude > SAFE_WORLD_COMPONENT {
            return Err(unsupported_vertex(
                vertex,
                "world clipping plane evaluation",
                (plane, point, magnitude),
            ));
        }
    }
    Ok(())
}

fn unsupported_vertex(vertex: u32, reason: &str, actual: impl std::fmt::Debug) -> RenderError {
    Failure::UnsupportedDraw {
        actual: format!(
            "vertex {vertex}: {reason} {actual:?}; expected finite bounded shader operands"
        ),
    }
    .into()
}

pub(super) fn has_world_planes(draw: &PrimitiveDraw) -> bool {
    draw.clipping
        .as_ref()
        .is_some_and(|clip| clip.chains().iter().any(|chain| !chain.is_empty()))
}

fn require_composition(
    behavior: Behavior,
    world_clipping: crate::RenderWorldClippingBehavior,
    triangle_coverage: crate::RenderTriangleCoverageBehavior,
    phong_interpolation: crate::RenderPhongInterpolationBehavior,
) -> Result<(), RenderError> {
    if supports_composition(
        behavior,
        world_clipping,
        triangle_coverage,
        phong_interpolation,
    ) {
        return Ok(());
    }
    Err(Failure::UnsupportedComposition {
        world_clipping,
        triangle_coverage,
        phong_interpolation,
    }
    .into())
}

pub(super) fn supports_composition(
    behavior: Behavior,
    world_clipping: crate::RenderWorldClippingBehavior,
    triangle_coverage: crate::RenderTriangleCoverageBehavior,
    phong_interpolation: crate::RenderPhongInterpolationBehavior,
) -> bool {
    world_clipping == crate::RenderWorldClippingBehavior::FragmentV1
        && match behavior {
            Behavior::HardwareV1 => false,
            Behavior::ProjectedGrid12V1 => {
                matches!(
                    triangle_coverage,
                    crate::RenderTriangleCoverageBehavior::FixedGrid12V2 { .. }
                        | crate::RenderTriangleCoverageBehavior::FixedGrid12V3 { .. }
                ) && phong_interpolation == crate::RenderPhongInterpolationBehavior::UnsnappedV2
            }
            Behavior::ProjectedGrid12V2 => {
                matches!(
                    triangle_coverage,
                    crate::RenderTriangleCoverageBehavior::FixedGrid12V3 { .. }
                        | crate::RenderTriangleCoverageBehavior::NativeGrid12V4 { .. }
                ) && phong_interpolation == crate::RenderPhongInterpolationBehavior::UnsnappedV3
            }
        }
}

fn require_draw(draw: &PrimitiveDraw) -> Result<(), RenderError> {
    if draw.topology() == PrimitiveTopology::Triangles
        && super::triangle_grid_coverage_upload::selected_phong(draw)
    {
        return Ok(());
    }
    Err(Failure::UnsupportedDraw {
        actual: format!(
            "topology={:?}, builtin-phong={}",
            draw.topology(),
            super::triangle_grid_coverage_upload::selected_phong(draw)
        ),
    }
    .into())
}

#[cfg(test)]
#[path = "world_clip_interpolation_upload_tests.rs"]
mod tests;
