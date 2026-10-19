//! The six complete original Approx adaptor request families, with frozen retained answers.
const CHUNKS: &[(&str, &str)] = &[(
    include_str!("fixtures/approx_adaptors_corpus/inputs.txt"),
    include_str!("fixtures/approx_adaptors_corpus/native.txt"),
)];
#[test]
fn all_requests_and_expected_bytes_are_frozen() {
    assert_eq!(CHUNKS[0].0.lines().count(), 454);
    assert_eq!(CHUNKS[0].1.lines().count(), 454);
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(r, _)| *r)),
        4298334580946723516
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, r)| *r)),
        5776126982196172594
    );
}
#[test]
fn axa_curve3d_complete_family() {
    super::replay_family(CHUNKS, "axa_curve3d", 102);
}
#[test]
fn axa_curve2d_complete_family() {
    super::replay_family(CHUNKS, "axa_curve2d", 45);
}
#[test]
fn axa_cos_complete_family() {
    super::replay_family(CHUNKS, "axa_cos", 202);
}
#[test]
fn axa_curvlin_complete_family() {
    super::replay_family(CHUNKS, "axa_curvlin", 17);
}
#[test]
fn axa_curvilinear_complete_family() {
    super::replay_family(CHUNKS, "axa_curvilinear", 40);
}
#[test]
fn axa_same_complete_family() {
    super::replay_family(CHUNKS, "axa_same", 48);
}
#[path = "support/approx_adaptors_controls.rs"]
mod independent_controls;

#[test]
fn every_family_rejects_a_changed_finite_result_bit() {
    use super::construction_corpus::policy::equivalent;
    for family in [
        "axa_curve3d",
        "axa_curve2d",
        "axa_cos",
        "axa_curvlin",
        "axa_curvilinear",
        "axa_same",
    ] {
        let expected = CHUNKS[0]
            .0
            .lines()
            .zip(CHUNKS[0].1.split_inclusive('\n'))
            .find(|(request, _)| request.split_whitespace().next() == Some(family))
            .unwrap()
            .1;
        let bits = expected
            .split_whitespace()
            .find_map(|token| {
                let value = token.parse::<u64>().ok()?;
                (token.len() >= 16 && value & 0x7ff0_0000_0000_0000 != 0x7ff0_0000_0000_0000)
                    .then_some((token, value))
            })
            .expect("each original first result includes a finite double");
        let changed = expected.replacen(bits.0, &(bits.1 ^ 1).to_string(), 1);
        assert!(
            !equivalent(expected, &changed),
            "{family} must keep finite bits exact"
        );
    }
}

#[test]
fn exception_types_and_dump_delimiters_are_not_normalized_away() {
    use super::construction_corpus::policy::equivalent;
    let expected = CHUNKS[0]
        .1
        .lines()
        .find(|row| row.contains("Standard_"))
        .unwrap();
    let changed = expected.replacen("Standard_", "Changed_", 1);
    assert!(!equivalent(expected, &changed));
    let dump = CHUNKS[0].1.lines().find(|row| row.contains('|')).unwrap();
    assert!(!equivalent(dump, &dump.replacen('|', "\n", 1)));
}
