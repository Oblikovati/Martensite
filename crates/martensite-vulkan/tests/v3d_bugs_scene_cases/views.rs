//! Objects across views: removed and shown again, highlighted then closed, fitted to an area,
//! hidden per view and detected in views of different sizes.
use super::prelude::*;
use crate::case_view::viewer_session::ViewParameters;
use crate::viewer_objects::point_object;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkmath::gp::Point3;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

/// Print `message` unless pixel (x, y) has the named colour.
fn expect_named(
    run: &mut SessionRun,
    (x, y): (i32, i32),
    name: &str,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    if run.session().pixel_color_name(x, y)? != name {
        println!("{message}");
    }
    Ok(())
}

/// `bug24391`: a box erased and removed from the context is displayed again and detected.
pub(crate) fn removed_object_displayed_again() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC24391: Erased AIS object can not be displayed in AisInteractiveContext after AisInteractiveContext::Remove",
    ]);
    banner(&["============", ""]);
    let a = box_sized([1., 1., 1.])?;
    let mut run = SessionRun::new()?;
    show(&mut run, "a", &a)?;
    run.session().erase(&["a"])?;
    run.session().remove_from_context("a")?;
    show(&mut run, "a", &a)?;
    run.session().fit()?;
    run.session().set_line_width("a", 5.)?;
    run.session().move_cursor(204, 205)?;
    let message = "Error : Erased object is not displayed after its removing.";
    expect_named(&mut run, (204, 205), "CYAN", message)?;
    run.dump("bug24391_with_lines.png")?;
    run.finish()
}

/// `bug24835`: the highlight of a removed object leaves nothing in a view reopened on the
/// same context.
pub(crate) fn removed_highlight_not_drawn() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR24835: Graphic structure for highlighting is not erased if presentable object for highlight is deleted.",
    ]);
    banner(&["============", ""]);
    let b = box_sized([1., 1., 1.])?;
    let mut run = SessionRun::new()?;
    show(&mut run, "b", &b)?;
    run.session().fit()?;
    run.session().set_object_selection_mode("b", 1, true)?;
    run.session().move_cursor(380, 104)?;
    run.session().move_cursor(29, 103)?;
    run.session().remove(&["b"])?;
    run.session().reopen_view()?;
    run.dump("bug24835.png")?;
    run.finish()
}

/// Fit the view to the view-space box of the two world points in the
/// plane z = 0.
fn fit_area(
    run: &mut SessionRun,
    [x1, y1]: [f64; 2],
    [x2, y2]: [f64; 2],
) -> Result<(), Box<dyn Error>> {
    run.session().update_view(|view| {
        let camera = view.camera();
        let (a, b) = (
            camera.convert_world_to_view(&Point3::new(x1, y1, 0.)),
            camera.convert_world_to_view(&Point3::new(x2, y2, 0.)),
        );
        view.fit_all_rect(
            a.x().min(b.x()),
            a.y().min(b.y()),
            a.x().max(b.x()),
            a.y().max(b.y()),
        )
    })?;
    Ok(())
}

/// `bug25363`: fitting to the area of four corner points puts each point at a view corner.
pub(crate) fn points_fitted_to_area() -> Result<(), Box<dyn Error>> {
    banner(&[
        "================",
        "OCC25335: Visualization - add test command for V3dView::FitAll by view area",
    ]);
    banner(&["================", ""]);
    let mut run = SessionRun::new()?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    let points = [[5., 5.], [15., 5.], [15., 15.], [5., 15.], [10., 10.]];
    for (index, [x, y]) in points.into_iter().enumerate() {
        let name = format!("p{}", index + 1);
        run.session()
            .display_object(&name, point_object([x, y, 0.])?)?;
    }
    fit_area(&mut run, [4.999, 4.999], [15.001, 15.001])?;
    for (x, y) in [(2, 2), (407, 2), (2, 407), (407, 407), (204, 204)] {
        run.session().move_cursor(x, y)?;
        if !run.session().check_color(x, y, [0., 1., 1.])? {
            println!("Error : Fit all by view area is broken.");
        }
    }
    run.dump("bug25363.png")?;
    run.finish()
}

