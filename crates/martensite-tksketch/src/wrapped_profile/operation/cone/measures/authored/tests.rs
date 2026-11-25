//! #523 projects actual issued authored-volume refusals through the original source map.
use super::*;
use crate::surface_sketch::HistorySource;
use std::error::Error;
const SQUARE: &[&[[f64; 2]]] = &[
    &[[2., 0.], [3., 0.]],
    &[[3., 0.], [3., 1.]],
    &[[3., 1.], [2., 1.]],
    &[[2., 1.], [2., 0.]],
];
fn original() -> DevelopedRegionRequest {
    DevelopedRegionRequest {
        region: SurfaceRegionId(9),
        loops: vec![crate::surface_sketch::wrapped_profile::tests::contour(
            101, SQUARE,
        )],
        depths: [-0.125, 0.125],
        tolerance: 1e-5,
        max_fit_level: 8,
    }
}
fn issued(source: &DevelopedRegionRequest, alpha: f64, limit: u32) -> ConicalAuthoredFluxError {
    let spans: Vec<_> = source.loops[0]
        .iter()
        .map(|piece| piece.poles.as_slice())
        .collect();
    let contours = [ConicalAuthoredContour {
        spans: &spans,
        reverse: false,
    }];
    let query = refusal_query(source, &contours, alpha, limit);
    certify_conical_authored_flux(query, || false)
        .err()
        .expect("issued authored refusal")
}
#[test]
fn ordinary_integral_span_refusal_keeps_actual_authored_curve_and_work() {
    let source = original();
    let cause = issued(&source, 0.5, 4096);
    assert!(cause.depth_cause().is_none());
    assert!(cause.arithmetic_cause().is_some());
    let location = cause
        .span_location()
        .expect("issued ordinary integral span");
    assert_owner_and_work(&source, cause, location);
}
#[test]
fn depth_clearance_refusal_keeps_its_existing_authored_curve_mapping() {
    let mut source = original();
    source.depths = [-100., -99.];
    let cause = issued(&source, 0.5, 100_000);
    let (contour, span, _) = cause.depth_cause().expect("issued depth-clearance refusal");
    let location = (contour, span);
    assert_eq!(cause.span_location(), Some(location));
    assert_owner_and_work(&source, cause, location);
}
#[test]
fn genuinely_global_authored_refusal_does_not_invent_a_curve() {
    let source = original();
    let cause = issued(&source, 0., 100_000);
    assert_eq!(cause.span_location(), None);
    let operations = cause.operations();
    let public = failed(&source, cause).public();
    assert_eq!(public.entities(), &[HistorySource::Region(source.region)]);
    assert_eq!(retained(&public).operations(), operations);
    assert_eq!(retained(&public).span_location(), None);
}
fn assert_owner_and_work(
    source: &DevelopedRegionRequest,
    cause: ConicalAuthoredFluxError,
    location: (usize, usize),
) {
    let operations = cause.operations();
    let expected = source.loops[location.0][location.1].span;
    let failure = failed(source, cause);
    assert_eq!(failure.region, Some(source.region));
    assert_eq!(failure.span, Some(expected));
    let public = failure.public();
    assert_eq!(
        public.entities(),
        &[
            HistorySource::Region(source.region),
            HistorySource::Curve(expected.curve)
        ]
    );
    assert_eq!(retained(&public).operations(), operations);
    assert_eq!(retained(&public).span_location(), Some(location));
}
fn retained(failure: &WrappedNormalFailure) -> &ConicalAuthoredFluxError {
    let mut current: &(dyn Error + 'static) = failure;
    loop {
        if let Some(error) = current.downcast_ref::<ConicalAuthoredFluxError>() {
            return error;
        }
        current = current
            .source()
            .expect("original public authored error source");
    }
}

fn refusal_query<'a>(
    source: &DevelopedRegionRequest,
    contours: &'a [ConicalAuthoredContour<'a>],
    alpha: f64,
    limit: u32,
) -> ConicalAuthoredFluxQuery<'a> {
    ConicalAuthoredFluxQuery {
        contours,
        semi_angle: alpha,
        depths: source.depths,
        outward: true,
        level: 6,
        clearance_level: 0,
        volume_width: 1e-7,
        flux: [0., 0.],
        arithmetic: ConeClearanceOptions {
            fractional_bits: 128,
            max_operations: limit,
        },
    }
}
