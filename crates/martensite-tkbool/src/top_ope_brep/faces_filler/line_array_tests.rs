//! Indexed array fixtures distinguish candidate kind from retained conic bounds.
use super::*;
use crate::top_ope_brep::{TopOpeBRepFacesIntersector, TopOpeBRepLineInter};
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkgeomalgo::int_patch::{IntPatchGLine, IntPatchLineTransitions, IntPatchPoint};
use martensite_tkmath::gp::{Circle3, Plane3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
fn circle(parameters: [f64; 2]) -> TopOpeBRepLineInter {
    let source = IntPatchGLine::from_circle(
        &Circle3::default(),
        false,
        IntPatchLineTransitions::Undecided,
    );
    for parameter in parameters {
        let mut point = IntPatchPoint::new();
        point.set_parameter(parameter);
        source.add_vertex(&point).unwrap();
    }
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &Handle::new(source).cast(),
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    for index in 1..=2 {
        line.change_vpoint(index).unwrap().change_keep(true);
    }
    line.set_vp_bounds();
    line
}
fn array() -> Handle<HArray1<TopOpeBRepLineInter>> {
    let mut slots = Array1::with_bounds(1, 2).unwrap();
    slots.set_value(1, circle([-1.0, 1.0])).unwrap();
    slots.set_value(2, circle([0.0, 1e-5])).unwrap();
    Handle::new(HArray1::from_owned(slots))
}
fn bind_context<'a>(
    filler: &mut TopOpeBRepFacesFiller<'a>,
    pair: &'a mut TopOpeBRepFacesIntersector,
) {
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-2., 2., -2., 2.))
        .unwrap()
        .face()
        .unwrap();
    pair.force_tolerances(1e-4, 1e-8);
    filler
        .bind_context(
            face.as_ref(),
            face.as_ref(),
            pair,
            &Handle::new(TopOpeBRepDSHDataStructure::new()),
        )
        .unwrap();
}
#[test]
fn shared_raw_input_fails_without_relocking_a_held_guard() {
    let lines = array();
    let mut filler = TopOpeBRepFacesFiller::new();
    let mut pair = TopOpeBRepFacesIntersector::new();
    bind_context(&mut filler, &mut pair);
    filler.bind_array_line(lines.clone(), 1);
    let mut slots = TopOpeBRepFacesFillerArrayMut::new(&lines).unwrap();
    assert_eq!(
        filler
            .check_line(slots.change_value(1).unwrap())
            .unwrap_err()
            .exception_type(),
        "Standard_ProgramError"
    );
    assert!(filler.check_line(slots.change_value(2).unwrap()).is_err());
    assert!(filler.check_line_in_array(&mut slots, 1).unwrap());
}
#[test]
fn shared_distinct_slots_keep_native_retained_conic_bounds() {
    let lines = array();
    let mut filler = TopOpeBRepFacesFiller::new();
    let mut pair = TopOpeBRepFacesIntersector::new();
    bind_context(&mut filler, &mut pair);
    filler.bind_array_line(lines.clone(), 1);
    let mut slots = TopOpeBRepFacesFillerArrayMut::new(&lines).unwrap();
    assert!(filler.check_line_in_array(&mut slots, 2).unwrap());
    filler.bind_array_line(lines.clone(), 2);
    assert!(!filler.check_line_in_array(&mut slots, 1).unwrap());
}
#[test]
fn shared_slot_provenance_survives_storage_relocation_and_replacement() {
    let lines = array();
    let mut filler = TopOpeBRepFacesFiller::new();
    let mut pair = TopOpeBRepFacesIntersector::new();
    bind_context(&mut filler, &mut pair);
    filler.bind_array_line(lines.clone(), 1);
    let mut slots = TopOpeBRepFacesFillerArrayMut::new(&lines).unwrap();
    let moved = std::mem::take(slots.change_value(1).unwrap());
    let mut replacement = Array1::with_bounds(1, 3).unwrap();
    replacement.set_value(1, moved).unwrap();
    *slots = replacement;
    assert!(filler.check_line_in_array(&mut slots, 1).unwrap());
    slots.set_value(1, circle([0.0, 1e-5])).unwrap();
    assert!(!filler.check_line_in_array(&mut slots, 1).unwrap());
}
#[test]
fn guarded_validation_rejects_wrong_owner_and_changed_bounds() {
    let lines = array();
    let other = array();
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_array_line(lines.clone(), 1);
    let mut wrong = TopOpeBRepFacesFillerArrayMut::new(&other).unwrap();
    assert_eq!(
        filler
            .check_line_in_array(&mut wrong, 1)
            .unwrap_err()
            .exception_type(),
        "Standard_DomainError"
    );
    let mut slots = TopOpeBRepFacesFillerArrayMut::new(&lines).unwrap();
    assert_eq!(
        filler
            .check_line_in_array(&mut slots, 0)
            .unwrap_err()
            .exception_type(),
        "Standard_OutOfRange"
    );
    *slots = Array1::with_bounds(2, 3).unwrap();
    assert_eq!(
        filler
            .check_line_in_array(&mut slots, 2)
            .unwrap_err()
            .exception_type(),
        "Standard_OutOfRange"
    );
}
#[test]
fn guarded_validation_requires_a_populated_owner_and_shared_binding() {
    assert!(TopOpeBRepFacesFillerArrayMut::new(&Handle::default()).is_err());
    let lines = array();
    let mut slots = TopOpeBRepFacesFillerArrayMut::new(&lines).unwrap();
    assert!(
        TopOpeBRepFacesFiller::new()
            .check_line_in_array(&mut slots, 1)
            .is_err()
    );
}
