//! Ordered source encoding stays owned through the existing allocation sequence.
use super::super::super::fixed_vertex_sources::{
    expanded_provoking_sources, uses_fixed_flat_vertices,
};
use super::*;

pub(super) fn encoded_geometry(runtime: &VulkanRuntime, draw: &PrimitiveDraw) -> EncodedGeometry {
    if draw.fixed_grid_point {
        return encoded_fixed_grid_geometry(draw);
    }
    let original = crate::quad_assembly::device_quad_indices(draw, runtime.quad_diagonal);
    let (vertices, source_count) = if uses_fixed_flat_vertices(draw) {
        let sources = expanded_provoking_sources(draw, &original);
        (
            encode_vertex_sources(draw, sources.iter().copied()),
            sources.len(),
        )
    } else {
        (encode_vertices(draw), draw.vertices.len())
    };
    if runtime.text_interpolation
        == crate::RenderTextInterpolationBehavior::ProjectedGrid12FlatCornersV4
        && draw.textured_style.is_some()
    {
        return encoded_flat_text(vertices, &original);
    }
    let expanded =
        super::super::super::fixed_vertex_sources::fixed_expanded_indices(draw, source_count);
    EncodedGeometry {
        indices: encoded_index_bytes(&original, expanded.as_deref()),
        vertex_alignment: (source_count > 0).then(|| (vertices.len() / source_count) as u64),
        vertices,
        #[cfg(feature = "command-state-diagnostics")]
        source_count,
    }
}

pub(super) fn compact_line_geometry(draw: &PrimitiveDraw) -> EncodedGeometry {
    let mut vertices = Vec::with_capacity(draw.vertices.len() * 16);
    for vertex in draw.vertices.iter() {
        let [x, y, z] = vertex.position;
        vertices.extend_from_slice(
            [x, y, z, vertex.weight]
                .map(f32::to_ne_bytes)
                .as_flattened(),
        );
    }
    EncodedGeometry {
        indices: encoded_index_bytes(&draw.indices, None),
        vertex_alignment: Some(16),
        vertices,
        #[cfg(feature = "command-state-diagnostics")]
        source_count: draw.vertices.len(),
    }
}

fn encoded_flat_text(vertices: Vec<u8>, original: &[u32]) -> EncodedGeometry {
    let packed = super::super::super::text_flat_corners::encode(&vertices, original);
    EncodedGeometry {
        indices: encoded_index_bytes(original, Some(&packed.indices)),
        vertices: packed.vertices,
        vertex_alignment: None,
        #[cfg(feature = "command-state-diagnostics")]
        source_count: original.len(),
    }
}

pub(super) fn encoded_index_bytes(original: &[u32], expanded: Option<&[u32]>) -> Vec<u8> {
    expanded
        .unwrap_or(original)
        .iter()
        .flat_map(|index| index.to_ne_bytes())
        .collect()
}

pub(super) fn encode_index_patch(
    draw: &PrimitiveDraw,
    range: std::ops::Range<usize>,
    full_size: u64,
) -> Option<(u64, Vec<u8>)> {
    let stride = size_of::<u32>();
    if u64::try_from(draw.indices.len().checked_mul(stride)?).ok()? != full_size {
        return None;
    }
    let offset = u64::try_from(range.start.checked_mul(stride)?).ok()?;
    let changed = draw.indices.get(range)?;
    Some((offset, encoded_index_bytes(changed, None)))
}

impl EncodedGeometry {
    pub(super) fn queue_transfers(
        &self,
        runtime: &Arc<VulkanRuntime>,
        pool: &super::super::super::host_upload_pages::HostUploadPool,
        transfers: &mut GeometryTransfers,
        retained: bool,
    ) -> Result<ResidentGeometry, RenderError> {
        let vertices = transfers.upload(
            runtime,
            pool,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            self.vertex_alignment.filter(|_| retained),
            &self.vertices,
        )?;
        let indices = transfers.upload(
            runtime,
            pool,
            vk::BufferUsageFlags::INDEX_BUFFER,
            retained.then_some(4),
            &self.indices,
        )?;
        Ok(ResidentGeometry { vertices, indices })
    }
}

