//! Dragging, rotating, scaling and translating attached cylinders through the manipulator's
//! parts, each drag released and followed by a hover at its end point.
use super::prelude::*;
use crate::primitive_shapes::{compound_of, cylinder, rotated_about, translated};
use martensite_tkbrep::topods::Shape;

/// The pivot of the two rotations that tilt the small cylinder.
const PIVOT: [f64; 3] = [100., 0., 25.];

/// A cylinder of height 10, moved by `offset`, then turned 90 degrees about Y and 10 about Z
/// through the pivot.
fn tilted_cylinder(radius: f64, offset: [f64; 3]) -> Result<Shape, Box<dyn Error>> {
    let moved = translated(&cylinder(radius, 10.)?, offset)?;
    let turned = rotated_about(&moved, PIVOT, [0., 1., 0.], 90.)?;
    Ok(rotated_about(&turned, PIVOT, [0., 0., 1.], 10.)?)
}

/// `c1` (the tilted cylinder at `offset`) and the upright `c2`.
fn single_pair(offset: [f64; 3]) -> Result<[Shape; 2], Box<dyn Error>> {
    Ok([
        tilted_cylinder(5., offset)?,
        translated(&cylinder(10., 20.)?, [100., 0., 0.])?,
    ])
}

/// `c1` as the compound of an upright and a tilted cylinder, and the upright `c2`.
fn compound_pair() -> Result<[Shape; 2], Box<dyn Error>> {
    let upright = translated(&cylinder(5., 10.)?, [100., 0., 20.])?;
    let tilted = tilted_cylinder(3., [100., 0., 20.])?;
    Ok([
        compound_of(&[upright, tilted])?,
        translated(&cylinder(10., 20.)?, [100., 0., 0.])?,
    ])
}

/// Display `c1` and `c2` shaded, axonometric and fitted, then
/// attach the manipulator `m` to `c1`, adjusted in position only, all modes on, size 40.
fn show_and_attach(run: &mut SessionRun, [c1, c2]: &[Shape; 2]) -> Outcome {
    run.session()
        .display("c1", c1, &DisplayRequest::default())?;
    run.session()
        .display("c2", c2, &DisplayRequest::default())?;
    run.session().set_display_mode(1)?;
    run.session().set_axonometric()?;
    run.session().fit()?;
    run.session().manipulate(
        "m",
        &ManipulatorCommand {
            placement: Some(ManipulatorPlacement::Center),
            adjust_size: Some(false),
            enable_modes: Some(true),
            size: Some(40.),
            ..attach("c1")
        },
    )?;
    Ok(())
}

/// Make `m` follow rotation and, when `translation`, translation.
fn follow(run: &mut SessionRun, translation: bool) -> Outcome {
    let rotation = ManipulatorCommand {
        follow_rotation: Some(true),
        ..Default::default()
    };
    run.session().manipulate("m", &rotation)?;
    if translation {
        let moving = ManipulatorCommand {
            follow_translation: Some(true),
            ..Default::default()
        };
        run.session().manipulate("m", &moving)?;
    }
    Ok(())
}

/// Disable mode 4 for each axis: no plane translation parts.
fn without_plane_parts(run: &mut SessionRun) -> Outcome {
    let parts = (0..3).map(|axis| ManipulatorPart {
        axis: Some(axis),
        mode: AisManipulatorMode::TranslationPlane,
        enabled: false,
    });
    for part in parts {
        let command = ManipulatorCommand {
            parts: vec![part],
            ..Default::default()
        };
        run.session().manipulate("m", &command)?;
    }
    Ok(())
}

