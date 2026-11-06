//! Source geometry, planar containment and exact history relations pin extension behavior.
use super::super::fixture::gap_incidence_fixture;
use super::*;
use martensite_tkbrep::{brep::BRepTool, brep_tools::uv_bounds, topods::Shape};
fn face_bounds(shape: &Shape) -> (f64, f64, f64, f64) {
    uv_bounds(&topods::face(shape).unwrap()).unwrap()
}
fn require_planar_extension(source: &Shape, extended: &Shape) {
    let before = face_bounds(source);
    let after = face_bounds(extended);
    assert!(after.0 < before.0 && after.1 > before.1);
    assert!(after.2 < before.2 && after.3 > before.3);
    assert_eq!(extended.orientation(), source.orientation());
    assert_eq!(
        BRepTool::face_tolerance(&topods::face(source).unwrap()).unwrap(),
        BRepTool::face_tolerance(&topods::face(extended).unwrap()).unwrap()
    );
    assert!(!source.is_same(extended));
}
fn require_modified_relation(history: &History, source: &Shape, extended: &Shape) {
    let modified = history.modified(source).unwrap();
    assert_eq!(modified.len(), 1);
    assert!(modified[0].is_same(extended));
    assert!(history.generated(source).unwrap().is_empty());
    assert!(!history.is_removed(source).unwrap());
}
#[test]
fn default_gap_extension_preserves_input_bounds_and_exact_modified_relations() {
    let fixture = gap_incidence_fixture(1);
    let mut gap = FillGap::new(&fixture.feature, false);
    let adjacent = gap.find_adjacent_faces(&fixture.connections()).unwrap();
    let before: Vec<_> = adjacent.iter().map(face_bounds).collect();
    let history = History::new();
    let extensions = gap.extend_adjacent_faces(&adjacent, &history).unwrap();
    assert_eq!(extensions.extent(), 4);
    for (index, source) in adjacent.iter().enumerate() {
        let extended = extensions.find_from_key(source).unwrap();
        require_planar_extension(source, extended);
        require_modified_relation(&history, source, extended);
        assert_eq!(face_bounds(source), before[index]);
    }
}

#[test]
fn first_extension_pulse_keeps_history_and_extension_map_empty() {
    use crate::bop_algo::{
        ProgressCancellation, progress_test_support::parent_census::parent_census_indicator,
    };
    let fixture = gap_incidence_fixture(1);
    let mut gap = FillGap::new(&fixture.feature, false);
    let adjacent = gap.find_adjacent_faces(&fixture.connections()).unwrap();
    let history = History::new();
    let (indicator, journal) = parent_census_indicator(1);
    let observed = ProgressCancellation::default();
    let extended = gap
        .extend_adjacent_faces_on_progress(&adjacent, &history, &indicator.start(), &observed)
        .unwrap();
    assert!(observed.is_cancelled());
    assert!(journal.observed.load(std::sync::atomic::Ordering::SeqCst));
    assert_eq!(extended.extent(), 0);
    assert!(
        adjacent
            .iter()
            .all(|face| history.modified(face).unwrap().is_empty())
    );
}

#[path = "extension_census.rs"]
mod census;

#[test]
fn middle_extension_pulse_retains_only_completed_input_history_prefix() {
    use crate::bop_algo::{
        ProgressCancellation, progress_test_support::parent_census::parent_census_indicator,
    };
    let fixture = gap_incidence_fixture(1);
    let mut gap = FillGap::new(&fixture.feature, false);
    let adjacent = gap.find_adjacent_faces(&fixture.connections()).unwrap();
    let history = History::new();
    let (indicator, _) = parent_census_indicator(3);
    let observed = ProgressCancellation::default();
    let extended = gap
        .extend_adjacent_faces_on_progress(&adjacent, &history, &indicator.start(), &observed)
        .unwrap();
    assert!(observed.is_cancelled());
    assert_eq!(extended.extent(), 2);
    for (ordinal, source) in adjacent.iter().enumerate() {
        assert_eq!(
            history.modified(source).unwrap().len(),
            usize::from(ordinal < 2)
        );
    }
}
fn malformed_extension_case() -> (FillGap, IndexedMapOfShape, Vec<Shape>) {
    use martensite_tkmath::gp::Point3;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
    let fixture = gap_incidence_fixture(1);
    let gap = FillGap::new(&fixture.feature, false);
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(1., 2., 3.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape();
    let sources = vec![fixture.faces[2].clone(), fixture.faces[3].clone(), vertex];
    let mut adjacent = IndexedMapOfShape::default();
    for source in &sources {
        adjacent.add(source.clone());
    }
    (gap, adjacent, sources)
}
fn require_type_mismatch(result: Result<ExtendedFaces, Failure>) {
    let failure = match result {
        Err(cause) => cause,
        Ok(_) => panic!("vertex input; expected typed face conversion failure"),
    };
    assert_eq!(failure.exception_type(), "Standard_TypeMismatch");
}
#[test]
fn extension_failure_retains_typed_cause_and_completed_prefix_history() {
    use crate::bop_algo::{
        ProgressCancellation, progress_test_support::parent_census::parent_census_indicator,
    };
    let (gap, adjacent, sources) = malformed_extension_case();
    let history = History::new();
    let observed = ProgressCancellation::default();
    let (indicator, journal) = parent_census_indicator(0);
    require_type_mismatch(gap.extend_adjacent_faces_on_progress(
        &adjacent,
        &history,
        &indicator.start(),
        &observed,
    ));
    assert!(!observed.is_cancelled());
    assert_eq!(journal.queries.load(std::sync::atomic::Ordering::SeqCst), 3);
    for (ordinal, source) in sources.iter().enumerate() {
        assert_eq!(
            history.modified(source).unwrap().len(),
            usize::from(ordinal < 2)
        );
    }
}
