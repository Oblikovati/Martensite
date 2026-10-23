use super::*;
use crate::int_start::TheArc;
use crate::int_surf::{IntSurfTransition, IntSurfTypeTrans};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkmath::gp::{Direction2, Point2, Point3};

fn boundary_arc() -> TheArc {
    let geometry: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::from_point(
        Point2::new(0.0, 0.0),
        Direction2::new(1.0, 0.0).unwrap(),
    ))
    .cast();
    Handle::new(Geom2dAdaptorCurve::with_range(&geometry, 0.0, 1.0).unwrap()).cast()
}

fn incidence(x: f64, parameter: f64, arc: &TheArc) -> IntPatchPoint {
    let mut point = IntPatchPoint::new();
    point.set_value_tangent(Point3::new(x, 0.0, 0.0), 1.0e-7, false);
    point.set_parameter(parameter);
    let transition = IntSurfTransition::with_type(false, IntSurfTypeTrans::In);
    point.set_arc(true, arc, x, &transition, &transition);
    point
}

#[test]
fn restriction_split_retains_distinct_endpoint_boundary_incidences() {
    let arcs = [boundary_arc(), boundary_arc()];
    let source = Handle::new(IntPatchRLine::new_undecided(false));
    for (x, parameter, owner) in [(0., 0., 0), (0., 0., 1), (1., 1., 0), (1., 1., 1)] {
        source.add_vertex(&incidence(x, parameter, &arcs[owner]), false);
    }
    let mut membership = [0, 7, 0, 7, 0, 0];
    let mut segments = Vec::new();
    add_line(&source.cast(), (1, 3), &mut membership, &mut segments).unwrap();
    let result = segments[0].cast::<IntPatchRLine>();
    assert_eq!(result.nb_vertex(), 4);
    for rank in 1..=4 {
        let point = result.vertex(rank).unwrap();
        assert_eq!(point.parameter_on_line(), if rank <= 2 { 0. } else { 1. });
        assert!(arcs.contains(&point.arc_on_s1().unwrap()));
    }
    for endpoint in [1, 3] {
        let first = result.vertex(endpoint).unwrap().arc_on_s1().unwrap();
        let second = result.vertex(endpoint + 1).unwrap().arc_on_s1().unwrap();
        assert!(
            first != second,
            "each endpoint must retain both boundary owners"
        );
    }
    assert_eq!(membership[1..5], [7, 7, 7, 7]);
    assert_eq!(source.nb_vertex(), 4);
}

#[test]
fn coincidence_uses_strict_larger_spatial_tolerance_and_rebinds_parameter() {
    let arc = boundary_arc();
    let source = IntPatchRLine::new_undecided(false);
    for (x, parameter, tolerance) in [(0., 2., 1.), (1., 8., 1.), (0.5, 9., 0.01)] {
        let mut point = incidence(x, parameter, &arc);
        point.set_tolerance(tolerance);
        source.add_vertex(&point, false);
    }
    let result = IntPatchRLine::new_undecided(false);
    let mut membership = [0, 6, 0, 0];
    assert_eq!(
        append_same_vertex_r(&result, &source, 1, &mut membership).unwrap(),
        1
    );
    let retained = result.vertex(1).unwrap();
    assert_eq!(retained.value().x(), 0.5);
    assert_eq!(retained.parameter_on_line(), 2.);
    assert_eq!(retained.parameter_on_arc1().unwrap(), 0.5);
    assert_eq!(source.vertex(3).unwrap().parameter_on_line(), 9.);
    assert_eq!(membership, [0, 6, 0, 6]);
}
