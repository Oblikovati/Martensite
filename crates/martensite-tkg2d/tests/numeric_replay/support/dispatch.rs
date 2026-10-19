//! One operation-local replay over ordinary public geometry APIs.
use super::{adaptor_cases, bspl, curves, eval, grid, output::Output, simple};
type NumericOperation = fn(&Output, &[f64]);
pub const OPERATIONS: &[(&str, NumericOperation)] = &[
    ("g2_point", simple::point_case),
    ("g2_direction", simple::direction_case),
    ("g2_vector", simple::vector_case),
    ("g2_axis", simple::axis_case),
    ("g2_trsf", simple::trsf_case),
    ("g2_curve", curves::curve_case),
    ("g2_transform", curves::transform_case),
    ("g2_conic", curves::conic_case),
    ("g2_trimmed", curves::trimmed_case),
    ("g2_offset", curves::offset_case),
    ("g2_offset_utils", curves::offset_utils_case),
    ("g2_bezier", bspl::bezier_case),
    ("g2_bspline", bspl::bspline_case),
    ("g2_evalrep", bspl::evalrep_case),
    ("a2_adaptor", adaptor_cases::adaptor_case),
    ("a2_load", adaptor_cases::load_case),
    ("a2_offset_load", adaptor_cases::offset_load_case),
    ("a2_evalrep", adaptor_cases::evalrep_case),
    ("g2_eval", eval::eval_case),
    ("g2_hash", eval::hash_case),
    ("g2_hash_frame", eval::hash_frame_case),
    ("g2_grid", grid::grid_case),
    ("g2_grid_adaptor", grid::grid_adaptor_case),
    ("g2_grid_evalrep", grid::grid_evalrep_case),
    ("g2_grid_null", grid::grid_null_case),
];
pub fn execute(operation: &str, values: &[f64]) -> String {
    let (_, execute) = OPERATIONS
        .iter()
        .find(|(name, _)| *name == operation)
        .unwrap_or_else(|| {
            panic!("operation {operation:?}; expected registered 2D numeric operation")
        });
    let output = Output::captured();
    execute(&output, values);
    output.finish()
}
