//! The exclusive phase clock and the counters of `FrameWorkLedger`, at explicit instants.
use super::{DeviceCount, FrameWorkLedger, PhaseAccounts, WorkPhase};
use crate::backend::VulkanWorkLedgerExt;
use ash::vk;
use std::time::Duration;

fn ms(value: u64) -> Duration {
    Duration::from_millis(value)
}

#[test]
fn one_phase_is_charged_from_entry_to_exit() {
    let mut accounts = PhaseAccounts::default();
    accounts.enter(WorkPhase::Upload, ms(10));
    accounts.leave(ms(25));
    assert_eq!(accounts.times(ms(40)).upload, ms(15));
    assert_eq!(accounts.times(ms(40)).redraw, ms(40));
}

#[test]
fn a_nested_phase_pauses_the_phase_it_interrupts() {
    let mut accounts = PhaseAccounts::default();
    accounts.enter(WorkPhase::Upload, ms(0));
    accounts.enter(WorkPhase::Waiting, ms(4));
    accounts.leave(ms(10));
    accounts.leave(ms(12));
    let times = accounts.times(ms(12));
    assert_eq!(times.upload, ms(6), "4 ms before and 2 ms after the wait");
    assert_eq!(times.waiting, ms(6));
}

#[test]
fn reentering_the_open_phase_counts_each_instant_once() {
    let mut accounts = PhaseAccounts::default();
    accounts.enter(WorkPhase::Recording, ms(1));
    accounts.enter(WorkPhase::Recording, ms(3));
    accounts.leave(ms(8));
    accounts.leave(ms(9));
    assert_eq!(accounts.times(ms(9)).recording, ms(8));
}

#[test]
fn time_outside_every_phase_is_charged_to_none() {
    let mut accounts = PhaseAccounts::default();
    accounts.enter(WorkPhase::Scene, ms(2));
    accounts.leave(ms(5));
    accounts.leave(ms(50));
    accounts.enter(WorkPhase::Readback, ms(60));
    accounts.leave(ms(61));
    let times = accounts.times(ms(70));
    assert_eq!((times.scene, times.readback), (ms(3), ms(1)));
    assert_eq!(
        times.upload + times.recording + times.submission + times.waiting,
        ms(0)
    );
}

#[test]
fn counts_and_redraws_accumulate_into_the_totals() {
    let ledger = FrameWorkLedger::default();
    ledger.add(DeviceCount::Buffers, 2);
    ledger.add(DeviceCount::UploadedBytes, 96);
    ledger.add(DeviceCount::UploadedBytes, 32);
    let layouts = [vk::DescriptorSetLayout::null(); 3];
    ledger.add_sets(&vk::DescriptorSetAllocateInfo::default().set_layouts(&layouts));
    ledger.finish_redraw(ms(7));
    let totals = ledger.totals();
    assert_eq!((totals.buffers_created, totals.uploaded_bytes), (2, 128));
    assert_eq!(totals.descriptor_sets_allocated, 3);
    assert_eq!((totals.redraws, totals.times.redraw), (1, ms(7)));
    assert_eq!(totals.images_created + totals.read_back_bytes, 0);
}

#[test]
fn a_guard_closes_its_phase_when_dropped() {
    let ledger = FrameWorkLedger::default();
    drop(ledger.enter(WorkPhase::Submission));
    let _scene = ledger.enter(WorkPhase::Scene);
    assert!(
        ledger.open_phase_count() == 1,
        "only the scene phase stays open"
    );
}
