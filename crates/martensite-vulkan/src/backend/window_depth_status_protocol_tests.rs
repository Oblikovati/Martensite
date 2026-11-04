use super::*;
struct FencedStatusReader {
    bytes: Vec<u8>,
}
impl FencedStatusReader {
    fn words(words: &[u32]) -> Self {
        Self {
            bytes: words.iter().flat_map(|w| w.to_ne_bytes()).collect(),
        }
    }
}
impl StatusReader for FencedStatusReader {
    fn read_status(&self, _: usize) -> Result<Vec<u8>, RenderError> {
        Ok(self.bytes.clone())
    }
}
#[test]
fn every_actual_primitive_including_shared_and_empty_carriers_requires_completion() {
    let indices = [0, 1, 2, 0, 0, 0];
    let words = [2, SCHEMA, 2, 0, DONE, 1, DONE | EMPTY, 1];
    assert!(validate(&FencedStatusReader::words(&words), &indices, 2).is_ok());
    let mut missing = words;
    missing[6] = 0;
    assert!(matches!(
        validate(&FencedStatusReader::words(&missing), &indices, 2)
            .unwrap_err()
            .window_depth_failure(),
        Some(Failure::GpuTriangle {
            triangle: 1,
            vertex_indices: [0, 0, 0],
            flags: 0
        })
    ));
}
#[test]
fn pending_failed_unknown_and_ambiguous_triangle_results_never_become_success() {
    for flag in [
        0,
        EMPTY,
        DONE | 4,
        DONE | 8,
        DONE | 16,
        DONE | 32,
        DONE | 64,
        DONE | 128,
    ] {
        assert!(
            validate(
                &FencedStatusReader::words(&[1, SCHEMA, 1, 0, flag, 1]),
                &[7, 8, 9],
                1
            )
            .is_err()
        );
    }
    assert_eq!(KNOWN, 127);
}
#[test]
fn aggregate_count_cannot_hide_missing_or_duplicate_per_primitive_invocations() {
    let indices = [0, 1, 2, 0, 0, 0];
    assert!(
        validate(
            &FencedStatusReader::words(&[2, SCHEMA, 4, 0, DONE, 3, DONE, 1]),
            &indices,
            4
        )
        .is_err()
    );
    assert!(
        validate(
            &FencedStatusReader::words(&[2, SCHEMA, 4, 0, DONE, 2, DONE, 2]),
            &indices,
            4
        )
        .is_ok()
    );
}
#[test]
fn exact_input_count_schema_invocation_total_and_readback_span_are_mandatory() {
    for words in [
        vec![2, SCHEMA, 1, 0, DONE, 1],
        vec![1, 0, 1, 0, DONE, 1],
        vec![1, SCHEMA, 0, 0, DONE, 1],
        vec![1, SCHEMA, 2, 0, DONE, 1],
        vec![1, SCHEMA, 1, 1, DONE, 1],
        vec![1, SCHEMA, 1, 0, DONE],
    ] {
        assert!(validate(&FencedStatusReader::words(&words), &[0, 1, 2], 1).is_err());
    }
    assert!(
        validate(
            &FencedStatusReader::words(&[1, SCHEMA, 0, 0, DONE, 1]),
            &[0, 1, 2],
            u64::from(u32::MAX) + 1
        )
        .is_err()
    );
}
#[test]
fn initialized_pending_is_incomplete_and_completed_empty_operation_is_distinct() {
    assert!(
        validate(
            &FencedStatusReader::words(&initial_words(1).unwrap()),
            &[0, 1, 2],
            1
        )
        .is_err()
    );
    assert!(
        validate(
            &FencedStatusReader::words(&initial_words(0).unwrap()),
            &[],
            0
        )
        .is_ok()
    );
}
#[test]
fn malformed_source_membership_and_unrepresentable_triangle_counts_refuse_before_allocation() {
    assert!(validate(&FencedStatusReader::words(&[0, SCHEMA, 0, 0]), &[0, 1], 0).is_err());
    assert!(initial_words(usize::MAX).is_err());
}
struct MissingStatusReadback;
impl StatusReader for MissingStatusReadback {
    fn read_status(&self, size: usize) -> Result<Vec<u8>, RenderError> {
        Err(Failure::GpuReadback {
            expected_bytes: size,
            observed_bytes: 0,
        }
        .into())
    }
}
#[test]
fn readback_failure_is_fatal_instead_of_an_empty_successful_result() {
    assert!(matches!(
        validate(&MissingStatusReadback, &[0, 1, 2], 1)
            .unwrap_err()
            .window_depth_failure(),
        Some(Failure::GpuReadback {
            expected_bytes: 24,
            observed_bytes: 0
        })
    ));
}

#[test]
fn no_dispatch_cannot_borrow_done_flags_and_empty_membership_cannot_claim_an_invocation() {
    assert!(
        validate(
            &FencedStatusReader::words(&[1, SCHEMA, 0, 0, DONE, 0]),
            &[0, 1, 2],
            0
        )
        .is_err()
    );
    assert!(validate(&FencedStatusReader::words(&[0, SCHEMA, 1, 0]), &[], 1).is_err());
}
