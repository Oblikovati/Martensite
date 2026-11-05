//! Expected supports and before/after states follow independent orientation tables.
use super::*;
use crate::top_ope_brep::TopOpeBRepPointGeomTool;
use crate::top_ope_brep_ds::{
    TopOpeBRepDSHDataStructure, TopOpeBRepDSKind as Kind, TopOpeBRepDSTransition as Transition,
};
use martensite_tkbrep::{
    brep_adaptor::{BRepAdaptorCurve, BRepAdaptorCurve2d, BRepAdaptorSurface},
    topexp::Explorer,
    topods::{self, Edge, Face},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{
    adaptor3d::Adaptor3dCurve,
    top_abs::{Orientation, ShapeEnum, State},
};
use martensite_tkgeomalgo::{
    int_patch::{IntPatchGLine, IntPatchLineTransitions, IntPatchPoint},
    int_start::TheArc,
    int_surf::{IntSurfTransition, IntSurfTypeTrans},
};
use martensite_tkmath::gp::{Line3, Plane3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

pub(super) fn face() -> Face {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1., 1., -1., 1.))
        .unwrap()
        .face()
        .unwrap()
}
pub(super) fn edge(face: &Face) -> Edge {
    let explorer = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    topods::edge(&explorer.current().oriented(Orientation::Forward)).unwrap()
}
pub(super) fn transition(kind: IntSurfTypeTrans) -> IntSurfTransition {
    let mut transition = IntSurfTransition::new();
    transition.set_type(false, kind);
    transition
}
fn sample(
    edge: &Edge,
    face: &Face,
    parameter: f64,
    line_parameter: f64,
    on_two: bool,
    known: bool,
) -> IntPatchPoint {
    let curve = BRepAdaptorCurve::with_edge(edge).unwrap();
    let arc: TheArc =
        Handle::new(BRepAdaptorCurve2d::with_edge_on_face(edge, face).unwrap()).cast();
    let mut point = IntPatchPoint::new();
    point.set_value_tangent(curve.value(parameter).unwrap(), 1e-7, false);
    point.set_parameter(line_parameter);
    let kind = if known {
        IntSurfTypeTrans::In
    } else {
        IntSurfTypeTrans::Undecided
    };
    point.set_arc(true, &arc, parameter, &transition(kind), &transition(kind));
    if on_two {
        point.set_arc(false, &arc, parameter, &transition(kind), &transition(kind));
    }
    point
}
pub(super) fn fixture(
    on_two: bool,
    known: bool,
) -> (TopOpeBRepFacesFiller<'static>, TopOpeBRepLineInter, Edge) {
    let first = face();
    let second = face();
    let edge = edge(&first);
    let curve = BRepAdaptorCurve::with_edge(&edge).unwrap();
    let (first_parameter, last_parameter) = (
        curve.first_parameter().unwrap(),
        curve.last_parameter().unwrap(),
    );
    let source =
        IntPatchGLine::from_line(&Line3::default(), false, IntPatchLineTransitions::Undecided)
            .unwrap();
    for (index, fraction) in [(1, 0.25), (2, 0.5), (3, 0.75)] {
        let point = sample(
            &edge,
            &first,
            first_parameter + (last_parameter - first_parameter) * fraction,
            f64::from(index),
            on_two,
            known,
        );
        source.add_vertex(&point).unwrap();
    }
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &Handle::new(source).cast(),
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    for index in 1..=3 {
        let point = line.change_vpoint(index).unwrap();
        point.change_keep(true);
    }
    line.set_vp_bounds();
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_storage(&Handle::new(TopOpeBRepDSHDataStructure::new()))
        .unwrap();
    filler.faces = [first, second];
    (filler, line, edge)
}

#[test]
fn fixed_other_face_orientation_precedes_unbound_point_geometry() {
    let (mut filler, mut line, edge) = fixture(false, true);
    filler.faces[1] = topods::face(&filler.faces[1].oriented(Orientation::Internal)).unwrap();
    let result = super::new_transition::new_transition_edge(
        &filler,
        &mut line,
        &super::new_transition::EdgeTransitionQuery {
            line_on_restriction: false,
            point: &TopOpeBRepVPointInter::new(),
            kind: Kind::Point,
            index: 0,
            other_side: 2,
            edge: &edge,
            restrictions: &Default::default(),
        },
    )
    .unwrap()
    .unwrap();
    assert_eq!((result.before(), result.after()), (State::In, State::In));
}

