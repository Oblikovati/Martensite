//! The three actual AIS boxes, local transforms and per-object drawers of OCC27818.
use super::*;
use crate::primitive_shapes::box_sized;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Transform3;
#[cfg(test)]
use martensite_tkservice::graphic3d::Graphic3dPresentationAttributesMembers as _;
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ZLAYER_ID_TOPMOST, Graphic3dPresentationAttributesSource as _,
};
use martensite_tkv3d::{
    ais::{AisInteractiveObject, AisShape},
    prs_mgr::{PrsMgrDisplayStatus, PrsMgrPresentableObjectMembers as _},
    prs3d::Prs3dDrawer,
};

type Interactive = Handle<dyn AisInteractiveObject>;

fn translated_box(index: i32) -> Result<Interactive, Failure> {
    let object: Interactive = AisShape::new(&box_sized([20., 20., 20.])?).cast();
    let offset = 30. * f64::from(index);
    object.set_local_transformation(&Transform3::translation([offset, offset, 0.]))?;
    object.set_hilight_mode(1);
    Ok(object)
}

fn object_style(transparency: f32) -> Handle<Prs3dDrawer> {
    let style = Handle::new(Prs3dDrawer::default());
    let drawer = style
        .get()
        .expect("new highlight drawer; expected a live drawer");
    drawer.set_display_mode(1);
    drawer.set_color(Color::named(NamedColor::RED));
    drawer.set_transparency(transparency);
    style
}

fn three_boxes(session: &ViewerSession) -> Result<Vec<Interactive>, Failure> {
    let boxes = (0..3).map(translated_box).collect::<Result<Vec<_>, _>>()?;
    boxes[1].set_dynamic_hilight_attributes(&object_style(0.8));
    let selected = object_style(0.);
    selected
        .get()
        .unwrap()
        .set_z_layer(GRAPHIC3D_ZLAYER_ID_TOPMOST);
    boxes[2].set_hilight_attributes(&selected);
    for object in &boxes {
        session
            .context()
            .display_with_modes(object, 1, 0, false, PrsMgrDisplayStatus::None)?;
    }
    session.context().update_current_viewer();
    Ok(boxes)
}

fn dynamic_colour_error(pixel: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let (colour, alpha) = pixel
        .split_once(' ')
        .ok_or_else(|| format!("pixel {pixel:?}; expected NAME alpha"))?;
    let alpha = alpha.parse::<f64>().map_err(|cause| {
        format!("pixel {pixel:?} alpha {alpha:?}; expected NAME numeric-alpha: {cause}")
    })?;
    Ok(colour != "CHOCOLATE3" && (alpha - 1.).abs() < 0.1)
}

fn dynamic_assertion(session: &mut ViewerSession) -> Outcome {
    session.move_cursor(200, 200)?;
    let pixel = session.pixel_rgba_name(200, 200)?;
    if dynamic_colour_error(&pixel)? {
        println!("ERROR: dynamic highlight of 2nd box is displayed with wrong color!");
    }
    Ok(())
}

fn select_three(session: &mut ViewerSession) -> Outcome {
    use martensite_tkv3d::ais::AisSelectionScheme;
    for pixel in [(50, 200), (200, 200), (350, 200)] {
        session.select_polyline(&[pixel], AisSelectionScheme::Xor)?;
    }
    let wrong = session.pixel_rgba_name(350, 200)? != "RED2 1"
        || session.pixel_rgba_name(350, 200)? == session.pixel_rgba_name(200, 200)?;
    if wrong {
        println!("ERROR: selection highlight of 3rd box is displayed with wrong color!");
    }
    Ok(())
}

/// Preserve the private command's objects and both original images; `object_highlights()?`.
pub(crate) fn object_highlights() -> Outcome {
    banner(&["============", "OCC27818_2", "============", ""]);
    let mut run = SessionRun::new()?;
    let _objects = three_boxes(run.session())?;
    run.session().fit()?;
    dynamic_assertion(run.session())?;
    run.dump("bug27818_2_dyn.png")?;
    select_three(run.session())?;
    run.dump("bug27818_2_sel.png")?;
    run.finish()
}

#[test]
fn object_dynamic_predicate_preserves_the_case_conjunction() {
    assert!(!dynamic_colour_error("CHOCOLATE3 1").unwrap());
    assert!(!dynamic_colour_error("RED 0.8").unwrap());
    assert!(dynamic_colour_error("RED 1").unwrap());
    assert!(dynamic_colour_error("not-a-pixel").is_err());
}

#[test]
fn object_boxes_preserve_local_transforms_and_actual_displayed_owners() {
    let session = null_highlight_session();
    let boxes = three_boxes(&session).unwrap();
    assert_eq!(boxes.len(), 3);
    for (index, object) in boxes.iter().enumerate() {
        let transform = object.local_transformation();
        assert_eq!(transform.value(1, 4).unwrap(), 30. * index as f64);
        assert_eq!(transform.value(2, 4).unwrap(), 30. * index as f64);
        assert_eq!(transform.value(3, 4).unwrap(), 0.);
        assert_eq!(object.hilight_mode(), 1);
        assert!(session.context().is_displayed(object));
    }
}

#[test]
fn object_styles_preserve_distinct_dynamic_and_selection_drawers() {
    let session = null_highlight_session();
    let boxes = three_boxes(&session).unwrap();
    let dynamic = boxes[1].dynamic_hilight_attributes();
    let selected = boxes[2].hilight_attributes();
    assert_ne!(dynamic, selected);
    assert_eq!(dynamic.get().unwrap().transparency(), 0.8);
    assert_eq!(selected.get().unwrap().transparency(), 0.);
    assert_eq!(
        selected.get().unwrap().z_layer(),
        GRAPHIC3D_ZLAYER_ID_TOPMOST
    );
    assert_eq!(
        dynamic.get().unwrap().color(),
        Color::named(NamedColor::RED)
    );
    assert_eq!(
        selected.get().unwrap().color(),
        Color::named(NamedColor::RED)
    );
}
