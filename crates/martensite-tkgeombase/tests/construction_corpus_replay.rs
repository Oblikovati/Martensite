//! Original elementary construction requests executed directly in Rust.
#[path = "support/construction_corpus/mod.rs"]
mod construction_corpus;
use construction_corpus::replay::{fixture_hash, replay_family};
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/construction_corpus/inputs-00.txt"),
        include_str!("fixtures/construction_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/construction_corpus/inputs-01.txt"),
        include_str!("fixtures/construction_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/construction_corpus/inputs-02.txt"),
        include_str!("fixtures/construction_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/construction_corpus/inputs-03.txt"),
        include_str!("fixtures/construction_corpus/expected-03.txt"),
    ),
    (
        include_str!("fixtures/construction_corpus/inputs-04.txt"),
        include_str!("fixtures/construction_corpus/expected-04.txt"),
    ),
    (
        include_str!("fixtures/construction_corpus/inputs-05.txt"),
        include_str!("fixtures/construction_corpus/expected-05.txt"),
    ),
    (
        include_str!("fixtures/construction_corpus/inputs-06.txt"),
        include_str!("fixtures/construction_corpus/expected-06.txt"),
    ),
    (
        include_str!("fixtures/construction_corpus/inputs-07.txt"),
        include_str!("fixtures/construction_corpus/expected-07.txt"),
    ),
    (
        include_str!("fixtures/construction_corpus/inputs-08.txt"),
        include_str!("fixtures/construction_corpus/expected-08.txt"),
    ),
    (
        include_str!("fixtures/construction_corpus/inputs-09.txt"),
        include_str!("fixtures/construction_corpus/expected-09.txt"),
    ),
];

#[test]
fn gce_status_complete_family() {
    replay_family(CHUNKS, "gce_status", 1);
}

#[test]
fn gce_circ_complete_family() {
    replay_family(CHUNKS, "gce_circ", 443);
}

#[test]
fn gce_circ2d_complete_family() {
    replay_family(CHUNKS, "gce_circ2d", 157);
}

#[test]
fn gce_cone_complete_family() {
    replay_family(CHUNKS, "gce_cone", 606);
}

#[test]
fn gce_cylinder_complete_family() {
    replay_family(CHUNKS, "gce_cylinder", 337);
}

#[test]
fn gce_dir_complete_family() {
    replay_family(CHUNKS, "gce_dir", 60);
}

#[test]
fn gce_dir2d_complete_family() {
    replay_family(CHUNKS, "gce_dir2d", 60);
}

#[test]
fn gce_elips_complete_family() {
    replay_family(CHUNKS, "gce_elips", 104);
}

#[test]
fn gce_hypr_complete_family() {
    replay_family(CHUNKS, "gce_hypr", 104);
}

#[test]
fn gce_elips2d_complete_family() {
    replay_family(CHUNKS, "gce_elips2d", 141);
}

#[test]
fn gce_hypr2d_complete_family() {
    replay_family(CHUNKS, "gce_hypr2d", 141);
}

#[test]
fn gce_lin_complete_family() {
    replay_family(CHUNKS, "gce_lin", 21);
}

#[test]
fn gce_lin2d_complete_family() {
    replay_family(CHUNKS, "gce_lin2d", 42);
}

#[test]
fn gce_parab_complete_family() {
    replay_family(CHUNKS, "gce_parab", 21);
}

#[test]
fn gce_parab2d_complete_family() {
    replay_family(CHUNKS, "gce_parab2d", 76);
}

#[test]
fn gce_pln_complete_family() {
    replay_family(CHUNKS, "gce_pln", 100);
}

#[test]
fn gce_mirror_complete_family() {
    replay_family(CHUNKS, "gce_mirror", 18);
}

#[test]
fn gce_mirror2d_complete_family() {
    replay_family(CHUNKS, "gce_mirror2d", 12);
}

#[test]
fn gce_rotation_complete_family() {
    replay_family(CHUNKS, "gce_rotation", 36);
}

#[test]
fn gce_rotation2d_complete_family() {
    replay_family(CHUNKS, "gce_rotation2d", 12);
}

