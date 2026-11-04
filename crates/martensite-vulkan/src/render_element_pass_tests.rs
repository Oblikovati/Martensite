use super::*;
/// Named aspect fixture: a fill aspect with front transparency and the given alpha mode.
fn aspect_with(transparency: f32, mode: Graphic3dAlphaMode) -> Handle<dyn Graphic3dAspectsSource> {
    let style = Handle::new(Graphic3dAspects::default());
    let mut material = style.front_material();
    material.set_transparency(transparency).unwrap();
    style.set_front_material(material);
    style.set_alpha_mode(mode, 0.5);
    style.cast()
}
#[test]
fn opaque_pass_skips_and_counts_only_transparent_filling_elements() {
    let pass = RenderElementPass::opaque(true);
    assert_eq!(pass.filter(), RenderElementFilter::OPAQUE_ONLY);
    assert!(pass.should_render(false, || true));
    assert!(pass.should_render(true, || false));
    assert!(!pass.should_render(true, || true));
    pass.note_builtin(true, || true);
    pass.note_builtin(false, || true);
    assert_eq!(pass.skipped(), 2);
}
#[test]
fn transparent_pass_keeps_the_counter_and_calls_transparent_fills_only() {
    let opaque = RenderElementPass::opaque(true);
    opaque.should_render(true, || true);
    let pass = opaque.transparent();
    assert_eq!(pass.filter(), RenderElementFilter::TRANSPARENT_ONLY);
    assert_eq!(pass.skipped(), 1);
    assert!(!pass.use_depth_write());
    assert!(pass.should_render(true, || true));
    assert!(!pass.should_render(true, || false));
    assert!(!pass.should_render(false, || true));
    assert!(!pass.lowers_group_geometry());
}
#[test]
fn unfiltered_extraction_calls_every_element() {
    let pass = RenderElementPass::unfiltered();
    assert_eq!(pass.filter(), RenderElementFilter::EMPTY);
    assert!(pass.should_render(true, || true));
    assert_eq!(pass.skipped(), 0);
}
#[test]
fn fill_types_follow_the_native_primitive_array() {
    use Graphic3dTypeOfPrimitiveArray as Kind;
    assert!(is_fill_primitive(Kind::Triangles, true));
    assert!(is_fill_primitive(Kind::Polygons, true));
    assert!(!is_fill_primitive(Kind::Triangles, false));
    assert!(!is_fill_primitive(Kind::Segments, true));
    assert!(!is_fill_primitive(Kind::Points, true));
}
#[test]
fn transparency_follows_alpha_mode_and_material_alpha() {
    let none = Handle::default();
    assert!(check_is_transparent(
        &aspect_with(0.5, Graphic3dAlphaMode::BlendAuto),
        &none
    ));
    assert!(!check_is_transparent(
        &aspect_with(0., Graphic3dAlphaMode::BlendAuto),
        &none
    ));
    assert!(check_is_transparent(
        &aspect_with(0., Graphic3dAlphaMode::Blend),
        &none
    ));
    assert!(!check_is_transparent(
        &aspect_with(0.5, Graphic3dAlphaMode::Mask),
        &none
    ));
}
#[test]
fn plain_highlight_color_supplies_the_alpha() {
    let style = Handle::new(Graphic3dPresentationAttributes::default());
    style.set_transparency(0.75);
    let highlight: Handle<dyn Graphic3dPresentationAttributesSource> = style.cast();
    assert!(check_is_transparent(
        &aspect_with(0., Graphic3dAlphaMode::BlendAuto),
        &highlight
    ));
}
#[test]
fn filter_bits_equal_the_native_enum() {
    for (filter, bits) in [
        (RenderElementFilter::EMPTY, 0x000),
        (RenderElementFilter::OPAQUE_ONLY, 0x001),
        (RenderElementFilter::TRANSPARENT_ONLY, 0x002),
        (RenderElementFilter::NON_RAYTRACEABLE_ONLY, 0x004),
        (RenderElementFilter::FILL_MODE_ONLY, 0x008),
        (RenderElementFilter::SKIP_TRSF_PERSISTENCE, 0x010),
    ] {
        assert_eq!(filter.bits(), bits);
        assert!(filter.contains(filter));
    }
    assert!(!RenderElementFilter::OPAQUE_ONLY.contains(RenderElementFilter::TRANSPARENT_ONLY));
}

#[test]
fn the_transparent_pass_keeps_the_layer_environment_and_null_textures_map_nothing() {
    use martensite_tkernel::standard::Handle;
    use martensite_tkservice::graphic3d::Graphic3dTextureEnv;
    let texture = Handle::new(Graphic3dTextureEnv::from_pix_map(Handle::default()));
    let opaque = RenderElementPass::opaque(true).with_environment(Some(texture.clone()));
    assert_eq!(opaque.transparent().environment(), Some(&texture));
    let cleared = RenderElementPass::opaque(true).with_environment(Some(Handle::default()));
    assert!(cleared.environment().is_none());
    assert!(RenderElementPass::unfiltered().environment().is_none());
}