/// One released drag: from, to and the image dumped after it.
type DumpedDrag = ((i32, i32), (i32, i32), &'static str);

/// A drag from `from` to `to`, then clear the selection and move to `to`.
fn drag_released(run: &mut SessionRun, from: (i32, i32), to: (i32, i32)) -> Outcome {
    drag(run, "m", from, to)?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(to.0, to.1)?;
    Ok(())
}

/// Each drag, released, followed by the dump of its image.
fn drags_dumped(run: &mut SessionRun, steps: &[DumpedDrag]) -> Outcome {
    for &(from, to, image) in steps {
        drag_released(run, from, to)?;
        run.dump(image)?;
    }
    Ok(())
}

/// Remove every named object, the manipulator included.
fn remove_all(run: &mut SessionRun) -> Outcome {
    run.session().remove(&["c1", "c2", "m"])?;
    Ok(())
}

/// `dragg`: an attached cylinder dragged through four parts, following rotation and translation.
pub(crate) fn dragged_object() -> Outcome {
    banner(&[
        "=====================================",
        "AisManipulator - drag an object",
        "=====================================",
    ]);
    let mut run = SessionRun::new()?;
    show_and_attach(&mut run, &single_pair([105., 0., 20.])?)?;
    follow(&mut run, true)?;
    run.dump("dragg_1.png")?;
    drags_dumped(
        &mut run,
        &[
            ((211, 129), (284, 248), "dragg_2.png"),
            ((316, 261), (281, 286), "dragg_3.png"),
            ((278, 267), (156, 276), "dragg_4.png"),
            ((168, 258), (166, 182), "dragg_5.png"),
        ],
    )?;
    run.finish()
}

/// `rotate`: rotations following the manipulator, then, on a fresh scene, without it.
pub(crate) fn rotated_object() -> Outcome {
    banner(&[
        "==================================",
        "AisManipulator - rotate an object",
        "==================================",
    ]);
    let mut run = SessionRun::new()?;
    let shapes = compound_pair()?;
    show_and_attach(&mut run, &shapes)?;
    let following = [
        ((199, 92), (176, 142), "rotate_1.png"),
        ((175, 135), (232, 144), "rotate_2.png"),
        ((200, 94), (182, 117), "rotate_3.png"),
    ];
    for step in following {
        follow(&mut run, false)?;
        drags_dumped(&mut run, &[step])?;
    }
    remove_all(&mut run)?;
    show_and_attach(&mut run, &shapes)?;
    let fixed = ManipulatorCommand {
        follow_rotation: Some(false),
        ..Default::default()
    };
    run.session().manipulate("m", &fixed)?;
    drags_dumped(
        &mut run,
        &[
            ((205, 88), (232, 127), "rotate_4.png"),
            ((228, 142), (184, 143), "rotate_5.png"),
        ],
    )?;
    run.finish()
}

/// `scale`: a hover over the scaling part, then a scaling drag left without a final hover.
pub(crate) fn scaled_object() -> Outcome {
    banner(&[
        "=================================",
        "AisManipulator - scale an object",
        "=================================",
    ]);
    let mut run = SessionRun::new()?;
    show_and_attach(&mut run, &compound_pair()?)?;
    run.session().move_cursor(229, 141)?;
    run.dump("scale_1.png")?;
    drag(&mut run, "m", (229, 141), (210, 129))?;
    run.session().select_point(0, 0)?;
    run.dump("scale_2.png")?;
    run.finish()
}

/// `translate`: translations without plane parts, then on a fresh scene two drags in a row.
pub(crate) fn translated_object() -> Outcome {
    banner(&[
        "=====================================",
        "AisManipulator - translate an object",
        "=====================================",
    ]);
    let mut run = SessionRun::new()?;
    let shapes = single_pair([100., 0., 20.])?;
    for fresh in [false, true] {
        if fresh {
            remove_all(&mut run)?;
        }
        show_and_attach(&mut run, &shapes)?;
        follow(&mut run, true)?;
        without_plane_parts(&mut run)?;
        if !fresh {
            drags_dumped(
                &mut run,
                &[
                    ((204, 112), (204, 156), "translate_1.png"),
                    ((215, 160), (265, 132), "translate_2.png"),
                    ((263, 145), (127, 65), "translate_3.png"),
                ],
            )?;
        }
    }
    drag_released(&mut run, (223, 144), (181, 140))?;
    drags_dumped(
        &mut run,
        &[
            ((199, 131), (175, 168), "translate_4.png"),
            ((199, 164), (246, 176), "translate_5.png"),
        ],
    )?;
    run.finish()
}
