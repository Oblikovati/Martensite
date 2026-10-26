//! Cache coefficients own their values after a scoped input borrow (#309).
use super::GeomBSplineCurve;
use crate::{adaptor3d::Adaptor3dCurve, geom_adaptor::GeomAdaptorCurve};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::{bsplclib::Cache, gp::Point3};
use std::sync::Arc;
const POLES: [Point3; 2] = [Point3::new(0., 0., 0.), Point3::new(2., 0., 0.)];

fn segment(weighted: bool) -> GeomBSplineCurve {
    let poles = Array1::from_shared(&POLES, 1);
    let knots = Array1::from_shared(&[0., 1.], 1);
    let multiplicities = Array1::from_shared(&[2, 2], 1);
    if weighted {
        let weights = Array1::from_shared(&[1., 2.], 1);
        return GeomBSplineCurve::with_weights(&poles, &weights, &knots, &multiplicities, 1, false)
            .unwrap();
    }
    GeomBSplineCurve::new(&poles, &knots, &multiplicities, 1, false).unwrap()
}

fn stored_pointers(curve: &GeomBSplineCurve) -> (*const f64, *const Point3, *const f64) {
    let state = curve.state.read();
    (
        state.flat_knots.as_slice().as_ptr(),
        state.poles.as_slice().as_ptr(),
        state.weights.as_slice().as_ptr(),
    )
}

fn owned_cache(curve: &GeomBSplineCurve) -> Cache {
    curve.with_cache_build_inputs(|(degree, periodic), flat, poles, weights| {
        let cache = Cache::new(degree, periodic, flat, poles, weights).unwrap();
        cache.build_cache(0.25, flat, poles, weights).unwrap();
        cache
    })
}

#[test]
fn cache_inputs_are_actual_arrays_under_one_read_guard() {
    let curve = segment(true);
    let expected = stored_pointers(&curve);
    curve.with_cache_build_inputs(|metadata, flat, poles, weights| {
        assert_eq!(metadata, (1, false));
        assert_eq!(flat.as_slice().as_ptr(), expected.0);
        assert_eq!(poles.as_slice().as_ptr(), expected.1);
        assert_eq!(weights.unwrap().as_slice().as_ptr(), expected.2);
        assert!(curve.state.try_write().is_none());
    });
    assert!(curve.state.try_write().is_some());
}

#[test]
fn cache_coefficients_survive_source_mutation_and_owner_retirement() {
    let curve = segment(false);
    let cache = owned_cache(&curve);
    assert_eq!(cache.d0::<Point3>(0.25).unwrap(), Point3::new(0.5, 0., 0.));
    curve.set_pole(1, Point3::new(4., 0., 0.)).unwrap();
    let updated = owned_cache(&curve);
    assert_eq!(
        updated.d0::<Point3>(0.25).unwrap(),
        Point3::new(3.5, 0., 0.)
    );
    drop(curve);
    assert_eq!(cache.d0::<Point3>(0.25).unwrap(), Point3::new(0.5, 0., 0.));
}

#[test]
fn a_refused_internal_builder_releases_the_input_guard() {
    let curve = segment(false);
    let refused: Result<(), &str> =
        curve.with_cache_build_inputs(|_, _, _, _| Err("intentional builder refusal"));
    assert_eq!(refused, Err("intentional builder refusal"));
    assert!(curve.state.try_write().is_some());
    assert_eq!(owned_cache(&curve).d0::<Point3>(0.25).unwrap().x(), 0.5);
}

#[test]
fn unwinding_an_internal_builder_releases_the_read_guard() {
    let curve = segment(false);
    let refused = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        curve.with_cache_build_inputs(|_, _, _, _| panic!("intentional builder panic"));
    }));
    assert!(refused.is_err());
    assert!(curve.state.try_write().is_some());
    curve.set_pole(1, Point3::new(4., 0., 0.)).unwrap();
    assert_eq!(owned_cache(&curve).d0::<Point3>(0.25).unwrap().x(), 3.5);
}

fn sample_shared_adaptor(adaptor: Arc<GeomAdaptorCurve>) {
    for _ in 0..1000 {
        for parameter in [0.125, 0.375, 0.625, 0.875] {
            assert_eq!(
                adaptor.d0(parameter).unwrap(),
                Point3::new(2. * parameter, 0., 0.)
            );
        }
    }
}

#[test]
fn shared_adaptor_readers_are_transferable_and_match_closed_form_values() {
    let curve = Handle::new(segment(false));
    let adaptor = Arc::new(GeomAdaptorCurve::with_curve(&curve.cast()).unwrap());
    let workers: Vec<_> = (0..4)
        .map(|_| {
            let shared = adaptor.clone();
            std::thread::spawn(move || sample_shared_adaptor(shared))
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
}
