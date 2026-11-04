//! Mathematical evaluation order, failure short-circuit and mutable settings are local.
#![cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
#[path = "sampling_capture/tracing_curve.rs"]
mod tracing_curve;
use martensite_geometry_raster::CurveSampling;
use martensite_raster_core::{RasterFailureContext, RasterViewer, VirtualWindowSystem};
use martensite_tkernel::standard::Failure;
use std::rc::Rc;
use tracing_curve::TracingSamplerCurve;
fn draw(sampler: &CurveSampling, curve: &TracingSamplerCurve) -> Result<(), Failure> {
    let system = VirtualWindowSystem::<RasterFailureContext>::connect_virtual().unwrap();
    let mut viewer: RasterViewer = RasterViewer::with_adapter(Some(Rc::new(system)));
    viewer.make_view(1, "+X+Y", 0, 0, 100, 100).unwrap();
    let mut display = viewer.make_display(1);
    let result = sampler.draw_curve_on(curve, &mut display);
    display.flush();
    viewer.delete_view(1);
    result
}
#[test]
fn dyadic_line_parameters_keep_interval_and_midpoint_order() {
    let sampler = CurveSampling::new(4, 0.01, 0);
    let curve = TracingSamplerCurve::new(None);
    draw(&sampler, &curve).unwrap();
    assert_eq!(
        curve.parameters(),
        [0., 0.5, 0.25, 1., 0.75, 1.5, 1.25, 2., 1.75]
    );
}
#[test]
fn midpoint_failure_retains_category_and_stops_further_evaluation() {
    let curve = TracingSamplerCurve::new(Some(0.25));
    let error = draw(&CurveSampling::new(4, 0.01, 0), &curve).unwrap_err();
    assert_eq!(error.exception_type(), "CurveEvaluationUnavailable");
    assert_eq!(error.message(), "u=0.25: expected defined tracing curve");
    assert_eq!(curve.parameters(), [0., 0.5, 0.25]);
}
#[test]
fn changed_sampling_count_is_read_on_the_next_draw() {
    let sampler = CurveSampling::new(4, 0.01, 0);
    let before = TracingSamplerCurve::new(None);
    draw(&sampler, &before).unwrap();
    sampler.set_discretisation(6);
    let after = TracingSamplerCurve::new(None);
    draw(&sampler, &after).unwrap();
    assert_eq!(
        (before.parameters().len(), after.parameters().len()),
        (9, 13)
    );
    assert_eq!(after.parameters()[1], 1. / 3.);
}
#[test]
fn settings_preserve_values_and_do_not_alias_another_sampler() {
    let sampler = CurveSampling::new(4, 0.01, 0);
    let other = CurveSampling::new(30, 0.125, 1);
    for count in [-4, 0, 1, 2, 3, 4, 6, 7] {
        sampler.set_discretisation(count);
        assert_eq!(sampler.discretisation(), count);
    }
    sampler.set_deflection(0.5);
    sampler.set_draw_mode(2);
    assert_eq!((sampler.deflection(), sampler.draw_mode()), (0.5, 2));
    assert_eq!(
        (
            other.discretisation(),
            other.deflection(),
            other.draw_mode()
        ),
        (30, 0.125, 1)
    );
}
