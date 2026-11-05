//! Face pair classification retains source operand masks and external context identity.
use martensite_tkbool::{
    top_ope_brep::{
        TopOpeBRepFacesFiller, TopOpeBRepFacesIntersector, TopOpeBRepVPointInter,
        TopOpeBRepVPointInterClassifier,
    },
    top_ope_brep_ds::TopOpeBRepDSHDataStructure,
};
use martensite_tkbrep::{
    brep_adaptor::BRepAdaptorCurve2d,
    topexp::Explorer,
    topods::{self, Face},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::{ShapeEnum, State};
use martensite_tkgeomalgo::{int_patch::IntPatchPoint, int_surf::IntSurfTransition};
use martensite_tkmath::gp::{Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

fn face() -> Face {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1.0, 1.0, -1.0, 1.0))
        .unwrap()
        .face()
        .unwrap()
}

fn source() -> IntPatchPoint {
    let mut point = IntPatchPoint::new();
    point.set_value_tangent(Point3::default(), 1.0e-7, false);
    point.set_parameters(0.0, 0.0, 2.0, 0.0);
    point
}

#[test]
fn unbound_face_access_and_invalid_indices_keep_native_contracts() {
    let mut filler = TopOpeBRepFacesFiller::new();
    assert!(filler.face(1).unwrap().is_null());
    assert!(filler.face(2).unwrap().is_null());
    assert!(filler.face(0).is_err());
    assert!(filler.p_faces_intersector_dummy().is_none());
    assert!(filler.change_faces_intersector().is_err());
    assert!(
        filler
            .vp_position(
                &mut TopOpeBRepVPointInter::new(),
                &mut TopOpeBRepVPointInterClassifier::new()
            )
            .is_err()
    );
}

#[test]
fn mask_zero_classifies_both_sides_and_context_mutation_keeps_identity() {
    let face = face();
    let mut pair = TopOpeBRepFacesIntersector::new();
    let expected = &pair as *const _;
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(
            face.as_ref(),
            face.as_ref(),
            &mut pair,
            &Handle::new(TopOpeBRepDSHDataStructure::new()),
        )
        .unwrap();
    assert_eq!(
        filler.p_faces_intersector_dummy().unwrap() as *const _,
        expected
    );
    filler
        .change_faces_intersector()
        .unwrap()
        .force_tolerances(1.0e-6, 2.0e-6);
    assert_eq!(
        filler.p_faces_intersector_dummy().unwrap().get_tolerances(),
        (1.0e-6, 2.0e-6)
    );
    let source = source();
    let mut point = TopOpeBRepVPointInter::new();
    point.set_point(&source);
    filler
        .vp_position(&mut point, &mut TopOpeBRepVPointInterClassifier::new())
        .unwrap();
    assert_eq!(point.state(1).unwrap(), State::In);
    assert_eq!(point.state(2).unwrap(), State::Out);
}

#[test]
fn first_domain_mask_skips_its_own_operand_classification() {
    let face = face();
    let edge = topods::edge(
        Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)
            .unwrap()
            .current(),
    )
    .unwrap();
    let curve = Handle::new(BRepAdaptorCurve2d::with_edge_on_face(&edge, &face).unwrap());
    let mut source = source();
    source.set_arc(
        true,
        &curve.cast(),
        0.25,
        &IntSurfTransition::new(),
        &IntSurfTransition::new(),
    );
    let mut point = TopOpeBRepVPointInter::new();
    point.set_point(&source);
    point.set_state(State::In, 1).unwrap();
    let mut pair = TopOpeBRepFacesIntersector::new();
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(
            face.as_ref(),
            face.as_ref(),
            &mut pair,
            &Handle::new(TopOpeBRepDSHDataStructure::new()),
        )
        .unwrap();
    filler
        .vp_position(&mut point, &mut TopOpeBRepVPointInterClassifier::new())
        .unwrap();
    assert_eq!(point.state(1).unwrap(), State::In);
    assert_eq!(point.state(2).unwrap(), State::Out);
}

#[test]
fn failed_context_binding_preserves_native_sequential_assignment_order() {
    let previous = face();
    let replacement = face();
    let edge = Explorer::new(&replacement, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .current()
        .clone();
    let mut initial_pair = TopOpeBRepFacesIntersector::new();
    let mut invalid_pair = TopOpeBRepFacesIntersector::new();
    let mut null_pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(
            previous.as_ref(),
            previous.as_ref(),
            &mut initial_pair,
            &hds,
        )
        .unwrap();
    assert!(
        filler
            .bind_context(replacement.as_ref(), &edge, &mut invalid_pair, &hds)
            .is_err()
    );
    assert!(filler.face(1).unwrap().is_same(replacement.as_ref()));
    assert!(filler.face(2).unwrap().is_same(previous.as_ref()));
    assert!(
        filler
            .bind_context(
                replacement.as_ref(),
                replacement.as_ref(),
                &mut null_pair,
                &Handle::default()
            )
            .is_err()
    );
    assert!(filler.face(2).unwrap().is_same(replacement.as_ref()));
    assert!(filler.hdata_structure().is_null());
}