/// The camera every view of `bug26082` takes.
fn bug26082_camera() -> ViewParameters {
    ViewParameters {
        scale: Some(606.309),
        proj: Some([0.577, -0.577, 0.577]),
        up: Some([-0.408, 0.408, 0.816]),
        at: Some([0.5, 0.5, 0.5]),
        eye: Some([1.5, -0.5, 1.5]),
    }
}

/// `bug26082`: one camera shows the box at the same scale in views of three sizes.
pub(crate) fn resized_views_scaled() -> Result<(), Box<dyn Error>> {
    banner(&["============", "CR26082", "============", ""]);
    banner(&["When view is resized horizontally the viusaliziation is not scaled"]);
    let b = box_sized([1., 1., 1.])?;
    let mut run = SessionRun::with_view_size(400, 400)?;
    run.session().open_view(200, 400)?;
    run.session().open_view(400, 200)?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    show(&mut run, "b", &b)?;
    let cursors = [(288, 53), (53, 127), (122, 154)];
    let images = ["bug26082_1.png", "bug26082_2.png", "bug26082_3.png"];
    for (index, ((x, y), image)) in cursors.into_iter().zip(images).enumerate() {
        run.session().activate_view(index)?;
        run.session().set_view_parameters(&bug26082_camera())?;
        run.session().move_cursor(x, y)?;
        run.session().check_color(x, y, [0., 1., 1.])?;
        run.dump(image)?;
    }
    run.finish()
}

/// The boxes of `bug25552`: b1 and b3 in both views, b2 only in the second.
fn show_affine_boxes(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    show(run, "b1", &box_at([0., 0., 0.], [1., 2., 3.])?)?;
    show(run, "b3", &box_at([0., 3., 0.], [2., 3., 1.])?)?;
    let b2 = box_at([3., 0., 0.], [2., 3., 1.])?;
    run.session().display_in_active_view("b2", &b2)?;
    run.session()
        .set_color("b1", Color::named(NamedColor::RED))?;
    run.session()
        .set_color("b2", Color::named(NamedColor::GREEN))?;
    Ok(())
}

/// Check the three box colours of one view; `expected` holds b1, b2 and b3 in that order.
fn expect_boxes(
    run: &mut SessionRun,
    view: &str,
    expected: [&str; 3],
) -> Result<(), Box<dyn Error>> {
    let shown = |name: &str| {
        if name == "BLACK" {
            "should NOT"
        } else {
            "should"
        }
    };
    let labels = ["b1 (red)", "b2 (green)", "b3 (goldenrod)"];
    let pixels = [(50, 250), (200, 350), (250, 200)];
    for ((label, pixel), name) in labels.into_iter().zip(pixels).zip(expected) {
        let message = format!("Error: box {label} {} be visible in {view}!", shown(name));
        expect_named(run, pixel, name, &message)?;
    }
    Ok(())
}

/// `bug25552`: an object hidden in one view of a viewer stays visible in the other.
pub(crate) fn objects_hidden_per_view() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR25552",
        "Provide the way to hide objects within different Views of the one Viewer",
    ]);
    banner(&["============", ""]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    for opened in [false, true] {
        if opened {
            run.session().open_view(409, 409)?;
            run.session().clear()?;
        }
        run.session().set_axonometric()?;
        run.session().set_display_mode(1)?;
    }
    show_affine_boxes(&mut run)?;
    run.session().fit()?;
    run.session().hide_in_active_view("b1")?;
    run.session().move_cursor(250, 347)?;
    expect_boxes(&mut run, "View2", ["BLACK", "GREEN2", "DARKGOLDENROD"])?;
    run.dump("bug25552_v2.png")?;
    run.session().activate_view(0)?;
    run.session().fit()?;
    expect_boxes(&mut run, "View1", ["RED2", "BLACK", "DARKGOLDENROD"])?;
    run.dump("bug25552_v1.png")?;
    run.finish()
}

