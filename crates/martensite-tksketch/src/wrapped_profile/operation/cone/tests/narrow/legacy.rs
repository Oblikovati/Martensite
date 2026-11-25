//! The old public mode retains its actual narrow-sector refusal; expanded success is separate.
use super::*;
use std::error::Error;
#[test]
fn legacy_public_cone_preserves_the_typed_narrow_sine_refusal_and_owned_input() {
    let request = fixture::request(NormalSense::Outward, fixture::Quadrant::PositiveX);
    let before = request.clone();
    let failure = crate::surface_sketch::normal_extrude_cone_profile(&request)
        .err()
        .expect("legacy cone unexpectedly completed the formerly unsupported narrow sector");
    eprintln!(
        r#"{{"event":"legacy_narrow_cone_refusal","semi_angle":{},"failure":{:?}}}"#,
        request.support.semi_angle,
        format!("{failure:?}")
    );
    assert_eq!(request, before);
    assert_eq!(
        failure.class(),
        WrappedFailureClass::Unsupported,
        "{failure:?}"
    );
    assert_eq!(failure.stage(), WrappedStage::Construction, "{failure:?}");
    assert!(has_sine_refusal(&failure), "{failure:?}");
}
fn has_sine_refusal(failure: &(dyn Error + 'static)) -> bool {
    let mut current = Some(failure);
    while let Some(cause) = current {
        if let Some(sector) = cause.downcast_ref::<ConicalSectorFailure>() {
            return matches!(sector, ConicalSectorFailure::UnresolvedSine);
        }
        current = cause.source();
    }
    false
}
