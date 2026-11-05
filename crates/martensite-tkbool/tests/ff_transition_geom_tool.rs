//! Independent transition truth tables and analytic/sample curve expectations.
use martensite_tkbool::top_ope_brep::{
    TopOpeBRepFFTransitionTool as Tool, TopOpeBRepGeomTool as Geometry, TopOpeBRepLineInter,
    TopOpeBRepVPointInter,
};
use martensite_tkbool::top_ope_brep_ds::TopOpeBRepDSCurve;
use martensite_tkbrep::{brep_adaptor::BRepAdaptorSurface, topods::Shape};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::top_abs::{Orientation as O, State as S};
use martensite_tkgeomalgo::{
    int_patch::{
        IntPatchGLine, IntPatchLine, IntPatchLineTransitions, IntPatchPoint, IntPatchWLine,
    },
    int_surf::{
        IntSurfLineOn2S, IntSurfPntOn2S, IntSurfSituation as Situation, IntSurfTransition,
        IntSurfTypeTrans as T,
    },
};
use martensite_tkmath::gp::{Line3, Point3};

fn sample_point(parameter: f64) -> IntPatchPoint {
    let mut point = IntPatchPoint::new();
    point.set_parameter(parameter);
    point
}
fn loaded_line(transitions: IntPatchLineTransitions) -> TopOpeBRepLineInter {
    let source = IntPatchGLine::from_line(&Line3::default(), false, transitions).unwrap();
    source.add_vertex(&sample_point(-2.)).unwrap();
    source.add_vertex(&sample_point(4.)).unwrap();
    let source: Handle<dyn IntPatchLine> = Handle::new(source).cast();
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &source,
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    line
}
fn expected(orientation: O) -> (S, S) {
    match orientation {
        O::Forward => (S::Out, S::In),
        O::Reversed => (S::In, S::Out),
        O::Internal => (S::In, S::In),
        O::External => (S::Out, S::Out),
    }
}
#[test]
fn fixed_orientations_short_circuit_unbound_point_and_line() {
    let point = TopOpeBRepVPointInter::new();
    let line = TopOpeBRepLineInter::new();
    for orientation in [O::Internal, O::External] {
        for transition in [
            Tool::process_line_transition(&point, 1, orientation),
            Tool::process_edge_transition(&point, 2, orientation),
            Tool::process_face_transition(&line, 1, orientation),
        ] {
            let transition = transition.unwrap();
            assert_eq!(
                (transition.before(), transition.after()),
                expected(orientation)
            );
        }
    }
}
#[test]
fn surface_transition_truth_table_and_reversal() {
    for (source, plain) in [
        (
            IntPatchLineTransitions::Types(T::In, T::Out),
            Some(O::Forward),
        ),
        (
            IntPatchLineTransitions::Touch(Situation::Inside, Situation::Outside),
            Some(O::Internal),
        ),
        (
            IntPatchLineTransitions::Touch(Situation::Unknown, Situation::Unknown),
            None,
        ),
        (IntPatchLineTransitions::Undecided, None),
    ] {
        let line = loaded_line(source);
        for orientation in [O::Forward, O::Reversed] {
            let result = Tool::process_face_transition(&line, 1, orientation).unwrap();
            let expected = plain.map(|value| {
                if orientation == O::Reversed {
                    martensite_tkg3d::top_abs::complement(value)
                } else {
                    value
                }
            });
            assert_eq!(
                (result.before(), result.after()),
                expected
                    .map(crate::expected)
                    .unwrap_or((S::Unknown, S::Unknown))
            );
        }
    }
}
#[test]
fn exact_line_endpoints_take_precedence_over_interior_points() {
    let line = loaded_line(IntPatchLineTransitions::Undecided);
    for (parameter, orientation) in [(-2., O::Forward), (4., O::Reversed), (1., O::Internal)] {
        let mut point = TopOpeBRepVPointInter::new();
        point.set_owned_point(sample_point(parameter));
        let result = Tool::process_line_transition_on_line(&point, &line).unwrap();
        assert_eq!((result.before(), result.after()), expected(orientation));
    }
}
#[test]
fn invalid_point_surface_index_preserves_unknown_transition() {
    let point = TopOpeBRepVPointInter::new();
    for result in [
        Tool::process_line_transition(&point, 0, O::Forward),
        Tool::process_edge_transition(&point, 3, O::Reversed),
    ] {
        let result = result.unwrap();
        assert_eq!((result.before(), result.after()), (S::Unknown, S::Unknown));
    }
}
#[test]
fn analytic_make_curves_keeps_pcurves_and_curve_tolerance() {
    let mut line = loaded_line(IntPatchLineTransitions::Undecided);
    let mut record = TopOpeBRepDSCurve::default();
    record.set_curve(&Handle::default(), 0.001);
    let mut pc1: Handle<dyn Geom2dCurve> = Handle::default();
    let mut pc2 = pc1.clone();
    Geometry::make_curves(
        -2.,
        4.,
        &mut line,
        &Shape::default(),
        &Shape::default(),
        &mut record,
        &mut pc1,
        &mut pc2,
    )
    .unwrap();
    assert!(!record.is_walk());
    assert_eq!(record.tolerance(), 0.001);
    assert!(pc1.is_null() && pc2.is_null());
    let curve = Geometry::make_curve(-2., 4., &mut line).unwrap();
    assert!(
        (curve
            .get()
            .unwrap()
            .value(2.)
            .unwrap()
            .distance(&Point3::default())
            - 2.)
            .abs()
            < 1e-12
    );
}
fn walking_line() -> TopOpeBRepLineInter {
    let points = Handle::new(IntSurfLineOn2S::new(None));
    for i in 0..3 {
        let mut point = IntSurfPntOn2S::new();
        point.set_point(Point3::new(i as f64, 2. * i as f64, 0.));
        point.set_parameters(i as f64, 0., 0., i as f64 + 5.);
        points.get().unwrap().add(&point);
    }
    let source: Handle<dyn IntPatchLine> =
        Handle::new(IntPatchWLine::new(&points, false, T::In, T::Out)).cast();
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &source,
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    line
}
#[test]
fn walking_bspline_uses_integer_knots_in_all_coordinate_spaces() {
    let mut line = walking_line();
    let spatial = Geometry::make_bspline1_from_walking3d(&mut line).unwrap();
    let first = Geometry::make_bspline1_from_walking2d(&mut line, 1).unwrap();
    let second = Geometry::make_bspline1_from_walking2d(&mut line, 2).unwrap();
    for (parameter, x) in [(1., 0.), (1.5, 0.5), (3., 2.)] {
        assert!(
            spatial
                .get()
                .unwrap()
                .value(parameter)
                .unwrap()
                .distance(&Point3::new(x, 2. * x, 0.))
                < 1e-12
        );
        assert_eq!(first.get().unwrap().value(parameter).unwrap().x(), x);
        assert_eq!(second.get().unwrap().value(parameter).unwrap().y(), x + 5.);
    }
}
#[test]
fn unsupported_curve_kind_reports_program_error() {
    let mut line = TopOpeBRepLineInter::new();
    let error = Geometry::make_curve(0., 1., &mut line).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_ProgramError");
}

