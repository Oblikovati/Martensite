//! V3d bugs regressions of small scenes: lines and points, persistent and transparent shapes,
//! shaded tori and fitting of erased objects.
mod children;
mod compounds;
mod culling;
#[cfg(test)]
mod culling_cpu_baseline;
#[cfg(test)]
mod culling_performance_contract;
mod culling_phase;
#[cfg(test)]
mod culling_redraw_contract;
mod highlights;
mod images;
mod layers;
mod materials;
mod mirrors;
mod modes;
mod orientations;
mod outlines;
mod prebuilt_selection;
mod primitives;
mod shapes;
mod sub_shapes;
mod textured;
mod views;
mod wire_colors;
pub(crate) use children::{children_keep_placement, erased_parent_keeps_children};
pub(crate) use compounds::{
    compound_material, nested_compound_colors_shaded, regrouped_faces_from_bottom,
};
pub(crate) use culling::{
    culled_boxes, culled_by_presentation_range, culled_spheres, transparent_triangulation,
};
pub(crate) use highlights::{
    hidden_line_box_highlighted, perspective_hidden_lines, plastic_dynamic_highlight,
    poly_hidden_line_sphere_highlighted, transparent_highlight_with_oit, wide_outline_highlight,
};
pub(crate) use images::{background_tolerance, gif_border_filter, sub_shape_triangulation};
pub(crate) use layers::{box_through_layers, boxes_in_deleted_layers};
pub(crate) use materials::standard_materials;
pub(crate) use mirrors::{mirrored_locations, mirrored_shells, mirrored_solids};
pub(crate) use modes::{
    detected_edge_dumped, isolines_on_triangulation, mode_change_with_selection,
    untriangulated_shaded_cone, vertex_after_shaded_mode,
};
pub(crate) use orientations::{
    flipped_output, free_boundary_properties, location_rotations_composed, projection_size_set,
    standard_orientations,
};
pub(crate) use outlines::{
    boundaries_up_to_continuity, cone_outlines_selected, dashed_face_boundaries,
};
pub(crate) use prebuilt_selection::prebuilt_face_rectangle;
pub(crate) use primitives::{flat_points_fitted, huge_point_fitted, line_from_top};
pub(crate) use shapes::{
    dashed_hollow_sphere, erased_object_not_fitted, meshed_torus_area, meshed_torus_color,
    persistent_box_highlight, planar_torus_color, transparent_cylinder, zoomed_view_not_clipped,
};
pub(crate) use sub_shapes::{
    colored_solid_culled, customized_sub_shapes, hidden_face_disables_culling,
    material_keeps_face_color, nested_compound_colors, transparency_unset_restores_color,
    untriangulated_colored_cone,
};
pub(crate) use textured::{repeated_face_texture, textured_box_beside_trihedron};
pub(crate) use views::{
    objects_hidden_per_view, panning_keeps_scale, points_fitted_to_area,
    removed_highlight_not_drawn, removed_object_displayed_again, resized_views_scaled,
    rotation_about_selection, rotation_after_erase,
};
pub(crate) use wire_colors::{
    persistent_box_z_fitted, recolored_compound_shaded, wire_colors_in_shading,
};

/// What every scene program uses.
mod prelude;
