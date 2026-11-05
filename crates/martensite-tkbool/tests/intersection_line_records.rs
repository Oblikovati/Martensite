//! Independent geometric and state invariants for intersection-line records.
use martensite_tkbool::top_ope_brep::{
    TopOpeBRepBipoint, TopOpeBRepLineInter, TopOpeBRepTypeLineCurve,
};
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::{Orientation, State};
use martensite_tkgeomalgo::{
    int_patch::*,
    int_surf::{IntSurfLineOn2S, IntSurfPntOn2S, IntSurfSituation, IntSurfTypeTrans},
};
use martensite_tkmath::gp::{
    Axis2, Circle3, Direction3, Ellipse3, Hyperbola3, Line3, Parabola3, Point3,
};

fn point(parameter: f64, value: Point3) -> IntPatchPoint {
    let mut point = IntPatchPoint::new();
    point.set_value_tangent(value, 1e-7, false);
    point.set_parameter(parameter);
    point
}
fn straight(parameters: &[f64]) -> TopOpeBRepLineInter {
    let line = IntPatchGLine::from_line(
        &Line3::new(Point3::default(), Direction3::X),
        false,
        IntPatchLineTransitions::Types(IntSurfTypeTrans::In, IntSurfTypeTrans::Out),
    )
    .unwrap();
    for &parameter in parameters {
        line.add_vertex(&point(parameter, Point3::new(parameter, 0., 0.)))
            .unwrap();
    }
    if !parameters.is_empty() {
        line.set_first_point(1);
        line.set_last_point(parameters.len() as i32);
    }
    load(Handle::new(line).cast())
}
fn load(source: Handle<dyn IntPatchLine>) -> TopOpeBRepLineInter {
    let mut result = TopOpeBRepLineInter::new();
    result
        .set_line(
            &source,
            &BRepAdaptorSurface::default(),
            &BRepAdaptorSurface::default(),
        )
        .unwrap();
    result
}
fn keep_all(line: &mut TopOpeBRepLineInter) {
    for index in 1..=line.nb_vpoint() {
        line.change_vpoint(index).unwrap().change_keep(true);
    }
}
#[test]
fn default_flags_and_missing_source_contracts() {
    let mut line = TopOpeBRepLineInter::new();
    assert!(line.ok());
    assert_eq!(line.type_line_curve(), TopOpeBRepTypeLineCurve::OtherType);
    assert_eq!((line.nb_vpoint(), line.nb_wpoint().unwrap()), (0, 0));
    assert!(!line.inl() && !line.is_v_closed() && !line.has_vp_on_r());
    assert_eq!(line.vp_bounds(), (0, 0, 0));
    assert!(!line.ok());
    line.set_ok(true);
    line.set_index(19);
    line.set_trace_index(-2, 7);
    assert_eq!((line.index(), line.get_trace_index()), (19, (-2, 7)));
    assert!(line.line_g().is_null() && line.line_w().is_null() && line.line_r().is_null());
    assert!(line.has_first_point().is_err() && line.has_last_point().is_err());
    assert!(line.vpoint(-1).is_err() && line.change_vpoint(1).is_err());
    assert!(line.face_face_transition(0).is_err() && line.face_face_transition(3).is_err());
    assert!(line.arc().unwrap().is_null());
    assert!(!line.arc_is_edge(1).unwrap());
}
#[test]
fn parameters_keep_bounds_and_owned_point_records() {
    let mut line = straight(&[2., 4., 8.]);
    assert_eq!(line.type_line_curve(), TopOpeBRepTypeLineCurve::Line);
    assert_eq!(line.nb_vpoint(), 3);
    assert_eq!(line.vpoint(2).unwrap().parameter_on_line().unwrap(), 4.);
    assert_eq!(line.vpoint(3).unwrap().index(), 3);
    assert!(line.vpoint(0).unwrap().value().is_err());
    assert_eq!(line.bounds().unwrap(), (2., 8.));
    assert_eq!(line.period().unwrap(), 6.);
    assert!(line.has_first_point().unwrap() && line.has_last_point().unwrap());
    line.set_vp_bounds();
    assert_eq!(line.vp_bounds(), (4, 0, 0));
    line.set_inl().unwrap();
    assert!(!line.inl());
    line.change_vpoint(2).unwrap().change_keep(true);
    line.set_vp_bounds();
    assert_eq!(line.vp_bounds(), (2, 2, 1));
    line.set_inl().unwrap();
    assert!(line.inl());
    line.set_is_v_closed().unwrap();
    assert!(!line.is_v_closed());
    keep_all(&mut line);
    line.set_inl().unwrap();
    assert!(!line.inl());
    assert!(!line.has_v_internal().unwrap());
    line.set_has_vp_on_r().unwrap();
    assert!(!line.has_vp_on_r());
}
#[test]
fn empty_and_spatially_coincident_endpoint_closure() {
    let mut line = straight(&[]);
    line.set_is_v_closed().unwrap();
    assert!(!line.ok() && !line.is_v_closed());
    let conic = IntPatchGLine::from_circle(
        &Circle3::new(Axis2::default(), 3.).unwrap(),
        false,
        IntPatchLineTransitions::Undecided,
    );
    conic
        .add_vertex(&point(0., Point3::new(3., 0., 0.)))
        .unwrap();
    conic
        .add_vertex(&point(std::f64::consts::TAU, Point3::new(3., 0., 5e-8)))
        .unwrap();
    let mut line = load(Handle::new(conic).cast());
    keep_all(&mut line);
    line.set_inl().unwrap();
    line.set_is_v_closed().unwrap();
    assert!(line.is_v_closed() && line.is_periodic());
    assert_eq!(line.bounds().unwrap(), (0., std::f64::consts::TAU));
    assert!((line.period().unwrap() - std::f64::consts::TAU).abs() < 1e-15);
}
#[test]
fn all_five_conics_create_exact_geometry_and_trims() {
    let frame = Axis2::default();
    let curves = vec![
        IntPatchGLine::from_line(
            &Line3::new(Point3::default(), Direction3::X),
            false,
            IntPatchLineTransitions::Undecided,
        )
        .unwrap(),
        IntPatchGLine::from_circle(
            &Circle3::new(frame, 3.).unwrap(),
            false,
            IntPatchLineTransitions::Undecided,
        ),
        IntPatchGLine::from_ellipse(
            &Ellipse3::new(frame, 4., 2.).unwrap(),
            false,
            IntPatchLineTransitions::Undecided,
        ),
        IntPatchGLine::from_parabola(
            &Parabola3::new(frame, 2.).unwrap(),
            false,
            IntPatchLineTransitions::Undecided,
        ),
        IntPatchGLine::from_hyperbola(
            &Hyperbola3::new(frame, 4., 2.).unwrap(),
            false,
            IntPatchLineTransitions::Undecided,
        ),
    ];
    for source in curves {
        let mut line = load(Handle::new(source).cast());
        assert!(!line.curve().unwrap().is_null());
        let trim = line.trimmed_curve(0.25, 0.75).unwrap();
        assert_eq!(trim.first_parameter(), 0.25);
        assert_eq!(trim.last_parameter(), 0.75);
    }
}
fn walking_line() -> Handle<IntPatchWLine> {
    let points = IntSurfLineOn2S::new(None);
    for parameter in [0., 1., 2.] {
        let mut point = IntSurfPntOn2S::new();
        point.set_value_all(Point3::new(parameter, 0., 0.), parameter, 0., parameter, 0.);
        points.add(&point);
    }
    Handle::new(IntPatchWLine::new(
        &Handle::new(points),
        false,
        IntSurfTypeTrans::In,
        IntSurfTypeTrans::Out,
    ))
}
#[test]
fn walking_and_restriction_sample_access_and_nonconic_failures() {
    let walking = walking_line();
    let mut line = load(walking.cast());
    assert_eq!(line.nb_wpoint().unwrap(), 3);
    assert_eq!(
        line.wpoint(2).unwrap().value().unwrap(),
        Point3::new(1., 0., 0.)
    );
    assert!(line.wpoint(0).is_err());
    assert_eq!(line.bounds().unwrap(), (0., 0.));
    assert!(!line.ok());
    assert!(line.curve().unwrap().is_null());
    let restriction = IntPatchRLine::new_undecided(false);
    restriction.add(&walking.curve());
    let mut restricted = load(Handle::new(restriction).cast());
    assert_eq!(restricted.nb_wpoint().unwrap(), 0);
    assert_eq!(
        restricted.wpoint(3).unwrap().value().unwrap(),
        Point3::new(2., 0., 0.)
    );
    assert!(!restricted.arc_is_edge(1).unwrap());
    assert!(restricted.arc_is_edge(2).unwrap());
}
#[test]
fn oriented_face_transitions_touch_situations_and_release_dumps() {
    let mut line = straight(&[0., 1.]);
    let mut face = martensite_tkbrep::topods::Face::default();
    face.set_orientation(Orientation::Forward);
    line.set_faces(&face, &face);
    line.compute_face_face_transition().unwrap();
    assert_eq!(line.face_face_transition(1).unwrap().before(), State::Out);
    assert_eq!(line.face_face_transition(2).unwrap().before(), State::In);
    assert_eq!(line.transition_on_s1().unwrap(), IntSurfTypeTrans::In);
    assert!(line.situation_s1().is_err());
    let circle = IntPatchGLine::from_circle(
        &Circle3::new(Axis2::default(), 3.).unwrap(),
        false,
        IntPatchLineTransitions::Touch(IntSurfSituation::Inside, IntSurfSituation::Outside),
    );
    let mut touch = load(Handle::new(circle).cast());
    assert_eq!(touch.situation_s1().unwrap(), IntSurfSituation::Inside);
    assert_eq!(touch.situation_s2().unwrap(), IntSurfSituation::Outside);
    touch.set_faces(&face, &face);
    touch.compute_face_face_transition().unwrap();
    assert_eq!(touch.face_face_transition(1).unwrap().before(), State::In);
    assert_eq!(touch.face_face_transition(2).unwrap().before(), State::Out);
    let mut output = b"unchanged".to_vec();
    line.dump_type();
    line.dump_vpoint(-1, "prefix", "suffix");
    line.dump_bipoint(&TopOpeBRepBipoint::new(1, 2), "", "");
    line.dump_line_transitions(&mut output).unwrap();
    assert_eq!(output, b"unchanged");
    face.set_orientation(Orientation::Reversed);
    line.set_faces(&face, &face);
    line.compute_face_face_transition().unwrap();
    assert_eq!(line.face_face_transition(1).unwrap().before(), State::In);
}

