//! Restriction edge selection retains its native endpoint and classifier rules.
use martensite_tkbool::{
    top_ope_brep::{TopOpeBRepFacesFiller, TopOpeBRepFacesIntersector, TopOpeBRepLineInter},
    top_ope_brep_ds::TopOpeBRepDSHDataStructure,
    top_ope_brep_tool::TopOpeBRepToolShapeClassifier,
};
use martensite_tkbrep::{
    brep_adaptor::{BRepAdaptorCurve, BRepAdaptorCurve2d, BRepAdaptorSurface},
    topexp::Explorer,
    topods::{self, Edge, Face},
};
use martensite_tkernel::{ncollection::List, standard::Handle};
use martensite_tkg3d::{
    adaptor3d::Adaptor3dCurve,
    top_abs::{ShapeEnum, State},
};
use martensite_tkgeomalgo::{
    int_patch::{IntPatchPoint, IntPatchPointLine, IntPatchRLine},
    int_surf::IntSurfTransition,
};
use martensite_tkmath::gp::{Axis3, Cylinder3, Direction3, Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

fn face(bounds: (f64, f64, f64, f64)) -> Face {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), bounds)
        .unwrap()
        .face()
        .unwrap()
}

fn restriction(face: &Face, parameters: &[f64], opposite: &[State]) -> TopOpeBRepLineInter {
    restriction_operand(face, parameters, opposite, true)
}

fn restriction_operand(
    face: &Face,
    parameters: &[f64],
    opposite: &[State],
    first: bool,
) -> TopOpeBRepLineInter {
    let edge = topods::edge(
        Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)
            .unwrap()
            .current(),
    )
    .unwrap();
    restriction_edge(face, &edge, parameters, opposite, first)
}

fn restriction_edge(
    face: &Face,
    edge: &Edge,
    parameters: &[f64],
    opposite: &[State],
    first: bool,
) -> TopOpeBRepLineInter {
    let arc = Handle::new(BRepAdaptorCurve2d::with_edge_on_face(edge, face).unwrap());
    let curve = BRepAdaptorCurve::with_edge(edge).unwrap();
    let source = Handle::new(IntPatchRLine::new_undecided(false));
    if first {
        source.get().unwrap().set_arc_on_s1(&arc.cast());
    } else {
        source.get().unwrap().set_arc_on_s2(&arc.cast());
    }
    for parameter in parameters {
        let mut point = IntPatchPoint::new();
        point.set_value_tangent(curve.value(*parameter).unwrap(), 1e-7, false);
        point.set_parameter(*parameter);
        point.set_arc(
            first,
            &arc.cast(),
            *parameter,
            &IntSurfTransition::new(),
            &IntSurfTransition::new(),
        );
        source.get().unwrap().add_vertex(&point, false);
    }
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &source.cast(),
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    for (index, state) in opposite.iter().enumerate() {
        let point = line.change_vpoint(index as i32 + 1).unwrap();
        point
            .set_state(State::On, if first { 1 } else { 2 })
            .unwrap();
        point.set_state(*state, if first { 2 } else { 1 }).unwrap();
        point.change_keep(true);
    }
    line.set_vp_bounds();
    line
}

#[test]
fn distinct_restriction_bounds_preserve_in_out_endpoint_precedence() {
    let face = face((-1., 1., -1., 1.));
    let mut filler = TopOpeBRepFacesFiller::new();
    assert!(
        filler
            .keep_rline(
                &mut restriction(&face, &[-0.5, 0.5], &[State::In, State::On]),
                false
            )
            .unwrap()
    );
    assert!(
        !filler
            .keep_rline(
                &mut restriction(&face, &[-0.5, 0.5], &[State::In, State::Out]),
                false
            )
            .unwrap()
    );
    assert!(
        !filler
            .keep_rline(&mut TopOpeBRepLineInter::new(), false)
            .unwrap()
    );
}

#[test]
fn equal_nonperiodic_parameters_and_filtered_empty_lines_are_rejected() {
    let face = face((-1., 1., -1., 1.));
    let mut filler = TopOpeBRepFacesFiller::new();
    assert!(
        !filler
            .keep_rline(
                &mut restriction(&face, &[0., 0.], &[State::In, State::In]),
                false
            )
            .unwrap()
    );
    let mut line = restriction(&face, &[-0.5, 0.5], &[State::In, State::In]);
    for index in 1..=2 {
        line.change_vpoint(index).unwrap().change_keep(false);
    }
    assert!(filler.keep_rline(&mut line, false).unwrap());
    assert!(!filler.keep_rline(&mut line, true).unwrap());
}

#[test]
fn ambiguous_endpoints_use_the_source_weighted_midpoint_classifier() {
    let first = face((-1., 1., -1., 1.));
    let second = face((-2., 2., -2., 2.));
    let mut classifier = TopOpeBRepToolShapeClassifier::new();
    let mut pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.set_p_shape_classifier(Some(&mut classifier));
    filler
        .bind_context(first.as_ref(), second.as_ref(), &mut pair, &hds)
        .unwrap();
    assert!(
        filler
            .keep_rline(
                &mut restriction(&first, &[-0.5, 0.5], &[State::Unknown, State::Unknown]),
                false
            )
            .unwrap()
    );
}

