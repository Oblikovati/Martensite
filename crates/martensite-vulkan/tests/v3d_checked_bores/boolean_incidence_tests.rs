use super::{IncidenceMeasurement, classify_incidence};
use crate::v3d_checked_bores::boolean_validation::LocalCheckMode;

#[test]
fn completed_measurement_and_stored_degenerate_pcurve_are_distinct() {
    assert_measurement([false, true, false], IncidenceMeasurement::Measured);
    assert_measurement([true, false, true], IncidenceMeasurement::Degenerated);
}

fn assert_measurement(state: [bool; 3], expected: IncidenceMeasurement) {
    let [degenerated, done, stored] = state;
    assert_eq!(
        classify_incidence(
            LocalCheckMode::DegeneratePcurveExclusionV2,
            degenerated,
            done,
            stored
        )
        .unwrap(),
        expected
    );
}

#[test]
fn legacy_nondegenerate_and_missing_pcurve_states_remain_incomplete() {
    let legacy =
        classify_incidence(LocalCheckMode::ExistingOrderedChecksV1, true, false, true).unwrap_err();
    assert_eq!(legacy.exception_type(), "BOP4_IncompleteCoverage");
    assert!(String::from_utf8_lossy(legacy.what()).contains("curve/surface measurement=NotDone"));
    assert_v2_incomplete([false, false, true]);
    assert_v2_incomplete([true, false, false]);
    assert_v2_incomplete([true, true, true]);
}

fn assert_v2_incomplete(state: [bool; 3]) {
    let [degenerated, done, stored] = state;
    assert!(
        classify_incidence(
            LocalCheckMode::DegeneratePcurveExclusionV2,
            degenerated,
            done,
            stored
        )
        .is_err()
    );
}
