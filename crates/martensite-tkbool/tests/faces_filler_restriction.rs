//! Restriction equality separates spatial and parameter tolerance rules.
use martensite_tkbool::top_ope_brep::{
    TopOpeBRepFacesFiller as Filler, TopOpeBRepLineInter, TopOpeBRepVPointInter,
};
use martensite_tkbrep::{
    brep_adaptor::{BRepAdaptorCurve, BRepAdaptorCurve2d, BRepAdaptorSurface},
    topexp::Explorer,
    topods,
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::{adaptor3d::Adaptor3dCurve, top_abs::ShapeEnum};
use martensite_tkgeomalgo::{
    int_patch::{IntPatchPoint, IntPatchRLine},
    int_surf::IntSurfTransition,
};
use martensite_tkmath::gp::{Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

fn restriction() -> (
    TopOpeBRepLineInter,
    BRepAdaptorCurve,
    Handle<BRepAdaptorCurve2d>,
) {
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1.0, 1.0, -1.0, 1.0))
        .unwrap()
        .face()
        .unwrap();
    let edge = topods::edge(
        Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)
            .unwrap()
            .current(),
    )
    .unwrap();
    let adaptor = Handle::new(BRepAdaptorCurve2d::with_edge_on_face(&edge, &face).unwrap());
    let source = Handle::new(IntPatchRLine::new_undecided(false));
    source.get().unwrap().set_arc_on_s1(&adaptor.cast());
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &source.cast(),
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    (line, BRepAdaptorCurve::with_edge(&edge).unwrap(), adaptor)
}

fn source(position: Point3) -> IntPatchPoint {
    let mut point = IntPatchPoint::new();
    point.set_value_tangent(position, 0.125, false);
    point
}

#[test]
fn explicit_arc_parameter_precedes_geometric_projection() {
    let (line, _, arc) = restriction();
    let mut original = source(Point3::new(100.0, 100.0, 100.0));
    original.set_arc(
        true,
        &arc.cast(),
        0.375,
        &IntSurfTransition::new(),
        &IntSurfTransition::new(),
    );
    let mut point = TopOpeBRepVPointInter::new();
    point.set_point(&original);
    assert_eq!(Filler::vp_param_on_er(&point, &line).unwrap(), 0.375);
}

#[test]
fn points_without_domain_incidence_are_projected_on_the_restriction() {
    let (line, curve, _) = restriction();
    let parameter = (curve.first_parameter().unwrap() + curve.last_parameter().unwrap()) * 0.5;
    let original = source(curve.value(parameter).unwrap());
    let mut point = TopOpeBRepVPointInter::new();
    point.set_point(&original);
    assert!(
        (Filler::vp_param_on_er(&point, &line).unwrap() - parameter).abs()
            <= precision::p_confusion()
    );
}

#[test]
fn equal_spatial_points_need_strict_parameter_equality_too() {
    let (line, _, arc) = restriction();
    let transition = IntSurfTransition::new();
    let mut first = source(Point3::default());
    let mut second = source(Point3::default());
    first.set_arc(true, &arc.cast(), 0.0, &transition, &transition);
    second.set_arc(
        true,
        &arc.cast(),
        precision::p_confusion(),
        &transition,
        &transition,
    );
    let mut a = TopOpeBRepVPointInter::new();
    let mut b = TopOpeBRepVPointInter::new();
    a.set_point(&first);
    b.set_point(&second);
    assert!(!Filler::equalp_ponr(&line, &a, &b).unwrap());
    assert!(Filler::equalp_ponr(&line, &a, &a).unwrap());
}

#[test]
fn vertex_incidence_precedes_an_explicit_arc_parameter() {
    use martensite_tkbrep::brep::BRepTool;
    use martensite_tkgeomalgo::int_start::TheVertex;
    use martensite_tktopalgo::brep_top_adaptor::BRepTopAdaptorHVertex;
    let (line, _, arc) = restriction();
    let edge = arc.get().unwrap().edge();
    let vertex = topods::vertex(
        Explorer::new(&edge, ShapeEnum::Vertex, ShapeEnum::Shape)
            .unwrap()
            .current(),
    )
    .unwrap();
    let expected = BRepTool::parameter(&vertex, &edge).unwrap();
    let adapted: TheVertex = Handle::new(BRepTopAdaptorHVertex::new(&vertex, &arc)).cast();
    let mut original = source(Point3::default());
    original.set_arc(
        true,
        &arc.cast(),
        expected + 0.5,
        &IntSurfTransition::new(),
        &IntSurfTransition::new(),
    );
    original.set_vertex(true, &adapted);
    let mut point = TopOpeBRepVPointInter::new();
    point.set_point(&original);
    assert_eq!(Filler::vp_param_on_er(&point, &line).unwrap(), expected);
}

fn point_line(point: &IntPatchPoint) -> TopOpeBRepLineInter {
    use martensite_tkgeomalgo::int_patch::{IntPatchGLine, IntPatchLineTransitions};
    use martensite_tkmath::gp::Line3;
    let source =
        IntPatchGLine::from_line(&Line3::default(), false, IntPatchLineTransitions::Undecided)
            .unwrap();
    source.add_vertex(point).unwrap();
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &Handle::new(source).cast(),
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    line
}

#[test]
fn off_domain_transition_keeps_existing_output_unchanged() {
    use martensite_tkbool::top_ope_brep_ds::TopOpeBRepDSTransition;
    use martensite_tkg3d::top_abs::Orientation;
    let line = point_line(&source(Point3::default()));
    let sentinel = TopOpeBRepDSTransition::from_orientation(Orientation::Internal);
    let mut transition = sentinel;
    assert!(!Filler::is_vp_trans_lok(&line, 1, 1, &mut transition).unwrap());
    assert_eq!(transition.before(), sentinel.before());
    assert_eq!(transition.after(), sentinel.after());
    assert!(!Filler::transvp_ok(&line, 1, 1, true).unwrap());
}

#[test]
fn on_domain_crossings_follow_the_boundary_orientation() {
    use martensite_tkbool::top_ope_brep_ds::TopOpeBRepDSTransition;
    use martensite_tkgeomalgo::int_surf::IntSurfTypeTrans;
    let (_, _, arc) = restriction();
    let mut source = source(Point3::default());
    let mut crossing = IntSurfTransition::new();
    crossing.set_type(false, IntSurfTypeTrans::In);
    source.set_arc(true, &arc.cast(), 0.0, &crossing, &crossing);
    let line = point_line(&source);
    let mut transition = TopOpeBRepDSTransition::default();
    assert!(Filler::is_vp_trans_lok(&line, 1, 1, &mut transition).unwrap());
    let inward = Filler::transvp_ok(&line, 1, 1, true).unwrap();
    let outward = Filler::transvp_ok(&line, 1, 1, false).unwrap();
    assert_ne!(inward, outward);
}
