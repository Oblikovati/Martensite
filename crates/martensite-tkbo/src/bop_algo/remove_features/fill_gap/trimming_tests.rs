//! A closed rectangular trim has independently known bounds and one resulting face.
use super::super::fixture::gap_incidence_fixture;
use super::*;
use crate::bop_algo::remove_features::helpers::compound_of;
use martensite_tkbrep::brep_tools::uv_bounds;
fn cropped_rectangle_case() -> (FillGap, ExtendedFaces, History, Shape) {
    let fixture = gap_incidence_fixture(1);
    let mut gap = FillGap::new(&fixture.feature, false);
    let source = fixture.faces[2].clone();
    let mut adjacent = IndexedMapOfShape::default();
    adjacent.add(source.clone());
    let history = History::new();
    let extended = gap.extend_adjacent_faces(&adjacent, &history).unwrap();
    // No original edge is omitted: the four input edges bound the closed crop.
    gap.feature = compound_of(&[]).unwrap();
    (gap, extended, history, source)
}
#[test]
fn default_trimming_crops_one_extended_plane_to_its_original_rectangle() {
    let (mut gap, extended, history, source) = cropped_rectangle_case();
    let before = uv_bounds(&topods::face(&source).unwrap()).unwrap();
    gap.trim_extended_faces(&extended, &history).unwrap();
    assert_eq!(gap.faces.extent(), 1);
    let faces = gap.faces.find_from_key(&source).unwrap();
    assert_eq!(faces.len(), 1);
    let result = faces.iter().next().unwrap();
    assert_eq!(uv_bounds(&topods::face(result).unwrap()).unwrap(), before);
    assert_eq!(uv_bounds(&topods::face(&source).unwrap()).unwrap(), before);
    assert!(!history.is_removed(&source).unwrap());
}

#[test]
fn first_trimming_pulse_is_retained_without_publishing_a_cropped_face() {
    use crate::bop_algo::{
        ProgressCancellation, ProgressCompletion,
        progress_test_support::parent_census::parent_census_indicator,
    };
    let (mut gap, extended, history, _) = cropped_rectangle_case();
    let (indicator, journal) = parent_census_indicator(1);
    let observed = ProgressCancellation::default();
    let completion = gap
        .trim_extended_faces_on_progress(&extended, &history, &indicator.start(), &observed)
        .unwrap();
    assert!(matches!(completion, ProgressCompletion::Cancelled { .. }));
    assert!(observed.is_cancelled());
    assert!(journal.observed.load(std::sync::atomic::Ordering::SeqCst));
    assert_eq!(gap.faces.extent(), 0);
}

fn trim_source_indices(profile: usize) -> &'static [usize] {
    match profile {
        0 => &[],
        1 | 4 | 6 => &[2],
        2 | 5 => &[2, 4],
        3 => &[2, 3, 4, 5],
        other => panic!("profile={other}; expected 0..=6"),
    }
}
fn replace_original_with_vertex(extended: &mut ExtendedFaces, sources: &mut Vec<Shape>) {
    use martensite_tkmath::gp::Point3;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
    let image = extended.find_from_index(1).unwrap().clone();
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(1., 2., 3.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape();
    extended.clear(false);
    extended.add(vertex.clone(), image);
    *sources = vec![vertex];
}
fn ranged_trimming_case(
    profile: usize,
    parallel: bool,
) -> (FillGap, ExtendedFaces, History, Vec<Shape>) {
    let fixture = gap_incidence_fixture(1);
    let mut gap = FillGap::new(&fixture.feature, parallel);
    let mut sources: Vec<_> = trim_source_indices(profile)
        .iter()
        .map(|index| fixture.faces[*index].clone())
        .collect();
    let mut adjacent = IndexedMapOfShape::default();
    for source in &sources {
        adjacent.add(source.clone());
    }
    let history = History::new();
    let mut extended = gap.extend_adjacent_faces(&adjacent, &history).unwrap();
    if profile == 4 || profile == 5 {
        gap.feature = compound_of(&[]).unwrap();
    }
    if profile == 6 {
        replace_original_with_vertex(&mut extended, &mut sources);
    }
    (gap, extended, history, sources)
}
#[test]
fn nested_pulse_stays_cancelled_after_raw_face_reconstruction() {
    use crate::bop_algo::{
        ProgressCancellation, ProgressCompletion, ProgressOperation,
        progress_test_support::parent_census::parent_census_indicator,
    };
    let (mut gap, extended, history, _) = ranged_trimming_case(2, false);
    let (indicator, _) = parent_census_indicator(1);
    let observed = ProgressCancellation::default();
    let completion = gap
        .trim_extended_faces_on_progress(&extended, &history, &indicator.start(), &observed)
        .unwrap();
    assert!(matches!(
        completion,
        ProgressCompletion::Cancelled {
            operation: ProgressOperation::PaveFiller,
            ..
        }
    ));
    assert!(observed.is_cancelled());
    // A non-sticky pulse allows later raw work; the retained completion must still stop publication.
    assert_eq!(gap.faces.extent(), 2);
}
#[test]
fn invalid_trimming_inputs_return_owned_typed_failures() {
    use crate::bop_algo::ProgressCancellation;
    use martensite_tkernel::message::ProgressRange;
    for (profile, expected) in [(0, "Standard_NoSuchObject"), (6, "Standard_TypeMismatch")] {
        let (mut gap, extended, history, _) = ranged_trimming_case(profile, false);
        let outcome = gap.trim_extended_faces_on_progress(
            &extended,
            &history,
            &ProgressRange::default(),
            &ProgressCancellation::default(),
        );
        let cause = match outcome {
            Err(cause) => cause,
            Ok(status) => panic!("profile={profile} status={status:?}; expected {expected}"),
        };
        assert_eq!(cause.exception_type(), expected);
        assert_eq!(gap.faces.extent(), 0);
    }
}

#[path = "trimming_census.rs"]
mod census;
