//! Actual AIS registry witnesses keep the fresh-view command separate from general scene clearing.
use super::super::super::device_options;
use super::*;

#[test]
fn actual_fresh_namespace_clear_keeps_zero_registered_objects()
-> Result<(), Box<dyn std::error::Error>> {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options())?;
    let stage = HatchSourceView::new(&driver)?;
    stage.source_clear()?;
    assert_eq!(registered_count(&stage)?, 0);
    stage.close();
    Ok(())
}

#[test]
fn nonempty_context_is_refused_without_erasing_its_actual_object()
-> Result<(), Box<dyn std::error::Error>> {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options())?;
    let stage = HatchSourceView::new(&driver)?;
    let object = AisShape::new(&super::super::unit_box_shape());
    stage
        .context
        .display_with_modes(&object.cast(), 1, -1, false, PrsMgrDisplayStatus::None)?;
    let error = stage.source_clear().unwrap_err();
    assert!(
        error
            .to_string()
            .contains("registered_objects=1; expected fresh empty AIS namespace")
    );
    assert_eq!(registered_count(&stage)?, 1);
    stage.close();
    Ok(())
}

fn registered_count(stage: &HatchSourceView) -> Result<i32, Failure> {
    let mut objects = martensite_tkernel::ncollection::List::default();
    stage
        .context
        .objects_inside(&mut objects, AisKindOfInteractive::None, -1)?;
    Ok(objects.extent())
}
