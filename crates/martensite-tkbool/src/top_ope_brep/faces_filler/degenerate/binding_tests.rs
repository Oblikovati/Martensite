//! Native per-record treatment persists across reloads and moves; new records reset it.
//! TopOpeBRepvprdeg::local_ReduceMapOfTreatedVertices retains the cache for the same record.
//! Rust SetLine mutates that record; moving another newly created value into its slot replaces it.
use super::tests::{point_line, pole_point, sphere_pole};
use super::*;
use martensite_tkgeomalgo::int_patch::IntPatchPoint;

fn seed_treated(filler: &mut TopOpeBRepFacesFiller<'_>, vertex: &Vertex, edge: &Edge) {
    let mut treated = List::default();
    treated.append(edge.clone().into_shape());
    filler
        .degenerate_context
        .treated
        .bind(vertex.clone().into_shape(), treated);
}

fn visit_without_pole(filler: &mut TopOpeBRepFacesFiller<'_>, line: &mut TopOpeBRepLineInter) {
    let mut point = TopOpeBRepVPointInter::new();
    point.set_owned_point(IntPatchPoint::new());
    let mut search = GeometrySearch {
        index: 1,
        ..Default::default()
    };
    assert!(
        !filler
            .process_vpondg_e(&point, 1, line, &mut search)
            .unwrap()
    );
}

#[test]
fn moving_a_loaded_line_retains_treated_edges_until_its_binding_ends() {
    let (face, edge, vertex) = sphere_pole();
    let source = pole_point(&face, &edge, &vertex);
    let mut line = Box::new(point_line(source.clone(), 1));
    let mut pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(face.as_ref(), face.as_ref(), &mut pair, &hds)
        .unwrap();
    filler.load_line(&mut line).unwrap();
    filler.initialize_degenerate_edges(&line).unwrap();
    visit_without_pole(&mut filler, &mut line);
    seed_treated(&mut filler, &vertex, &edge);
    let mut moved = std::mem::replace(&mut *line, point_line(source, 1));
    assert!(!std::ptr::eq(&*line, &moved));
    visit_without_pole(&mut filler, &mut moved);
    assert_eq!(filler.degenerate_context.treated.extent(), 1);
}

#[test]
fn a_new_record_in_the_same_slot_discards_previous_treatment() {
    let (face, edge, vertex) = sphere_pole();
    let source = pole_point(&face, &edge, &vertex);
    let mut line = Box::new(point_line(source.clone(), 1));
    let mut pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(face.as_ref(), face.as_ref(), &mut pair, &hds)
        .unwrap();
    filler.load_line(&mut line).unwrap();
    filler.initialize_degenerate_edges(&line).unwrap();
    visit_without_pole(&mut filler, &mut line);
    seed_treated(&mut filler, &vertex, &edge);
    *line = point_line(source, 1);
    visit_without_pole(&mut filler, &mut line);
    assert!(filler.degenerate_context.treated.is_empty());
    assert_eq!(filler.degenerate_context.entries.extent(), 1);
}

#[test]
fn reloading_the_same_record_preserves_its_native_treatment_scope() {
    let (face, edge, vertex) = sphere_pole();
    let mut line = point_line(pole_point(&face, &edge, &vertex), 1);
    let mut pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(face.as_ref(), face.as_ref(), &mut pair, &hds)
        .unwrap();
    filler.load_line(&mut line).unwrap();
    filler.initialize_degenerate_edges(&line).unwrap();
    visit_without_pole(&mut filler, &mut line);
    seed_treated(&mut filler, &vertex, &edge);
    filler.load_line(&mut line).unwrap();
    visit_without_pole(&mut filler, &mut line);
    assert_eq!(filler.degenerate_context.treated.extent(), 1);
}

#[test]
fn changing_a_records_intersection_source_preserves_its_treatment_scope() {
    let (face, edge, vertex) = sphere_pole();
    let source = pole_point(&face, &edge, &vertex);
    let mut line = point_line(source.clone(), 1);
    let mut pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(face.as_ref(), face.as_ref(), &mut pair, &hds)
        .unwrap();
    filler.load_line(&mut line).unwrap();
    filler.initialize_degenerate_edges(&line).unwrap();
    visit_without_pole(&mut filler, &mut line);
    seed_treated(&mut filler, &vertex, &edge);
    let replacement = point_line(source, 1);
    line.set_line(
        &replacement.line_r().cast(),
        &martensite_tkbrep::brep_adaptor::BRepAdaptorSurface::default(),
        &martensite_tkbrep::brep_adaptor::BRepAdaptorSurface::default(),
    )
    .unwrap();
    filler.load_line(&mut line).unwrap();
    visit_without_pole(&mut filler, &mut line);
    assert_eq!(filler.degenerate_context.treated.extent(), 1);
}