#[test]
fn known_crossing_stores_face_support_with_exact_source_states() {
    let (mut filler, mut line, edge) = fixture(false, true);
    filler.line_is_on_edge = true;
    filler
        .process_vpion_r(
            &mut crate::top_ope_brep::TopOpeBRepVPointInterIterator::with_line(&mut line).unwrap(),
            &Transition::from_orientation(Orientation::Forward),
            &filler.faces[0].clone(),
            1,
        )
        .unwrap();
    let storage = filler.hds.get().unwrap();
    let ds = storage.ds();
    let list = ds.shape_interferences(&edge, true);
    assert_eq!(list.len(), 1);
    let entry = list.iter().next().unwrap().get().unwrap();
    assert_eq!(entry.support_type(), Kind::Face);
    assert_eq!(entry.support(), ds.shape_index(&filler.faces[1], true));
    assert_eq!(entry.geometry_type(), Kind::Point);
    assert_eq!(
        (entry.transition().before(), entry.transition().after()),
        (State::Out, State::In)
    );
    assert_eq!(ds.nb_points(), 1);
    assert!(filler.curve_interferences.is_empty());
}

#[test]
fn two_edge_crossing_keeps_edge_then_face_supports() {
    let (mut filler, mut line, edge) = fixture(true, true);
    filler.line_is_on_edge = true;
    filler
        .process_vpion_r(
            &mut crate::top_ope_brep::TopOpeBRepVPointInterIterator::with_line(&mut line).unwrap(),
            &Transition::from_orientation(Orientation::Reversed),
            &filler.faces[0].clone(),
            1,
        )
        .unwrap();
    let storage = filler.hds.get().unwrap();
    let ds = storage.ds();
    let list = ds.shape_interferences(&edge, true);
    let kinds: Vec<_> = list
        .iter()
        .map(|entry| entry.get().unwrap().support_type())
        .collect();
    assert_eq!(kinds, vec![Kind::Edge, Kind::Face]);
    for entry in list.iter() {
        assert_eq!(
            (
                entry.get().unwrap().transition().before(),
                entry.get().unwrap().transition().after()
            ),
            (State::In, State::Out)
        );
    }
}

#[test]
fn repeated_equal_line_parameter_does_not_duplicate_curve_point() {
    let (mut filler, mut line, edge) = fixture(false, true);
    let point = line.vpoint(1).unwrap().clone();
    let context = super::context::Context::new(&filler, &point, 1).unwrap();
    let mut search = GeometrySearch::default();
    filler
        .find_vp_geometry(&line, &point, 1, 1, &mut search)
        .unwrap();
    let transition = Transition::from_orientation(Orientation::Forward);
    filler
        .process_curve_point(&point, &context, &mut line, &transition, &search)
        .unwrap();
    filler
        .process_curve_point(&point, &context, &mut line, &transition, &search)
        .unwrap();
    assert_eq!(filler.curve_interferences.len(), 1);
    assert_eq!(filler.kept_point_count, 1);
    assert_eq!(filler.hds.get().unwrap().ds().shape_index(&edge, true), 0);
}

#[test]
fn current_edge_parameter_fallback_and_curve_geometry_override_are_independent() {
    let (mut filler, line, edge) = fixture(false, true);
    let point = line.vpoint(1).unwrap();
    let storage = filler.hds.clone();
    let storage = storage.get().unwrap();
    storage.change_ds().add_shape(&edge);
    let distant = storage
        .change_ds()
        .add_point(&crate::top_ope_brep_ds::TopOpeBRepDSPoint::new(
            martensite_tkmath::gp::Point3::new(100., 0., 0.),
            1e-7,
        ));
    let edge_interference =
        crate::top_ope_brep_ds::process_interferences_tool::make_epv_interference(
            &Transition::default(),
            1,
            distant,
            point.edge_parameter(1).unwrap(),
            Kind::Point,
            false,
        );
    storage
        .store_interference(&edge_interference, &edge)
        .unwrap();
    let mut search = GeometrySearch::default();
    filler
        .find_vp_geometry(&line, point, 1, 0, &mut search)
        .unwrap();
    assert_eq!(search.index, distant);
    assert!(search.edge_found && !search.curve_found);
    let nearby = storage
        .change_ds()
        .add_point(&TopOpeBRepPointGeomTool::from_vpoint(point).unwrap());
    filler.curve_interferences.append(
        crate::top_ope_brep_ds::process_interferences_tool::make_cpv_interference(
            &Transition::default(),
            0,
            nearby,
            9.,
            Kind::Point,
        ),
    );
    filler
        .find_vp_geometry(&line, point, 1, 0, &mut search)
        .unwrap();
    assert_eq!(search.index, nearby);
    assert!(search.edge_found && search.curve_found);
}

