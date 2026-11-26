use super::*;
#[test]
fn fixed_filter_controls_use_real_sampler_coordinate_and_mipmap_parameters() {
    let center = cap_texture("generated-fixed-bilinear-fixed-center");
    assert_eq!(
        center.get_params().filter(),
        Graphic3dTypeOfTextureFilter::Bilinear
    );
    assert_eq!(center.get_params().translation(), Vec2::new(-0.5, -0.5));
    assert_eq!(center.get_params().gen_plane_s(), Vec4::new(0., 0., 0., 0.));
    let minified = cap_texture("generated-fixed-trilinear-fixed-minified");
    assert_eq!(
        minified.get_params().filter(),
        Graphic3dTypeOfTextureFilter::Trilinear
    );
    assert_eq!(minified.get_params().scale(), Vec2::new(128., 128.));
    assert!(minified.has_mipmaps());
}
#[test]
fn compound_oblique_policies_keep_the_actual_native_plane() {
    for mode in [
        "oblique",
        "oblique-mask",
        "object-generated-oblique",
        "fixed-oblique",
        "oblique-image-updated",
        "oblique-mask-blend-alpha-updated",
    ] {
        assert_eq!(cap_fixture_equation(mode), [0.6, 0., -0.8, -2.], "{mode}");
    }
    assert_eq!(cap_fixture_equation("mask"), [0., 0., -1., 0.]);
}
