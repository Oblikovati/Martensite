use super::*;
use martensite_raster_core::{PaletteColor, PaletteKind, drawable::RasterDrawable};
use martensite_tkg2d::geom2d::Geom2dCircle;
use martensite_tkgeomalgo::geom2d_gcc::unqualified;
use martensite_tkmath::gp::{Axis2d, Direction2, Point2};

fn independent_unit_circle(x: f64) -> QualifiedCurve {
    let axis = Axis2d::new(Point2::new(x, 0.), Direction2::new(1., 0.).unwrap());
    let curve: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dCircle::from_axis(axis, 1., true).unwrap()).cast();
    unqualified(&Geom2dAdaptorCurve::with_curve(&curve).unwrap()).unwrap()
}

#[test]
fn actual_analytic_tangents_match_independent_two_unit_circle_geometry() {
    let solver = Lin2d2Tan::with_parameters(
        &independent_unit_circle(0.),
        &independent_unit_circle(5.),
        1e-9,
        0.,
        0.,
    )
    .unwrap();
    assert!(solver.is_done());
    assert_eq!(solver.nb_solutions(), 4);
    assert!(solver.this_solution(0).is_err());
    for index in 1..=solver.nb_solutions() {
        let line = solver.this_solution(index).unwrap();
        assert!((line.distance(Point2::default()) - 1.).abs() <= 1e-12);
        assert!((line.distance(Point2::new(5., 0.)) - 1.).abs() <= 1e-12);
    }
}

#[test]
fn authored_solution_range_keeps_zero_based_iteration_and_empty_branch() {
    assert_eq!(authored_solution_indices(2).collect::<Vec<_>>(), [0, 1]);
    assert!(authored_solution_indices(0).next().is_none());
}

#[test]
fn actual_hidden_command_retains_both_named_curves_and_partial_context_on_catch() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let mut bindings = Vec::new();
    let _catch_result = execute_case_command(&session, &mut bindings);
    require_source_curve_drawings(&bindings);
    let mut displayed = martensite_tkernel::ncollection::List::default();
    session.context().displayed_objects(&mut displayed).unwrap();
    assert_eq!(displayed.extent(), 2);
}

fn require_source_curve_drawings(bindings: &drawings::NamedPlanarCurves) {
    assert_eq!(
        bindings
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>(),
        ["ell", "cil"]
    );
    for (_, drawing) in bindings {
        assert!(drawing.visible());
        assert!(!drawing.is_3d());
        assert!(drawing.style().display_origin());
        assert_default_curve_controls(drawing);
    }
}

fn assert_default_curve_controls(drawing: &martensite_geometry_raster::PlanarCurveDrawable) {
    assert_eq!(drawing.sampling().discretisation(), 30);
    assert_eq!(drawing.sampling().deflection(), 0.01);
    assert_eq!(drawing.sampling().draw_mode(), 0);
    assert_eq!(
        drawing.style().color(),
        PaletteColor::new(PaletteKind::Jaune)
    );
    assert!(!drawing.style().curvature_shown());
    assert_eq!(drawing.style().radius_max(), 1000.);
    assert_eq!(drawing.style().radius_ratio(), 0.1);
}

fn independent_bound_pair() -> (Handle<Geom2dCircle>, drawings::NamedPlanarCurves) {
    let axis = Axis2d::new(Point2::default(), Direction2::new(1., 0.).unwrap());
    let circle = Handle::new(Geom2dCircle::from_axis(axis, 1., true).unwrap());
    let mut bindings = Vec::new();
    let source = circle.cast::<dyn Geom2dCurve>();
    drawings::bind_planar_curve(&mut bindings, "first", &source);
    drawings::bind_planar_curve(&mut bindings, "second", &source);
    (circle, bindings)
}

#[test]
fn named_curves_retain_actual_geometry_handle_and_rc_identity() {
    let (circle, bindings) = independent_bound_pair();
    let retained = std::rc::Rc::clone(&bindings[0].1);
    assert!(std::rc::Rc::ptr_eq(&retained, &bindings[0].1));
    assert_eq!(retained.curve(), circle);
    assert_eq!(bindings[1].1.curve(), circle);
    circle.get().unwrap().set_radius(2.).unwrap();
    assert_eq!(
        retained.curve().get().unwrap().value(0.).unwrap(),
        Point2::new(2., 0.)
    );
    assert_eq!(
        bindings[1].1.curve().get().unwrap().value(0.).unwrap(),
        Point2::new(2., 0.)
    );
}

#[test]
fn named_curve_controls_and_screen_state_are_per_binding() {
    let (_, bindings) = independent_bound_pair();
    let first = &bindings[0].1;
    first.sampling().set_discretisation(7);
    first.sampling().set_deflection(0.125);
    first.sampling().set_draw_mode(1);
    first
        .style()
        .set_color(PaletteColor::new(PaletteKind::Rouge));
    first.set_visible(false);
    first.set_bounds([1., 2., 3., 4.]);
    assert_eq!(first.sampling().discretisation(), 7);
    assert!(!first.visible());
    assert_eq!(first.bounds(), [1., 2., 3., 4.]);
    assert_default_curve_controls(&bindings[1].1);
    assert!(bindings[1].1.visible());
    assert_eq!(bindings[1].1.bounds(), [0.; 4]);
}
