//! Two authored manipulator regressions omitted by the older wide host.
use super::prelude::*;
use crate::primitive_shapes::{box_at, box_sized};
use martensite_tkservice::graphic3d::Graphic3dTypeOfShadingModel;
use martensite_tkv3d::v3d::V3dTypeOfOrientation as Orientation;

/// Replay the moved-box picking program; `moved_box_selection()?`.
pub(crate) fn moved_box_selection() -> Outcome {
    banner(&[
        "========",
        "0032750: Visualization, AisManipulator - selection of moved object is broken",
        "========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    moved_box_setup(&mut run)?;
    drag(&mut run, "m", (206, 155), (206, 55))?;
    run.session().select_point(0, 0)?;
    run.session().remove(&["m"])?;
    moved_box_images(&mut run)?;
    run.finish()
}

fn moved_box_setup(run: &mut SessionRun) -> Outcome {
    run.session()
        .display("b", &box_sized([100.; 3])?, &DisplayRequest::default())?;
    run.session().set_display_mode(1)?;
    run.session()
        .set_shading_model(Graphic3dTypeOfShadingModel::Unlit)?;
    run.session()
        .set_standard_projection(Orientation::ZUP_FRONT)?;
    run.session().fit()?;
    run.session().zoom(0.5)?;
    run.session().manipulate("m", &attach("b"))?;
    Ok(())
}

fn moved_box_images(run: &mut SessionRun) -> Outcome {
    for (point, image) in [
        ((0, 0), "bug32750_unselected.png"),
        ((250, 10), "bug32750_selectTop.png"),
        ((250, 200), "bug32750_selectBot.png"),
    ] {
        run.session().move_cursor(point.0, point.1)?;
        run.dump(image)?;
    }
    Ok(())
}

/// Replay two separately anchored zoom-persistent attachments; `persistent_pair_drag()?`.
pub(crate) fn persistent_pair_drag() -> Outcome {
    banner(&[
        "============",
        "0032116: Visualization - AisManipulator is unusable when attaching to objects with Graphic3dTMF_ZoomPers",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    persistent_boxes(&mut run)?;
    run.session().fit()?;
    persistent_drag(&mut run)?;
    run.dump("bug32116_1.png")?;
    run.session().move_cursor(0, 0)?;
    run.session().zoom(0.2)?;
    persistent_zoom_checks(&mut run)?;
    run.dump("bug32116_2.png")?;
    run.finish()
}

fn persistent_boxes(run: &mut SessionRun) -> Outcome {
    for (name, corner, anchor) in [("b1", [10.; 3], [0.; 3]), ("b2", [50.; 3], [10.; 3])] {
        let request = DisplayRequest::zoom_persistent(anchor).with_display_mode(1);
        run.session()
            .display(name, &box_at(corner, [10., 20., 30.])?, &request)?;
    }
    Ok(())
}

fn assert_named_pixels(run: &mut SessionRun, checks: &[(i32, i32, &str, &str)]) -> Outcome {
    for &(x, y, expected, message) in checks {
        if run.session().pixel_color_name(x, y)? != expected {
            println!("Error: wrong {message}");
        }
    }
    Ok(())
}

fn persistent_drag(run: &mut SessionRun) -> Outcome {
    for name in ["b1", "b2"] {
        let command = ManipulatorCommand {
            add_object: true,
            ..attach(name)
        };
        run.session().manipulate("m", &command)?;
    }
    drag(run, "m", (90, 225), (90, 150))?;
    run.session().move_cursor(90, 150)?;
    let checks = [
        (90, 150, "CYAN", "manipulator position"),
        (115, 170, "DARKGOLDENROD", "b1 transformation"),
        (400, 10, "DARKGOLDENROD", "b2 transformation"),
    ];
    assert_named_pixels(run, &checks)
}

fn persistent_zoom_checks(run: &mut SessionRun) -> Outcome {
    let checks = [
        (210, 170, "DARKGOLDENROD", "b1 transformation"),
        (310, 120, "DARKGOLDENROD", "b2 transformation"),
    ];
    assert_named_pixels(run, &checks)
}
