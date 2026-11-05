//! Sphere poles and independent topology identities exercise the degenerate state machine.
use super::*;
use martensite_tkbrep::brep_adaptor::{BRepAdaptorCurve2d, BRepAdaptorSurface};
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkgeomalgo::{
    int_patch::{IntPatchPoint, IntPatchPointLine, IntPatchRLine},
    int_surf::IntSurfTransition,
};
use martensite_tkmath::gp::{Axis3, Direction3, Plane3, Sphere3};
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeFace, brep_top_adaptor::BRepTopAdaptorHVertex,
};

pub(super) fn sphere_pole() -> (Face, Edge, Vertex) {
    let face = BRepBuilderAPIMakeFace::from_sphere(Sphere3::new(Axis3::default(), 1.).unwrap())
        .unwrap()
        .face()
        .unwrap();
    for shape in Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap() {
        let edge = topods::edge(&shape).unwrap();
        if BRepTool::degenerated(&edge).unwrap() {
            return (
                face,
                edge.clone(),
                collect::first_vertex(edge.as_ref()).unwrap(),
            );
        }
    }
    panic!("sphere expected degenerate pole edge");
}

pub(super) fn pole_point(face: &Face, edge: &Edge, vertex: &Vertex) -> IntPatchPoint {
    let mut point = IntPatchPoint::new();
    point.set_value_tangent(BRepTool::pnt(vertex).unwrap(), 1e-7, false);
    add_domain(&mut point, face, edge, vertex, true);
    point
}

fn add_domain(point: &mut IntPatchPoint, face: &Face, edge: &Edge, vertex: &Vertex, first: bool) {
    let arc = Handle::new(BRepAdaptorCurve2d::with_edge_on_face(edge, face).unwrap());
    let parameter = BRepTool::parameter(vertex, edge).unwrap();
    let uv = arc.get().unwrap().value(parameter).unwrap();
    point.set_arc(
        first,
        &arc.cast(),
        parameter,
        &IntSurfTransition::new(),
        &IntSurfTransition::new(),
    );
    point.set_vertex(
        first,
        &Handle::new(BRepTopAdaptorHVertex::new(vertex, &arc)).cast(),
    );
    if first {
        point.set_parameters(uv.x(), uv.y(), 0., 0.);
    } else {
        let (u, v) = point.parameters_on_s1();
        point.set_parameters(u, v, uv.x(), uv.y());
    }
}

pub(super) fn point_line(point: IntPatchPoint, mask: i32) -> TopOpeBRepLineInter {
    let source = Handle::new(IntPatchRLine::new_undecided(false));
    if point.is_on_dom_s1() {
        source
            .get()
            .unwrap()
            .set_arc_on_s1(&point.arc_on_s1().unwrap());
    }
    if point.is_on_dom_s2() {
        source
            .get()
            .unwrap()
            .set_arc_on_s2(&point.arc_on_s2().unwrap());
    }
    source.get().unwrap().add_vertex(&point, false);
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &source.cast(),
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    line.change_vpoint(1).unwrap().set_shape_index(mask);
    line
}

#[test]
fn collection_keeps_unselected_poles_and_finds_missing_closing_incidence() {
    let (face, edge, vertex) = sphere_pole();
    let line = point_line(pole_point(&face, &edge, &vertex), 1);
    assert!(!line.vpoint(1).unwrap().keep());
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&hds).unwrap();
    filler.faces = [face.clone(), Face::default()];
    filler.initialize_degenerate_edges(&line).unwrap();
    let edges = filler
        .degenerate_context
        .entries
        .find(vertex.as_ref())
        .unwrap();
    assert_eq!(edges.len(), 2);
    assert!(TopOpeBRepToolShapeTool::closed(edges.first().unwrap(), face.as_ref()).unwrap());
    assert!(edges.last().unwrap().is_same(edge.as_ref()));
}