#[test]
fn opposite_parameter_only_hit_does_not_claim_a_geometry_in_find_mode() {
    let (mut filler, line, _) = fixture(false, true);
    let mut point = line.vpoint(1).unwrap().clone();
    let second = filler.faces[1].clone();
    let other = edge(&second);
    let curve = BRepAdaptorCurve::with_edge(&other).unwrap();
    let parameter = 0.5 * (curve.first_parameter().unwrap() + curve.last_parameter().unwrap());
    let arc: TheArc =
        Handle::new(BRepAdaptorCurve2d::with_edge_on_face(&other, &second).unwrap()).cast();
    let mut source = point.point_of_intersection().unwrap().clone();
    source.set_arc(
        false,
        &arc,
        parameter,
        &transition(IntSurfTypeTrans::In),
        &transition(IntSurfTypeTrans::In),
    );
    point.set_owned_point(source);
    let storage = filler.hds.clone();
    let storage = storage.get().unwrap();
    storage.change_ds().add_shape(&other);
    let distant = storage
        .change_ds()
        .add_point(&crate::top_ope_brep_ds::TopOpeBRepDSPoint::new(
            martensite_tkmath::gp::Point3::new(100., 0., 0.),
            1e-7,
        ));
    let interference = crate::top_ope_brep_ds::process_interferences_tool::make_epv_interference(
        &Transition::default(),
        1,
        distant,
        parameter,
        Kind::Point,
        false,
    );
    storage.store_interference(&interference, &other).unwrap();
    let mut search = GeometrySearch::default();
    filler
        .find_vp_geometry(&line, &point, 1, 0, &mut search)
        .unwrap();
    assert_eq!(search.index, 0);
    assert!(!search.edge_found && !search.curve_found);
    filler
        .find_vp_geometry(&line, &point, 1, 2, &mut search)
        .unwrap();
    assert_ne!(search.index, distant);
    assert_eq!(storage.nb_points(), 2);
}

#[test]
fn inl_uses_the_geometric_tangent_even_beyond_trim_bounds() {
    let (mut filler, line, edge) = fixture(false, true);
    let point = line.vpoint(1).unwrap();
    let mut context = super::context::Context::new(&filler, point, 1).unwrap();
    context.current.ensure_index(&filler, 1).unwrap();
    let curve = BRepAdaptorCurve::with_edge(&edge).unwrap();
    let parameter = curve.last_parameter().unwrap() + 1.;
    let (location, tangent) = curve.d1(parameter).unwrap();
    let normal = martensite_tkmath::gp::Direction3::from_xyz(*tangent.xyz()).unwrap();
    filler.faces[1] = BRepBuilderAPIMakeFace::from_plane(Plane3::new(location, normal).unwrap())
        .unwrap()
        .face()
        .unwrap();
    context.other_face = filler.faces[1].clone();
    context.current.parameter = parameter;
    let search = GeometrySearch {
        index: filler
            .change_data_structure()
            .unwrap()
            .add_point(&TopOpeBRepPointGeomTool::from_vpoint(point).unwrap()),
        ..GeometrySearch::default()
    };
    let mut result = Transition::default();
    super::edge_interferences::process_inl(
        &filler,
        point,
        &mut context,
        &search,
        true,
        &mut result,
    )
    .unwrap();
    let storage = filler.hds.get().unwrap();
    let ds = storage.ds();
    let stored = ds
        .shape_interferences(&edge, true)
        .iter()
        .last()
        .unwrap()
        .get()
        .unwrap()
        .transition();
    assert_eq!((stored.before(), stored.after()), (State::In, State::Out));
}

#[test]
fn opposite_interference_before_same_keeps_the_native_early_selection() {
    let (filler, line, _) = fixture(false, true);
    let point = line.vpoint(1).unwrap();
    let mut context = super::context::Context::new(&filler, point, 1).unwrap();
    context.current.ensure_index(&filler, 1).unwrap();
    let storage = filler.hds.get().unwrap();
    let search = GeometrySearch {
        edge_found: true,
        index: storage
            .change_ds()
            .add_point(&TopOpeBRepPointGeomTool::from_vpoint(point).unwrap()),
        ..GeometrySearch::default()
    };
    for orientation in [Orientation::Reversed, Orientation::Forward] {
        let interference =
            crate::top_ope_brep_ds::process_interferences_tool::make_epv_interference_sk(
                &Transition::from_orientation(orientation),
                context.other_face_index,
                search.index,
                0.,
                Kind::Point,
                Kind::Face,
                false,
            )
            .unwrap();
        storage
            .store_interference(&interference, &context.current.edge)
            .unwrap();
    }
    assert!(
        super::edge_interferences::should_add_face(
            &filler,
            &context,
            &search,
            &Transition::from_orientation(Orientation::Forward)
        )
        .unwrap()
    );
}
