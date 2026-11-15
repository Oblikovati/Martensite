use super::*;
use martensite_tkservice::graphic3d::*;
#[test]
fn cap_edges_preserve_selected_alpha_and_skip_policy() {
    let aspect = Graphic3dAspects::default();
    aspect.set_alpha_mode(Graphic3dAlphaMode::MaskBlend, 0.5);
    configure_section_mesh_edges(&aspect, "section-edges-dashed-skip-first");
    assert!(aspect.to_draw_edges());
    assert_eq!(aspect.line_width(), 3.);
    assert!(aspect.to_skip_first_edge());
    assert_eq!(aspect.alpha_mode(), Graphic3dAlphaMode::MaskBlend);
    assert_eq!(aspect.line_type(), AspectTypeOfLine::Dash);
}
#[test]
fn actual_edge_alpha_preserves_hollow_and_background_sentinels() {
    assert_eq!(section_edge_alpha("negative-alpha"), -0.25);
    assert_eq!(section_edge_alpha("edge-alpha"), 0.25);
    assert_eq!(section_edge_alpha("edge-background"), 0.05);
    assert_eq!(section_edge_alpha("opaque"), 1.);
}
#[test]
fn section_line_and_fill_offset_masks_remain_independent() {
    for (mode, mask) in [("line-offset", 2), ("fill-offset", 1), ("both-offset", 3)] {
        let aspect = Graphic3dAspects::default();
        configure_section_mesh_offset(&aspect, mode);
        assert_eq!(aspect.polygon_offsets(), (mask, 2., 3.));
    }
}