#[test]
fn point_transition_truth_table_uses_both_surface_slots() {
    use martensite_tkgeomalgo::int_start::TheArc;
    let cases = [
        (IntSurfTransition::with_type(false, T::In), Some(O::Forward)),
        (
            IntSurfTransition::with_type(false, T::Out),
            Some(O::Reversed),
        ),
        (
            IntSurfTransition::touch(true, Situation::Inside, false),
            Some(O::Internal),
        ),
        (
            IntSurfTransition::touch(true, Situation::Outside, true),
            Some(O::External),
        ),
        (
            IntSurfTransition::touch(true, Situation::Unknown, false),
            None,
        ),
        (IntSurfTransition::new(), None),
    ];
    for (source, plain) in cases {
        let mut point = IntPatchPoint::new();
        for first in [true, false] {
            point.set_arc(first, &TheArc::default(), 0., &source, &source);
        }
        let mut view = TopOpeBRepVPointInter::new();
        view.set_point(&point);
        for index in [1, 2] {
            assert_point_transition(&view, index, plain);
        }
    }
}
fn assert_point_transition(point: &TopOpeBRepVPointInter<'_>, index: i32, plain: Option<O>) {
    for orientation in [O::Forward, O::Reversed] {
        let expected = plain.map(|value| {
            if orientation == O::Reversed {
                martensite_tkg3d::top_abs::complement(value)
            } else {
                value
            }
        });
        for result in [
            Tool::process_line_transition(point, index, orientation),
            Tool::process_edge_transition(point, index, orientation),
        ] {
            let result = result.unwrap();
            assert_eq!(
                (result.before(), result.after()),
                expected
                    .map(crate::expected)
                    .unwrap_or((S::Unknown, S::Unknown))
            );
        }
    }
}
#[test]
fn published_stateless_tools_are_send_and_sync() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<Tool>();
    assert_traits::<Geometry>();
}