#[test]
fn empty_intersector_appends_no_edges_and_registers_no_sections() {
    let first = face((-1., 1., -1., 1.));
    let second = face((-2., 2., -2., 2.));
    let mut pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(first.as_ref(), second.as_ref(), &mut pair, &hds)
        .unwrap();
    let mut edges = List::default();
    filler.get_esl(&mut edges).unwrap();
    assert!(edges.is_empty());
    filler.process_section_edges().unwrap();
    assert_eq!(hds.get().unwrap().ds().nb_section_edges(), 0);
}

#[test]
fn shared_restrictions_deduplicate_and_register_opposite_operand_domains() {
    let first = face((-1., 1., -1., 1.));
    let duplicate = face((-1., 1., -1., 1.));
    let second = BRepBuilderAPIMakeFace::from_plane_bounds(
        Plane3::new(
            Point3::new(0., -1., 0.),
            Direction3::new(0., 1., 0.).unwrap(),
        )
        .unwrap(),
        (-1., 1., -1., 1.),
    )
    .unwrap()
    .face()
    .unwrap();
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.perform(first.as_ref(), second.as_ref()).unwrap();
    assert!(pair.nb_lines() >= 2);
    let a = restriction(&first, &[-0.5, 0.5], &[State::In, State::On]);
    let b = restriction_operand(&duplicate, &[-0.5, 0.5], &[State::In, State::On], false);
    let edge_a = a.arc().unwrap();
    let edge_b = b.arc().unwrap();
    // The published mutable line array is a supported seam; repeated slots must not duplicate sections.
    for index in 1..=pair.nb_lines() {
        *pair.change_line(index).unwrap() = if index % 2 == 1 {
            restriction(&first, &[-0.5, 0.5], &[State::In, State::On])
        } else {
            restriction_operand(&duplicate, &[-0.5, 0.5], &[State::In, State::On], false)
        };
    }
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(first.as_ref(), second.as_ref(), &mut pair, &hds)
        .unwrap();
    let mut selected = List::default();
    selected.append(edge_b.clone());
    filler.get_esl(&mut selected).unwrap();
    assert_eq!(selected.len(), 3);
    filler.process_section_edges().unwrap();
    let storage = hds.get().unwrap().ds();
    assert_eq!(storage.nb_section_edges(), 2);
    assert_eq!(storage.ancestor_rank(&edge_a), 1);
    assert_eq!(storage.ancestor_rank(&edge_b), 2);
    assert!(
        storage
            .shape_same_domain(&edge_a)
            .iter()
            .any(|shape| shape.is_same(&edge_b))
    );
    assert!(
        storage
            .shape_same_domain(&edge_b)
            .iter()
            .any(|shape| shape.is_same(&edge_a))
    );
}

fn circle_rim() -> (Face, Edge) {
    let circle_face = BRepBuilderAPIMakeFace::from_cylinder_bounds(
        Cylinder3::new(Axis3::default(), 1.).unwrap(),
        (0., std::f64::consts::TAU, 0., 1.),
    )
    .unwrap()
    .face()
    .unwrap();
    let mut explorer = Explorer::new(&circle_face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let edge = loop {
        assert!(explorer.more());
        let edge = topods::edge(explorer.current()).unwrap();
        if BRepAdaptorCurve::with_edge(&edge)
            .unwrap()
            .get_type()
            .unwrap()
            == martensite_tkmath::geom_abs::CurveType::Circle
        {
            break edge;
        }
        explorer.next().unwrap();
    };
    (circle_face, edge)
}

#[test]
fn closed_circle_restrictions_keep_periodic_coincident_and_interior_bounds() {
    let (circle_face, edge) = circle_rim();
    let mut filler = TopOpeBRepFacesFiller::new();
    for parameters in [
        [0., 0.],
        [0., std::f64::consts::PI],
        [0., std::f64::consts::TAU],
    ] {
        let mut line = restriction_edge(
            &circle_face,
            &edge,
            &parameters,
            &[State::In, State::In],
            true,
        );
        assert!(
            filler.keep_rline(&mut line, false).unwrap(),
            "parameters={parameters:?}"
        );
    }
    for states in [[State::Unknown, State::In], [State::In, State::Unknown]] {
        let mut line = restriction_edge(&circle_face, &edge, &[0., 0.], &states, true);
        assert!(filler.keep_rline(&mut line, false).unwrap());
    }
    for states in [[State::Unknown, State::Unknown], [State::In, State::Out]] {
        let mut line = restriction_edge(&circle_face, &edge, &[0., 0.], &states, true);
        assert!(!filler.keep_rline(&mut line, false).unwrap());
    }
}

#[test]
fn reversed_periodic_end_parameters_are_repaired_before_midpoint_classification() {
    let (circle_face, edge) = circle_rim();
    let location = BRepAdaptorCurve::with_edge(&edge)
        .unwrap()
        .value(0.)
        .unwrap();
    let opposite = BRepBuilderAPIMakeFace::from_plane_bounds(
        Plane3::new(location, Direction3::new(0., 0., 1.).unwrap()).unwrap(),
        (-3., 3., -3., 3.),
    )
    .unwrap()
    .face()
    .unwrap();
    let mut classifier = TopOpeBRepToolShapeClassifier::new();
    let mut pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.set_p_shape_classifier(Some(&mut classifier));
    filler
        .bind_context(circle_face.as_ref(), opposite.as_ref(), &mut pair, &hds)
        .unwrap();
    let mut line = restriction_edge(
        &circle_face,
        &edge,
        &[std::f64::consts::TAU, 0.],
        &[State::On, State::On],
        true,
    );
    assert!(filler.keep_rline(&mut line, false).unwrap());
}
