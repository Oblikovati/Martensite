#[path = "../support/default_shape_counts.rs"]
mod default_shape_counts;
use default_shape_counts::assert_vertex_first_counts;
use martensite_tkbrep::topods::Shape;
#[path = "../boolean_area_assertion.rs"]
mod boolean_area_assertion;
use boolean_area_assertion::check_area;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

const COUNTS: [[usize; 9]; 4] = [
    [21, 37, 16, 16, 2, 1, 0, 1, 94],
    [3, 5, 2, 2, 1, 1, 0, 1, 15],
    [21, 34, 15, 15, 2, 1, 0, 1, 89],
    [3, 8, 3, 3, 1, 1, 0, 1, 20],
];

pub(super) fn check_result(result: &Shape, case: usize) {
    check_area(result, [438194., 3769.91, 438194., 3769.89][case - 1]);
    assert!(
        BRepCheckAnalyzer::new(result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap(),
        "bug13116_{case}: checkshape"
    );
    assert_vertex_first_counts(result, &COUNTS[case - 1]);
}
