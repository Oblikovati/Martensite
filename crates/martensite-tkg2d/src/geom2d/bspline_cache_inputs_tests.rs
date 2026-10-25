//! Planar input views preserve detached public arrays and the existing owned span cache (#309).
use super::Geom2dBSplineCurve;
use crate::{adaptor2d::Adaptor2dCurve2d, geom2d_adaptor::Geom2dAdaptorCurve};
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkmath::gp::Point2;
const POLES: [Point2; 2] = [Point2::new(0., 0.), Point2::new(2., 0.)];

fn segment() -> Handle<Geom2dBSplineCurve> {
    let poles = Array1::from_shared(&POLES, 1);
    let knots = Array1::from_shared(&[0., 1.], 1);
    let mults = Array1::from_shared(&[2, 2], 1);
    Handle::new(Geom2dBSplineCurve::new(&poles, &knots, &mults, 1, false).unwrap())
}

#[test]
fn planar_nonrational_views_borrow_the_actual_sequence() {
    let curve = segment();
    let pointer = curve.state.read().poles.as_slice().as_ptr();
    curve.with_cache_build_inputs(|metadata, _, poles, weights| {
        assert_eq!(metadata, (1, false));
        assert_eq!(poles.as_slice().as_ptr(), pointer);
        assert!(weights.is_none());
        assert!(curve.state.try_write().is_none());
    });
    assert!(curve.state.try_write().is_some());
}

#[test]
fn public_pole_copies_remain_detached_from_private_cache_views() {
    let curve = segment();
    let mut copied = curve.poles();
    copied.set_value(1, Point2::new(4., 0.)).unwrap();
    curve.with_cache_build_inputs(|_, _, poles, _| {
        assert_ne!(poles.as_slice().as_ptr(), copied.as_slice().as_ptr());
        assert_eq!(poles.as_slice()[0], Point2::new(0., 0.));
    });
    assert_eq!(curve.pole(1).unwrap(), Point2::new(0., 0.));
}

#[test]
fn cached_coefficients_stay_owned_while_a_fresh_adaptor_reads_the_edit() {
    let curve = segment();
    let adaptor = Geom2dAdaptorCurve::with_curve(&curve.cast()).unwrap();
    assert_eq!(adaptor.d0(0.25).unwrap(), Point2::new(0.5, 0.));
    curve.set_pole(1, Point2::new(4., 0.)).unwrap();
    let fresh = Geom2dAdaptorCurve::with_curve(&curve.cast()).unwrap();
    assert_eq!(fresh.d0(0.25).unwrap(), Point2::new(3.5, 0.));
    drop(curve);
    assert_eq!(adaptor.d0(0.25).unwrap(), Point2::new(0.5, 0.));
}
