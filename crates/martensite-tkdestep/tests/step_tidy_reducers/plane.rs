//! `StepTidyPlaneReducer`.
use crate::step_tidy_fixture::{Fixture, ReplacedEntities, name};
use martensite_tkdestep::step_geom::{StepGeomPcurve, StepGeomPlane};
use martensite_tkdestep::step_repr::StepReprDefinitionalRepresentation;
use martensite_tkdestep::step_shape::{StepShapeAdvancedFace, StepShapeFaceBound};
use martensite_tkdestep::step_tidy::StepTidyPlaneReducer;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::Handle;

fn replaced(fixture: &Fixture) -> ReplacedEntities {
    fixture.replaced_by(&mut StepTidyPlaneReducer::new(&fixture.ws))
}

fn add_face_on(fixture: &Fixture, plane: &Handle<StepGeomPlane>) {
    let face = Handle::new(StepShapeAdvancedFace::new());
    let bounds = HArray1::<Handle<StepShapeFaceBound>>::default();
    face.init(&name(""), &Handle::new(bounds), &plane.cast(), true);
    fixture.add(&face);
}

fn add_pcurve_on(fixture: &Fixture, plane: &Handle<StepGeomPlane>) {
    let pcurve = Handle::new(StepGeomPcurve::new());
    let reference = Handle::new(StepReprDefinitionalRepresentation::new());
    pcurve.init(&name(""), &plane.cast(), &reference);
    fixture.add(&pcurve);
}

fn assert_merged_in(share: fn(&Fixture, &Handle<StepGeomPlane>)) {
    let fixture = Fixture::new();
    let (first, second) = (fixture.add_plane(""), fixture.add_plane(""));
    share(&fixture, &first);
    share(&fixture, &second);
    replaced(&fixture).assert_one_of(&first, &second);
}

#[test]
fn different_names() {
    let fixture = Fixture::new();
    let (first, second) = (fixture.add_plane("Plane1"), fixture.add_plane("Plane2"));
    add_face_on(&fixture, &first);
    add_face_on(&fixture, &second);
    replaced(&fixture).assert_empty();
}

#[test]
fn step_shape_advanced_face() {
    assert_merged_in(add_face_on);
}

#[test]
fn step_geom_pcurve() {
    assert_merged_in(add_pcurve_on);
}
