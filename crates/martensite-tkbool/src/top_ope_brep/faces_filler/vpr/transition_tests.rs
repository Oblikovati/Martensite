//! Closed-form normal crossings and explicit incomplete/no-incidence paths.
use super::{GeometrySearch, TopOpeBRepVPointInter, process_tests::fixture};
use crate::top_ope_brep_ds::{
    TopOpeBRepDSKind as Kind, TopOpeBRepDSTransition as Transition,
    process_interferences_tool::make_epv_interference_sk,
};
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkg3d::{
    adaptor3d::Adaptor3dCurve,
    top_abs::{Orientation, State},
};
use martensite_tkmath::gp::{Direction3, Plane3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

#[test]
fn no_incidence_leaves_new_transition_uncomputed() {
    let (filler, mut line, edge) = fixture(false, true);
    let mut point = TopOpeBRepVPointInter::new();
    point.set_owned_point(martensite_tkgeomalgo::int_patch::IntPatchPoint::new());
    let result = super::new_transition_edge(
        &filler,
        &mut line,
        &super::EdgeTransitionQuery {
            line_on_restriction: false,
            point: &point,
            kind: Kind::Point,
            index: 0,
            other_side: 2,
            edge: &edge,
            restrictions: &Default::default(),
        },
    )
    .unwrap();
    assert!(result.is_none());
}

#[test]
fn normal_plane_crossing_has_analytic_enter_leave_states() {
    let (mut filler, mut line, edge) = fixture(false, true);
    let point = line.vpoint(1).unwrap().clone();
    let curve = BRepAdaptorCurve::with_edge(&edge).unwrap();
    let (location, tangent) = curve.d1(point.edge_parameter(1).unwrap()).unwrap();
    let normal = Direction3::from_xyz(*tangent.xyz()).unwrap();
    filler.faces[1] = BRepBuilderAPIMakeFace::from_plane(Plane3::new(location, normal).unwrap())
        .unwrap()
        .face()
        .unwrap();
    let result = super::new_transition_edge(
        &filler,
        &mut line,
        &super::EdgeTransitionQuery {
            line_on_restriction: false,
            point: &point,
            kind: Kind::Point,
            index: 0,
            other_side: 2,
            edge: &edge,
            restrictions: &Default::default(),
        },
    )
    .unwrap()
    .unwrap();
    assert_eq!((result.before(), result.after()), (State::In, State::Out));
}

#[test]
fn unknown_crossing_on_an_edge_does_not_create_geometry_or_interferences() {
    let (mut filler, mut line, edge) = fixture(false, false);
    filler.line_is_on_edge = true;
    filler
        .process_vpion_r(
            &mut crate::top_ope_brep::TopOpeBRepVPointInterIterator::with_line(&mut line).unwrap(),
            &Transition::default(),
            &filler.faces[0].clone(),
            1,
        )
        .unwrap();
    let storage = filler.hds.get().unwrap();
    let ds = storage.ds();
    assert_eq!(ds.nb_points(), 0);
    assert_eq!(ds.shape_index(&edge, true), 0);
    assert!(filler.curve_interferences.is_empty());
}

#[test]
fn existing_same_geometry_face_transition_wins_unknown_edge_recomputation() {
    let (mut filler, mut line, edge) = fixture(false, true);
    filler.line_is_on_edge = true;
    let mut point = line.vpoint(1).unwrap().clone();
    let source = point.point_of_intersection().unwrap().clone();
    let curve = martensite_tkbrep::brep_adaptor::BRepAdaptorCurve2d::with_edge_on_face(
        &edge,
        &filler.faces[0],
    )
    .unwrap();
    let mut source = source;
    let arc = martensite_tkernel::standard::Handle::new(curve).cast();
    source.set_arc(
        true,
        &arc,
        point.edge_parameter(1).unwrap(),
        &super::process_tests::transition(martensite_tkgeomalgo::int_surf::IntSurfTypeTrans::In),
        &super::process_tests::transition(
            martensite_tkgeomalgo::int_surf::IntSurfTypeTrans::Undecided,
        ),
    );
    point.set_owned_point(source);
    let context = super::context::Context::new(&filler, &point, 1).unwrap();
    filler.change_data_structure().unwrap().add_shape(&edge);
    let mut search = GeometrySearch::default();
    filler
        .find_vp_geometry(&line, &point, 1, 1, &mut search)
        .unwrap();
    let mut expected = Transition::from_orientation(Orientation::Reversed);
    expected.set_index(context.other_face_index);
    let interference = make_epv_interference_sk(
        &expected,
        context.other_face_index,
        search.index,
        point.edge_parameter(1).unwrap(),
        Kind::Point,
        Kind::Face,
        false,
    )
    .unwrap();
    filler
        .hds
        .get()
        .unwrap()
        .store_interference(&interference, &edge)
        .unwrap();
    filler
        .process_vpon_r(
            &point,
            &Transition::from_orientation(Orientation::Forward),
            &filler.faces[0].clone(),
            1,
            &mut line,
        )
        .unwrap();
    let storage = filler.hds.get().unwrap();
    let ds = storage.ds();
    let final_entry = ds
        .shape_interferences(&edge, true)
        .iter()
        .last()
        .unwrap()
        .get()
        .unwrap();
    assert_eq!(
        (
            final_entry.transition().before(),
            final_entry.transition().after()
        ),
        (State::In, State::Out)
    );
}

#[test]
fn iterator_wrappers_retain_current_identity_and_report_exhaustion() {
    let (mut filler, mut line, _) = fixture(false, true);
    let mut iterator =
        crate::top_ope_brep::TopOpeBRepVPointInterIterator::with_line(&mut line).unwrap();
    filler.process_vpinoton_r(&mut iterator).unwrap();
    assert_eq!(iterator.current_vp_index().unwrap(), 1);
    assert_eq!(filler.curve_interferences.len(), 1);
    iterator.next().unwrap();
    filler.process_vpinoton_r(&mut iterator).unwrap();
    assert_eq!(iterator.current_vp_index().unwrap(), 2);
    assert_eq!(filler.curve_interferences.len(), 1);
    iterator.next().unwrap();
    filler.process_vpinoton_r(&mut iterator).unwrap();
    assert_eq!(filler.curve_interferences.len(), 2);
    iterator.next().unwrap();
    let error = filler.process_vpinoton_r(&mut iterator).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_ProgramError");
    assert_eq!(filler.curve_interferences.len(), 2);
}
