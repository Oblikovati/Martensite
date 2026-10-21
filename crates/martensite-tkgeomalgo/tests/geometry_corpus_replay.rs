//! Frozen source-bound numeric requests, directly invoking the retained Rust operations.
#[path = "support/geometry_corpus/mod.rs"]
mod geometry_corpus;
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/geometry_corpus/inputs-00.txt"),
        include_str!("fixtures/geometry_corpus/reference-00.txt"),
    ),
    (
        include_str!("fixtures/geometry_corpus/inputs-01.txt"),
        include_str!("fixtures/geometry_corpus/reference-01.txt"),
    ),
    (
        include_str!("fixtures/geometry_corpus/inputs-02.txt"),
        include_str!("fixtures/geometry_corpus/reference-02.txt"),
    ),
    (
        include_str!("fixtures/geometry_corpus/inputs-03.txt"),
        include_str!("fixtures/geometry_corpus/reference-03.txt"),
    ),
    (
        include_str!("fixtures/geometry_corpus/inputs-04.txt"),
        include_str!("fixtures/geometry_corpus/reference-04.txt"),
    ),
    (
        include_str!("fixtures/geometry_corpus/inputs-05.txt"),
        include_str!("fixtures/geometry_corpus/reference-05.txt"),
    ),
    (
        include_str!("fixtures/geometry_corpus/inputs-06.txt"),
        include_str!("fixtures/geometry_corpus/reference-06.txt"),
    ),
];

fn replay_request(request: &str, answer: &str, family: &str) -> bool {
    let mut fields = request.split_whitespace();
    if fields.next() != Some(family) {
        return false;
    }
    let values: Vec<f64> = fields.map(|field| field.parse().unwrap()).collect();
    let actual = geometry_corpus::run_values(family, &values).expect("owned family");
    assert!(
        geometry_corpus::policy::equivalent(answer, &actual),
        "{request}: {}",
        geometry_corpus::policy::difference(answer, &actual)
    );
    true
}

fn replay_family(family: &str, expected_count: usize) {
    let mut count = 0;
    for (inputs, expected) in CHUNKS {
        assert_eq!(inputs.lines().count(), expected.lines().count());
        for (request, answer) in inputs.lines().zip(expected.split_inclusive('\n')) {
            count += usize::from(replay_request(request, answer, family));
        }
    }
    assert_eq!(
        count, expected_count,
        "{family} must retain every original request"
    );
}

#[test]
fn tt_states_complete_original_family() {
    replay_family("tt_states", 4);
}

#[test]
fn tt_curve_complete_original_family() {
    replay_family("tt_curve", 305);
}

#[test]
fn tt_surface_complete_original_family() {
    replay_family("tt_surface", 672);
}

#[test]
fn plate_solve_complete_original_family() {
    replay_family("plate_solve", 44);
}

#[test]
fn plate_constraint_complete_original_family() {
    replay_family("plate_constraint", 62);
}

#[test]
fn la_curve_complete_original_family() {
    replay_family("la_curve", 168);
}

#[test]
fn la_surface_complete_original_family() {
    replay_family("la_surface", 238);
}

#[test]
fn fc_law_complete_original_family() {
    replay_family("fc_law", 4);
}

#[test]
fn fc_density_complete_original_family() {
    replay_family("fc_density", 51);
}

#[test]
fn fc_energy_complete_original_family() {
    replay_family("fc_energy", 67);
}

#[test]
fn fc_batten_complete_original_family() {
    replay_family("fc_batten", 56);
}

#[test]
fn dispatch_unknown_operation_preserves_no_output() {
    assert!(geometry_corpus::run_values("missing-operation", &[]).is_none());
}
#[test]
fn transitions_match_independent_orientation_tables() {
    for (orientation, expected) in ["1 0 \n", "0 1 \n", "0 0 \n", "1 1 \n"].iter().enumerate() {
        assert_eq!(
            geometry_corpus::run_values("tt_states", &[orientation as f64]).unwrap(),
            *expected
        );
    }
}

#[test]
fn constant_batten_height_has_independent_closed_form_values() {
    let actual = geometry_corpus::run_values(
        "fc_law",
        &[2.0, 0.0, 1.0, 4.0, 0.0, 1.0, 3.0, -1.0, 0.0, 1.0],
    )
    .unwrap();
    let expected = "1 4611686018427387904 1 4611686018427387904 1 4611686018427387904 1 4616189618054758400 1 4616189618054758400 1 4616189618054758400 \n";
    assert_eq!(actual, expected);
}

#[test]
fn operation_local_output_preserves_exceptions_and_separate_buffers() {
    use geometry_corpus::print::{Output, attempt, end, flag, int, real, show};
    use martensite_tkernel::standard::Failure;
    let first = Output::default();
    let second = Output::default();
    real(&first, -0.0);
    flag(&first, true);
    int(&second, -7);
    attempt(&first, || Err(Failure::out_of_range("owned test")));
    show(&second, Ok(3), |value| int(&second, value));
    end(&first);
    assert_eq!(
        first.finish(),
        "9223372036854775808 1 Standard_OutOfRange \n"
    );
    assert_eq!(second.finish(), "-7 3 ");
}

#[test]
fn frozen_request_and_expected_bytes_are_unchanged() {
    assert_eq!(
        geometry_corpus::policy::fixture_bytes_hash(CHUNKS.iter().map(|(requests, _)| *requests)),
        0x4a074004faa72054
    );
    assert_eq!(
        geometry_corpus::policy::fixture_bytes_hash(CHUNKS.iter().map(|(_, expected)| *expected)),
        0xbfd735992e3a7a6b
    );
}
