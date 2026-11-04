use super::*;
fn observed(triangles: usize) -> Vec<u8> {
    let mut bytes = vec![0; 16 + triangles * 8];
    write_word(&mut bytes, 0, triangles as u32);
    write_word(
        &mut bytes,
        1,
        super::super::window_depth_status_protocol::SCHEMA,
    );
    write_word(&mut bytes, 2, triangles as u32);
    bytes.extend(initial(triangles).unwrap());
    bytes
}
fn write_word(bytes: &mut [u8], word: usize, value: u32) {
    bytes[word * 4..word * 4 + 4].copy_from_slice(&value.to_ne_bytes());
}
fn failure(bytes: &mut [u8], triangle: usize, triangles: usize, key: u32) {
    let base = 4 + triangles * 2 + triangle * STRIDE;
    write_word(bytes, base, key);
    write_word(bytes, base + 1, MAGIC);
    write_word(bytes, 4 + triangle * 2 + 1, 1);
}
#[test]
fn bounded_initialization_uses_one_sentinel_per_primitive_and_rejects_excess() {
    let words = record_words(&initial(2).unwrap(), 2).unwrap();
    assert_eq!(words[0][0], u32::MAX);
    assert!(words[0][1..].iter().all(|&w| w == 0));
    assert!(initial(TRIANGLE_LIMIT + 1).is_err());
    assert!(record_words(&[0; 3], 1).is_err());
}
#[test]
fn deterministic_order_and_minimum_key_do_not_depend_on_arrival_order() {
    let mut bytes = observed(2);
    failure(&mut bytes, 1, 2, 50);
    failure(&mut bytes, 0, 2, 20);
    let report = report(101, &bytes, 32, &[3, 4, 5, 8, 9, 10]).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&report).unwrap();
    assert_eq!(parsed["records"][0]["primitive"], 0);
    assert_eq!(
        parsed["records"][0]["source_indices"],
        serde_json::json!([3, 4, 5])
    );
    assert_eq!(parsed["records"][1]["sample_key"], 50);
}
#[test]
fn missing_or_duplicate_gs_payload_and_out_of_domain_sample_keys_refuse() {
    for (magic, count, key) in [(0, 1, 3), (MAGIC, 2, 3), (MAGIC, 1, 0x8000_0000)] {
        let mut bytes = observed(1);
        failure(&mut bytes, 0, 1, key);
        write_word(&mut bytes, 7, magic);
        write_word(&mut bytes, 5, count);
        assert!(report(0, &bytes, 24, &[0, 1, 2]).is_err());
    }
}
#[test]
fn immutable_tail_mutation_or_truncated_storage_prevents_diagnostic_attribution() {
    let tail: Vec<u8> = [1u32, 2, 3, 64]
        .into_iter()
        .flat_map(u32::to_ne_bytes)
        .collect();
    let mut bytes = observed(1);
    bytes.splice(24..24, tail.iter().copied());
    assert!(checked_report(0, &bytes, 24, &tail, &[0, 1, 2]).is_ok());
    bytes[24] ^= 1;
    assert!(checked_report(0, &bytes, 24, &tail, &[0, 1, 2]).is_err());
    assert!(checked_report(0, &bytes[..20], 24, &tail, &[0, 1, 2]).is_err());
}
#[test]
fn report_cap_keeps_total_failure_count_and_lowest_primitive_ids_explicit() {
    let mut bytes = observed(33);
    let indices: Vec<u32> = (0..99).collect();
    for triangle in (0..33).rev() {
        failure(&mut bytes, triangle, 33, triangle as u32);
    }
    let value: serde_json::Value =
        serde_json::from_str(&report(9, &bytes, 280, &indices).unwrap()).unwrap();
    assert_eq!(value["failed_primitives"], 33);
    assert_eq!(value["records"].as_array().unwrap().len(), 32);
    assert_eq!(value["records"][31]["primitive"], 31);
}
#[test]
fn packed_key_roundtrips_pixel_sample_id_and_exact_observed_offsets() {
    let (x, y, sx, sy, sample) = (523u32, 450u32, 9u32, 5u32, 7u32);
    let key = ((((y * 1024 + x) * 16 + sy) * 16 + sx) * 8) + sample;
    assert_eq!(
        [
            key / 2048 % 1024,
            key / 2097152,
            key / 8 % 16,
            key / 128 % 16,
            key % 8
        ],
        [x, y, sx, sy, sample]
    );
    assert_eq!((0..8).map(|s| key - 7 + s).min(), Some(key - 7));
}

#[test]
fn malformed_header_and_reserved_payload_never_produce_an_attributed_record() {
    let mut bytes = observed(1);
    failure(&mut bytes, 0, 1, 3);
    write_word(&mut bytes, 2, 2);
    assert!(report(0, &bytes, 24, &[0, 1, 2]).is_err());
    write_word(&mut bytes, 2, 1);
    write_word(&mut bytes, 6 + 62, 1);
    assert!(report(0, &bytes, 24, &[0, 1, 2]).is_err());
}
