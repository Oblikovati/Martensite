//! Complete analytic intersection families use the shared owned driver and exact comparator.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/analytic_intersection_corpus/inputs-00.txt"),
        include_str!("fixtures/analytic_intersection_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/analytic_intersection_corpus/inputs-01.txt"),
        include_str!("fixtures/analytic_intersection_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/analytic_intersection_corpus/inputs-02.txt"),
        include_str!("fixtures/analytic_intersection_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/analytic_intersection_corpus/inputs-03.txt"),
        include_str!("fixtures/analytic_intersection_corpus/expected-03.txt"),
    ),
];

#[test]
fn ia2_lines_complete_family() {
    super::replay_family(CHUNKS, "ia2_lines", 93);
}

#[test]
fn ia2_circles_complete_family() {
    super::replay_family(CHUNKS, "ia2_circles", 29);
}

#[test]
fn ia2_line_circle_complete_family() {
    super::replay_family(CHUNKS, "ia2_line_circle", 68);
}

#[test]
fn ia2_conic_complete_family() {
    super::replay_family(CHUNKS, "ia2_conic", 109);
}

#[test]
fn ia2_conic_eval_complete_family() {
    super::replay_family(CHUNKS, "ia2_conic_eval", 15);
}

#[test]
fn ia2_roots_complete_family() {
    super::replay_family(CHUNKS, "ia2_roots", 44);
}

#[test]
fn ia2_outils_complete_family() {
    super::replay_family(CHUNKS, "ia2_outils", 21);
}

#[test]
fn ia2_int_point_complete_family() {
    super::replay_family(CHUNKS, "ia2_int_point", 4);
}

#[test]
fn ia_quadric_complete_family() {
    super::replay_family(CHUNKS, "ia_quadric", 14);
}

#[test]
fn ia_conic_quad_complete_family() {
    super::replay_family(CHUNKS, "ia_conic_quad", 80);
}

#[test]
fn ia_conic_plane_complete_family() {
    super::replay_family(CHUNKS, "ia_conic_plane", 52);
}

#[test]
fn ia_lin_torus_complete_family() {
    super::replay_family(CHUNKS, "ia_lin_torus", 21);
}

#[test]
fn ia_qqgeo_complete_family() {
    super::replay_family(CHUNKS, "ia_qqgeo", 313);
}

#[test]
fn ia_qq_complete_family() {
    super::replay_family(CHUNKS, "ia_qq", 70);
}

#[test]
fn ia_curve_complete_family() {
    super::replay_family(CHUNKS, "ia_curve", 12);
}

#[test]
fn analytic_request_and_retained_answer_bytes_stay_frozen() {
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0xd42f9cc1dcd7aa00
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0xc34a03b8570f44c1
    );
}

#[test]
fn orthogonal_lines_meet_at_independently_known_point_and_parameters() {
    let output = super::construction_corpus::run_values(
        "ia2_lines",
        &[0.0, 3.0, 1.0, 0.0, 2.0, 0.0, 0.0, 1.0],
    )
    .unwrap();
    let (two, three) = (2.0f64.to_bits(), 3.0f64.to_bits());
    assert_eq!(
        output,
        format!("1 0 0 0 1 {two} {three} {two} 0 {three} Standard_OutOfRange \n")
    );
}
#[test]
fn quadratic_factorization_pins_independent_root_set() {
    let output =
        super::construction_corpus::run_values("ia2_roots", &[1.0, 1.0, -5.0, 6.0]).unwrap();
    let tokens: Vec<&str> = output.split_whitespace().collect();
    assert_eq!(&tokens[..3], &["1", "0", "2"]);
    let mut roots: Vec<u64> = tokens[3..]
        .iter()
        .map(|token| token.parse().unwrap())
        .collect();
    roots.sort_unstable();
    assert_eq!(roots, [2.0f64.to_bits(), 3.0f64.to_bits()]);
}
