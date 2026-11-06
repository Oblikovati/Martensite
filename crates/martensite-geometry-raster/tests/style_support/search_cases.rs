//! Picking resumes in the exact original index order and scales only pole targets.
use super::style_api::{find_index, find_net_index, scaled_target};
use martensite_raster_core::{
    RasterDisplay, RasterFailureContext, RasterViewer, VirtualWindowSystem,
};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Point2, Point3};
use std::rc::Rc;
fn with_display<T>(body: impl FnOnce(&RasterDisplay<'_>) -> T) -> T {
    let system = VirtualWindowSystem::<RasterFailureContext>::connect_virtual().unwrap();
    let mut viewer: RasterViewer = RasterViewer::with_adapter(Some(Rc::new(system)));
    viewer.make_view(1, "+X+Y", 0, 0, 100, 100).unwrap();
    viewer.set_zoom(1, 10.);
    let display = viewer.make_display(1);
    let result = body(&display);
    viewer.delete_view(1);
    result
}
#[test]
fn pole_target_scales_by_zoom_and_linear_pick_resumes_after_index() {
    with_display(|display| {
        assert_eq!(
            scaled_target(display, (40., 30.), 5.),
            (Point2::new(4., 3.), 0.5)
        );
        let point = |index| Ok::<Point3, Failure>(Point3::new(f64::from(index), 0., 0.));
        assert_eq!(
            find_index(display, (Point2::new(2., 0.), 0.), (0, 2), point).unwrap(),
            2
        );
        assert_eq!(
            find_index(display, (Point2::new(2., 0.), 0.), (2, 2), point).unwrap(),
            0
        );
    });
}
#[test]
fn net_search_preserves_advance_both_indices_before_next_column() {
    with_display(|display| {
        let pole = |u, v| Ok::<Point3, Failure>(Point3::new(f64::from(u), f64::from(v), 0.));
        let target = (Point2::new(1., 2.), 0.);
        assert_eq!(
            find_net_index(display, target, (0, 0), (2, 2), pole).unwrap(),
            (1, 2)
        );
        assert_eq!(
            find_net_index(display, target, (2, 1), (2, 2), pole).unwrap(),
            (0, 0)
        );
    });
}
#[test]
fn pick_callback_failure_retains_typed_context() {
    with_display(|display| {
        let point = |index| {
            Err::<Point3, Failure>(Failure::with_message(
                "PickPointUnavailable",
                &format!("index={index}: expected available pick point"),
            ))
        };
        let error = find_index(display, (Point2::new(0., 0.), 1.), (0, 2), point).unwrap_err();
        assert_eq!(error.exception_type(), "PickPointUnavailable");
        assert_eq!(error.message(), "index=1: expected available pick point");
    });
}
