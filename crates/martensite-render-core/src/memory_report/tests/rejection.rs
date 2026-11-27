use super::*;
#[test]
fn driver_extension_error_retains_its_typed_result() {
    let provider = FixedMemoryQuery {
        result: Err(RenderMemoryQueryError::ExtensionEnumeration { result: -9 }),
    };
    assert_eq!(
        report(&provider).unwrap_err(),
        RenderMemoryQueryError::ExtensionEnumeration { result: -9 }
    );
    assert!(report(&provider).unwrap_err().to_string().contains("-9"));
}
#[test]
fn malformed_heap_counts_fail_without_indexing_arrays() {
    for received in [0, MEMORY_HEAP_LIMIT as u32 + 1] {
        let mut provider = capacity_provider();
        provider.result.as_mut().unwrap().heap_count = received;
        assert!(
            report(&provider)
                .unwrap_err()
                .to_string()
                .contains("expected 1..=")
        );
        assert_eq!(
            report(&provider).unwrap_err(),
            RenderMemoryQueryError::HeapCount {
                received,
                maximum: MEMORY_HEAP_LIMIT as u32
            }
        );
    }
}
#[test]
fn supported_budget_must_be_nonzero_and_at_most_physical_capacity() {
    for budget in [0, 1025] {
        let mut provider = mixed_budget_provider();
        provider.result.as_mut().unwrap().heaps[0].budget = budget;
        assert!(
            report(&provider)
                .unwrap_err()
                .to_string()
                .contains("budget <= capacity")
        );
        assert_eq!(
            report(&provider).unwrap_err(),
            RenderMemoryQueryError::HeapBudget {
                heap: 0,
                budget,
                capacity: 1024
            }
        );
    }
}
#[test]
fn unused_budget_array_values_are_rejected_instead_of_hidden() {
    let mut provider = mixed_budget_provider();
    provider.result.as_mut().unwrap().heaps[3].usage = 7;
    assert!(
        report(&provider)
            .unwrap_err()
            .to_string()
            .contains("expected both zero")
    );
    assert_eq!(
        report(&provider).unwrap_err(),
        RenderMemoryQueryError::UnusedHeapBudget {
            heap: 3,
            budget: 0,
            usage: 7
        }
    );
}
#[test]
fn checked_capacity_total_retains_both_overflow_operands() {
    let mut capacity = mixed_budget_provider();
    capacity.result.as_mut().unwrap().budget_supported = false;
    capacity.result.as_mut().unwrap().heaps[0].capacity = u64::MAX;
    assert_eq!(
        report(&capacity).unwrap_err(),
        RenderMemoryQueryError::DeviceLocalTotalOverflow {
            quantity: RenderMemoryQuantity::Capacity,
            left: u64::MAX,
            right: 2048,
        }
    );
}
#[test]
fn checked_usage_total_retains_both_overflow_operands() {
    let mut usage = mixed_budget_provider();
    usage.result.as_mut().unwrap().heaps[0].usage = u64::MAX;
    assert_eq!(
        report(&usage).unwrap_err(),
        RenderMemoryQueryError::DeviceLocalTotalOverflow {
            quantity: RenderMemoryQuantity::Usage,
            left: u64::MAX,
            right: 20,
        }
    );
}
#[test]
fn aggregate_error_description_preserves_quantity_and_expected_shape() {
    let error = RenderMemoryQueryError::DeviceLocalTotalOverflow {
        quantity: RenderMemoryQuantity::Budget,
        left: u64::MAX,
        right: 17,
    };
    assert!(error.to_string().contains("Budget"));
    assert!(error.to_string().contains("expected a u64"));
    assert!(error.to_string().contains(&format!("{} + 17", u64::MAX)));
}
