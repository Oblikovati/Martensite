//! Exhaustive metadata copying keeps replacement geometry independent without cloning discarded vectors.
use super::{PrimitiveDraw, RenderVertex};

// One complete field list makes newly added draw state a compile error until its copying is reviewed.
macro_rules! clone_with_geometry {
    ($($field:ident),+ $(,)?) => {
        impl PrimitiveDraw {
            /// Preserve derived-Clone metadata semantics while replacing only vertex/index storage.
            /// Example: `source.with_replacement_geometry(vertices, vec![0, 1])`.
            pub(crate) fn with_replacement_geometry(
                &self,
                vertices: Vec<RenderVertex>,
                indices: Vec<u32>,
            ) -> Self {
                // Replaced geometry no longer derives from any retained guard-band source.
                Self { vertices, indices, guard_band_source: None, #[cfg(feature = "vertex-output-diagnostics")] vertex_output_target: self.vertex_output_target, $($field: Clone::clone(&self.$field)),+ }
            }
        }
    };
}
clone_with_geometry!(
    projected_dash,
    quad_corners,
    clip_transform,
    projection_factors,
    fixed_function_factors,
    depth_clamp,
    polygon_offset,
    mirrored,
    face_culling,
    capping_pass,
    clipping,
    hatch_rows,
    lighting,
    fixed_function_lighting,
    fixed_function_mask,
    compatibility_color_target,
    native_material_cutoff,
    source_color_array,
    unlit_back_color,
    source_line_strip,
    native_flat_sources,
    annotation,
    stencil_rect,
    text_shadow_bias,
    text_logic_xor,
    text_background_blend,
    surface_blend,
    surface_cutoff,
    surface_forced_blend,
    surface_texture_binding_only,
    mesh_edges,
    polygon_edge_style,
    polygon_face_lines,
    polygon_points,
    fixed_grid_point,
    fixed_grid_point_unit12,
    fixed_grid_point_sprite,
    edge_background_fallback,
    line_style,
    textured_style,
    surface_texture,
    surface_texture_unavailable,
    point_style,
    depth_pass,
    user_program,
);
