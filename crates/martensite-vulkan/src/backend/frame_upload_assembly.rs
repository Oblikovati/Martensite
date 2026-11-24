//! Final assembly preserves prepared allocation and binding ownership.
//! Assemble prepared draw owners without changing allocation or command order.
use super::*;
impl DrawUploadContext<'_> {
    /// The draw's push constants before any point viewports.
    pub(super) fn push_constants(
        &self,
        pipeline: &DrawPipeline,
        mesh_decode: Option<&super::super::super::mesh_vertex_codec::MeshPositionDecode>,
    ) -> Vec<u8> {
        let mut transform = encode_transform(self.draw, self.extent);
        if pipeline.kind.line_tint.uses_draw_color()
            && let Some((_, color)) = self.line_tint
        {
            transform.extend_from_slice(color.map(f32::to_ne_bytes).as_flattened());
        }
        if let Some(decode) = mesh_decode {
            decode.append_push_bytes(&mut transform);
        }
        crate::backend::factorized_projection::transform(
            self.runtime.projection_behavior,
            self.semantic_draw,
            transform,
        )
    }

    pub(super) fn assemble(
        &self,
        geometry: PreparedGeometry,
        bindings: PreparedBindings,
    ) -> UploadedDraw {
        let point_viewports = self.draw.fixed_grid_point.then(|| {
            crate::backend::fixed_grid_viewport::viewport_pair(
                self.runtime.attachment_coordinates,
                self.extent,
            )
        });
        let mut transform =
            self.push_constants(&geometry.pipeline, geometry.resident.mesh_decode.as_deref());
        if let Some(viewports) = point_viewports {
            crate::backend::fixed_grid_viewport::append_viewports(&mut transform, viewports);
        }
        UploadedDraw {
            window_depth_expected: self.window_depth.is_some_and(|p| p.fragment_depth()),
            window_depth_status: None,
            #[cfg(feature = "vertex-output-diagnostics")]
            vertex_output: None,
            #[cfg(feature = "command-state-diagnostics")]
            geometry_arguments: crate::backend::geometry_command_diagnostic::GeometryArguments::new(
                self.draw,
                &geometry.encoding.vertices,
                &geometry.encoding.indices,
                geometry.encoding.source_count,
            ),
            native_alpha: None,
            oit_alpha: vec![],
            sample_alpha_cutoff: self.sample_cutoff(),
            oit: None,
            bindings: bindings.into_draw(),
            presentation: self.presentation(),
            pipeline: geometry.pipeline,
            vertices: geometry.resident.vertices,
            indices: geometry.resident.indices,
            index_format: geometry.resident.index_format,
            mesh_decode: geometry.resident.mesh_decode,
            count: geometry.count,
            instances: if self.draw.fixed_grid_point {
                self.draw.indices.len() as u32
            } else {
                1
            },
            line: self.line_state(),
            transform,
            point_viewports,
            user_program: None,
            table_run: None,
            borrowed_lighting: false,
        }
    }
}
