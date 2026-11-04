//! Ordered source encoding stays owned through the existing allocation sequence.
use super::*;

pub(super) fn encoded_geometry<'a>(
    runtime: &VulkanRuntime,
    draw: &'a PrimitiveDraw,
) -> EncodedGeometry<'a> {
    if draw.fixed_grid_point {
        return encoded_fixed_grid_geometry(draw);
    }
    let original = crate::quad_assembly::device_quad_indices(draw, runtime.quad_diagonal);
    let sources = super::super::super::fixed_vertex_sources::fixed_vertex_sources(draw, &original);
    if runtime.text_interpolation
        == crate::RenderTextInterpolationBehavior::ProjectedGrid12FlatCornersV4
        && draw.textured_style.is_some()
    {
        return encoded_flat_text(encode_vertex_sources(draw, &sources), original);
    }
    encode_prepared(draw, original, sources)
}

fn encoded_flat_text(
    vertices: Vec<u8>,
    original: std::borrow::Cow<'_, [u32]>,
) -> EncodedGeometry<'_> {
    let packed = super::super::super::text_flat_corners::encode(&vertices, &original);
    EncodedGeometry {
        indices: encoded_index_bytes(&original, Some(&packed.indices)),
        _expanded: Some(packed.indices),
        vertices: packed.vertices,
        _sources: packed.sources,
        _original_indices: original,
    }
}

fn encode_prepared<'a>(
    draw: &PrimitiveDraw,
    original: std::borrow::Cow<'a, [u32]>,
    sources: Vec<(usize, usize)>,
) -> EncodedGeometry<'a> {
    let vertices = encode_vertex_sources(draw, &sources);
    let expanded =
        super::super::super::fixed_vertex_sources::fixed_expanded_indices(draw, sources.len());
    EncodedGeometry {
        indices: encoded_index_bytes(&original, expanded.as_deref()),
        _expanded: expanded,
        vertices,
        _sources: sources,
        _original_indices: original,
    }
}

pub(super) fn encoded_index_bytes(original: &[u32], expanded: Option<&[u32]>) -> Vec<u8> {
    expanded
        .unwrap_or(original)
        .iter()
        .flat_map(|index| index.to_ne_bytes())
        .collect()
}

impl EncodedGeometry<'_> {
    pub(super) fn upload_vertices(
        &self,
        runtime: &Arc<VulkanRuntime>,
    ) -> Result<DeviceBuffer, RenderError> {
        // The original encoded vectors remain owned until final assembly, including failure cleanup.
        upload_bytes(runtime, &self.vertices, vk::BufferUsageFlags::VERTEX_BUFFER)
    }
    pub(super) fn upload_indices(
        &self,
        runtime: &Arc<VulkanRuntime>,
    ) -> Result<DeviceBuffer, RenderError> {
        // Keep this after vertex allocation so the failed-upload cleanup sequence remains unchanged.
        upload_bytes(runtime, &self.indices, vk::BufferUsageFlags::INDEX_BUFFER)
    }
}

