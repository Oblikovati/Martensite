//! Selection regressions of the selection bugs group: counts and highlights after picking by
//! point, rectangle and polygon.
mod circle_picks;
mod colors;
mod connected_picks;
mod counts;
mod erased_objects;
mod filters;
mod highlights;
mod locations;
mod persistent_picks;
mod placements;
mod planes;
mod scenes;
mod schemes;
mod selected_objects;
mod selection_images;
pub(crate) use circle_picks::{
    outline_circle_depth, overlapping_circles_detected, partial_rectangle_overlap,
    polygon_selection_without_overlap,
};
pub(crate) use colors::{
    recolored_selection, small_face_highlighted, toggled_faces_detected, trimmed_cylinder_selected,
};
pub(crate) use connected_picks::connected_instance_sub_shapes;
pub(crate) use counts::{
    deselect_second_object, overlapping_rectangle_counts, polygon_edge_counts, polygon_xor_counts,
};
pub(crate) use erased_objects::{
    deactivated_mode_recomputed, detected_then_cleared, detected_then_erased,
    erased_selection_presentation, twice_erased_not_detected,
};
pub(crate) use filters::{
    combined_context_filters, empty_rectangle_clears, perspective_pick_of_small_boxes,
    pixel_tolerance_detection, sensitivity_follows_width, vertex_filter_blocks_object,
};
pub(crate) use highlights::{
    face_highlight_color, persistent_object_picked, shifted_object_highlights,
    shifted_sub_shape_highlights, vertex_tolerance_hit,
};
pub(crate) use locations::{
    dynamic_highlight_moved, moved_global_highlight, moved_local_highlight,
    removed_object_relocated, reset_location_detected,
};
pub(crate) use persistent_picks::persistent_box_detected_over_face;
pub(crate) use placements::{
    moved_box_edges_picked, parent_selected_without_children, placed_view_detected,
    turned_walls_detected,
};
pub(crate) use planes::{box_over_plane, changed_plane_sensitivity, plane_sensitivity_types};
pub(crate) use scenes::{
    infinite_cylinder_selected, moved_face_fitted, point_set_compound_removed,
    removed_object_not_picked, single_pick_clears_multiple,
};
pub(crate) use schemes::{
    fit_selected_matches_fit_all, picked_face_in_exploded_faces, polyline_strict_inclusion,
    replace_extra_toggles,
};
pub(crate) use selected_objects::{
    erase_selected_objects, fit_to_selection, highlight_selected_objects,
    highlight_selected_sub_shapes, redisplayed_selection_color,
};
pub(crate) use selection_images::{rotated_selection_images, selection_mode_image};

/// What every selection bugs program uses.
mod prelude {
    pub(super) use crate::case_view::viewer_session::DisplayRequest;
    pub(super) use crate::primitive_shapes::{box_at, box_sized};
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use martensite_tkv3d::ais::AisSelectionScheme::{Replace, Xor};
    pub(super) use std::error::Error;

    /// Print `message` unless `count` selected owners are found; `expect_selected(&mut run, 2, "Error")`.
    pub(super) fn expect_selected(run: &mut SessionRun, count: i32, message: &str) {
        if run.session().selected_count() != count {
            println!("{message}");
        }
    }
}

mod persistent_priority;
pub(crate) use persistent_priority::{
    priority_arrays, priority_boxes, priority_point, priority_quad,
};

mod selection_frustum;
pub(crate) use selection_frustum::{
    persistent_ranked_points, perspective_front_point, perspective_grid_regions,
    wire_edge_characteristics,
};

mod owner_lifecycle;
pub(crate) use owner_lifecycle::{
    direct_selected_object, obsolete_shape_owner, removed_object_detection,
};

mod child_selection;
pub(crate) use child_selection::{child_selected_through_parent, child_selection_recomputed};

mod selection_queries;
pub(crate) use selection_queries::{
    line_shape_discrimination, local_and_neutral_rows, selected_topology_roundtrip,
};

mod hover_picking;
pub(crate) use hover_picking::{
    free_edge_camera_pick, occluded_box_projections, overlapping_box_cycle,
};
