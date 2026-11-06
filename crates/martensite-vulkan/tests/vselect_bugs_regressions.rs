//! Selection bugs regressions, each in an isolated worker under the selection group policy.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "case_host/dump_diff.rs"]
mod dump_diff;
#[path = "support/selection_occurrence_rank.rs"]
mod selection_occurrence_rank;
mod vselect_bugs_cases;
use std::error::Error;
use vselect_bugs_cases as cases;

/// The selection group runs every regression under a 300 s CPU limit.
fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case_within(identity, 300, body);
}

#[test]
fn bug727() {
    run("bug727", cases::deselect_second_object);
}

#[test]
fn bug23539_1() {
    run("bug23539_1", cases::polygon_edge_counts);
}

#[test]
fn bug23539_2() {
    run("bug23539_2", cases::polygon_xor_counts);
}

#[test]
fn bug26147() {
    run("bug26147", cases::overlapping_rectangle_counts);
}

#[test]
fn bug23649_1() {
    run("bug23649_1", cases::shifted_sub_shape_highlights);
}

#[test]
fn bug23649_3() {
    run("bug23649_3", cases::shifted_object_highlights);
}

#[test]
fn bug26159() {
    run("bug26159", cases::vertex_tolerance_hit);
}

#[test]
fn bug26721() {
    run("bug26721", cases::face_highlight_color);
}

#[test]
fn bug26641() {
    run("bug26641", cases::persistent_object_picked);
}

#[test]
fn bug23649_2() {
    run("bug23649_2", cases::highlight_selected_sub_shapes);
}

#[test]
fn bug23649_4() {
    run("bug23649_4", cases::highlight_selected_objects);
}

#[test]
fn bug26209() {
    run("bug26209", cases::fit_to_selection);
}

#[test]
fn bug22016() {
    run("bug22016", cases::redisplayed_selection_color);
}

#[test]
fn bug26726() {
    run("bug26726", cases::erase_selected_objects);
}

#[test]
fn bug26596() {
    run("bug26596", cases::twice_erased_not_detected);
}

#[test]
fn bug27621_1() {
    run("bug27621_1", cases::detected_then_erased);
}

#[test]
fn bug27621_2() {
    run("bug27621_2", cases::detected_then_cleared);
}

#[test]
fn bug26870() {
    run("bug26870", cases::deactivated_mode_recomputed);
}

#[test]
fn bug30717() {
    run("bug30717", cases::erased_selection_presentation);
}

#[test]
fn bug26960() {
    run("bug26960", cases::dynamic_highlight_moved);
}

#[test]
fn bug27986_1() {
    run("bug27986_1", cases::moved_global_highlight);
}

#[test]
fn bug27986_2() {
    run("bug27986_2", cases::moved_local_highlight);
}

#[test]
fn bug28988() {
    run("bug28988", cases::reset_location_detected);
}

#[test]
fn bug30672() {
    run("bug30672", cases::removed_object_relocated);
}

#[test]
fn bug26146() {
    run("bug26146", cases::point_set_compound_removed);
}

/// The infinite isolines span more than 2^15 px; `line_guard_band` clips them as GL does.
#[test]
fn bug129_1() {
    run("bug129_1", cases::infinite_cylinder_selected);
}

#[test]
fn bug22018() {
    run("bug22018", cases::overlapping_circles_detected);
}

#[test]
fn bug26217() {
    run("bug26217", cases::partial_rectangle_overlap);
}

#[test]
fn bug27477() {
    run("bug27477", cases::outline_circle_depth);
}

#[test]
fn bug31440() {
    run("bug31440", cases::polygon_selection_without_overlap);
}

// The recorded reference build has no image library, so it cannot load the
// dumps; those dumps are compared byte for byte with these, and the image diff runs here.
#[test]
fn bug27805() {
    run("bug27805", cases::fit_selected_matches_fit_all);
}

#[test]
fn bug25492() {
    run("bug25492", cases::picked_face_in_exploded_faces);
}

#[test]
fn bug32205() {
    run("bug32205", cases::replace_extra_toggles);
}

#[test]
fn bug_polyline_inclusion() {
    run("bug_polyline_inclusion", cases::polyline_strict_inclusion);
}

#[test]
fn bug12121() {
    run("bug12121", cases::connected_instance_sub_shapes);
}

#[test]
fn bug26719_1() {
    run("bug26719_1", cases::persistent_box_detected_over_face);
}

#[test]
fn bug1164_1() {
    run("bug1164_1", cases::moved_face_fitted);
}

#[test]
fn bug32482() {
    run("bug32482", cases::removed_object_not_picked);
}

#[test]
fn bug22048() {
    run("bug22048", cases::single_pick_clears_multiple);
}

