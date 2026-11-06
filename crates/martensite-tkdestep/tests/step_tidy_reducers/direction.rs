//! `StepTidyDirectionReducer`.
use crate::step_tidy_fixture::{Fixture, ORIGIN, ReplacedEntities, Z_AXIS, name};
use martensite_tkdestep::step_geom::{
    StepGeomAxis1Placement, StepGeomAxis2Placement3d, StepGeomCartesianPoint, StepGeomDirection,
    StepGeomVector,
};
use martensite_tkdestep::step_tidy::StepTidyDirectionReducer;
use martensite_tkernel::standard::Handle;

type Direction = Handle<StepGeomDirection>;

fn replaced(fixture: &Fixture) -> ReplacedEntities {
    fixture.replaced_by(&mut StepTidyDirectionReducer::new(&fixture.ws))
}

fn add_vector_on(fixture: &Fixture, direction: &Direction) {
    let vector = Handle::new(StepGeomVector::new());
    vector.init(&name(""), direction, 1.0);
    fixture.add(&vector);
}

fn assert_merged_with_location(share: fn(&Fixture, &Handle<StepGeomCartesianPoint>, &Direction)) {
    let fixture = Fixture::new();
    let first = fixture.add_direction("", Z_AXIS);
    let second = fixture.add_direction("", Z_AXIS);
    let location = fixture.add_cartesian_point("", ORIGIN);
    share(&fixture, &location, &first);
    share(&fixture, &location, &second);
    replaced(&fixture).assert_one_of(&first, &second);
}

#[test]
fn different_names() {
    let fixture = Fixture::new();
    let (first, second) = (
        fixture.add_direction("dir1", Z_AXIS),
        fixture.add_direction("dir2", Z_AXIS),
    );
    add_vector_on(&fixture, &first);
    add_vector_on(&fixture, &second);
    replaced(&fixture).assert_empty();
}

#[test]
fn step_geom_axis1_placement() {
    assert_merged_with_location(|fixture, location, direction| {
        let axis = Handle::new(StepGeomAxis1Placement::new());
        axis.init(&name(""), location, true, direction);
        fixture.add(&axis);
    });
}

#[test]
fn step_geom_axis2_placement() {
    assert_merged_with_location(|fixture, location, direction| {
        let axis = Handle::new(StepGeomAxis2Placement3d::new());
        axis.init(
            &name(""),
            location,
            true,
            direction,
            false,
            &Handle::default(),
        );
        fixture.add(&axis);
    });
}

#[test]
fn step_geom_vector() {
    let fixture = Fixture::new();
    let (first, second) = (
        fixture.add_direction("", Z_AXIS),
        fixture.add_direction("", Z_AXIS),
    );
    add_vector_on(&fixture, &first);
    add_vector_on(&fixture, &second);
    replaced(&fixture).assert_one_of(&first, &second);
}
