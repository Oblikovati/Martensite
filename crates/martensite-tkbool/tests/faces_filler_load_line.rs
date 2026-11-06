//! Current-line validation retains source count, vertex identity, and conic thresholds.
use martensite_tkbool::{
    top_ope_brep::{TopOpeBRepFacesFiller, TopOpeBRepFacesIntersector, TopOpeBRepLineInter},
    top_ope_brep_ds::TopOpeBRepDSHDataStructure,
};
use martensite_tkbrep::{brep_adaptor::BRepAdaptorSurface, topods::Face};
use martensite_tkernel::standard::Handle;
use martensite_tkgeomalgo::{
    int_patch::{IntPatchGLine, IntPatchLineTransitions, IntPatchPoint},
    int_surf::IntSurfTypeTrans,
};
use martensite_tkmath::gp::{Circle3, Line3, Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

fn face() -> Face {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-2., 2., -2., 2.))
        .unwrap()
        .face()
        .unwrap()
}

fn line(parameters: &[f64], circle: bool) -> TopOpeBRepLineInter {
    let transitions = IntPatchLineTransitions::Types(IntSurfTypeTrans::In, IntSurfTypeTrans::Out);
    let source = if circle {
        IntPatchGLine::from_circle(&Circle3::default(), false, transitions)
    } else {
        IntPatchGLine::from_line(&Line3::default(), false, transitions).unwrap()
    };
    for parameter in parameters {
        let mut point = IntPatchPoint::new();
        point.set_value_tangent(Point3::new(*parameter, 0., 0.), 1e-7, false);
        point.set_parameter(*parameter);
        source.add_vertex(&point).unwrap();
    }
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &Handle::new(source).cast(),
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    for index in 1..=line.nb_vpoint() {
        line.change_vpoint(index).unwrap().change_keep(true);
    }
    line.set_faces(&face(), &face());
    line.set_vp_bounds();
    line.set_inl().unwrap();
    line
}

#[test]
fn loaded_line_computes_both_operand_transitions() {
    use martensite_tkg3d::top_abs::State;
    let first = face();
    let second = face();
    let mut pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(first.as_ref(), second.as_ref(), &mut pair, &hds)
        .unwrap();
    let mut line = line(&[-1., 1.], false);
    filler.load_line(&mut line).unwrap();
    assert!(line.ok());
    let transition = TopOpeBRepFacesFiller::face_face_transition_of(&line, 1).unwrap();
    assert_eq!(
        (transition.before(), transition.after()),
        (State::Out, State::In)
    );
}

#[test]
fn close_circle_parameters_follow_the_first_context_tolerance() {
    let first = face();
    let second = face();
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.force_tolerances(1e-4, 1e-8);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(first.as_ref(), second.as_ref(), &mut pair, &hds)
        .unwrap();
    let mut line = line(&[0., 1e-5], true);
    assert!(!filler.check_line(&mut line).unwrap());
    filler.load_line(&mut line).unwrap();
    assert!(!line.ok());
}

#[test]
fn unrestricted_interior_points_are_ignored_until_known_geometry_exists() {
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&hds).unwrap();
    let mut line = line(&[-1., 0., 1.], false);
    let interior = line.vpoint(2).unwrap().clone();
    filler.process_vpnoton_r(&interior, &mut line).unwrap();
    assert_eq!(hds.get().unwrap().nb_points(), 0);
    let first = line.vpoint(1).unwrap().clone();
    filler.process_vpnoton_r(&first, &mut line).unwrap();
    assert_eq!(hds.get().unwrap().nb_points(), 1);
    assert_eq!(hds.get().unwrap().ds().nb_curves(), 1);
}

#[test]
fn coincident_vertex_rejection_is_overridden_for_identical_line_parameters() {
    use martensite_tkgeomalgo::int_start::TheVertex;
    use martensite_tktopalgo::{
        brep_builder_api::BRepBuilderAPIMakeVertex, brep_top_adaptor::BRepTopAdaptorHVertex,
    };
    let first = face();
    let second = face();
    let mut pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(first.as_ref(), second.as_ref(), &mut pair, &hds)
        .unwrap();
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::default())
        .unwrap()
        .vertex()
        .unwrap();
    let adapted: TheVertex =
        Handle::new(BRepTopAdaptorHVertex::new(&vertex, &Handle::default())).cast();
    let mut line = line(&[-1., 1.], false);
    for index in 1..=2 {
        let mut raw = line
            .vpoint(index)
            .unwrap()
            .point_of_intersection()
            .unwrap()
            .clone();
        raw.set_vertex(true, &adapted);
        line.change_vpoint(index).unwrap().set_owned_point(raw);
    }
    assert!(!filler.check_line(&mut line).unwrap());
    filler.load_line(&mut line).unwrap();
    assert!(!line.ok());
    let mut raw = line
        .vpoint(2)
        .unwrap()
        .point_of_intersection()
        .unwrap()
        .clone();
    raw.set_parameter(-1.);
    line.change_vpoint(2).unwrap().set_owned_point(raw);
    line.set_inl().unwrap();
    filler.load_line(&mut line).unwrap();
    assert!(line.ok());
}

#[test]
fn retained_external_line_preserves_identity_and_native_conic_check_quirk() {
    let first = face();
    let second = face();
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.force_tolerances(1e-4, 1e-8);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut retained = line(&[-1., 1.], true);
    let pointer = &retained as *const _;
    let mut candidate = line(&[0., 1e-5], true);
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(first.as_ref(), second.as_ref(), &mut pair, &hds)
        .unwrap();
    assert!(filler.p_line_inter_dummy().unwrap().is_none());
    filler.bind_line(&mut retained).unwrap();
    {
        let view = filler.p_line_inter_dummy().unwrap().unwrap();
        assert_eq!(&*view as *const _, pointer);
    }
    filler
        .change_line_inter_dummy()
        .unwrap()
        .unwrap()
        .set_index(91);
    assert_eq!(filler.p_line_inter_dummy().unwrap().unwrap().index(), 91);
    assert!(filler.check_line(&mut candidate).unwrap());
    filler.load_line(&mut candidate).unwrap();
    assert!(!candidate.ok());
    assert!(filler.face_face_transition(1).is_ok());
    assert!(filler.face_face_transition(0).is_err());
    assert_eq!(filler.p_data_structure_dummy().unwrap().nb_points(), 0);
}
