//! Actual cut, rational surface conversion and authored angular interior mesh.
use super::*;

#[test]
fn bug30008_2() {
    case_bugs_process_case::run_bugs_case(
        "v3d_angular_spline_regressions::bug30008_2",
        v3d_angular_spline::angular_spline_view,
    );
}
