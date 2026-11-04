//! Manipulators under a changed camera: a translation under a perspective axonometric view
//! with some parts and selection modes switched off, the flat skin after a mouse rotation, and
//! the zoom persistence option.
use super::prelude::*;
use crate::case_view::viewer_session::ViewTrihedronSettings;
use crate::primitive_shapes::box_at;
use martensite_tkservice::graphic3d::Graphic3dCameraProjection;
use martensite_tkv3d::v3d::V3dTypeOfOrientation as Orientation;

/// Disable each (axis, mode) pair, in order.
fn disable_parts(run: &mut SessionRun, parts: &[(i32, AisManipulatorMode)]) -> Outcome {
    for &(axis, mode) in parts {
        let command = ManipulatorCommand {
            parts: vec![ManipulatorPart {
                axis: Some(axis),
                mode,
                enabled: false,
            }],
            ..Default::default()
        };
        run.session().manipulate("m", &command)?;
    }
    Ok(())
}

/// The `Location:` translation of `b` is 46 along X (within 0.1) and exactly 0 along Y
/// and Z.
fn translated_by_46(text: &str) -> bool {
    let values: Vec<f64> = text
        .split_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect();
    matches!(values[..], [x, y, z] if (x - 46.).abs() <= 0.1 && y == 0. && z == 0.)
}

/// `translate2`: dragging the X translation arrow moves the box along +X in a perspective view.
pub(crate) fn turned_camera_translation() -> Outcome {
    use AisManipulatorMode::{Rotation, Scaling, Translation};
    banner(&[
        "=====================================",
        "0029885: Visualization, AisManipulator - Translation is done in wrong direction depending on Camera orientation",
        "=====================================",
    ]);
    let mut run = SessionRun::new()?;
    let b = box_at([500., 500., 1.], [50., 100., 150.])?;
    run.session().clear()?;
    run.session()
        .show_view_trihedron(&ViewTrihedronSettings::default())?;
    run.session()
        .set_projection(Graphic3dCameraProjection::Perspective)?;
    run.session().set_axonometric()?;
    run.session()
        .display("b", &b, &DisplayRequest::default().with_display_mode(1))?;
    run.session().fit()?;
    run.session().manipulate("m", &attach("b"))?;
    let parts = [
        (0, Rotation),
        (1, Rotation),
        (2, Rotation),
        (0, Scaling),
        (1, Scaling),
        (2, Scaling),
        (2, Translation),
    ];
    disable_parts(&mut run, &parts)?;
    run.session().set_object_selection_mode("m", 2, false)?;
    run.session().set_object_selection_mode("m", 3, false)?;
    drag(&mut run, "m", (245, 220), (340, 265))?;
    if run.session().pixel_color_name(370, 300)? == "BLACK" {
        println!("Error: wrong translation");
    }
    if !translated_by_46(&run.session().location_text("b")?) {
        println!("Error: wrong translation");
    }
    run.dump("translate2.png")?;
    run.finish()
}

/// `flat`: the flat, zoomable skin on a box seen from the right after a mouse rotation.
pub(crate) fn flat_skin() -> Outcome {
    banner(&[
        "===========================",
        "AisManipulator - flat skin",
        "===========================",
    ]);
    let mut run = SessionRun::new()?;
    let b = box_at([0.; 3], [20.; 3])?;
    run.session()
        .display("b", &b, &DisplayRequest::default().with_display_mode(1))?;
    run.session()
        .set_standard_projection(Orientation::ZUP_RIGHT)?;
    run.session().mouse_rotate((400, 200), (300, 300))?;
    run.session().zoom(3.)?;
    let skin = ManipulatorCommand {
        zoomable: Some(true),
        flat: Some(true),
        ..attach("b")
    };
    run.session().manipulate("m", &skin)?;
    hover(&mut run, (180, 215))?;
    run.dump("flat_1.png")?;
    selected_drag(&mut run, "m", (180, 215), (65, 350))?;
    run.session().select_point(0, 0)?;
    run.dump("flat_2.png")?;
    hover(&mut run, (115, 280))?;
    run.dump("flat_3.png")?;
    selected_drag(&mut run, "m", (115, 280), (320, 40))?;
    run.session().select_point(0, 0)?;
    run.finish()
}

/// One zoom persistence step: hover and dump, zoom by `factor`, hover and dump, then drag
/// from the second hover and dump.
fn zoomed_drag(
    run: &mut SessionRun,
    name: &str,
    hovers: [(i32, i32); 2],
    factor: f64,
    to: (i32, i32),
    images: [&'static str; 3],
) -> Outcome {
    hover(run, hovers[0])?;
    run.dump(images[0])?;
    run.session().zoom(factor)?;
    hover(run, hovers[1])?;
    run.dump(images[1])?;
    selected_drag(run, name, hovers[1], to)?;
    run.session().select_point(0, 0)?;
    run.dump(images[2])
}

/// `zoom_persistence`: a zoomable manipulator keeps its screen size under zoom, a fixed
/// one of size 40 scales with the view.
pub(crate) fn zoom_persistence() -> Outcome {
    banner(&[
        "===============================================",
        "AisManipulator - check zoom persistence option",
        "===============================================",
    ]);
    let mut run = SessionRun::new()?;
    let (b1, b2) = (box_at([0.; 3], [20.; 3])?, box_at([80., 0., 0.], [20.; 3])?);
    run.session()
        .display("b1", &b1, &DisplayRequest::default())?;
    run.session()
        .display("b2", &b2, &DisplayRequest::default())?;
    run.session().display_trihedron("tri")?;
    run.session().set_display_mode(1)?;
    run.session().set_axonometric()?;
    run.session().fit()?;
    let zoomable = ManipulatorCommand {
        placement: Some(ManipulatorPlacement::Center),
        adjust_size: Some(true),
        enable_modes: Some(true),
        zoomable: Some(true),
        ..attach("b1")
    };
    run.session().manipulate("m1", &zoomable)?;
    let images = [
        "zoom_persistence_1.png",
        "zoom_persistence_2.png",
        "zoom_persistence_3.png",
    ];
    zoomed_drag(
        &mut run,
        "m1",
        [(80, 133), (29, 103)],
        1.5,
        (121, 126),
        images,
    )?;
    run.session().fit()?;
    let fixed = ManipulatorCommand {
        placement: Some(ManipulatorPlacement::Center),
        adjust_size: Some(false),
        enable_modes: Some(true),
        zoomable: Some(false),
        size: Some(40.),
        ..attach("b2")
    };
    run.session().manipulate("m2", &fixed)?;
    let images = [
        "zoom_persistence_4.png",
        "zoom_persistence_5.png",
        "zoom_persistence_6.png",
    ];
    zoomed_drag(
        &mut run,
        "m2",
        [(340, 282), (277, 245)],
        0.5,
        (210, 210),
        images,
    )?;
    run.finish()
}
