use super::*;
use martensite_tkernel::tcollection::AsciiString;
#[test]
fn unsupported_budget_preserves_capacity_and_explicit_unknowns() {
    let snapshot = report(&capacity_provider()).unwrap();
    assert_eq!(snapshot.source(), RenderMemorySource::VulkanCapacityOnly);
    assert_eq!(snapshot.heaps()[0].index(), 0);
    assert_eq!(snapshot.heaps()[0].raw_flags(), 3);
    assert_eq!(snapshot.device_local_capacity_bytes(), 4096);
    assert_eq!(
        snapshot.device_local_budget(),
        RenderMemoryBudget::Unavailable
    );
    assert_eq!(snapshot.global_free_bytes(), None);
    assert_eq!(snapshot.device().unwrap().vendor_id, 7);
}
#[test]
fn local_heap_totals_exclude_host_heaps_and_never_become_global_free() {
    let snapshot = report(&mixed_budget_provider()).unwrap();
    let indices: Vec<u32> = snapshot.heaps().iter().map(|heap| heap.index()).collect();
    assert_eq!(indices, [0, 1, 2]);
    assert_local_totals(&snapshot);
    assert_legacy_budget_output(snapshot);
}
fn assert_local_totals(snapshot: &RenderDeviceMemoryReport) {
    let expected = RenderMemoryBudget::Estimated {
        budget_bytes: 1536,
        usage_bytes: 120,
    };
    assert_eq!(snapshot.device_local_capacity_bytes(), 3072);
    assert_eq!(snapshot.device_local_budget(), expected);
    assert_eq!(
        snapshot.device_local_budget().remaining_estimate_bytes(),
        Some(1416)
    );
    assert_eq!(snapshot.global_free_bytes(), None);
}
fn assert_legacy_budget_output(snapshot: RenderDeviceMemoryReport) {
    let (mut free, mut text) = (0xA55, AsciiString::from("sentinel"));
    assert!(super::super::legacy::write_result(
        Ok(snapshot),
        &mut free,
        &mut text
    ));
    assert_eq!(free, 0);
    let description = std::str::from_utf8(text.as_bytes()).unwrap();
    assert!(description.contains("process budget estimate"));
    assert!(description.contains("actual global free memory: unknown"));
}
#[test]
fn over_budget_usage_is_a_valid_estimate_with_zero_advisory_headroom() {
    let mut provider = mixed_budget_provider();
    provider.result.as_mut().unwrap().heaps[0].usage = 9999;
    let snapshot = report(&provider).unwrap();
    assert_eq!(
        snapshot.heaps()[0].budget().remaining_estimate_bytes(),
        Some(0)
    );
    assert_eq!(
        snapshot.heaps()[0].budget(),
        RenderMemoryBudget::Estimated {
            budget_bytes: 512,
            usage_bytes: 9999
        }
    );
    assert_eq!(snapshot.global_free_bytes(), None);
}
#[test]
fn owned_sample_survives_provider_mutation_drop_and_thread_transfer() {
    assert_report_thread_contract();
    let snapshot = {
        let mut provider = capacity_provider();
        let snapshot = report(&provider).unwrap();
        provider.result.as_mut().unwrap().heaps[0].capacity = 9999;
        assert_eq!(provider.result.as_ref().unwrap().heaps[0].capacity, 9999);
        snapshot
    };
    let recovered = std::thread::spawn(move || snapshot).join().unwrap();
    assert_eq!(recovered.heaps()[0].capacity_bytes(), 4096);
    assert_eq!(recovered.device().unwrap().device_name, "fixed heap device");
}
#[test]
fn injected_and_failed_queries_preserve_both_legacy_outputs() {
    for result in [
        Ok(RenderDeviceMemoryReport::unavailable()),
        Err(RenderMemoryQueryError::ExtensionEnumeration { result: -1 }),
    ] {
        let (mut free, mut text) = (0xA55, AsciiString::from("sentinel"));
        assert!(!super::super::legacy::write_result(
            result, &mut free, &mut text
        ));
        assert_eq!(free, 0xA55);
        assert_eq!(text, AsciiString::from("sentinel"));
    }
    assert_eq!(
        RenderMemoryBudget::Unavailable.remaining_estimate_bytes(),
        None
    );
}
