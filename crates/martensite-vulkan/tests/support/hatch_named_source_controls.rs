//! One registration observes actual batch bindings and later global-mode transitions.
use crate::hatch_surface::scene::{
    HatchSourceView, NamedSourceScene, SourceViewOutput, unit_box_shape,
};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
use martensite_vulkan::Graphic3dRenderDriver;
#[path = "../../examples/support/device_options.rs"]
mod device_options;

#[test]
fn named_source_batch_then_result_preserves_bindings_and_global_modes()
-> Result<(), Box<dyn std::error::Error>> {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options())?;
    let initialized = HatchSourceView::new(&driver)?;
    let mut scene = NamedSourceScene::new(initialized);
    let shapes: Vec<_> = (0..5).map(|_| unit_box_shape()).collect();
    let names: [&str; 5] = ["c_1", "c_2", "c_3", "c_4", "c_5"];
    let group: Vec<_> = names.iter().copied().zip(shapes.iter()).collect();
    scene.source_display_group(&group)?;
    require_named_modes(&scene, &names, 0);
    require_result_keeps_old_objects(&mut scene)?;
    scene.source_global_mode(1)?;
    require_named_modes(&scene, &["c_1", "c_2", "c_3", "c_4", "c_5", "result"], 1);
    scene.source_fit()?;
    assert_eq!(scene.source_window_size(), (409, 409));
    scene.close();
    Ok(())
}

fn require_result_keeps_old_objects(
    scene: &mut NamedSourceScene,
) -> Result<(), martensite_tkernel::standard::Failure> {
    let old = named_identities(scene);
    require_distinct_identities(&old);
    let view_before = scene.source_view_handle().clone();
    let result = unit_box_shape();
    scene.source_display_group(&[("result", &result)])?;
    let actual = named_identities(scene);
    assert_eq!(
        old,
        actual[..old.len()],
        "expected unchanged prior AIS bindings"
    );
    assert!(std::ptr::eq(&*view_before, &**scene.source_view_handle()));
    Ok(())
}

fn named_identities(scene: &NamedSourceScene) -> Vec<*const martensite_tkv3d::ais::AisShape> {
    scene
        .named_objects()
        .map(|(_, object)| object.as_ptr())
        .collect()
}

fn require_distinct_identities(objects: &[*const martensite_tkv3d::ais::AisShape]) {
    let unique: std::collections::HashSet<_> = objects.iter().copied().collect();
    assert_eq!(
        unique.len(),
        objects.len(),
        "AIS pointers={objects:?}; expected distinct presentations"
    );
}

fn require_named_modes(scene: &NamedSourceScene, expected: &[&str], mode: i32) {
    let names: Vec<&str> = scene.named_objects().map(|(name, _)| name).collect();
    assert_eq!(names, expected, "named order differs from expected batch");
    for (name, object) in scene.named_objects() {
        assert!(
            !object.has_display_mode(),
            "object={name}; expected no own mode"
        );
        require_context_mode(scene, name, object, mode);
    }
}

fn require_context_mode(
    scene: &NamedSourceScene,
    name: &str,
    object: &martensite_tkernel::standard::Handle<martensite_tkv3d::ais::AisShape>,
    mode: i32,
) {
    let displayed = scene.displayed_in_mode(object, mode);
    let other = scene.displayed_in_mode(object, 1 - mode);
    assert!(
        displayed,
        "object={name}, displayed={displayed}; expected mode={mode}"
    );
    assert!(
        !other,
        "object={name}, other={other}; unexpected mode={}",
        1 - mode
    );
}
