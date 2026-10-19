//! Complete mass-property families reuse the owned numeric driver and comparison policy.
const CHUNKS: &[(&str, &str)] = &[(
    include_str!("fixtures/mass_property_corpus/inputs.txt"),
    include_str!("fixtures/mass_property_corpus/native.txt"),
)];

#[test]
fn gprop_points_complete_family() {
    super::replay_family(CHUNKS, "gprop_points", 59);
}

#[test]
fn gprop_pequation_complete_family() {
    super::replay_family(CHUNKS, "gprop_pequation", 49);
}

#[test]
fn gprop_hoperator_complete_family() {
    super::replay_family(CHUNKS, "gprop_hoperator", 3);
}

#[test]
fn gprop_elementary_complete_family() {
    super::replay_family(CHUNKS, "gprop_elementary", 72);
}

#[test]
fn gprop_compose_complete_family() {
    super::replay_family(CHUNKS, "gprop_compose", 6);
}

#[test]
fn mass_property_inputs_and_retained_answers_stay_frozen() {
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0x03fa59a7becd2fe8
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0x6acb096e7d76eb54
    );
}

#[test]
fn inertia_translation_matches_independent_parallel_axis_matrix() {
    let output = super::construction_corpus::run_values(
        "gprop_hoperator",
        &[0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 2.0],
    )
    .unwrap();
    let entries: [f64; 9] = [26.0, -4.0, -6.0, -4.0, 20.0, -12.0, -6.0, -12.0, 10.0];
    let expected: String = entries
        .iter()
        .map(|entry| format!("{} ", entry.to_bits()))
        .collect();
    assert_eq!(output, expected + "\n");
}
#[test]
fn weighted_points_have_independent_total_mass_and_barycentre() {
    let request = [1.0, 2.0, 1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 0.0, 0.0, 3.0];
    let output = super::construction_corpus::run_values("gprop_points", &request).unwrap();
    let prefix: Vec<u64> = output
        .split_whitespace()
        .take(4)
        .map(|word| word.parse().unwrap())
        .collect();
    assert_eq!(
        prefix,
        [
            4.0f64.to_bits(),
            1.5f64.to_bits(),
            0.0f64.to_bits(),
            0.0f64.to_bits()
        ]
    );
}
