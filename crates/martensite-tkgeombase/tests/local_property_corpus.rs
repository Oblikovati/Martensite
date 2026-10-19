//! Complete local-property families use the canonical builders and shared replay policy.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/local_property_corpus/inputs-00.txt"),
        include_str!("fixtures/local_property_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/local_property_corpus/inputs-01.txt"),
        include_str!("fixtures/local_property_corpus/expected-01.txt"),
    ),
];

#[test]
fn lprop_cl2d_complete_family() {
    super::replay_family(CHUNKS, "lprop_cl2d", 171);
}

#[test]
fn lprop_cl3d_complete_family() {
    super::replay_family(CHUNKS, "lprop_cl3d", 55);
}

#[test]
fn lprop_cl3d_adaptor_complete_family() {
    super::replay_family(CHUNKS, "lprop_cl3d_adaptor", 55);
}

#[test]
fn lprop_sl_complete_family() {
    super::replay_family(CHUNKS, "lprop_sl", 70);
}

#[test]
fn lprop_curinf2d_complete_family() {
    super::replay_family(CHUNKS, "lprop_curinf2d", 72);
}

#[test]
fn lprop_curandinf_complete_family() {
    super::replay_family(CHUNKS, "lprop_curandinf", 7);
}

#[test]
fn lprop_continuity_complete_family() {
    super::replay_family(CHUNKS, "lprop_continuity", 80);
}

#[test]
fn local_property_requests_and_answers_stay_frozen() {
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0x52db01d4c4302445
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0xc28e747ef3369994
    );
}

#[test]
fn circle_local_properties_match_independent_position_and_curvature() {
    let request = [1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 2.0, 0.0, 3.0, 1e-9, 1.0, 0.0];
    let output = super::construction_corpus::run_values("lprop_cl2d", &request).unwrap();
    let tokens: Vec<&str> = output.split_whitespace().collect();
    assert_eq!(tokens[0], 2.0f64.to_bits().to_string());
    assert_eq!(tokens[1], 0.0f64.to_bits().to_string());
    assert_eq!(tokens[2], "1");
    assert_eq!(tokens[5], 0.5f64.to_bits().to_string());
}
#[test]
fn plane_fixture_has_independent_point_normal_and_zero_curvatures() {
    use super::construction_corpus::{common::Reader, geometry_fixtures};
    use martensite_tkgeombase::geom_lprop::SLProps;
    use martensite_tkmath::gp::{Direction3, Point3};
    let values = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 99.0];
    let mut reader = Reader::new(&values);
    let surface = geometry_fixtures::surface(&mut reader).unwrap();
    assert_eq!(reader.real(), 99.0);
    let mut props = SLProps::with_parameters(surface, 1.0, 2.0, 2, 1e-9).unwrap();
    assert_eq!(props.value(), Point3::new(1.0, 2.0, 0.0));
    assert_eq!(props.normal().unwrap(), Direction3::Z);
    assert_eq!(props.max_curvature().unwrap(), 0.0);
    assert_eq!(props.min_curvature().unwrap(), 0.0);
    assert_eq!(props.mean_curvature().unwrap(), 0.0);
    assert_eq!(props.gaussian_curvature().unwrap(), 0.0);
}
