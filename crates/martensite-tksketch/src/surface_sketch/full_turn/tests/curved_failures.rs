// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkbo::profile_extrusion::ProfileCurve;
use martensite_tkmath::predicates::roots::metric_bounds::{
    BoundedArithmeticFailure, ConicalSectorFailure, CylindricalProfileFailure,
    RationalReferenceFailure,
};
use std::error::Error;

static_assertions::assert_impl_all!(CurvedBandFailure: Send, Sync);

#[test]
fn curved_cone_retains_typed_arithmetic_work_limit_and_selected_aperture() {
    let (band, mut request) = curved_cone_aperture::fixture();
    request.proof.arithmetic.max_operations = 100;
    let failure =
        normal_extrude_cone_band_with_curved_hole(&band, &request, options()).unwrap_err();
    assert_eq!(failure.band(), band.region);
    assert_eq!(failure.stage(), SurfaceStage::Construction);
    let CurvedBandFailure::Wrapped {
        apertures, cause, ..
    } = &failure
    else {
        panic!("{failure:?}");
    };
    assert_eq!(apertures.as_slice(), &[request.selections[0].region]);
    assert_eq!(cause.class(), WrappedFailureClass::Incomplete);
    assert!(failure.source().unwrap().is::<WrappedNormalFailure>());
    let original = find_source::<ConicalSectorFailure>(&failure)
        .unwrap_or_else(|| panic!("missing original conical cause: {failure:?}"));
    let arithmetic = match original {
        ConicalSectorFailure::Arithmetic(source) => source,
        ConicalSectorFailure::Trigonometric(CylindricalProfileFailure::Arithmetic(source)) => {
            source.as_ref()
        }
        other => panic!("expected bounded arithmetic exhaustion: {other:?}"),
    };
    assert!(matches!(
        arithmetic,
        BoundedArithmeticFailure::WorkLimit {
            completed: 100,
            limit: 100
        }
    ));
}

#[test]
fn rational_aperture_retains_original_reference_limits_and_selected_aperture() {
    let (band, mut request) = curved_aperture::fixture();
    let ProfileCurve::Spline { weights, .. } = &mut request.profile.curves[0] else {
        panic!("quadratic aperture fixture");
    };
    *weights = Some(vec![1., 1., 2.]);
    let failure = normal_extrude_cylinder_band_with_rational_hole(&band, &request, options(), 1)
        .err()
        .unwrap();
    assert_eq!(failure.band(), band.region);
    assert_eq!(failure.stage(), SurfaceStage::Construction);
    let CurvedBandFailure::Rational { apertures, .. } = &failure else {
        panic!("{failure:?}");
    };
    assert_eq!(apertures.as_slice(), &[request.selections[0].region]);
    assert!(failure.source().unwrap().is::<RationalWrappedFailure>());
    let Some(RationalReferenceFailure::Exceeds { achieved, limits }) =
        find_source::<RationalReferenceFailure>(&failure)
    else {
        panic!("missing original rational reference limits: {failure:?}");
    };
    assert!(achieved.position > limits[0] || achieved.derivative > limits[1]);
    assert!(achieved.operations > 0);
}

fn find_source<'a, T: Error + 'static>(error: &'a (dyn Error + 'static)) -> Option<&'a T> {
    std::iter::successors(Some(error), |&current| current.source())
        .find_map(|current| current.downcast_ref())
}