#[test]
fn analytic_intersection_is_converted_to_the_first_walking_line() {
    use martensite_tkg3d::adaptor3d::{Adaptor3dSurface, Adaptor3dTopolTool};
    use martensite_tkmath::gp::{Axis3, Cylinder3, Sphere3};
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
    let first = BRepBuilderAPIMakeFace::from_cylinder_bounds(
        Cylinder3::new(Axis3::default(), 1.).unwrap(),
        (0., std::f64::consts::TAU, -2., 2.),
    )
    .unwrap()
    .face()
    .unwrap();
    let frame = Axis3::from_normal(Point3::new(0.8, 0.2, 0.3), Direction3::Z).unwrap();
    let second = BRepBuilderAPIMakeFace::from_sphere(Sphere3::new(frame, 1.).unwrap())
        .unwrap()
        .face()
        .unwrap();
    let surfaces = [
        BRepAdaptorSurface::with_face(&first, true).unwrap(),
        BRepAdaptorSurface::with_face(&second, true).unwrap(),
    ];
    let handles: [Handle<dyn Adaptor3dSurface>; 2] = [
        Handle::new(surfaces[0].clone()).cast(),
        Handle::new(surfaces[1].clone()).cast(),
    ];
    let domains = [
        Adaptor3dTopolTool::with_surface(&handles[0]).unwrap(),
        Adaptor3dTopolTool::with_surface(&handles[1]).unwrap(),
    ];
    let intersection = IntPatchImpImpIntersection::with_surfaces(
        &handles[0],
        &domains[0],
        &handles[1],
        &domains[1],
        1e-7,
        1e-6,
        false,
    )
    .unwrap();
    let mut converted = 0;
    for index in 1..=intersection.nb_lines().unwrap() {
        let source = intersection.line(index).unwrap();
        if source.arc_type() != IntPatchIType::Analytic {
            continue;
        }
        let mut line = TopOpeBRepLineInter::new();
        line.set_line(source, &surfaces[0], &surfaces[1]).unwrap();
        assert_eq!(line.type_line_curve(), TopOpeBRepTypeLineCurve::Walking);
        for index in 1..=line.nb_wpoint().unwrap() {
            let point = line.wpoint(index).unwrap().value().unwrap();
            assert!((martensite_cmath::hypot(point.x(), point.y()) - 1.).abs() < 1e-6);
            assert!((point.distance(&frame.location()) - 1.).abs() < 1e-6);
        }
        converted += 1;
    }
    assert!(converted > 0);
}

