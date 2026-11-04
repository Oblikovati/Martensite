use super::*;
#[test]
fn all_color_positions_preserve_the_authored_twenty_six_column_wrap() {
    assert_eq!(colors::position(0), [0., 0., 0.]);
    assert_eq!(colors::position(25), [300., 0., 0.]);
    assert_eq!(colors::position(26), [0., 12., 0.]);
    assert_eq!(colors::position(53), [12., 24., 0.]);
}
#[test]
fn all_twenty_materials_and_four_source_columns_resolve_without_replacement() {
    assert_eq!(MATERIALS.len(), 20);
    for name in MATERIALS {
        assert!(material_objects::resolve_material(name).is_ok(), "{name}");
    }
    for name in ["GRAY10", "BLACK", "red", "green", "blue", "blue1"] {
        named_color(name).unwrap();
    }
}
#[test]
fn material_placement_preserves_distinct_sphere_and_box_offsets() {
    assert_eq!(
        material_objects::placements(0, 0),
        ([30., 15., 0.], [40., 15., 0.])
    );
    assert_eq!(
        material_objects::placements(3, 19),
        ([135., 205., 0.], [145., 205., 0.])
    );
}

#[test]
fn source_column_color_retains_default_and_explicit_blue_one() {
    assert!(material_objects::column_color(0).unwrap().is_none());
    assert_eq!(
        material_objects::column_color(3).unwrap(),
        Some(named_color("blue1").unwrap())
    );
}
#[test]
fn sample_label_request_retains_screen_font_and_all_alignment_options() {
    use martensite_tkservice::graphic3d::Graphic3dHorizontalTextAlignment;
    let request = material_labels::source_label_request(
        "brass",
        (
            [20., 15., 0.],
            named_color("GRAY10").unwrap(),
            Graphic3dHorizontalTextAlignment::Right,
        ),
    );
    assert_eq!(request.position, [20., 15., 0.]);
    assert_eq!(request.font, Some("Arial"));
    assert_eq!(request.height, Some(14.));
    assert_eq!(request.zoomable, Some(false));
    assert_eq!(request.angle_degrees, Some(0.));
}
