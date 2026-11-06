use super::*;
#[test]
fn sample_has_all_fifteen_authored_edge_dimensions_in_source_groups() {
    assert_eq!(
        dimensions::LENGTHS.len() + dimensions::RADII.len() + dimensions::ANGLES.len(),
        15
    );
    assert_eq!(dimensions::LENGTHS[7], ("ld_8", 0, 42, 55., true));
    assert_eq!(dimensions::RADII, [("rd_1", 45, true), ("rd_2", 63, false)]);
    assert_eq!(dimensions::ANGLES[2], ("ad_3", 0, 48, 42));
}
#[test]
fn authored_arrow_angle_keeps_three_point_one_four_literal() {
    assert_eq!(
        dimensions::arrow_angle().to_bits(),
        (3.14_f64 * 12. / 180.).to_bits()
    );
    assert_ne!(
        dimensions::arrow_angle().to_bits(),
        (std::f64::consts::PI * 12. / 180.).to_bits()
    );
}

#[test]
fn authentic_hatch_caps_retain_signed_scale_and_distinct_object_colors() {
    use martensite_tkservice::graphic3d::{Graphic3dAspectsMembers, Graphic3dTextureRootMembers};
    let path = crate::dataset_shape::locate_dataset_file("images/hatch_1.png").unwrap();
    let plane = capping::capping_plane(&path, Color::named(NamedColor::GOLD)).unwrap();
    let scale = plane.capping_texture().get_params().scale();
    assert_eq!(scale.x().to_bits(), 0.05_f32.to_bits());
    assert_eq!(scale.y().to_bits(), (-0.05_f32).to_bits());
    assert!(plane.is_capping());
    // The legacy CappingColor getter returns the aspect-material diffuse color,
    // while SetCappingColor writes the actual interior color used with that material.
    assert_eq!(
        plane.capping_aspect().interior_color(),
        Color::named(NamedColor::GOLD)
    );
}