#[test]
fn restriction_edges_domain_flags_and_identity_are_preserved() {
    use martensite_tkbrep::{brep_adaptor::BRepAdaptorCurve2d, topexp::Explorer, topods};
    use martensite_tkg3d::top_abs::ShapeEnum;
    use martensite_tkgeomalgo::{int_start::TheArc, int_surf::IntSurfTransition};
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(
        martensite_tkmath::gp::Plane3::default(),
        (-1., 1., -1., 1.),
    )
    .unwrap()
    .face()
    .unwrap();
    let explorer = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let edge = topods::edge(explorer.current()).unwrap();
    let arc: TheArc =
        Handle::new(BRepAdaptorCurve2d::with_edge_on_face(&edge, &face).unwrap()).cast();
    let source = IntPatchRLine::new_undecided(false);
    source.set_arc_on_s1(&arc);
    let mut vertex = point(0., Point3::default());
    vertex.set_arc(
        true,
        &arc,
        0.,
        &IntSurfTransition::new(),
        &IntSurfTransition::new(),
    );
    source.add_vertex(&vertex, false);
    let mut line = load(Handle::new(source).cast());
    assert!(line.arc().unwrap().is_same(&edge));
    assert!(line.arc_is_edge(1).unwrap());
    assert!(!line.arc_is_edge(2).unwrap());
    line.set_has_vp_on_r().unwrap();
    assert!(!line.has_vp_on_r());
    line.change_vpoint(1).unwrap().change_keep(true);
    line.set_has_vp_on_r().unwrap();
    assert!(line.has_vp_on_r());
}

#[test]
fn intersection_line_can_be_transferred_and_read_across_threads() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TopOpeBRepLineInter>();
    let line = straight(&[1., 2.]);
    let index = std::thread::spawn(move || line.vpoint(2).unwrap().index())
        .join()
        .unwrap();
    assert_eq!(index, 2);
}

#[test]
fn closure_uses_first_point_at_equal_extreme_parameter() {
    let source = IntPatchGLine::from_line(
        &Line3::new(Point3::default(), Direction3::X),
        false,
        IntPatchLineTransitions::Undecided,
    )
    .unwrap();
    for (parameter, x) in [(0., 0.), (1., 0.), (1., 10.)] {
        source
            .add_vertex(&point(parameter, Point3::new(x, 0., 0.)))
            .unwrap();
    }
    let mut line = load(Handle::new(source).cast());
    keep_all(&mut line);
    line.set_is_v_closed().unwrap();
    assert!(line.is_v_closed());
    assert_eq!(
        line.vpoint(4).err().unwrap().exception_type(),
        "Standard_OutOfRange"
    );
}
