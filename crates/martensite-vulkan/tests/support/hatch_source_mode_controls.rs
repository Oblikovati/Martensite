//! Default display and a later global-mode command must not become an own-mode display.
use super::super::super::device_options;
use super::*;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;

#[test]
fn actual_default_display_then_global_mode_retains_no_own_mode()
-> Result<(), Box<dyn std::error::Error>> {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options())?;
    let stage = HatchSourceView::new(&driver)?;
    stage.source_clear()?;
    let scene = stage.with_shape(unit_box_shape());
    scene.source_display_default()?;
    require_modes(&scene, 0);
    assert!(!scene.object.has_display_mode());
    scene.source_global_mode(1)?;
    require_modes(&scene, 1);
    assert!(!scene.object.has_display_mode());
    scene.close();
    Ok(())
}

fn require_modes(scene: &HatchBoxScene, expected: i32) {
    assert_eq!(scene._context.display_mode(), expected);
    assert!(
        scene
            ._context
            .is_displayed_mode(&scene.object.cast(), expected)
    );
    assert!(
        !scene
            ._context
            .is_displayed_mode(&scene.object.cast(), 1 - expected)
    );
}