#[test]
fn both_operand_vertices_link_without_keep_filter() {
    let (a, edge_a, vertex_a) = sphere_pole();
    let (b, edge_b, vertex_b) = sphere_pole();
    let mut point = pole_point(&a, &edge_a, &vertex_a);
    add_domain(&mut point, &b, &edge_b, &vertex_b, false);
    let line = point_line(point, 3);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&hds).unwrap();
    filler.faces = [a, b];
    filler.initialize_degenerate_edges(&line).unwrap();
    let ds = hds.get().unwrap().ds();
    assert!(
        ds.shape_same_domain(vertex_a.as_ref())
            .iter()
            .any(|shape| shape.is_same(vertex_b.as_ref()))
    );
    assert!(
        ds.shape_same_domain(vertex_b.as_ref())
            .iter()
            .any(|shape| shape.is_same(vertex_a.as_ref()))
    );
    assert_eq!(filler.degenerate_context.entries.extent(), 2);
}

#[test]
fn treated_edge_state_is_scoped_to_line_identity_and_operation() {
    let (_, edge, vertex) = sphere_pole();
    let (line_a, line_b) = (TopOpeBRepLineInter::new(), TopOpeBRepLineInter::new());
    let mut context = DegenerateContext::default();
    context.reduce_treated(&line_a);
    let mut edges = List::default();
    edges.append(edge.clone().into_shape());
    context.treated.bind(vertex.clone().into_shape(), edges);
    context.reduce_treated(&line_a);
    assert_eq!(context.treated.extent(), 1);
    context.reduce_treated(&line_b);
    assert!(context.treated.is_empty());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.degenerate_context = context;
    filler.reset_degenerate_context();
    assert!(filler.degenerate_context.line_identity.is_none());
}

#[test]
fn empty_degenerate_context_performs_lookup_without_creating_geometry() {
    let (face, edge, vertex) = sphere_pole();
    let source = pole_point(&face, &edge, &vertex);
    let line = point_line(source.clone(), 1);
    let mut point = TopOpeBRepVPointInter::new();
    point.set_owned_point(source);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&hds).unwrap();
    let mut search = GeometrySearch::default();
    assert!(
        !filler
            .process_vpondg_e(&point, 1, &mut { line }, &mut search)
            .unwrap()
    );
    assert_eq!(search.index, 0);
    assert_eq!(hds.get().unwrap().ds().nb_points(), 0);
}

#[test]
fn sphere_pole_cut_by_vertical_plane_has_two_antipodal_transitions() {
    let (sphere, edge, vertex) = sphere_pole();
    let location = BRepTool::pnt(&vertex).unwrap();
    let plane = BRepBuilderAPIMakeFace::from_plane(
        Plane3::new(location, Direction3::new(1., 0., 0.).unwrap()).unwrap(),
    )
    .unwrap()
    .face()
    .unwrap();
    let source = pole_point(&sphere, &edge, &vertex);
    let line = point_line(source.clone(), 1);
    let mut point = TopOpeBRepVPointInter::new();
    point.set_owned_point(source);
    point.set_shape_index(1);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&hds).unwrap();
    filler.faces = [sphere, plane];
    filler.initialize_degenerate_edges(&line).unwrap();
    let crossing = filler
        .degenerate_crossing(&point)
        .unwrap()
        .expect("transverse plane cut");
    assert_eq!(crossing.rank, 1);
    assert!(!crossing.planar);
    assert_eq!(crossing.transitions.0, 3);
    let (a, b) = (crossing.transitions.1, crossing.transitions.2);
    assert!(martensite_cmath::cos(a).abs() < 1e-12, "a={a}");
    assert!(martensite_cmath::cos(b).abs() < 1e-12, "b={b}");
    assert!(((a - b).abs() - std::f64::consts::PI).abs() < 1e-12);
}

