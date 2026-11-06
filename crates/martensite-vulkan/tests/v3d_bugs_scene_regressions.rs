//! V3d bugs scene regressions, each in an isolated worker under the V3d group policy.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
#![allow(non_snake_case)] // C1 follows the reference case name.
include!("case_host/session_modules.rs");
#[path = "case_host/dump_diff.rs"]
mod dump_diff;
#[allow(dead_code)] // Only the units of the line fixture are used.
#[path = "viewer_fixtures/support/ais_line_fixture.rs"]
mod line_fixture;
mod v3d_bugs_scene_cases;
use std::error::Error;
use v3d_bugs_scene_cases as cases;

/// The V3d group runs every regression under a 1000 s CPU limit.
fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

/// A regression of the bugs group (1000 s CPU limit, bugs and common rules).
fn bugs(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_bugs_process_case::run_bugs_case(identity, body);
}

#[test]
fn bug25063() {
    run("bug25063", cases::line_from_top);
}

#[test]
fn bug25767() {
    run("bug25767", cases::huge_point_fitted);
}

#[test]
fn bug29837() {
    run("bug29837", cases::flat_points_fitted);
}

#[test]
fn bug28361() {
    run("bug28361", cases::persistent_box_highlight);
}

#[test]
fn bug24996() {
    run("bug24996", cases::zoomed_view_not_clipped);
}

#[test]
fn bug21757_1() {
    run("bug21757_1", cases::meshed_torus_color);
}

#[test]
fn bug21757() {
    run("bug21757", cases::meshed_torus_area);
}

#[test]
fn bug21757_2() {
    run("bug21757_2", cases::planar_torus_color);
}

/// lavapipe: `edge_tie` (45-degree dash axes). AMD: `edge_tie_shift`, after the native
/// `AMD_DEBUG=nonggc` control reproduces the 9 dashed polygon-edge pixels radeonsi culls.
#[test]
fn buc60738() {
    run("buc60738", cases::dashed_hollow_sphere);
}

#[test]
fn buc60740() {
    run("buc60740", cases::transparent_cylinder);
}

#[test]
fn bug24610() {
    run("bug24610", cases::erased_object_not_fitted);
}

#[test]
fn bug25466() {
    run("bug25466", cases::mode_change_with_selection);
}

#[test]
fn bug23525() {
    run("bug23525", cases::detected_edge_dumped);
}

#[test]
fn bug25251() {
    run("bug25251", cases::vertex_after_shaded_mode);
}

#[test]
fn bug23200_1() {
    run("bug23200_1", cases::untriangulated_shaded_cone);
}

#[test]
fn C1() {
    run("C1", cases::prebuilt_face_rectangle);
}

#[test]
fn bug23407_2() {
    run("bug23407_2", cases::dashed_face_boundaries);
}

#[test]
fn bug25071() {
    run("bug25071", cases::cone_outlines_selected);
}

#[test]
fn bug24391() {
    run("bug24391", cases::removed_object_displayed_again);
}

#[test]
fn bug24835() {
    run("bug24835", cases::removed_highlight_not_drawn);
}

#[test]
fn bug25363() {
    run("bug25363", cases::points_fitted_to_area);
}

#[test]
fn bug26082() {
    run("bug26082", cases::resized_views_scaled);
}

#[test]
fn bug25552() {
    run("bug25552", cases::objects_hidden_per_view);
}

#[test]
fn bug26357() {
    run("bug26357", cases::panning_keeps_scale);
}

#[test]
fn bug26792() {
    run("bug26792", cases::persistent_box_z_fitted);
}

#[test]
fn bug5990() {
    run("bug5990", cases::wire_colors_in_shading);
}

#[test]
fn bug22900() {
    run("bug22900", cases::recolored_compound_shaded);
}

#[test]
fn bug28036_1() {
    run("bug28036_1", cases::nested_compound_colors);
}

#[test]
fn bug25687_1() {
    run("bug25687_1", cases::colored_solid_culled);
}

