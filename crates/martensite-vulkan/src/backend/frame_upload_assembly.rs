//! Final assembly preserves prepared allocation and binding ownership.
//! Assemble prepared draw owners without changing allocation or command order.
use super::*;
impl DrawUploadContext<'_> {
    pub(super) fn assemble(
        &self,
        geometry: PreparedGeometry<'_>,
        bindings: PreparedBindings,
    ) -> UploadedDraw {
        let point_viewports = self.draw.fixed_grid_point.then(|| {
            crate::backend::fixed_grid_viewport::viewport_pair(
                self.runtime.attachment_coordinates,
                self.extent,
            )
        });
        let mut transform = encode_transform(self.draw, self.extent);
        transform = crate::backend::factorized_projection::transform(
            self.runtime.projection_behavior,
            self.semantic_draw,
            transform,
        );
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
                &geometry._encoding.vertices,
                &geometry._encoding.indices,
                geometry._encoding._sources.len(),
            ),
            native_alpha: None,
            oit_alpha: vec![],
            sample_alpha_cutoff: self.sample_cutoff(),
            oit: None,
            bindings: bindings.into_draw(),
            presentation: self.presentation(),
            pipeline: geometry.pipeline,
            vertices: geometry.vertices,
            indices: geometry.indices,
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
        }
    }
}
