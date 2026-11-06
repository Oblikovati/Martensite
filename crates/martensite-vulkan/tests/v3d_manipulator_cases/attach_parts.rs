//! Attaching with parts switched off, placing at a shape location, per-view display and
//! a manipulator that keeps its place under an axial scale.
use super::prelude::*;
use crate::case_view::viewer_session::{LocationChange, ViewParameters, ViewTrihedronSettings};
use crate::primitive_shapes::{box_at, box_sized, translated};

/// `disable_parts`: translation, rotation and scaling parts of axis 0 switched off in turn,
/// then every translation part.
pub(crate) fn disabled_parts() -> Outcome {
    banner(&[
        "=====================================",
        "Disable functional parts for manipulator axes",
        "=====================================",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session()
        .show_view_trihedron(&ViewTrihedronSettings::default())?;
    let b = box_at([500., 500., 1.], [50., 100., 150.])?;
    run.session()
        .display("b", &b, &DisplayRequest::default().with_display_mode(1))?;
    run.session().fit()?;
    let parts = [
        (
            Some(0),
            AisManipulatorMode::Translation,
            "disable_parts_1.png",
        ),
        (Some(0), AisManipulatorMode::Rotation, "disable_parts_2.png"),
        (Some(0), AisManipulatorMode::Scaling, "disable_parts_3.png"),
        (None, AisManipulatorMode::Translation, "disable_parts_4.png"),
    ];
    for (index, (axis, mode, image)) in parts.into_iter().enumerate() {
        if index > 0 {
            run.session().manipulate("m", &detach())?;
        }
        let part = ManipulatorPart {
            axis,
            mode,
            enabled: false,
        };
        run.session().manipulate(
            "m",
            &ManipulatorCommand {
                parts: vec![part],
                ..attach("b")
            },
        )?;
        run.dump(image)?;
    }
    run.finish()
}

/// `shape_location`: a manipulator placed at the shape's own location times its object's.
pub(crate) fn located_shape() -> Outcome {
    banner(&[
        "===============================================",
        "0032055: Test Commands, ViewerCommands - add more vmanipulator position adjusting options",
        "===============================================",
    ]);
    let b = translated(&box_sized([1., 2., 3.])?, [1., 0., 0.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .display("b", &b, &DisplayRequest::default().with_display_mode(1))?;
    run.session().set_location_translation("b", [5., 0., 0.])?;
    let turn = LocationChange::Rotate {
        origin: [5., 0., 0.],
        axis: [0., 0., 1.],
        degrees: 30.,
    };
    run.session().change_location("b", turn)?;
    display_point(&mut run, "p", [0.; 3])?;
    run.session().fit()?;
    let off = ManipulatorCommand {
        placement: Some(ManipulatorPlacement::Off),
        ..attach("b")
    };
    run.session().manipulate("m", &off)?;
    let shape = ManipulatorCommand {
        placement: Some(ManipulatorPlacement::ShapeLocation),
        ..Default::default()
    };
    run.session().manipulate("m", &shape)?;
    run.dump("shape_location.png")?;
    run.finish()
}

/// `view_affinity`: a manipulator shown in one of two views, then in the other.
pub(crate) fn view_affinity() -> Outcome {
    banner(&[
        "=====================================",
        "Display manipulator in one view only",
        "=====================================",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().open_view(409, 409)?;
    run.session().set_axonometric()?;
    let b = box_at([500., 500., 1.], [50., 100., 150.])?;
    run.session()
        .display("b", &b, &DisplayRequest::default().with_display_mode(1))?;
    run.session().fit()?;
    run.session().activate_view(0)?;
    run.session().manipulate(
        "m",
        &ManipulatorCommand {
            view: Some(0),
            ..attach("b")
        },
    )?;
    run.session().fit()?;
    run.session().move_cursor(200, 200)?;
    run.dump("view_affinity_View1_1.png")?;
    run.session().activate_view(1)?;
    run.dump("view_affinity_View2_1.png")?;
    run.session().manipulate("m", &detach())?;
    run.session().manipulate(
        "m",
        &ManipulatorCommand {
            view: Some(1),
            ..attach("b")
        },
    )?;
    run.session().move_cursor(200, 200)?;
    run.dump("view_affinity_View2_2.png")?;
    run.session().activate_view(0)?;
    run.dump("view_affinity_View1_2.png")?;
    run.session().manipulate("m", &detach())?;
    run.finish()
}

/// `bug33514`: the manipulator keeps its place under an axial scale, fixed-size and zoomable.
pub(crate) fn held_trihedron() -> Outcome {
    banner(&[
        "========",
        "0033514: Visualization - Scaled view twists zoom persistence objects",
        "========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    for (zoomable, scale, image) in [
        (None, 6.28866, "bug33514.png"),
        (Some(true), 4.28866, "bug33514_zoomable.png"),
    ] {
        run.session().clear()?;
        run.session()
            .display("b", &box_sized([10.; 3])?, &DisplayRequest::default())?;
        run.session().fit()?;
        run.session().set_display_mode(1)?;
        run.session().manipulate(
            "vm",
            &ManipulatorCommand {
                zoomable,
                ..attach("b")
            },
        )?;
        run.session()
            .view()
            .view_base()
            .set_axial_scale(1., 3., 10.)?;
        let parameters = ViewParameters {
            scale: Some(scale),
            proj: Some([0.57735, -0.57735, 0.57735]),
            up: Some([-0.408248, 0.408248, 0.816497]),
            at: Some([0., 10., 30.]),
            eye: None,
        };
        run.session().set_view_parameters(&parameters)?;
        if zoomable.is_none() {
            expect_colours(&mut run)?;
        }
        run.dump(image)?;
    }
    run.finish()
}

fn expect_colours(run: &mut SessionRun) -> Outcome {
    for (x, y, expected) in [
        (235, 170, "RED3"),
        (223, 155, "BLUE3"),
        (235, 155, "GREEN3"),
    ] {
        let colour = run.session().pixel_color_name(x, y)?;
        if colour != expected {
            println!("ERROR: trihedron does not maintain position");
            println!("       additional investigation is needed");
            println!("       expected color is: {expected}");
            println!("       current color is:  {colour}");
        }
    }
    Ok(())
}