pub(super) fn encoded_fixed_grid_geometry(draw: &PrimitiveDraw) -> EncodedGeometry {
    let sources = draw.indices.iter().map(|&i| (i as usize, i as usize));
    EncodedGeometry {
        indices: [0u32, 1, 2, 2, 1, 3]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect(),
        vertices: encode_vertex_sources(draw, sources),
        vertex_alignment: None,
        #[cfg(feature = "command-state-diagnostics")]
        source_count: draw.indices.len(),
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
        assert_eq!(
            encoded.vertices,
            encode_vertex_sources(&draw, [(1, 1), (0, 0), (1, 1)].into_iter())
        );
        let expected: Vec<u8> = [0u32, 1, 2, 2, 1, 3]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect();
        assert_eq!(encoded.indices, expected);
        assert_eq!(draw.indices(), &[1, 0, 1]);
    }
    #[test]
    fn fixed_grid_lit_uv_instances_retain_every_original_attribute() {
        use martensite_tkservice::graphic3d::Graphic3dTypeOfShadingModel as Model;
        for model in [Model::Unlit, Model::Gouraud, Model::Phong] {
            let mut source = crate::point_surface_texture_tests::fixture::point_draw(model, 3.);
            source.indices = vec![1, 0, 1].into();
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
        lighting.normals = vec![[1., 0., 0.], [0., 1., 0.]].into();
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

#[cfg(all(
    test,
    not(any(
        feature = "command-state-diagnostics",
        feature = "vertex-output-diagnostics"
    ))
))]
mod compact_line_tests {
    use crate::backend::resource_lifecycle_test_support::create_resource_renderer;
    use crate::{
        LineStyle, PrimitiveDraw, RenderBackend, RenderFrame, RenderVertex, identity_clip,
    };

    #[test]
    fn retained_line_packing_keeps_ordered_pixels_and_depth() {
        let mut renderer = create_resource_renderer();
        let plain = weighted_segments();
        let mut compact = plain.clone();
        compact.retain_uniform_line();
        let first = renderer.render(&frame(plain)).unwrap();
        let before = renderer.work_ledger().totals();
        let second = renderer.render(&frame(compact.clone())).unwrap();
        let uploaded = renderer.work_ledger().totals().since(&before);
        assert_eq!(uploaded.geometry_uploaded_bytes, 4 * 16 + 6 * 4);
        assert!(first.rgba.as_chunks::<4>().0.contains(&[255, 0, 0, 255]));
        assert_eq!(second.rgba, first.rgba);
        assert_eq!(second.depth, first.depth);
        let before = renderer.work_ledger().totals();
        renderer.render_deferred(&frame(compact)).unwrap();
        let warm = renderer.work_ledger().totals().since(&before);
        assert_eq!((warm.geometry_uploaded_bytes, warm.buffers_created), (0, 0));
        assert!(renderer.validation_errors().unwrap().is_empty());
    }

    #[test]
    fn replacing_vertex_stream_removes_uniform_line_admission() {
        let mut draw = weighted_segments();
        draw.retain_uniform_line();
        assert_eq!(draw.uniform_line_color(), Some([1., 0., 0., 1.]));
        let unchanged = draw.clone();
        draw.vertices.make_mut()[1].color = [0., 1., 0., 1.];
        assert!(crate::highlight_style::HighlightLineSource::resolve(&draw).is_none());
        assert!(crate::highlight_style::HighlightLineSource::resolve(&unchanged).is_some());
    }

    fn frame(draw: PrimitiveDraw) -> RenderFrame {
        RenderFrame::new(16, 16, [0., 0., 0., 1.], vec![draw]).unwrap()
    }

    fn weighted_segments() -> PrimitiveDraw {
        let vertices = [
            [-0.6, -0.3, 0.2, 1.],
            [1.2, 0.6, 1., 2.],
            [-0.6, 0.3, 0.6, 1.],
            [0.6, -0.3, 0.1, 1.],
        ]
        .map(|position| RenderVertex::homogeneous(position, [1., 0., 0., 1.]));
        PrimitiveDraw::segments(
            vertices.into(),
            vec![2, 3, 0, 1, 2, 1],
            identity_clip(),
            LineStyle::new(1., u16::MAX, 1).unwrap(),
        )
        .unwrap()
    }
}
