//! A cancelled normal-volume proof and exhausted arithmetic keep different classes.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    BoundedArithmeticFailure, CylindricalMeasureError, CylindricalMeasureFailure,
    CylindricalMeasureLimit,
};
#[test]
fn member_normal_measure_preserves_cancellation_and_real_refusals() {
    for (cause, cancelled) in [
        (BoundedArithmeticFailure::Cancelled, true),
        (
            BoundedArithmeticFailure::WorkLimit {
                completed: 7,
                limit: 7,
            },
            false,
        ),
    ] {
        assert_eq!(normal_measure_failure(cause).cancelled(), cancelled);
    }
}
fn normal_measure_failure(cause: BoundedArithmeticFailure) -> FamilyFailure {
    let certificate = CylindricalMeasureError {
        requested_options: Default::default(),
        requested_limit: CylindricalMeasureLimit::Report,
        requested_depths: [-1., 2.],
        source_count: 1,
        cause: CylindricalMeasureFailure::Arithmetic(Box::new(cause)),
    };
    let cause = normal_measure::NormalMeasureFailure::Certificate(Box::new(certificate));
    let member = Refusal {
        stage: Stage::Measure,
        region: SurfaceRegionId(5),
        cause: RefusalCause::NormalMeasure(Box::new(cause)),
    };
    FamilyFailure::Member(Box::new(member))
}