#[test]
fn bug26014() {
    run("bug26014", cases::untriangulated_colored_cone);
}

#[test]
fn bug28621() {
    run("bug28621", cases::transparency_unset_restores_color);
}

#[test]
fn bug23253() {
    run("bug23253", cases::standard_orientations);
}

#[test]
fn bug24714() {
    run("bug24714", cases::projection_size_set);
}

#[test]
fn bug32073() {
    run("bug32073", cases::flipped_output);
}

#[test]
fn bug31673() {
    run("bug31673", cases::location_rotations_composed);
}

#[test]
fn bug25773() {
    run("bug25773", cases::free_boundary_properties);
}

#[test]
fn bug25695() {
    run("bug25695", cases::plastic_dynamic_highlight);
}

#[test]
fn bug29051() {
    run("bug29051", cases::transparent_highlight_with_oit);
}

#[test]
fn bug28061() {
    run("bug28061", cases::hidden_line_box_highlighted);
}

#[test]
fn bug31652() {
    run("bug31652", cases::poly_hidden_line_sphere_highlighted);
}

#[test]
fn bug24762_coloredshape() {
    run("bug24762_coloredshape", cases::customized_sub_shapes);
}

#[test]
fn bug25459_coloredshape() {
    run("bug25459_coloredshape", cases::material_keeps_face_color);
}

#[test]
fn bug25723() {
    run("bug25723", cases::rotation_after_erase);
}

#[test]
fn bug25723_1() {
    run("bug25723_1", cases::rotation_about_selection);
}

#[test]
fn bug30561() {
    run("bug30561", cases::children_keep_placement);
}

#[test]
fn bug30823() {
    run("bug30823", cases::erased_parent_keeps_children);
}

#[test]
fn bug25400() {
    run("bug25400", cases::box_through_layers);
}

#[test]
fn bug22795() {
    run("bug22795", cases::boxes_in_deleted_layers);
}

#[test]
fn bug29847() {
    run("bug29847", cases::background_tolerance);
}

#[test]
fn bug23425() {
    run("bug23425", cases::gif_border_filter);
}

#[test]
fn bug25814() {
    run("bug25814", cases::sub_shape_triangulation);
}

#[test]
fn bug24307_1() {
    run("bug24307_1", cases::culled_spheres);
}

#[test]
fn bug24307_2() {
    run("bug24307_2", cases::culled_boxes);
}

#[test]
fn bug27655() {
    run("bug27655", cases::transparent_triangulation);
}

#[test]
fn bug29787() {
    run("bug29787", cases::boundaries_up_to_continuity);
}

#[test]
fn bug24855() {
    run("bug24855", cases::standard_materials);
}

#[test]
fn bug2883_1() {
    run("bug2883_1", cases::compound_material);
}

#[test]
fn bug28036_2() {
    run("bug28036_2", cases::nested_compound_colors_shaded);
}

#[test]
fn bug23191() {
    run("bug23191", cases::regrouped_faces_from_bottom);
}

#[test]
fn bug31341() {
    run("bug31341", cases::culled_by_presentation_range);
}

#[test]
fn bug25687_2() {
    run("bug25687_2", cases::hidden_face_disables_culling);
}

#[test]
fn bug24867() {
    run("bug24867", cases::wide_outline_highlight);
}

#[test]
fn bug349() {
    run("bug349", cases::perspective_hidden_lines);
}

#[test]
fn bug30630_1() {
    run("bug30630_1", cases::mirrored_shells);
}

#[test]
fn bug30630_2() {
    run("bug30630_2", cases::mirrored_locations);
}

#[test]
fn bug30630_3() {
    run("bug30630_3", cases::mirrored_solids);
}

#[test]
fn bug25300_2() {
    run("bug25300_2", cases::isolines_on_triangulation);
}

#[test]
fn bug5988() {
    run("bug5988", cases::textured_box_beside_trihedron);
}

#[test]
fn bug22196() {
    bugs("bug22196", cases::repeated_face_texture);
}
