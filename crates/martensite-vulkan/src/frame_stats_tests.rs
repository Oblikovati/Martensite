//! Counting flags derive from the collection bits exactly as native updateStatistics.
use super::*;

#[test]
fn memory_alone_still_walks_elements_groups_and_structures() {
    let flags = CountingFlags::new(Perf::ESTIM_MEM);
    assert!(flags.memory && flags.elements && flags.groups && flags.structures);
    assert!(!flags.detailed);
}

#[test]
fn any_primitive_count_requests_detailed_elements() {
    for bits in [Perf::TRIANGLES, Perf::LINES, Perf::POINTS] {
        let flags = CountingFlags::new(bits);
        assert!(flags.detailed && flags.elements, "bits {bits:?}");
    }
}

#[test]
fn layer_bits_count_structures_without_groups() {
    let flags = CountingFlags::new(Perf::LAYERS);
    assert!(flags.structures && !flags.groups);
}

#[test]
fn statistics_without_a_render_view_store_and_reset_the_active_frame() {
    let stats = RenderFrameStats::new();
    stats.change_counter(Counter::NbStructs, 7);
    stats.update_statistics(&Handle::default(), false);
    assert_eq!(stats.counter_value(Counter::NbStructs), 7);
    stats.active_data_frame(&mut |tmp| assert_eq!(tmp[Counter::NbStructs], 0));
}
