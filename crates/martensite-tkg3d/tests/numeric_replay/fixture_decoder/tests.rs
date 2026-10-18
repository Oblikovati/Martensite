use super::FixtureReader;
const TWO_LINES: &[u8] = &[
    31, 139, 8, 0, 0, 0, 0, 0, 2, 3, 75, 203, 44, 42, 46, 225, 42, 78, 77, 206, 207, 75, 225, 2, 0,
    32, 81, 69, 8, 13, 0, 0, 0,
];

#[test]
fn fixture_decoder_preserves_line_endings_and_reports_end() {
    let mut decoded = FixtureReader::new(TWO_LINES);
    assert_eq!(decoded.next_line().unwrap().as_deref(), Some("first\n"));
    assert_eq!(decoded.next_line().unwrap().as_deref(), Some("second\n"));
    assert!(decoded.next_line().unwrap().is_none());
}

#[test]
fn fixture_decoder_rejects_bad_headers_and_damaged_checksums() {
    assert!(FixtureReader::new(b"invalid gzip").next_line().is_err());
    let mut damaged = TWO_LINES.to_vec();
    let checksum = damaged.len() - 8;
    damaged[checksum] ^= 1;
    let mut decoded = FixtureReader::new(&damaged);
    assert!(
        std::iter::from_fn(|| decoded.next_line().transpose())
            .collect::<Result<Vec<_>, _>>()
            .is_err()
    );
}