pub(super) fn encoded_fixed_grid_geometry(draw: &PrimitiveDraw) -> EncodedGeometry<'_> {
    let sources: Vec<_> = draw
        .indices
        .iter()
        .map(|&i| (i as usize, i as usize))
        .collect();
    EncodedGeometry {
        indices: [0u32, 1, 2, 2, 1, 3]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect(),
        vertices: encode_vertex_sources(draw, &sources),
        _sources: sources,
        _expanded: None,
        _original_indices: std::borrow::Cow::Borrowed(&draw.indices),
    }
}
#[cfg(test)]
mod fixed_grid_encoding_tests {
    use super::*;
    #[test]
    fn fixed_grid_instances_keep_repeated_index_order_and_homogeneous_sources() {
        let mut vertices = vec![
            crate::RenderVertex::new([1., 2., 3.], [0.1, 0.2, 0.3, 0.4]),
            crate::RenderVertex::new([4., 5., 6.], [0.5, 0.6, 0.7, 0.8]),
        ];
        vertices[0].weight = 0.75;
        vertices[1].weight = 2.;
        let style = crate::PointStyle::new(2., [1, 1], vec![[1.; 4]]).unwrap();
        let draw =
            PrimitiveDraw::points(vertices, vec![1, 0, 1], crate::identity_clip(), style).unwrap();
        let encoded = encoded_fixed_grid_geometry(&draw);
        assert_eq!(encoded._sources, vec![(1, 1), (0, 0), (1, 1)]);
        assert_eq!(
            encoded.vertices,
            encode_vertex_sources(&draw, &[(1, 1), (0, 0), (1, 1)])
        );
        let expected: Vec<u8> = [0u32, 1, 2, 2, 1, 3]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect();
        assert_eq!(encoded.indices, expected);
        assert_eq!(draw.indices, vec![1, 0, 1]);
    }
    #[test]
    fn fixed_grid_lit_uv_instances_retain_every_original_attribute() {
        use martensite_tkservice::graphic3d::Graphic3dTypeOfShadingModel as Model;
        for model in [Model::Unlit, Model::Gouraud, Model::Phong] {
            let mut source = crate::point_surface_texture_tests::fixture::point_draw(model, 3.);
            source.indices = vec![1, 0, 1];
            let snapshot = source.clone();
            let encoded = encoded_fixed_grid_geometry(&source);
            let stride = if source.lighting.is_some() { 60 } else { 48 };
            assert_eq!(encoded.vertices.len(), 3 * stride);
            for (instance, index) in [1usize, 0, 1].into_iter().enumerate() {
                let vertex = &source.vertices[index];
                let expected: Vec<u8> = vertex
                    .position
                    .into_iter()
                    .chain([vertex.weight])
                    .chain(vertex.color)
                    .flat_map(f32::to_ne_bytes)
                    .collect();
                assert_eq!(
                    &encoded.vertices[instance * stride..instance * stride + 32],
                    expected
                );
                let uv: Vec<u8> = source.surface_texture.as_ref().unwrap().uv[index]
                    .into_iter()
                    .flat_map(f32::to_ne_bytes)
                    .collect();
                assert_eq!(
                    &encoded.vertices[instance * stride + 32..instance * stride + 48],
                    uv
                );
                if let Some(lighting) = &source.lighting {
                    let normal: Vec<u8> = lighting.normals[index]
                        .into_iter()
                        .flat_map(f32::to_ne_bytes)
                        .collect();
                    assert_eq!(
                        &encoded.vertices[instance * stride + 48..instance * stride + 60],
                        normal
                    );
                }
            }
            assert_eq!(source, snapshot);
        }
    }
    #[test]
    fn fixed_grid_fixed_lighting_preserves_logical_point_source_attributes() {
        let vertices = vec![
            crate::RenderVertex::new([1., 2., 3.], [0.1, 0.2, 0.3, 1.]),
            crate::RenderVertex::new([4., 5., 6.], [0.7, 0.8, 0.9, 1.]),
        ];
        let style = crate::PointStyle::new(3., [1, 1], vec![[1.; 4]]).unwrap();
        let mut source =
            PrimitiveDraw::points(vertices, vec![1, 0, 1], crate::identity_clip(), style).unwrap();
        let mut lighting = crate::backend::fixed_vertex_sources::fixed_test_lighting(
            2,
            crate::RenderShading::Gouraud,
        );
        lighting.normals = vec![[1., 0., 0.], [0., 1., 0.]];
        source.lighting = Some(lighting);
        let encoded = encoded_fixed_grid_geometry(&source);
        assert_eq!(encoded.vertices.len(), 3 * 88);
        for (instance, index) in [1usize, 0, 1].into_iter().enumerate() {
            let vertex = &source.vertices[index];
            let position: Vec<u8> = vertex
                .position
                .into_iter()
                .chain([vertex.weight])
                .flat_map(f32::to_ne_bytes)
                .collect();
            let normal: Vec<u8> = source.lighting.as_ref().unwrap().normals[index]
                .into_iter()
                .flat_map(f32::to_ne_bytes)
                .collect();
            let color: Vec<u8> = vertex
                .color
                .into_iter()
                .flat_map(f32::to_ne_bytes)
                .collect();
            assert_eq!(
                &encoded.vertices[instance * 88 + 44..instance * 88 + 60],
                position
            );
            assert_eq!(
                &encoded.vertices[instance * 88 + 60..instance * 88 + 72],
                normal
            );
            assert_eq!(
                &encoded.vertices[instance * 88 + 72..instance * 88 + 88],
                color
            );
        }
    }
}
