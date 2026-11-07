//! The healing parameters of a node, as the transfers take them.
use martensite_tkde::de::{DeShapeFixParameters, DeShapeFixParametersFixMode};
use martensite_tkxsbase::xs_algo::{DEShapeFixMode, DEShapeFixParameters};

fn fix_mode(value: DeShapeFixParametersFixMode) -> DEShapeFixMode {
    match value {
        DeShapeFixParametersFixMode::NOT_FIX => DEShapeFixMode::NotFix,
        DeShapeFixParametersFixMode::FIX => DEShapeFixMode::Fix,
        _ => DEShapeFixMode::FixOrNot,
    }
}

macro_rules! same_fields {
    ($from:ident, [$($plain:ident),* $(,)?], [$($mode:ident),* $(,)?]) => {
        DEShapeFixParameters {
            $($plain: $from.$plain,)*
            $($mode: fix_mode($from.$mode),)*
        }
    };
}

/// The same values for the transfers; `transfer_shape_fix_parameters(&node.shape_fix_parameters())`.
pub fn transfer_shape_fix_parameters(parameters: &DeShapeFixParameters) -> DEShapeFixParameters {
    #[rustfmt::skip]
    let same = same_fields!(parameters, [tolerance3d, max_tolerance3d, min_tolerance3d, detalization_level, non_manifold], [
        fix_free_shell_mode, fix_free_face_mode, fix_free_wire_mode,
        fix_same_parameter_mode, fix_solid_mode, fix_shell_orientation_mode,
        create_open_solid_mode, fix_shell_mode, fix_face_orientation_mode, fix_face_mode,
        fix_wire_mode, fix_orientation_mode, fix_add_natural_bound_mode,
        fix_missing_seam_mode, fix_small_area_wire_mode, remove_small_area_face_mode,
        fix_intersecting_wires_mode, fix_loop_wires_mode, fix_split_face_mode,
        auto_correct_precision_mode, modify_topology_mode, modify_geometry_mode,
        closed_wire_mode, preference_pcurve_mode, fix_reorder_mode, fix_small_mode,
        fix_connected_mode, fix_edge_curves_mode, fix_degenerated_mode, fix_lacking_mode,
        fix_self_intersection_mode, remove_loop_mode, fix_reversed2d_mode,
        fix_remove_pcurve_mode, fix_remove_curve3d_mode, fix_add_pcurve_mode,
        fix_add_curve3d_mode, fix_seam_mode, fix_shifted_mode, fix_edge_same_parameter_mode,
        fix_notched_edges_mode, fix_tail_mode, max_tail_angle, max_tail_width,
        fix_self_intersecting_edge_mode, fix_intersecting_edges_mode,
        fix_non_adjacent_intersecting_edges_mode, fix_vertex_position_mode,
        fix_vertex_tolerance_mode,
    ]);
    same
}
