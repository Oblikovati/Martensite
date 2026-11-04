//! Boxes kept in screen coordinates (2D persistence) picked in orthographic and perspective views.
use super::prelude::*;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::aspect::AspectTypeOfTriedronPosition as Corner;
use martensite_tkservice::graphic3d::{
    Graphic3dCameraProjection as Projection, Graphic3dNameOfMaterial,
    Graphic3dTransModeFlags as Flags,
};

/// The three 2D-persistent boxes and the pixels that pick them: `{x y}` of `test_1..3`.
const PICKS: [[i32; 2]; 3] = [[220, 120], [350, 150], [50, 350]];

/// The dumps of the orthographic picks, then of the perspective ones.
const DUMPS: [[&str; 3]; 2] = [
    ["bug27739_1.png", "bug27739_2.png", "bug27739_3.png"],
    ["bug27739_4.png", "bug27739_5.png", "bug27739_6.png"],
];

/// `bug27739`: 2D-persistent boxes (centred, bottom-left, centred and moved) are detected
/// under the cursor in an orthographic and in a perspective view.
pub(crate) fn screen_boxes_detection() -> Outcome {
    banner(&[
        "========",
        "OCC27739: Visualization, TKV3d - implement individual acceleration data structure for selection of 2D persistent objects",
        "========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_projection(Projection::Orthographic)?;
    run.session().display_trihedron("tri1")?;
    display_boxes(&mut run)?;
    run.session().fit()?;
    check_picks(&mut run, DUMPS[0])?;
    run.session().set_projection(Projection::Perspective)?;
    run.session().update_view(|view| {
        if let Some(camera) = view.camera().get() {
            camera.set_distance(1000.);
        }
        Ok(())
    })?;
    check_picks(&mut run, DUMPS[1])?;
    run.finish()
}

/// The five boxes of the original with their persistence, location, material and colour.
fn display_boxes(run: &mut SessionRun) -> Outcome {
    let shaded = DisplayRequest::default()
        .with_display_mode(1)
        .with_highlight_mode(0);
    let screen = DisplayRequest::default()
        .with_display_mode(1)
        .with_highlight_mode(1);
    let session = run.session();
    session.display("box3d", &box_sized([100., 100., 100.])?, &shaded)?;
    for (name, corner) in [
        ("box2d", Corner::CENTER),
        ("box2d_pos", Corner::LEFT_LOWER),
        ("box2d_loc", Corner::CENTER),
    ] {
        let persistence = corner_persistence(Flags::TMF_2D, corner, [0, 0])?;
        session.display_persistent(name, &box_sized([100., 100., 1.])?, persistence, &screen)?;
    }
    let zoom = DisplayRequest::zoom_persistent_at_origin()
        .with_display_mode(1)
        .with_highlight_mode(0);
    session.display("box_zoom", &box_sized([100., 100., 100.])?, &zoom)?;
    session.display_again("box2d")?;
    session.set_location("box2d_loc", [100., 0., 0.])?;
    session.set_location("box_zoom", [-100., -100., 100.])?;
    for name in ["box3d", "box2d", "box2d_pos", "box2d_loc", "box_zoom"] {
        session.set_material(name, Graphic3dNameOfMaterial::Plastified)?;
    }
    for (name, color) in [
        ("box3d", NamedColor::GOLD),
        ("box2d", NamedColor::GREEN),
        ("box2d_pos", NamedColor::GREEN),
        ("box2d_loc", NamedColor::GREEN),
        ("box_zoom", NamedColor::RED),
    ] {
        session.set_color(name, Color::named(color))?;
    }
    Ok(())
}

/// Hover each pick pixel, require the highlight colour there and dump the view.
fn check_picks(run: &mut SessionRun, dumps: [&'static str; 3]) -> Outcome {
    for ([x, y], dump) in PICKS.into_iter().zip(dumps) {
        run.session().move_cursor(x, y)?;
        if run.session().pixel_rgba_name(x, y)? != "CYAN 1" {
            println!("ERROR: zoom persistent box is not detected!");
        }
        run.dump(dump)?;
    }
    Ok(())
}
