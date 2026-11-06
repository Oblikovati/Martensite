//! `StepTidyVectorReducer`.
use crate::step_tidy_fixture::{Fixture, ORIGIN, ReplacedEntities, name};
use martensite_tkdestep::step_geom::{StepGeomCartesianPoint, StepGeomLine, StepGeomVector};
use martensite_tkdestep::step_tidy::StepTidyVectorReducer;
use martensite_tkernel::standard::Handle;

fn replaced(fixture: &Fixture) -> ReplacedEntities {
    fixture.replaced_by(&mut StepTidyVectorReducer::new(&fixture.ws))
}

fn add_line_on(
    fixture: &Fixture,
    point: &Handle<StepGeomCartesianPoint>,
    vector: &Handle<StepGeomVector>,
) {
    let line = Handle::new(StepGeomLine::new());
    line.init(&name(""), point, vector);
    fixture.add(&line);
}

#[test]
fn different_names() {
    let fixture = Fixture::new();
    let (first, second) = (fixture.add_vector("vec1"), fixture.add_vector("vec2"));
    let point = fixture.add_cartesian_point("", ORIGIN);
    add_line_on(&fixture, &point, &first);
    add_line_on(&fixture, &point, &second);
    replaced(&fixture).assert_empty();
}

#[test]
fn step_geom_line() {
    let fixture = Fixture::new();
    let (first, second) = (fixture.add_vector(""), fixture.add_vector(""));
    let point = fixture.add_cartesian_point("", ORIGIN);
    add_line_on(&fixture, &point, &first);
    add_line_on(&fixture, &point, &second);
    replaced(&fixture).assert_one_of(&first, &second);
}
