//! Closing incidences preserve independent face and edge supports.
use martensite_tkbool::{
    top_ope_brep::{
        TopOpeBRepFacesFiller as Filler, TopOpeBRepFacesIntersector, TopOpeBRepVPointInter,
    },
    top_ope_brep_ds::{
        TopOpeBRepDSHDataStructure, TopOpeBRepDSKind as Kind, TopOpeBRepDSTransition,
    },
};
use martensite_tkbrep::{
    brep_adaptor::{BRepAdaptorCurve, BRepAdaptorCurve2d},
    topexp::Explorer,
    topods::{self, Edge, Face},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{
    adaptor3d::Adaptor3dCurve,
    top_abs::{Orientation, ShapeEnum, State},
};
use martensite_tkgeomalgo::{int_patch::IntPatchPoint, int_surf::IntSurfTransition};
use martensite_tkmath::gp::Plane3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

fn face() -> Face {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1., 1., -1., 1.))
        .unwrap()
        .face()
        .unwrap()
}
fn arc(face: &Face) -> (Edge, Handle<BRepAdaptorCurve2d>) {
    let edge = topods::edge(
        Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)
            .unwrap()
            .current(),
    )
    .unwrap();
    let arc = Handle::new(BRepAdaptorCurve2d::with_edge_on_face(&edge, face).unwrap());
    (edge, arc)
}
fn point(first: &Face, second: &Face, both: bool) -> (TopOpeBRepVPointInter<'static>, Edge, Edge) {
    let (edge, a) = arc(first);
    let (other, b) = arc(second);
    let curve = BRepAdaptorCurve::with_edge(&edge).unwrap();
    let parameter = (curve.first_parameter().unwrap() + curve.last_parameter().unwrap()) * 0.5;
    let mut original = IntPatchPoint::new();
    original.set_value_tangent(curve.value(parameter).unwrap(), 1.0e-7, false);
    let transition = IntSurfTransition::new();
    original.set_arc(true, &a.cast(), parameter, &transition, &transition);
    if both {
        original.set_arc(false, &b.cast(), parameter, &transition, &transition);
    }
    let mut point = TopOpeBRepVPointInter::new();
    point.set_owned_point(original);
    point.set_state(State::In, 2).unwrap();
    (point, edge, other)
}
fn store(both: bool) {
    let first = face();
    let second = face();
    let (point, edge, opposite) = point(&first, &second, both);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut pair = TopOpeBRepFacesIntersector::new();
    let mut filler = Filler::new();
    filler
        .bind_context(first.as_ref(), second.as_ref(), &mut pair, &hds)
        .unwrap();
    let (kind, index) = filler.make_geometry(&point, 0).unwrap();
    let transition = TopOpeBRepDSTransition::from_orientation(Orientation::Forward);
    filler
        .process_vponclosing_r(
            &point,
            first.as_ref(),
            1,
            &transition,
            kind,
            index,
            false,
            &Handle::default(),
        )
        .unwrap();
    let storage = hds.get().unwrap();
    let ds = storage.ds();
    let interferences = ds.shape_interferences(edge.as_ref(), true);
    assert_eq!(interferences.len(), if both { 2 } else { 1 });
    let face_index = ds.shape_index(second.as_ref(), true);
    for (position, interference) in interferences.iter().enumerate() {
        let interference = interference.get().unwrap();
        assert_eq!(interference.geometry_type(), Kind::Point);
        assert_eq!(interference.geometry(), index);
        assert_eq!(interference.transition().index().unwrap(), face_index);
        assert_eq!(interference.transition().before(), State::Out);
        assert_eq!(interference.transition().after(), State::In);
        assert_eq!(
            interference.support_type(),
            if position == 0 {
                Kind::Face
            } else {
                Kind::Edge
            }
        );
        assert_eq!(
            interference.support(),
            if position == 0 {
                face_index
            } else {
                ds.shape_index(opposite.as_ref(), true)
            }
        );
    }
}
#[test]
fn supplied_transition_creates_face_support() {
    store(false);
}
#[test]
fn two_domain_arcs_create_face_and_opposite_edge_support() {
    store(true);
}
#[test]
fn transition_rejects_points_outside_requested_domain() {
    let point = TopOpeBRepVPointInter::new();
    let filler = Filler::new();
    assert!(
        filler
            .get_edge_trans(&point, Kind::Point, 1, 1, &face())
            .is_err()
    );
}