/// Pan the active axonometric view by `dx`, check the detected box at `forward`, pan back and
/// check it at `back`; `pan_round_trip(&mut run, "View1", 200, (399, 200), (200, 200))?`.
fn pan_round_trip(
    run: &mut SessionRun,
    view: &str,
    dx: i32,
    forward: (i32, i32),
    back: (i32, i32),
) -> Result<(), Box<dyn Error>> {
    run.session().set_axonometric()?;
    run.session().fit()?;
    for (offset, pixel, way) in [(dx, forward, "forward"), (-dx, back, "backward")] {
        run.session().pan(offset, 0)?;
        run.session().move_cursor(pixel.0, pixel.1)?;
        let message = format!("Error: incorrect color after {way} panning in {view}");
        expect_named(run, pixel, "CYAN2", &message)?;
    }
    Ok(())
}

/// `bug26357`: panning moves a shaded box without zooming in views of three aspect ratios.
pub(crate) fn panning_keeps_scale() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "0026357: Visualization - Panning zooms view if aspect ratio > 1",
    ]);
    banner(&["============", ""]);
    let b = box_sized([100., 100., 100.])?;
    let mut run = SessionRun::with_view_size(400, 400)?;
    run.session().clear()?;
    run.session().open_view(200, 400)?;
    run.session().open_view(400, 200)?;
    let shaded = DisplayRequest::default()
        .with_display_mode(1)
        .with_highlight_mode(1);
    run.session().display("b", &b, &shaded)?;
    let steps = [
        ("View1", 200, (399, 200), (200, 200), "bug26357_1.png"),
        ("View2", 100, (199, 100), (100, 100), "bug26357_2.png"),
        ("View3", 200, (399, 100), (200, 100), "bug26357_3.png"),
    ];
    for (index, (view, dx, forward, back, image)) in steps.into_iter().enumerate() {
        run.session().activate_view(index)?;
        pan_round_trip(&mut run, view, dx, forward, back)?;
        run.dump(image)?;
    }
    run.finish()
}

/// The four stacked boxes of `bug25723`, shaded in axonometry with solid selection, fitted.
fn stacked_boxes() -> Result<SessionRun, Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().set_display_mode(1)?;
    run.session().set_selection_mode(6, true)?;
    for (index, z) in [0., 20., -20., 40.].into_iter().enumerate() {
        let name = format!("b{}", index + 1);
        show(&mut run, &name, &box_at([0., 0., z], [10., 10., 10.])?)?;
    }
    run.session().fit()?;
    Ok(run)
}

/// Rotate with the mouse from (100, 100) to (300, 300).
fn mouse_rotation(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    run.session().update_view(|view| {
        view.start_rotation(100, 100, 0.)?;
        view.rotation(300, 300)
    })?;
    Ok(())
}

/// `bug25723`: rotating after the selected boxes are erased turns about the remaining scene.
pub(crate) fn rotation_after_erase() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC25723 Calculate the center of rotation taking into account structure visibility",
    ]);
    banner(&["============", ""]);
    let mut run = stacked_boxes()?;
    run.session()
        .select_pixels(&[(100, 100), (400, 410)], false)?;
    run.session().erase_selected()?;
    mouse_rotation(&mut run)?;
    expect_named(
        &mut run,
        (220, 50),
        "GOLDENROD3",
        "Error: Rotation is not correct",
    )?;
    run.dump("bug25723.png")?;
    run.finish()
}

/// `bug25723_1`: rotating with a selected box turns about the selection.
pub(crate) fn rotation_about_selection() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC25723",
        "Calculate the center of rotation taking into account active selection",
    ]);
    banner(&["============", ""]);
    let mut run = stacked_boxes()?;
    run.session().select_point(220, 50)?;
    mouse_rotation(&mut run)?;
    run.session().select_point(0, 0)?;
    expect_named(
        &mut run,
        (220, 50),
        "GOLDENROD3",
        "Error : Rotation is not correct",
    )?;
    run.dump("bug25723_1.png")?;
    run.finish()
}
