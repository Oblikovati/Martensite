//! Camera animations of a shaded box under the view trihedron: rotation, zoom and panning.
use super::prelude::*;
use super::timeline::{play_at, schedule, timeline};
use crate::case_view::viewer_session::{ViewTrihedronSettings, ViewerSession};
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::aspect::AspectTypeOfTriedronPosition as Corner;
use martensite_tkservice::graphic3d::Graphic3dCamera;
use martensite_tkv3d::ais::AisAnimationCamera;

/// One camera setting of a view animation end, applied in command-line order.
#[derive(Clone, Copy)]
pub(super) enum CameraSet {
    Up([f64; 3]),
    At([f64; 3]),
    Eye([f64; 3]),
    Scale(f64),
}

/// A copy of the current camera with `settings` applied; the scale is relative to the
/// default camera's scale.
fn camera_end(
    view: &martensite_tkv3d::v3d::V3dView,
    settings: &[CameraSet],
) -> Result<Handle<Graphic3dCamera>, Failure> {
    let current = view.camera();
    let camera = Graphic3dCamera::from_camera(current.get().expect("a view camera"));
    let default_scale = view.default_camera().get().map_or(0.0, |c| c.scale());
    for setting in settings {
        match *setting {
            CameraSet::Up([x, y, z]) => camera.set_up(&Direction3::new(x, y, z)?),
            CameraSet::At([x, y, z]) => camera.set_center(&Point3::new(x, y, z))?,
            CameraSet::Eye([x, y, z]) => camera.set_eye(&Point3::new(x, y, z))?,
            CameraSet::Scale(scale) => camera.set_scale(default_scale / scale),
        }
    }
    Ok(Handle::new(camera))
}

/// The camera animation `movecam` of the active view between two ends.
pub(super) fn camera_move(
    session: &ViewerSession,
    name: &str,
    first: &[CameraSet],
    second: &[CameraSet],
) -> Result<Handle<AisAnimationCamera>, Failure> {
    let animation = AisAnimationCamera::new(name, session.view());
    let view = session.view().view_base();
    animation.set_camera_start(&camera_end(view, first)?);
    animation.set_camera_end(&camera_end(view, second)?);
    Ok(Handle::new(animation))
}

/// The shaded box `b1` with the view trihedron at `corner`, displayed without update.
fn shaded_box(run: &mut SessionRun, corner: Corner) -> Outcome {
    run.session().clear()?;
    run.session().set_axonometric()?;
    let settings = ViewTrihedronSettings {
        position: corner,
        ..Default::default()
    };
    run.session().show_view_trihedron(&settings)?;
    let request = DisplayRequest::default().with_display_mode(1);
    run.session().without_updates(|session| {
        session.display(
            "b1",
            &box_at([-50., -50., 0.], [100., 100., 100.])?,
            &request,
        )
    })?;
    Ok(())
}

/// Pixels with their expected colour names, and the image dumped after checking them.
type FrameCheck<'a> = (&'a [(i32, i32, &'a str)], &'static str);

/// Play `root` at 0, 1 and 2 seconds, checking the two pixels before each dump.
fn play_checked(
    mut run: SessionRun,
    root: &Handle<dyn martensite_tkv3d::ais::AisAnimationSource>,
    checks: [FrameCheck<'_>; 3],
    message: &str,
) -> Outcome {
    for (pts, (pixels, image)) in checks.into_iter().enumerate() {
        play_at(run.session(), root, pts as f64)?;
        let mut wrong = false;
        for &(x, y, name) in pixels {
            wrong |= run.session().pixel_color_name(x, y)? != name;
        }
        if wrong {
            println!("Error: {message}");
        }
        run.dump(image)?;
    }
    interactive_hint();
    run.finish()
}

/// `rotate`: the camera turns about the box centre.
pub(crate) fn camera_rotation() -> Outcome {
    banner(&[
        "==================================",
        "Viewer animation - rotate the view camera",
        "==================================",
    ]);
    let mut run = SessionRun::new()?;
    shaded_box(&mut run, Corner::LEFT_LOWER)?;
    run.session().view().fit_all(0.01, false)?;
    let first = [
        CameraSet::At([0., 0., 50.]),
        CameraSet::Eye([100., -100., 150.]),
    ];
    let second = [
        CameraSet::At([0., 0., 50.]),
        CameraSet::Eye([-153., -70., 8.]),
    ];
    let root = timeline("anim");
    schedule(
        &root,
        camera_move(run.session(), "movecam", &first, &second)?.cast(),
        0.,
        3.,
    )?;
    play_checked(
        run,
        &root,
        [
            (
                &[(306, 280, "DARKGOLDENROD"), (325, 280, "DARKGOLDENROD")],
                "rotate_0.png",
            ),
            (
                &[(306, 280, "DARKORANGE4"), (325, 280, "BLACK")],
                "rotate_1.png",
            ),
            (
                &[(306, 280, "GOLDENROD3"), (325, 280, "GOLDENROD3")],
                "rotate_2.png",
            ),
        ],
        "Camera rotate result is wrong!",
    )
}

/// `scale`: the camera zooms in on the box.
pub(crate) fn camera_zoom() -> Outcome {
    banner(&[
        "==================================",
        "Viewer animation - scale the view camera",
        "==================================",
    ]);
    let mut run = SessionRun::new()?;
    shaded_box(&mut run, Corner::LEFT_LOWER)?;
    run.session().view().fit_all(0.01, false)?;
    run.session().zoom(0.2)?;
    let root = timeline("anim");
    let zoom = camera_move(
        run.session(),
        "zoom",
        &[CameraSet::Scale(1.2)],
        &[CameraSet::Scale(4.8)],
    )?;
    schedule(&root, zoom.cast(), 0., 2.)?;
    play_checked(
        run,
        &root,
        [
            (
                &[(230, 220, "DARKGOLDENROD"), (250, 220, "BLACK")],
                "scale_0.png",
            ),
            (
                &[(250, 220, "DARKGOLDENROD"), (270, 220, "BLACK")],
                "scale_1.png",
            ),
            (
                &[(334, 220, "DARKGOLDENROD"), (350, 220, "BLACK")],
                "scale_2.png",
            ),
        ],
        "Camera scale result is wrong!",
    )
}

/// `translate`: the camera pans along the box.
pub(crate) fn camera_translation() -> Outcome {
    banner(&[
        "==================================",
        "Viewer animation - translate the view camera",
        "==================================",
    ]);
    let mut run = SessionRun::new()?;
    shaded_box(&mut run, Corner::RIGHT_LOWER)?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(1.156),
        up: Some([-0.3588, 0.3458, 0.867]),
        at: Some([116., 355., 327.]),
        eye: Some([225., 253., 413.]),
        ..Default::default()
    })?;
    let first = [
        CameraSet::At([116., 355., 327.]),
        CameraSet::Eye([225., 253., 413.]),
    ];
    let second = [
        CameraSet::At([-174., 47., 330.]),
        CameraSet::Eye([-65., -55., 415.]),
    ];
    let root = timeline("anim");
    schedule(
        &root,
        camera_move(run.session(), "movecam", &first, &second)?.cast(),
        0.,
        2.,
    )?;
    play_checked(
        run,
        &root,
        [
            (&[(60, 360, "DARKGOLDENROD")], "translate_0.png"),
            (&[(160, 360, "DARKGOLDENROD")], "translate_1.png"),
            (&[(260, 360, "DARKGOLDENROD")], "translate_2.png"),
        ],
        "Camera translation result is wrong!",
    )
}