#[test]
fn closing_transition_crossing_a_plane_matches_its_signed_half_spaces() {
    use martensite_tkmath::gp::{Direction3, Point3};
    let owner = BRepBuilderAPIMakeFace::from_plane_bounds(
        Plane3::new(Point3::default(), Direction3::Y).unwrap(),
        (-1., 1., -1., 1.),
    )
    .unwrap()
    .face()
    .unwrap();
    let opposite = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-2., 2., -2., 2.))
        .unwrap()
        .face()
        .unwrap();
    let edge = Explorer::new(&owner, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .find_map(|shape| {
            let edge = topods::edge(&shape).unwrap();
            let curve = BRepAdaptorCurve::with_edge(&edge).unwrap();
            let first = curve.value(curve.first_parameter().unwrap()).unwrap();
            let last = curve.value(curve.last_parameter().unwrap()).unwrap();
            ((first.z() - last.z()).abs() > 1.).then_some(edge)
        })
        .unwrap();
    let curve = BRepAdaptorCurve::with_edge(&edge).unwrap();
    let parameter = (curve.first_parameter().unwrap() + curve.last_parameter().unwrap()) * 0.5;
    let position = curve.value(parameter).unwrap();
    let upward = curve.value(curve.last_parameter().unwrap()).unwrap().z()
        > curve.value(curve.first_parameter().unwrap()).unwrap().z();
    let arc = Handle::new(BRepAdaptorCurve2d::with_edge_on_face(&edge, &owner).unwrap());
    let mut source = IntPatchPoint::new();
    source.set_value_tangent(position, 1e-7, false);
    source.set_parameters(0., 0., position.x(), position.y());
    source.set_arc(
        true,
        &arc.cast(),
        parameter,
        &IntSurfTransition::new(),
        &IntSurfTransition::new(),
    );
    let mut point = TopOpeBRepVPointInter::new();
    point.set_owned_point(source);
    point.set_state(State::In, 2).unwrap();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut pair = TopOpeBRepFacesIntersector::new();
    let mut filler = Filler::new();
    filler
        .bind_context(owner.as_ref(), opposite.as_ref(), &mut pair, &hds)
        .unwrap();
    let (kind, index) = filler.make_geometry(&point, 0).unwrap();
    let expected = if upward {
        (State::In, State::Out)
    } else {
        (State::Out, State::In)
    };
    let transition = filler
        .get_edge_trans(&point, kind, index, 1, &opposite)
        .unwrap();
    assert_eq!((transition.before(), transition.after()), expected);
    hds.get()
        .unwrap()
        .change_ds()
        .add_shape_with_rank(edge.as_ref(), 1);
    let registered = filler
        .get_edge_trans(&point, kind, index, 1, &opposite)
        .unwrap();
    assert_eq!((registered.before(), registered.after()), expected);
    filler
        .process_vponclosing_r(
            &point,
            owner.as_ref(),
            1,
            &TopOpeBRepDSTransition::default(),
            kind,
            index,
            false,
            &Handle::default(),
        )
        .unwrap();
    let storage = hds.get().unwrap();
    let ds = storage.ds();
    let interferences = ds.shape_interferences(edge.as_ref(), true);
    assert_eq!(interferences.len(), 1);
    let interference = interferences.iter().next().unwrap().get().unwrap();
    assert_eq!(
        (
            interference.transition().before(),
            interference.transition().after()
        ),
        expected
    );
    assert_eq!(interference.support_type(), Kind::Face);
}
