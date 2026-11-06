//! `DeShapeFixParameters` and `DEShapeFixParameters` hold the same values for
//! two crates. This gives the STEP defaults in the shape the transfer actors take.
use crate::de_step::DeStepParameters;
use martensite_tkde::de::{DeShapeFixParameters, DeShapeFixParametersFixMode};
use martensite_tkxsbase::xs_algo::{DEShapeFixMode, DEShapeFixParameters};

/// The mode of the same value.
fn mode(value: DeShapeFixParametersFixMode) -> DEShapeFixMode {
    match value {
        DeShapeFixParametersFixMode::NOT_FIX => DEShapeFixMode::NotFix,
        DeShapeFixParametersFixMode::FIX => DEShapeFixMode::Fix,
        _ => DEShapeFixMode::FixOrNot,
    }
}

/// The same parameters, field by field.
/// The parameters of the framework as the shape processor takes them.
pub fn converted(parameters: &DeShapeFixParameters) -> DEShapeFixParameters {
    DEShapeFixParameters {
        tolerance3d: parameters.tolerance3d,
        max_tolerance3d: parameters.max_tolerance3d,
        min_tolerance3d: parameters.min_tolerance3d,
        detalization_level: parameters.detalization_level,
        non_manifold: parameters.non_manifold,
        fix_free_shell_mode: mode(parameters.fix_free_shell_mode),
        fix_free_face_mode: mode(parameters.fix_free_face_mode),
        fix_free_wire_mode: mode(parameters.fix_free_wire_mode),
        fix_same_parameter_mode: mode(parameters.fix_same_parameter_mode),
        fix_solid_mode: mode(parameters.fix_solid_mode),
        fix_shell_orientation_mode: mode(parameters.fix_shell_orientation_mode),
        create_open_solid_mode: mode(parameters.create_open_solid_mode),
        fix_shell_mode: mode(parameters.fix_shell_mode),
        fix_face_orientation_mode: mode(parameters.fix_face_orientation_mode),
        fix_face_mode: mode(parameters.fix_face_mode),
        fix_wire_mode: mode(parameters.fix_wire_mode),
        fix_orientation_mode: mode(parameters.fix_orientation_mode),
        fix_add_natural_bound_mode: mode(parameters.fix_add_natural_bound_mode),
        fix_missing_seam_mode: mode(parameters.fix_missing_seam_mode),
        fix_small_area_wire_mode: mode(parameters.fix_small_area_wire_mode),
        remove_small_area_face_mode: mode(parameters.remove_small_area_face_mode),
        fix_intersecting_wires_mode: mode(parameters.fix_intersecting_wires_mode),
        fix_loop_wires_mode: mode(parameters.fix_loop_wires_mode),
        fix_split_face_mode: mode(parameters.fix_split_face_mode),
        auto_correct_precision_mode: mode(parameters.auto_correct_precision_mode),
        modify_topology_mode: mode(parameters.modify_topology_mode),
        modify_geometry_mode: mode(parameters.modify_geometry_mode),
        closed_wire_mode: mode(parameters.closed_wire_mode),
        preference_pcurve_mode: mode(parameters.preference_pcurve_mode),
        fix_reorder_mode: mode(parameters.fix_reorder_mode),
        fix_small_mode: mode(parameters.fix_small_mode),
        fix_connected_mode: mode(parameters.fix_connected_mode),
        fix_edge_curves_mode: mode(parameters.fix_edge_curves_mode),
        fix_degenerated_mode: mode(parameters.fix_degenerated_mode),
        fix_lacking_mode: mode(parameters.fix_lacking_mode),
        fix_self_intersection_mode: mode(parameters.fix_self_intersection_mode),
        remove_loop_mode: mode(parameters.remove_loop_mode),
        fix_reversed2d_mode: mode(parameters.fix_reversed2d_mode),
        fix_remove_pcurve_mode: mode(parameters.fix_remove_pcurve_mode),
        fix_remove_curve3d_mode: mode(parameters.fix_remove_curve3d_mode),
        fix_add_pcurve_mode: mode(parameters.fix_add_pcurve_mode),
        fix_add_curve3d_mode: mode(parameters.fix_add_curve3d_mode),
        fix_seam_mode: mode(parameters.fix_seam_mode),
        fix_shifted_mode: mode(parameters.fix_shifted_mode),
        fix_edge_same_parameter_mode: mode(parameters.fix_edge_same_parameter_mode),
        fix_notched_edges_mode: mode(parameters.fix_notched_edges_mode),
        fix_tail_mode: mode(parameters.fix_tail_mode),
        max_tail_angle: mode(parameters.max_tail_angle),
        max_tail_width: mode(parameters.max_tail_width),
        fix_self_intersecting_edge_mode: mode(parameters.fix_self_intersecting_edge_mode),
        fix_intersecting_edges_mode: mode(parameters.fix_intersecting_edges_mode),
        fix_non_adjacent_intersecting_edges_mode: mode(
            parameters.fix_non_adjacent_intersecting_edges_mode,
        ),
        fix_vertex_position_mode: mode(parameters.fix_vertex_position_mode),
        fix_vertex_tolerance_mode: mode(parameters.fix_vertex_tolerance_mode),
    }
}

/// `DeStepParameters::GetDefaultShapeFixParameters`.
pub(super) fn default_shape_fix_parameters() -> DEShapeFixParameters {
    converted(&DeStepParameters::default_shape_fix_parameters())
}
