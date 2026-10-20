use super::print::Output;
use super::{app_blend, fair_curve, local_analysis, plate, toptrans};
type Operation = fn(&Output, &[f64]);
const OPERATIONS: &[(&str, Operation)] = &[
    ("ab_appsurf", app_blend::appsurf_case),
    ("ab_context", app_blend::context_case),
    ("tt_curve", toptrans::curve_case),
    ("tt_surface", toptrans::surface_case),
    ("tt_states", toptrans::states_case),
    ("plate_solve", plate::solve_case),
    ("plate_constraint", plate::constraint_case),
    ("la_curve", local_analysis::curve_case),
    ("la_surface", local_analysis::surface_case),
    ("fc_law", fair_curve::law_case),
    ("fc_density", fair_curve::density_case),
    ("fc_energy", fair_curve::energy_case),
    ("fc_batten", fair_curve::batten_case),
];

/// Executes an owned family with an operation-local output buffer.
pub fn run_values(operation: &str, values: &[f64]) -> Option<String> {
    let (_, case) = OPERATIONS.iter().find(|(name, _)| *name == operation)?;
    let output = Output::default();
    case(&output, values);
    Some(output.finish())
}
