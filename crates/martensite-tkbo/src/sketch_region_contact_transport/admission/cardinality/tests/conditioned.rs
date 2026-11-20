// SPDX-License-Identifier: GPL-2.0-only
//! A complete excluded report still derives native observations before comparing zero.
use super::super::super::super::general_tests::exclusion_bridge;
use super::*;

#[test]
fn conditioned_empty_report_preserves_native_extra_vertex_gap_and_literal_report_order() {
    let retained = exclusion_bridge(degenerate_fixture(15));
    assert!(
        compare(&retained)
            .iter()
            .any(|(_, expected, observed)| *expected == 0 && *observed > 0)
    );
    assert!(retained.census.paired.as_ref().unwrap().reports.is_empty());
}
#[test]
fn conditioned_source_domain_mutations_keep_literal_cardinality_gaps() {
    for missing in [false, true] {
        let mut retained = exclusion_bridge(degenerate_fixture(15));
        if missing {
            retained.current_partition = None;
        } else {
            reject_source_domains(&mut retained);
        }
        compare(&retained);
    }
}
fn reject_source_domains(retained: &mut PairedContactTransaction) {
    let current = retained
        .current_partition
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for row in &mut current.domains {
        row.decision = Err(crate::sketch_region_partition_bindings::PartitionSourceDomainCause::CopiedSourceChanged);
    }
}
#[test]
fn conditioned_empty_cardinality_discovers_native_endpoints_once_without_cartesian_rows() {
    let retained = exclusion_bridge(degenerate_fixture(15));
    let index = super::super::super::super::topology::compile_source_index(&retained);
    let (mut endpoints, mut products) = (0, 0);
    let gaps =
        report_cardinality_gaps_observed(&retained, None, index.as_ref(), &mut |step| match step {
            DiscoveryStep::Endpoint => endpoints += 1,
            DiscoveryStep::ParameterProduct => products += 1,
            _ => {}
        });
    assert_eq!(result(gaps), literal(&retained));
    assert_eq!(endpoints, endpoint_count(&retained));
    assert!(endpoints > 0);
    assert_eq!(products, 0);
}