#[test]
fn gce_scale_complete_family() {
    replay_family(CHUNKS, "gce_scale", 18);
}

#[test]
fn gce_scale2d_complete_family() {
    replay_family(CHUNKS, "gce_scale2d", 18);
}

#[test]
fn gce_translation_complete_family() {
    replay_family(CHUNKS, "gce_translation", 6);
}

#[test]
fn gce_translation2d_complete_family() {
    replay_family(CHUNKS, "gce_translation2d", 6);
}

#[test]
fn unknown_operation_has_no_output() {
    assert!(construction_corpus::run_values("missing-operation", &[]).is_none());
}

#[test]
fn request_and_retained_expected_bytes_are_frozen() {
    assert_eq!(
        fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0xefa775799e35427a
    );
    assert_eq!(
        fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0x9f2569f7dae2a730
    );
}

#[test]
fn translation_between_points_matches_independent_difference_and_identity_matrix() {
    let actual =
        construction_corpus::run_values("gce_translation2d", &[1.0, 2.0, 3.0, 5.0, 7.0]).unwrap();
    let (one, three, four) = (1.0f64.to_bits(), 3.0f64.to_bits(), 4.0f64.to_bits());
    let transformation = format!("2 {one} {one} 0 0 {one} {three} {four} ");
    assert_eq!(actual, transformation.repeat(2) + "\n");
}

#[path = "geometric_construction_corpus.rs"]
mod geometric_construction_corpus;

#[path = "analytic_intersection_corpus.rs"]
mod analytic_intersection_corpus;

#[path = "mass_property_corpus.rs"]
mod mass_property_corpus;

#[path = "local_property_corpus.rs"]
mod local_property_corpus;

#[path = "hermite_corpus.rs"]
mod hermite_corpus;

#[path = "bounding_corpus.rs"]
mod bounding_corpus;

#[path = "point_distribution_corpus.rs"]
mod point_distribution_corpus;

#[path = "geometry_utilities_elementary_corpus.rs"]
mod geometry_utilities_elementary_corpus;

#[path = "continuity_tables_corpus.rs"]
mod continuity_tables_corpus;

#[path = "continuity_matrix_corpus.rs"]
mod continuity_matrix_corpus;

#[path = "continuity_least_squares_corpus.rs"]
mod continuity_least_squares_corpus;

#[path = "app_par_container_corpus.rs"]
mod app_par_container_corpus;

#[path = "fit_constraints_container_corpus.rs"]
mod fit_constraints_container_corpus;

#[path = "planar_conversion_corpus.rs"]
mod planar_conversion_corpus;

#[path = "spatial_conversion_corpus.rs"]
mod spatial_conversion_corpus;

#[path = "elementary_extrema_corpus.rs"]
mod elementary_extrema_corpus;

#[path = "point_curve_corpus.rs"]
mod point_curve_corpus;

#[path = "classic_point_curve_corpus.rs"]
mod classic_point_curve_corpus;

#[path = "curve_curve_corpus.rs"]
mod curve_curve_corpus;

#[path = "point_surface_corpus.rs"]
mod point_surface_corpus;

#[path = "curve_surface_corpus.rs"]
mod curve_surface_corpus;

#[path = "geomlib_static_corpus.rs"]
mod geomlib_static_corpus;

#[path = "geomlib_class_corpus.rs"]
mod geomlib_class_corpus;

#[path = "multi_curve_fit_corpus.rs"]
mod multi_curve_fit_corpus;
#[path = "projlib_elementary_corpus.rs"]
mod projlib_elementary_corpus;

#[path = "approx_fitting_corpus.rs"]
mod approx_fitting_corpus;
#[path = "fit_constraints_corpus.rs"]
mod fit_constraints_corpus;
#[path = "projlib_curves_corpus.rs"]
mod projlib_curves_corpus;

#[path = "approx_adaptors_corpus.rs"]
mod approx_adaptors_corpus;
#[path = "approx_sweep_corpus.rs"]
mod approx_sweep_corpus;

#[path = "surface_projection_corpus.rs"]
mod surface_projection_corpus;
