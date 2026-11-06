//! `merger` on the fixture of `fixture`, and the cleaner on a real
//! transfer. The reducers themselves are in `step_tidy_reducers`.
#[path = "support/step_tidy_fixture.rs"]
mod step_tidy_fixture;

use martensite_tkdestep::step_control::StepControlWriter;
use martensite_tkdestep::step_geom::{StepGeomAxis1Placement, StepGeomDirection, StepGeomVector};
use martensite_tkdestep::step_tidy::StepTidyDuplicateCleaner;
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use step_tidy_fixture::{Fixture, ORIGIN, Z_AXIS, name};

fn perform_removal(fixture: &Fixture) {
    StepTidyDuplicateCleaner::new(&fixture.ws)
        .perform()
        .expect("perform");
}

#[test]
fn different_entities() {
    let fixture = Fixture::new();
    for direction in [
        fixture.add_direction("dir1", Z_AXIS),
        fixture.add_direction("dir2", Z_AXIS),
    ] {
        let vector = Handle::new(StepGeomVector::new());
        vector.init(&name(""), &direction, 1.0);
        fixture.add(&vector);
    }
    let before = fixture.count_of_kind(StepGeomDirection::static_type());
    perform_removal(&fixture);
    assert_eq!(before, 2);
    assert_eq!(
        fixture.count_of_kind(StepGeomDirection::static_type()),
        before
    );
}

#[test]
fn equal_entities() {
    let fixture = Fixture::new();
    let (first, second) = (
        fixture.add_direction("", Z_AXIS),
        fixture.add_direction("", Z_AXIS),
    );
    let location = fixture.add_cartesian_point("", ORIGIN);
    for direction in [first, second] {
        let axis = Handle::new(StepGeomAxis1Placement::new());
        axis.init(&name(""), &location, true, &direction);
        fixture.add(&axis);
    }
    let before = fixture.count_of_kind(StepGeomDirection::static_type());
    perform_removal(&fixture);
    assert_eq!(before, 2);
    assert_eq!(fixture.count_of_kind(StepGeomDirection::static_type()), 1);
}

#[test]
fn a_writer_cleans_a_model_it_has_not_transferred_to() {
    // an empty model has nothing to merge.
    StepControlWriter::new()
        .expect("writer")
        .clean_duplicate_entities()
        .expect("clean");
}
