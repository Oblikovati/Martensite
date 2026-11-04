//! Mouse drags of attached objects: a child under a turned parent, a near-half-turn rotation
//! in steps, and drags in a 2D view and under pan and zoom gestures.
use super::prelude::*;
use crate::primitive_shapes::{box_at, box_sized, cylinder};
use martensite_tkservice::aspect::ASPECT_VKEY_MOUSE_LEFT_BUTTON;
use martensite_tkv3d::ais::AisMouseGesture;
use martensite_tkv3d::v3d::V3dTypeOfOrientation as Orientation;

/// `bug30669`: dragging a child moves it in its parent's turned frame.
pub(crate) fn child_drag() -> Outcome {
    banner(&[
        "=============",
        "0030669: Visualization - AisManipulator ignores parent transformation",
        "=============",
    ]);
    let (b1, b2) = (
        box_sized([1., 2., 3.])?,
        box_at([2., 0., 0.], [2., 1., 3.])?,
    );
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    display_point(&mut run, "p0", [0.; 3])?;
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().display("b1", &b1, &shaded)?;
    run.session().display("b2", &b2, &shaded)?;
    run.session().add_child("b1", "b2")?;
    run.session()
        .set_location_rotation("b1", [1., 0., 0., 1.])?;
    run.session()
        .set_location_translation("b1", [4., -2., 0.])?;
    run.session().fit()?;
    run.session().manipulate("m", &attach("b2"))?;
    drag(&mut run, "m", (265, 265), (265, 200))?;
    run.session().select_point(0, 0)?;
    if run.session().location_text("b2")? != "0 1.80711 0" {
        println!("Error: wrong location");
    }
    run.dump("bug30669.png")?;
    run.finish()
}

/// `rotate2`: a rotation dragged in small steps towards half a turn.
pub(crate) fn stepped_rotation() -> Outcome {
    banner(&[
        "==================================",
        "0029084: Visualization, AisManipulator - broken transformation is applied at Rotation angles near to Pi",
        "==================================",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().display(
        "b",
        &box_sized([1., 2., 3.])?,
        &DisplayRequest::default().with_display_mode(0),
    )?;
    display_point(&mut run, "p0", [0.; 3])?;
    run.session()
        .set_standard_projection(Orientation::ZUP_TOP)?;
    run.session().fit()?;
    run.session().manipulate("m", &attach("b"))?;
    run.session().move_cursor(135, 275)?;
    run.session().select_point(135, 275)?;
    drag_step(
        &mut run,
        ManipulatorDrag {
            start: Some((135, 275)),
            ..Default::default()
        },
    )?;
    run.dump("rotate2_0.png")?;
    for x in (250..400).step_by(5) {
        drag_step(
            &mut run,
            ManipulatorDrag {
                to: Some((x, 400)),
                ..Default::default()
            },
        )?;
    }
    run.dump("rotate2_1.png")?;
    for y in (301..=400).rev() {
        drag_step(
            &mut run,
            ManipulatorDrag {
                to: Some((400, y)),
                ..Default::default()
            },
        )?;
    }
    run.dump("rotate2_2.png")?;
    drag_step(
        &mut run,
        ManipulatorDrag {
            stop: Some(true),
            ..Default::default()
        },
    )?;
    run.finish()
}

fn drag_step(run: &mut SessionRun, step: ManipulatorDrag) -> Outcome {
    run.session().manipulate(
        "m",
        &ManipulatorCommand {
            drag: step,
            ..Default::default()
        },
    )?;
    Ok(())
}

/// `drag_in_2d_view`: a translation drag where the view input does not rotate.
pub(crate) fn drag_in_2d() -> Outcome {
    banner(&[
        "==============================================",
        "0032861: Visualization - Separate rotate and move the object behavior in AisViewController",
        "==============================================",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().set_2d_input();
    run.session()
        .display("b", &box_sized([10.; 3])?, &DisplayRequest::default())?;
    run.session().set_axonometric()?;
    run.session().fit()?;
    run.session().zoom(0.5)?;
    run.session().manipulate("m", &attach("b"))?;
    run.dump("drag_in_2d_view_1.png")?;
    drag(&mut run, "m", (231, 207), (311, 258))?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(311, 258)?;
    run.dump("drag_in_2d_view_2.png")?;
    run.session().location_text("b")?;
    run.finish()
}

/// `drag_pan_zoom`: manipulator drags while the left button pans, then zooms.
pub(crate) fn gesture_drags() -> Outcome {
    banner(&[
        "==============================================",
        "0032879: Visualization - Separate pan/zoom and move the object behavior in AisViewController",
        "==============================================",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session()
        .display("c1", &cylinder(5., 10.)?, &DisplayRequest::default())?;
    run.session().set_display_mode(1)?;
    let sized = ManipulatorCommand {
        placement: Some(ManipulatorPlacement::Center),
        adjust_size: Some(false),
        enable_modes: Some(true),
        size: Some(40.),
        ..attach("c1")
    };
    run.session().manipulate("m", &sized)?;
    run.session().fit()?;
    let steps = [
        (
            AisMouseGesture::Pan,
            (204, 194),
            (369, 35),
            "drag_pan_zoom_pan.png",
        ),
        (
            AisMouseGesture::Zoom,
            (206, 32),
            (365, 330),
            "drag_pan_zoom_zoom.png",
        ),
    ];
    for (gesture, from, to, image) in steps {
        run.session()
            .bind_mouse_gesture(ASPECT_VKEY_MOUSE_LEFT_BUTTON, gesture);
        drag(&mut run, "m", from, to)?;
        run.session().select_point(0, 0)?;
        run.dump(image)?;
    }
    run.finish()
}