#[test]
fn process_creates_vertex_geometry_and_two_face_supported_pole_interferences() {
    let (sphere, edge, vertex) = sphere_pole();
    let location = BRepTool::pnt(&vertex).unwrap();
    let plane = BRepBuilderAPIMakeFace::from_plane(
        Plane3::new(location, Direction3::new(1., 0., 0.).unwrap()).unwrap(),
    )
    .unwrap()
    .face()
    .unwrap();
    let source = pole_point(&sphere, &edge, &vertex);
    let mut line = point_line(source.clone(), 1);
    let mut point = TopOpeBRepVPointInter::new();
    point.set_owned_point(source);
    point.set_shape_index(1);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&hds).unwrap();
    filler.faces = [sphere, plane];
    filler.initialize_degenerate_edges(&line).unwrap();
    let mut search = GeometrySearch::default();
    assert!(
        filler
            .process_vpondg_e(&point, 1, &mut line, &mut search)
            .unwrap()
    );
    assert_eq!(search.kind, TopOpeBRepDSKind::Vertex);
    assert!(search.index > 0);
    let ds = hds.get().unwrap().ds();
    let records = ds.shape_interferences(edge.as_ref(), true);
    assert_eq!(records.len(), 2);
    for (record, orientation) in records
        .iter()
        .zip([Orientation::Forward, Orientation::Reversed])
    {
        let entry = record.get().unwrap();
        assert_eq!(entry.geometry(), search.index);
        assert_eq!(entry.geometry_type(), TopOpeBRepDSKind::Vertex);
        assert_eq!(entry.support_type(), TopOpeBRepDSKind::Face);
        assert_eq!(entry.transition().index().unwrap(), entry.support());
        assert_eq!(
            entry
                .transition()
                .orientation(martensite_tkg3d::top_abs::State::In, ShapeEnum::Face),
            orientation
        );
        let vertex_record =
            Handle::<crate::top_ope_brep_ds::TopOpeBRepDSEdgeVertexInterference>::down_cast(record);
        let vertex_record = vertex_record.get().unwrap();
        assert_eq!(vertex_record.gbound(), orientation == Orientation::Forward);
        let parameter = vertex_record.parameter();
        assert!(
            martensite_cmath::cos(parameter).abs() < 1e-12,
            "parameter={parameter}"
        );
    }
}

#[test]
fn planar_crossings_register_shapes_but_suppress_interference_records() {
    let (sphere, edge, vertex) = sphere_pole();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let geometry = hds
        .get()
        .unwrap()
        .change_ds()
        .add_shape_with_rank(vertex.as_ref(), 2);
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&hds).unwrap();
    filler.faces = [sphere.clone(), sphere];
    let crossing = DegenerateCrossing {
        rank: 2,
        edge: edge.clone(),
        transitions: (3, 0.25, 1.25),
        planar: true,
    };
    filler
        .store_degenerate_crossing(&crossing, geometry)
        .unwrap();
    let ds = hds.get().unwrap().ds();
    assert!(ds.shape_interferences(edge.as_ref(), true).is_empty());
    assert_eq!(ds.ancestor_rank(edge.as_ref()), 2);
}

#[test]
fn identical_parallel_spheres_use_restrictions_without_three_dimensional_fallback() {
    let (sphere, edge, vertex) = sphere_pole();
    let mut source = pole_point(&sphere, &edge, &vertex);
    let (u, v) = source.parameters_on_s1();
    source.set_parameters(u, v, u, v);
    let line = point_line(source.clone(), 1);
    let mut point = TopOpeBRepVPointInter::new();
    point.set_owned_point(source);
    point.set_shape_index(1);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&hds).unwrap();
    filler.faces = [sphere.clone(), sphere];
    filler.initialize_degenerate_edges(&line).unwrap();
    // No explicit opposite restriction and all incident edges are degenerate or closing.
    // Parallel normals do not authorize a 3D transition when that search has no result.
    assert!(filler.degenerate_crossing(&point).unwrap().is_none());
    assert_eq!(hds.get().unwrap().ds().nb_points(), 0);
}

#[test]
fn published_geometry_search_record_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<super::super::TopOpeBRepFacesFillerGeometrySearch>();
}
