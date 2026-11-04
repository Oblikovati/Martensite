//! Differential entity selection uses actual selecting volumes and retained lifecycle state.
use super::Select3DSensitiveEntityMembers;
use super::quadric_reference_geometry::quadric_entity;
use super::quadric_reference_snapshot::{quadric_seed, quadric_snapshot, reset_quadric_detection};
use super::quadric_reference_volume::quadric_volume;
use super::triangulation_reference_snapshot::compare_reference_snapshot;
use crate::select_mgr::SelectMgrEntityOwner;

fn quadric_scenario(ordinal: i32, native: &mut std::str::Lines<'_>) {
    let owner = SelectMgrEntityOwner::new(13).cast();
    let original = quadric_entity(ordinal / 36, &owner);
    original.get().unwrap().set_sensitivity_factor(9).unwrap();
    let connected = original.get().unwrap().get_connected().unwrap();
    for step in 0..5 {
        let probe = if step == 3 { &connected } else { &original };
        let mut pick = quadric_seed();
        let (failure, hit) = match_quadric_step(probe, ordinal, step, &mut pick);
        let actual = quadric_snapshot(probe, &owner, ordinal, step, failure, hit, &pick);
        compare_reference_snapshot(&actual, native.next().unwrap());
    }
}
fn match_quadric_step(
    probe: &martensite_tkernel::standard::Handle<dyn super::Select3DSensitiveEntity>,
    ordinal: i32,
    step: i32,
    pick: &mut crate::select_basics::SelectBasicsPickResult,
) -> (bool, i32) {
    if step == 4 {
        reset_quadric_detection(probe);
        return (false, -1);
    }
    let volume = quadric_volume(
        ordinal / 4 % 9,
        ordinal / 2 % 2 != 0,
        ordinal % 2 != 0,
        step == 1,
    );
    let outcome = probe.get().unwrap().matches(&volume, pick);
    (outcome.is_err(), outcome.map_or(-1, i32::from))
}
#[test]
fn native_quadric_picks_containment_connected_copies_and_resets() {
    let mut native = super::quadric_reference_fixtures::QUADRIC_NATIVE.lines();
    for ordinal in 0..828 {
        quadric_scenario(ordinal, &mut native);
    }
    assert!(native.next().is_none());
}
