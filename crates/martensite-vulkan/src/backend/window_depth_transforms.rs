//! Bounds follow the actual selected VS factor order, including contraction and underflow.
use crate::{
    PrimitiveDraw, RenderProjectionBehavior as Projection, RenderVertex,
    window_depth_bounds::DepthShaderBound as Bound,
};

pub(super) fn projected_bounds(
    draw: &PrimitiveDraw,
    projection: Projection,
    negative: bool,
) -> Option<Vec<[Bound; 4]>> {
    let (camera, model) = matrices(draw, projection)?;
    draw.vertices
        .iter()
        .map(|vertex| {
            let position = vertex_bounds(*vertex)?;
            let world = if let Some(matrix) = model {
                transform(matrix, position)?
            } else {
                position
            };
            let mut clip = transform(camera, world)?;
            if !negative {
                clip[2] = clip[2].add(clip[3])?.multiply(Bound::source(0.5)?)?;
            }
            Some(clip)
        })
        .collect()
}

fn matrices(
    draw: &PrimitiveDraw,
    projection: Projection,
) -> Option<([f32; 16], Option<[f32; 16]>)> {
    if projection.factorizes_triangles() {
        let factors = draw.projection_factors.as_ref()?;
        return Some((factors.camera_product(), Some(factors.model())));
    }
    Some((draw.clip_transform, None))
}

fn vertex_bounds(vertex: RenderVertex) -> Option<[Bound; 4]> {
    let [x, y, z] = vertex.position;
    Some([
        Bound::source(x)?,
        Bound::source(y)?,
        Bound::source(z)?,
        Bound::source(vertex.weight)?,
    ])
}

fn transform(matrix: [f32; 16], vertex: [Bound; 4]) -> Option<[Bound; 4]> {
    Some([
        transform_row(matrix, vertex, 0)?,
        transform_row(matrix, vertex, 1)?,
        transform_row(matrix, vertex, 2)?,
        transform_row(matrix, vertex, 3)?,
    ])
}

fn transform_row(matrix: [f32; 16], vertex: [Bound; 4], row: usize) -> Option<Bound> {
    let x = Bound::source(matrix[row])?.multiply(vertex[0])?;
    let y = Bound::source(matrix[4 + row])?.multiply(vertex[1])?;
    let z = Bound::source(matrix[8 + row])?.multiply(vertex[2])?;
    let w = Bound::source(matrix[12 + row])?.multiply(vertex[3])?;
    y.add(x)?.add(z)?.add(w)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn draw() -> PrimitiveDraw {
        let vertex = RenderVertex::new([0.25, -0.5, 0.2], [1.; 4]);
        PrimitiveDraw::new(vec![vertex; 3], vec![0, 1, 2], crate::identity_clip()).unwrap()
    }
    #[test]
    fn precomposed_identity_and_explicit_depth_mapping_are_enclosed() {
        let draw = draw();
        let raw = projected_bounds(&draw, Projection::PrecomposedV1, true).unwrap();
        let mapped = projected_bounds(&draw, Projection::PrecomposedV1, false).unwrap();
        assert!(
            raw[0][2].endpoints()[0] <= f64::from(0.2_f32)
                && raw[0][2].endpoints()[1] >= f64::from(0.2_f32)
        );
        assert!(
            mapped[0][2].endpoints()[0] <= f64::from(0.6_f32)
                && mapped[0][2].endpoints()[1] >= f64::from(0.6_f32)
        );
        assert!(projected_bounds(&draw, Projection::FactorizedTrianglesV2, false).is_none());
    }
    #[test]
    fn factorized_bounds_keep_model_rounding_before_camera_cancellation() {
        let mut draw = draw();
        let mut model = crate::identity_clip();
        model[12] = 16_777_216.;
        let mut view = crate::identity_clip();
        view[12] = -16_777_216.;
        draw.projection_factors =
            Some(crate::RenderProjectionFactors::new(crate::identity_clip(), view, model).unwrap());
        let bounds = projected_bounds(&draw, Projection::FactorizedTrianglesV2, true).unwrap();
        let [minimum, maximum] = bounds[0][0].endpoints();
        assert!(minimum <= 0. && maximum >= 0.);
        assert!(maximum - minimum > 0.25);
    }
    #[test]
    fn identity_model_preserves_a_certified_raster_empty_column_without_case_exemption() {
        let camera = [
            0.038_108_934,
            -0.022_002_205,
            -0.030_769_002,
            0.,
            0.038_108_934,
            0.022_002_205,
            0.030_769_002,
            0.,
            0.,
            0.044_004_41,
            -0.030_769_002,
            0.,
            -3.810_893,
            1.650_165_1,
            3.461_512,
            1.,
        ];
        let positions = [
            [106.427_88, 7.660_444_3, 20.],
            [107.660_446, 6.427_876, 0.],
            [106.427_88, 7.660_444_3, 0.],
        ];
        let mut draw = PrimitiveDraw::new(
            positions.map(|p| RenderVertex::new(p, [1.; 4])).to_vec(),
            vec![0, 1, 2],
            camera,
        )
        .unwrap();
        draw.projection_factors = Some(
            crate::RenderProjectionFactors::new(
                camera,
                crate::identity_clip(),
                crate::identity_clip(),
            )
            .unwrap(),
        );
        assert_eq!(
            triangle_closure(&draw),
            Ok(crate::window_depth_closure::WindowDepthClosure::RasterEmpty)
        );
        draw.vertices[2].position[0] += 0.25;
        assert_eq!(
            triangle_closure(&draw),
            Ok(crate::window_depth_closure::WindowDepthClosure::ReferencePlane)
        );
    }
    fn triangle_closure(
        draw: &PrimitiveDraw,
    ) -> Result<
        crate::window_depth_closure::WindowDepthClosure,
        crate::window_depth_closure::WindowDepthClosureFailure,
    > {
        let bounds = projected_bounds(draw, Projection::FactorizedTrianglesV2, false).unwrap();
        crate::window_depth_closure::prove_window_depth_closure(
            [bounds[0], bounds[1], bounds[2]],
            [0., 0., 409., 409.],
            false,
            crate::RenderViewportGridPrecision::Fractional12,
            crate::RenderViewportGridPrecision::Fractional8,
        )
    }
}
