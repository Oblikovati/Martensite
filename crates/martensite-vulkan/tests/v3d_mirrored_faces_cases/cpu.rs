use super::dataset;
use crate::shape_checks::{MassCheck, MassKind, check_mass};
use crate::shape_report_checks::{ShapeCounts, check_shape_counts};

#[test]
fn actual_case_mirrored_face_operation_keeps_all_fixed_authored_checks() {
    let shape = dataset::mirrored_face_result().unwrap();
    let expected_counts = [6, 11, 4, 2, 0, 0, 0, 1, 24];
    let expected = ShapeCounts {
        per_location: true,
        ..ShapeCounts::all(expected_counts)
    };
    assert_eq!(
        check_shape_counts(&shape, "result", &expected).unwrap(),
        expected_counts
    );
    let area = check_mass(&shape, &MassCheck::near(MassKind::Area, 306.285))
        .unwrap()
        .mass();
    assert!((area - 306.285).abs() <= 306.285 * 0.01);
    assert!(crate::shape_checks::check_shape_valid(&shape).unwrap());
}
