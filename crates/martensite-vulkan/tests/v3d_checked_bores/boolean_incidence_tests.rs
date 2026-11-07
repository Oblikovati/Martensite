use super::{IncidenceMeasurement, classify_incidence};
use crate::v3d_checked_bores::boolean_validation::LocalCheckMode;

#[test]
fn completed_measurement_and_stored_degenerate_pcurve_are_distinct() {
    assert_eq!(
        classify_incidence(
            LocalCheckMode::DegeneratePcurveExclusionV2,
            false,
            true,
            false
        )
        .unwrap(),
        IncidenceMeasurement::Measured
    );
    assert_eq!(
        classify_incidence(
            LocalCheckMode::DegeneratePcurveExclusionV2,
            true,
            false,
            true
        )
        .unwrap(),
        IncidenceMeasurement::Degenerated
    );
}

#[test]
fn legacy_nondegenerate_and_missing_pcurve_states_remain_incomplete() {
    let legacy =
        classify_incidence(LocalCheckMode::ExistingOrderedChecksV1, true, false, true).unwrap_err();
    assert_eq!(legacy.exception_type(), "BOP4_IncompleteCoverage");
    assert!(String::from_utf8_lossy(legacy.what()).contains("curve/surface measurement=NotDone"));
    assert!(
        classify_incidence(
            LocalCheckMode::DegeneratePcurveExclusionV2,
            false,
            false,
            true
        )
        .is_err()
    );
    assert!(
        classify_incidence(
            LocalCheckMode::DegeneratePcurveExclusionV2,
            true,
            false,
            false
        )
        .is_err()
    );
    assert!(
        classify_incidence(
            LocalCheckMode::DegeneratePcurveExclusionV2,
            true,
            true,
            true
        )
        .is_err()
    );
}