#[test]
fn edge_on_transition_uses_crossing_tangents_and_only_edge_orientation() {
    use martensite_tkgeomalgo::int_start::TheArc;
    use martensite_tkmath::gp::Plane3;
    use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace};
    let edge =
        BRepBuilderAPIMakeEdge::from_points(Point3::new(-1., 0., 0.), Point3::new(1., 0., 0.))
            .unwrap()
            .edge()
            .unwrap()
            .into_shape();
    let boundary =
        BRepBuilderAPIMakeEdge::from_points(Point3::new(0., -1., 0.), Point3::new(0., 1., 0.))
            .unwrap()
            .edge()
            .unwrap()
            .into_shape();
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-2., 2., -2., 2.))
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
    let mut point = IntPatchPoint::new();
    point.set_arc(
        true,
        &TheArc::default(),
        1.,
        &IntSurfTransition::new(),
        &IntSurfTransition::new(),
    );
    point.set_parameters(0., 0., 0., 0.);
    let mut view = TopOpeBRepVPointInter::new();
    view.set_point(&point);
    let transition = Tool::process_edge_on_transition(&view, 1, &boundary, &edge, &face).unwrap();
    assert_eq!((transition.before(), transition.after()), (S::Out, S::In));
    let transition =
        Tool::process_edge_on_transition(&view, 1, &boundary, &edge.reversed(), &face).unwrap();
    assert_eq!((transition.before(), transition.after()), (S::In, S::Out));
}
#[test]
fn walking_make_curves_sets_range_and_both_output_pcurves() {
    let mut line = walking_line();
    let mut curve = TopOpeBRepDSCurve::default();
    let mut first: Handle<dyn Geom2dCurve> = Handle::default();
    let mut second = first.clone();
    Geometry::make_curves(
        1.,
        3.,
        &mut line,
        &Shape::default(),
        &Shape::default(),
        &mut curve,
        &mut first,
        &mut second,
    )
    .unwrap();
    assert!(curve.is_walk());
    assert_eq!(curve.range(), Some((1., 3.)));
    assert!(!first.is_null() && !second.is_null());
    assert!(
        curve
            .curve()
            .get()
            .unwrap()
            .value(2.)
            .unwrap()
            .distance(&Point3::new(1., 2., 0.))
            < 1e-12
    );
}

#[test]
fn every_conic_kind_preserves_exact_point_and_trimming_contract() {
    use martensite_tkmath::gp::{Axis2, Circle3, Ellipse3, Hyperbola3, Parabola3};
    let transitions = IntPatchLineTransitions::Undecided;
    let cases = [
        (
            IntPatchGLine::from_circle(
                &Circle3::new(Axis2::default(), 3.).unwrap(),
                false,
                transitions,
            ),
            3.,
            false,
        ),
        (
            IntPatchGLine::from_ellipse(
                &Ellipse3::new(Axis2::default(), 4., 2.).unwrap(),
                false,
                transitions,
            ),
            4.,
            false,
        ),
        (
            IntPatchGLine::from_parabola(
                &Parabola3::new(Axis2::default(), 2.).unwrap(),
                false,
                transitions,
            ),
            0.,
            true,
        ),
        (
            IntPatchGLine::from_hyperbola(
                &Hyperbola3::new(Axis2::default(), 5., 2.).unwrap(),
                false,
                transitions,
            ),
            5.,
            true,
        ),
    ];
    for (source, x, trimmed) in cases {
        assert_conic(source, x, trimmed);
    }
}
fn assert_conic(source: IntPatchGLine, x: f64, trimmed: bool) {
    let source: Handle<dyn IntPatchLine> = Handle::new(source).cast();
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &source,
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    let curve = Geometry::make_curve(-1., 1., &mut line).unwrap();
    let curve = curve.get().unwrap();
    assert!(curve.value(0.).unwrap().distance(&Point3::new(x, 0., 0.)) < 1e-12);
    if trimmed {
        assert_eq!((curve.first_parameter(), curve.last_parameter()), (-1., 1.));
    }
}

#[test]
fn line_arc_and_surface_arc_transitions_are_not_interchanged() {
    use martensite_tkgeomalgo::int_start::TheArc;
    let mut point = IntPatchPoint::new();
    point.set_arc(
        true,
        &TheArc::default(),
        0.,
        &IntSurfTransition::with_type(false, T::In),
        &IntSurfTransition::with_type(false, T::Out),
    );
    let mut view = TopOpeBRepVPointInter::new();
    view.set_point(&point);
    let line = Tool::process_line_transition(&view, 1, O::Forward).unwrap();
    let edge = Tool::process_edge_transition(&view, 1, O::Forward).unwrap();
    assert_eq!((line.before(), line.after()), (S::Out, S::In));
    assert_eq!((edge.before(), edge.after()), (S::In, S::Out));
}
#[test]
fn edge_on_transition_without_curve_returns_unknown_states() {
    use martensite_tkbrep::{brep::BRepBuilder, topods::Edge};
    use martensite_tkgeomalgo::int_start::TheArc;
    use martensite_tkmath::gp::Plane3;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
    let mut edge = Edge::default();
    BRepBuilder::default().make_edge(&mut edge).unwrap();
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1., 1., -1., 1.))
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
    let mut point = IntPatchPoint::new();
    point.set_arc(
        true,
        &TheArc::default(),
        0.,
        &IntSurfTransition::new(),
        &IntSurfTransition::new(),
    );
    let mut view = TopOpeBRepVPointInter::new();
    view.set_point(&point);
    let result = Tool::process_edge_on_transition(
        &view,
        1,
        &edge.clone().into_shape(),
        &edge.into_shape(),
        &face,
    )
    .unwrap();
    assert_eq!((result.before(), result.after()), (S::Unknown, S::Unknown));
}
