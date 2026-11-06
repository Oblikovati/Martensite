use super::*;
fn record(index: u32, value: u32) -> Vec<u8> {
    [value, 0, 0, 0, 0, 0, 0, index]
        .into_iter()
        .flat_map(u32::to_ne_bytes)
        .collect()
}
#[test]
fn cyclic_primitive_order_uses_direct_identity_and_retains_all_records() {
    let request = VertexOutputRequest::new(vec![0, 1, 2, 0, 2, 3], 192).unwrap();
    let bytes = [1, 2, 0, 2, 3, 0]
        .into_iter()
        .flat_map(|i| record(i, i))
        .collect::<Vec<_>>();
    let outputs = request.decode(&bytes).unwrap();
    assert_eq!(outputs.len(), 6);
    assert_eq!(
        outputs.iter().map(|r| r.vertex_index).collect::<Vec<_>>(),
        [1, 2, 0, 2, 3, 0]
    );
}
#[test]
fn repeated_id_conflict_is_incomplete_not_last_record_wins() {
    let request = VertexOutputRequest::new(vec![0, 0, 1], 96).unwrap();
    let bytes = [record(0, 1), record(0, 2), record(1, 3)].concat();
    assert_eq!(
        request.decode(&bytes),
        Err(VertexOutputFailure::ConflictingVertex { index: 0 })
    );
}
#[test]
fn overflow_and_missing_count_never_pass() {
    let request = VertexOutputRequest::new(vec![0, 1, 2], 96).unwrap();
    assert!(request.counts(1, 2).is_err());
    assert!(request.counts(0, 1).is_err());
    assert!(request.decode(&[0; 64]).is_err());
    assert!(VertexOutputRequest::new(vec![0, 1, 2], 95).is_err());
}
#[test]
fn unknown_id_and_changed_multiplicity_are_refused() {
    let request = VertexOutputRequest::new(vec![0, 0, 1], 96).unwrap();
    assert!(matches!(
        request.decode(&[record(0, 1), record(1, 2), record(2, 3)].concat()),
        Err(VertexOutputFailure::UnexpectedVertex { index: 2 })
    ));
    assert!(
        request
            .decode(&[record(0, 1), record(1, 2), record(1, 2)].concat())
            .is_err()
    );
}
#[test]
fn record_bits_preserve_negative_zero_and_nan_payload() {
    let bytes = record(0, 0x80000000);
    let value = VertexOutputRecord::decode(&bytes);
    assert_eq!(value.clip_bits[0], 0x80000000);
    let value = VertexOutputRecord::decode(&record(7, 0x7fc00013));
    assert_eq!(value.clip_bits[0], 0x7fc00013);
    assert_eq!(value.vertex_index, 7);
}

#[test]
fn original_uploaded_ebo_must_match_request_without_identity_reconstruction() {
    let request = VertexOutputRequest::new(vec![0, 1, 2], 96).unwrap();
    let bytes = [0u32, 1, 2]
        .into_iter()
        .flat_map(u32::to_ne_bytes)
        .collect::<Vec<_>>();
    assert!(request.require_actual_indices(&bytes).is_ok());
    assert!(request.require_actual_indices(&bytes[..8]).is_err());
    let changed = [1u32, 2, 0]
        .into_iter()
        .flat_map(u32::to_ne_bytes)
        .collect::<Vec<_>>();
    assert!(request.require_actual_indices(&changed).is_err());
}

#[test]
fn vertex_output_request_completion_requires_a_selected_success_across_clones() {
    let request = VertexOutputRequest::new(vec![0, 1, 2], 96).unwrap();
    assert!(request.finish().is_err());
    request.clone().record_capture();
    assert!(request.finish().is_ok());
}
