//! Restriction classifications are reused before ordinary-face point classification.
use martensite_tkbool::{
    top_ope_brep::{TopOpeBRepFacesFiller, TopOpeBRepFacesIntersector, TopOpeBRepLineInter},
    top_ope_brep_ds::TopOpeBRepDSHDataStructure,
};
use martensite_tkbrep::{
    brep_adaptor::{BRepAdaptorCurve, BRepAdaptorCurve2d, BRepAdaptorSurface},
    topexp::Explorer,
    topods::{self, Face},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{
    adaptor3d::Adaptor3dCurve,
    top_abs::{ShapeEnum, State},
};
use martensite_tkgeomalgo::{
    int_patch::{
        IntPatchGLine, IntPatchLineTransitions, IntPatchPoint, IntPatchPointLine, IntPatchRLine,
    },
    int_surf::IntSurfTransition,
};
use martensite_tkmath::gp::{Direction3, Line3, Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

fn face(plane: Plane3) -> Face {
    BRepBuilderAPIMakeFace::from_plane_bounds(plane, (-1., 1., -1., 1.))
        .unwrap()
        .face()
        .unwrap()
}

fn restriction(face: &Face) -> (TopOpeBRepLineInter, IntPatchPoint) {
    let edge = topods::edge(
        Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)
            .unwrap()
            .current(),
    )
    .unwrap();
    let arc = Handle::new(BRepAdaptorCurve2d::with_edge_on_face(&edge, face).unwrap());
    let curve = BRepAdaptorCurve::with_edge(&edge).unwrap();
    let parameter = (curve.first_parameter().unwrap() + curve.last_parameter().unwrap()) * 0.5;
    let mut point = IntPatchPoint::new();
    point.set_value_tangent(curve.value(parameter).unwrap(), 1e-7, false);
    point.set_parameters(0., 0., 100., 100.);
    point.set_arc(
        true,
        &arc.cast(),
        parameter,
        &IntSurfTransition::new(),
        &IntSurfTransition::new(),
    );
    let source = Handle::new(IntPatchRLine::new_undecided(false));
    source.get().unwrap().set_arc_on_s1(&arc.cast());
    source.get().unwrap().add_vertex(&point, false);
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &source.cast(),
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    line.set_index(1);
    (line, point)
}

fn ordinary(point: &IntPatchPoint) -> TopOpeBRepLineInter {
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
    line.set_index(2);
    line
}

#[test]
fn restriction_projection_is_reused_without_reclassifying_the_other_operand() {
    let face = face(Plane3::default());
    let (mut restriction, source) = restriction(&face);
    let mut line = ordinary(&source);
    line.change_vpoint(1)
        .unwrap()
        .set_state(State::In, 2)
        .unwrap();
    let mut pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(face.as_ref(), face.as_ref(), &mut pair, &hds)
        .unwrap();
    filler.vp_position_line(&mut restriction, &[]).unwrap();
    let expected = restriction.vpoint(1).unwrap().edge_on_parameter(1).unwrap();
    assert_eq!(restriction.vpoint(1).unwrap().state(1).unwrap(), State::On);
    filler.vp_position_line(&mut line, &[&restriction]).unwrap();
    assert_eq!(line.vpoint(1).unwrap().state(1).unwrap(), State::On);
    assert_eq!(line.vpoint(1).unwrap().state(2).unwrap(), State::In);
    assert_eq!(
        line.vpoint(1).unwrap().edge_on_parameter(1).unwrap(),
        expected
    );
}

#[test]
fn all_line_classification_uses_one_array_lock_and_preserves_traversal_end() {
    let horizontal = face(Plane3::default());
    let vertical =
        face(Plane3::new(Point3::default(), Direction3::new(1., 0., 0.).unwrap()).unwrap());
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.perform(horizontal.as_ref(), vertical.as_ref())
        .unwrap();
    let count = pair.nb_lines();
    assert!(count > 0);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(horizontal.as_ref(), vertical.as_ref(), &mut pair, &hds)
        .unwrap();
    filler.vp_position_intersections().unwrap();
    let pair = filler.change_faces_intersector().unwrap();
    assert!(!pair.more_line());
    assert_eq!(pair.current_line_index(), count + 1);
    let line = pair.change_line(1).unwrap();
    assert!(line.nb_vpoint() > 0);
    for index in 1..=line.nb_vpoint() {
        assert!(matches!(
            line.vpoint(index).unwrap().state(1).unwrap(),
            State::In | State::On
        ));
        assert!(matches!(
            line.vpoint(index).unwrap().state(2).unwrap(),
            State::In | State::On
        ));
    }
}
