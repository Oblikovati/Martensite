//! Complete original geometric-construction families share the owned direct driver and comparator.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/geometric_construction_corpus/inputs-00.txt"),
        include_str!("fixtures/geometric_construction_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/geometric_construction_corpus/inputs-01.txt"),
        include_str!("fixtures/geometric_construction_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/geometric_construction_corpus/inputs-02.txt"),
        include_str!("fixtures/geometric_construction_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/geometric_construction_corpus/inputs-03.txt"),
        include_str!("fixtures/geometric_construction_corpus/expected-03.txt"),
    ),
    (
        include_str!("fixtures/geometric_construction_corpus/inputs-04.txt"),
        include_str!("fixtures/geometric_construction_corpus/expected-04.txt"),
    ),
    (
        include_str!("fixtures/geometric_construction_corpus/inputs-05.txt"),
        include_str!("fixtures/geometric_construction_corpus/expected-05.txt"),
    ),
    (
        include_str!("fixtures/geometric_construction_corpus/inputs-06.txt"),
        include_str!("fixtures/geometric_construction_corpus/expected-06.txt"),
    ),
    (
        include_str!("fixtures/geometric_construction_corpus/inputs-07.txt"),
        include_str!("fixtures/geometric_construction_corpus/expected-07.txt"),
    ),
];

#[test]
fn gc_arc_circle_complete_family() {
    super::replay_family(CHUNKS, "gc_arc_circle", 99);
}

#[test]
fn gc_arc_ellipse_complete_family() {
    super::replay_family(CHUNKS, "gc_arc_ellipse", 60);
}

#[test]
fn gc_arc_hyperbola_complete_family() {
    super::replay_family(CHUNKS, "gc_arc_hyperbola", 60);
}

#[test]
fn gc_arc_parabola_complete_family() {
    super::replay_family(CHUNKS, "gc_arc_parabola", 60);
}

#[test]
fn gc_arc_circle2d_complete_family() {
    super::replay_family(CHUNKS, "gc_arc_circle2d", 96);
}

#[test]
fn gc_arc_ellipse2d_complete_family() {
    super::replay_family(CHUNKS, "gc_arc_ellipse2d", 60);
}

#[test]
fn gc_arc_hyperbola2d_complete_family() {
    super::replay_family(CHUNKS, "gc_arc_hyperbola2d", 60);
}

#[test]
fn gc_arc_parabola2d_complete_family() {
    super::replay_family(CHUNKS, "gc_arc_parabola2d", 60);
}

#[test]
fn gc_circle_complete_family() {
    super::replay_family(CHUNKS, "gc_circle", 136);
}

#[test]
fn gc_circle2d_complete_family() {
    super::replay_family(CHUNKS, "gc_circle2d", 101);
}

#[test]
fn gc_ellipse_complete_family() {
    super::replay_family(CHUNKS, "gc_ellipse", 42);
}

#[test]
fn gc_hyperbola_complete_family() {
    super::replay_family(CHUNKS, "gc_hyperbola", 42);
}

#[test]
fn gc_ellipse2d_complete_family() {
    super::replay_family(CHUNKS, "gc_ellipse2d", 54);
}

#[test]
fn gc_hyperbola2d_complete_family() {
    super::replay_family(CHUNKS, "gc_hyperbola2d", 54);
}

#[test]
fn gc_parabola2d_complete_family() {
    super::replay_family(CHUNKS, "gc_parabola2d", 52);
}

#[test]
fn gc_line_complete_family() {
    super::replay_family(CHUNKS, "gc_line", 20);
}

#[test]
fn gc_line2d_complete_family() {
    super::replay_family(CHUNKS, "gc_line2d", 29);
}

#[test]
fn gc_segment_complete_family() {
    super::replay_family(CHUNKS, "gc_segment", 27);
}

#[test]
fn gc_segment2d_complete_family() {
    super::replay_family(CHUNKS, "gc_segment2d", 36);
}

#[test]
fn gc_plane_complete_family() {
    super::replay_family(CHUNKS, "gc_plane", 58);
}

#[test]
fn gc_conical_complete_family() {
    super::replay_family(CHUNKS, "gc_conical", 96);
}

#[test]
fn gc_trimmed_cone_complete_family() {
    super::replay_family(CHUNKS, "gc_trimmed_cone", 54);
}

#[test]
fn gc_cylindrical_complete_family() {
    super::replay_family(CHUNKS, "gc_cylindrical", 293);
}

#[test]
fn gc_trimmed_cylinder_complete_family() {
    super::replay_family(CHUNKS, "gc_trimmed_cylinder", 134);
}

#[test]
fn gc_mirror_complete_family() {
    super::replay_family(CHUNKS, "gc_mirror", 18);
}

#[test]
fn gc_mirror2d_complete_family() {
    super::replay_family(CHUNKS, "gc_mirror2d", 12);
}

#[test]
fn gc_rotation_complete_family() {
    super::replay_family(CHUNKS, "gc_rotation", 36);
}

#[test]
fn gc_rotation2d_complete_family() {
    super::replay_family(CHUNKS, "gc_rotation2d", 12);
}

#[test]
fn gc_scale_complete_family() {
    super::replay_family(CHUNKS, "gc_scale", 18);
}

#[test]
fn gc_scale2d_complete_family() {
    super::replay_family(CHUNKS, "gc_scale2d", 18);
}

#[test]
fn gc_translation_complete_family() {
    super::replay_family(CHUNKS, "gc_translation", 6);
}

#[test]
fn gc_translation2d_complete_family() {
    super::replay_family(CHUNKS, "gc_translation2d", 6);
}

#[test]
fn geometric_construction_request_and_answer_bytes_stay_frozen() {
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0x6c125ebe2cfc6648
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0x06f394d5afac92e4
    );
}

#[test]
fn wrapped_translation_matches_independent_point_difference_and_identity_matrix() {
    let actual =
        super::construction_corpus::run_values("gc_translation2d", &[1.0, 2.0, 3.0, 5.0, 7.0])
            .unwrap();
    let (one, three, four) = (1.0f64.to_bits(), 3.0f64.to_bits(), 4.0f64.to_bits());
    let transformation = format!("Geom2d_Transformation 2 {one} {one} 0 0 {one} {three} {four} ");
    assert_eq!(actual, transformation.repeat(2) + "\n");
}