#[test]
fn buc60587() {
    run("buc60587", cases::recolored_selection);
}

#[test]
fn buc60688() {
    run("buc60688", cases::small_face_highlighted);
}

#[test]
fn bug113() {
    run("bug113", cases::toggled_faces_detected);
}

#[test]
fn bug129_2() {
    run("bug129_2", cases::trimmed_cylinder_selected);
}

#[test]
fn bug28365() {
    run("bug28365", cases::vertex_filter_blocks_object);
}

#[test]
fn bug26462_1() {
    run("bug26462_1", cases::sensitivity_follows_width);
}

#[test]
fn bug25624() {
    run("bug25624", cases::perspective_pick_of_small_boxes);
}

#[test]
fn bug24420() {
    run("bug24420", cases::plane_sensitivity_types);
}

#[test]
fn bug24564() {
    run("bug24564", cases::box_over_plane);
}

#[test]
fn bug24569() {
    run("bug24569", cases::changed_plane_sensitivity);
}

#[test]
fn bug28205_1() {
    run("bug28205_1", cases::rotated_selection_images);
}

#[test]
fn bug28205_2() {
    run("bug28205_2", cases::selection_mode_image);
}

#[test]
fn bug31221() {
    run("bug31221", cases::combined_context_filters);
}

#[test]
fn bug7186() {
    run("bug7186", cases::empty_rectangle_clears);
}

#[test]
fn bug26413() {
    run("bug26413", cases::pixel_tolerance_detection);
}

#[test]
fn bug25103() {
    run("bug25103", cases::turned_walls_detected);
}

#[test]
fn bug25335() {
    run("bug25335", cases::moved_box_edges_picked);
}

#[test]
fn bug25507() {
    run("bug25507", cases::placed_view_detected);
}

#[test]
fn bug30824() {
    run("bug30824", cases::parent_selected_without_children);
}

mod vselect_clipping_cases;

#[test]
fn clipping_bug26973() {
    run(
        "clipping_bug26973",
        vselect_clipping_cases::four_plane_targets,
    );
}

#[test]
fn clipping_bug29791() {
    run(
        "clipping_bug29791",
        vselect_clipping_cases::orthogonal_and_negative_depth,
    );
}

#[test]
fn clipping_bug30695() {
    run(
        "clipping_bug30695",
        vselect_clipping_cases::rectangle_containment_overlap,
    );
}

#[test]
fn clipping_bug30777() {
    run(
        "clipping_bug30777",
        vselect_clipping_cases::clipped_sphere_point,
    );
}

#[test]
fn clipping_bug30906() {
    run(
        "clipping_bug30906",
        vselect_clipping_cases::override_does_not_leak,
    );
}

#[test]
fn clipping_bug31650() {
    run(
        "clipping_bug31650",
        vselect_clipping_cases::shared_global_local_plane,
    );
}

#[test]
fn bug30484_1() {
    run("bug30484_1", cases::priority_boxes);
}

#[test]
fn bug30484_2() {
    run("bug30484_2", cases::priority_arrays);
}

#[test]
fn bug30484_3() {
    run("bug30484_3", cases::priority_point);
}

#[test]
fn bug30484_4() {
    run("bug30484_4", cases::priority_quad);
}

#[test]
fn bug25098() {
    run("bug25098", cases::wire_edge_characteristics);
}

#[test]
fn bug25230() {
    run("bug25230", cases::perspective_grid_regions);
}

#[test]
fn bug25265() {
    run("bug25265", cases::perspective_front_point);
}

#[test]
fn bug26304() {
    run("bug26304", cases::persistent_ranked_points);
}

#[test]
fn bug27523() {
    run("bug27523", cases::obsolete_shape_owner);
}

#[test]
fn bug27893() {
    run("bug27893", cases::direct_selected_object);
}

#[test]
fn bug28310() {
    run("bug28310", cases::removed_object_detection);
}

#[test]
fn bug26284() {
    run("bug26284", cases::child_selection_recomputed);
}

#[test]
fn bug27757() {
    run("bug27757", cases::child_selected_through_parent);
}

#[test]
fn bug25627() {
    run("bug25627", cases::selected_topology_roundtrip);
}

#[test]
fn bug26779() {
    run("bug26779", cases::local_and_neutral_rows);
}

#[test]
fn bug27848() {
    run("bug27848", cases::line_shape_discrimination);
}

#[test]
fn bug27530() {
    run("bug27530", cases::overlapping_box_cycle);
}

#[test]
fn bug31412() {
    run("bug31412", cases::occluded_box_projections);
}

#[test]
fn bug31425() {
    run("bug31425", cases::free_edge_camera_pick);
}
