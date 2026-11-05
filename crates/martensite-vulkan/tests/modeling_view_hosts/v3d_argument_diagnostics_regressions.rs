//! Dataset argument diagnostics preserve the complete analyzer and authored substring checks.
use super::*;

#[test]
fn bug24220() {
    case_bugs_process_case::run_bugs_case("v3d_argument_diagnostics_regressions::bug24220", || {
        v3d_argument_diagnostics_cases::argument_diagnostic_view(true)
    });
}

#[test]
fn bug24654() {
    case_bugs_process_case::run_bugs_case("v3d_argument_diagnostics_regressions::bug24654", || {
        v3d_argument_diagnostics_cases::argument_diagnostic_view(false)
    });
}
