//! Zoom-, rotate- and trihedron-persistent boxes picked and selected, and their pick depths.
use super::prelude::*;
use crate::case_view::viewer_session::{PersistenceAnchor, ViewParameters, ViewerSession};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::GeomCartesianPoint;
use martensite_tkservice::aspect::AspectTypeOfTriedronPosition as Corner;
use martensite_tkservice::graphic3d::{Graphic3dNameOfMaterial, Graphic3dTransModeFlags as Flags};
use martensite_tkv3d::ais::{AisInteractiveObject, AisPoint};

/// Display a point named `name`.
fn display_point(session: &mut ViewerSession, name: &str, [x, y, z]: [f64; 3]) -> Outcome {
    let point = Handle::new(GeomCartesianPoint::new(x, y, z));
    let object: Handle<dyn AisInteractiveObject> = AisPoint::new(&point.cast())?.cast();
    session.display_object(name, object)?;
    Ok(())
}

/// A display request with transform persistence `flags` anchored at `point`, optionally shaded.
fn anchored(flags: Flags, point: [f64; 3], shaded: bool) -> DisplayRequest {
    let request = DisplayRequest {
        persistence: Some((flags, PersistenceAnchor::Point(point))),
        ..DisplayRequest::default()
    };
    if shaded {
        request.with_display_mode(1).with_highlight_mode(1)
    } else {
        request
    }
}

/// Clear the selection, select at `pixels`, then require `name` at (x, y).
fn select_and_read(
    session: &mut ViewerSession,
    pixels: &[(i32, i32)],
    reads: &[(i32, i32, &str)],
    error: &str,
) -> Outcome {
    session.select_pixels(&[(0, 0)], false)?;
    session.select_pixels(pixels, false)?;
    let mut wrong = false;
    for &(x, y, name) in reads {
        wrong |= session.pixel_color_name(x, y)? != name;
    }
    if wrong {
        println!("{error}");
    }
    Ok(())
}

/// `bug26344`: zoom-, rotate-, zoom-rotate- and trihedron-persistent boxes are picked and
/// selected by rectangle.
pub(crate) fn persistent_box_selection() -> Outcome {
    banner(&[
        "============",
        "0026344: Visualization - provide a support of zoom persistent selection",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    let s = run.session();
    s.display_trihedron("tri")?;
    s.pan(50, 50)?;
    let zoom_anchor = [200., 200., 200.];
    display_point(s, "pz", zoom_anchor)?;
    s.display(
        "bz1",
        &box_sized([25., 25., 25.])?,
        &anchored(Flags::ZOOM_PERS, zoom_anchor, true),
    )?;
    s.display(
        "bz2",
        &box_sized([25., 25., 25.])?,
        &anchored(Flags::ZOOM_PERS, zoom_anchor, true),
    )?;
    s.set_location("bz2", [-25., -25., -25.])?;
    select_and_read(
        s,
        &[(387, 77)],
        &[(387, 77, "GRAY73")],
        "Error picking zoom persistence object(s)",
    )?;
    select_and_read(
        s,
        &[(330, 120), (410, 50)],
        &[(387, 77, "GRAY73"), (352, 96, "GRAY73")],
        "Error selecting zoom persistence object(s)",
    )?;
    let rotate_anchor = [-200., -200., -200.];
    display_point(s, "pr", rotate_anchor)?;
    s.display(
        "br",
        &box_sized([100., 100., 100.])?,
        &anchored(Flags::ROTATE_PERS, rotate_anchor, true),
    )?;
    s.set_material("br", Graphic3dNameOfMaterial::Plastified)?;
    select_and_read(
        s,
        &[(160, 200)],
        &[(160, 180, "GRAY89")],
        "Error picking rotate persistence object",
    )?;
    select_and_read(
        s,
        &[(130, 230), (190, 170)],
        &[(160, 180, "GRAY89")],
        "Error selecting rotate persistence object",
    )?;
    let both_anchor = [-200., 100., 0.];
    display_point(s, "pzr", both_anchor)?;
    s.display(
        "bzr",
        &box_sized([50., 25., 20.])?,
        &anchored(Flags::ZOOM_ROTATE_PERS, both_anchor, true),
    )?;
    s.set_material("bzr", Graphic3dNameOfMaterial::Plastified)?;
    select_and_read(
        s,
        &[(250, 90)],
        &[(250, 90, "GRAY89")],
        "Error picking zoom-rotate persistence object",
    )?;
    select_and_read(
        s,
        &[(200, 70), (286, 110)],
        &[(250, 90, "GRAY89")],
        "Error selecting zoom-rotate persistence object",
    )?;
    let corner = corner_persistence(Flags::TRIEDRON_PERS, Corner::LEFT_LOWER, [62, 62])?;
    let shaded = DisplayRequest::default()
        .with_display_mode(1)
        .with_highlight_mode(1);
    s.display_persistent("bt", &box_sized([100., 100., 100.])?, corner, &shaded)?;
    select_and_read(
        s,
        &[(132, 300)],
        &[(132, 300, "GRAY73")],
        "Error picking trihedron persistence object",
    )?;
    select_and_read(
        s,
        &[(50, 223), (235, 395)],
        &[(132, 300, "GRAY73")],
        "Error selecting trihedron persistence object",
    )?;
    s.select_pixels(&[(50, 410), (410, 50)], false)?;
    run.dump("bug26344.png")?;
    run.finish()
}

/// The first two picked depths differ by at most 0.55, as the original's `compareDepth`.
fn compare_depths(session: &ViewerSession) -> Outcome {
    let depths = session.picked_depths()?;
    let (first, second) = (
        depths.first().copied().unwrap_or(0.),
        depths.get(1).copied().unwrap_or(0.),
    );
    if (first - second).abs() > 0.55 {
        println!("ERROR: the depths diff is bigger than adequate tolerance, see vstate output!");
    }
    Ok(())
}

/// `bug26719_2`: a zoom-persistent box and a box face almost on one plane pick at equal depth.
pub(crate) fn zoom_persistent_pick_depth() -> Outcome {
    banner(&[
        "============",
        "CR26719: Visualization - cannot pick zoom persistent object",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    let s = run.session();
    s.display_trihedron("tri")?;
    s.pan(50, 50)?;
    let anchor = [200., 200., 200.];
    display_point(s, "p1", anchor)?;
    s.display(
        "b1",
        &box_sized([20.3, 20.3, 20.3])?,
        &anchored(Flags::ZOOM_PERS, anchor, false),
    )?;
    s.display(
        "b2",
        &box_sized([20.3, 20.3, 20.3])?,
        &anchored(Flags::ZOOM_PERS, anchor, false),
    )?;
    s.set_location("b2", [-20.3, -20.3, -20.3])?;
    s.display(
        "b3",
        &box_at([150., 150., 150.], [100., 100., 100.])?,
        &DisplayRequest::default(),
    )?;
    s.set_display_mode(1)?;
    s.set_color("b1", Color::named(NamedColor::RED))?;
    s.set_color("b2", Color::named(NamedColor::GREEN))?;
    view_steps(
        s,
        0.99,
        [-0.04, -0.99, 0.11],
        ([-0.58, 0.16, 0.8], [8.48, 160.93, 282.42]),
        [-14.63, -379.49, 343.06],
    )?;
    s.move_cursor(230, 280)?;
    compare_depths(s)?;
    run.dump("bug26719_2_1.png")?;
    let s = run.session();
    s.move_cursor(0, 0)?;
    view_steps(
        s,
        0.99,
        [-0.17, 0.09, 0.98],
        ([-0.1, 0.99, -0.1], [-49.98, 231.47, 25.2]),
        [-142.03, 280.17, 559.45],
    )?;
    s.move_cursor(276, 110)?;
    compare_depths(s)?;
    run.dump("bug26719_2_2.png")?;
    run.finish()
}

/// The three view steps: scale and projection, up and centre, then eye.
fn view_steps(
    s: &ViewerSession,
    scale: f64,
    proj: [f64; 3],
    (up, at): ([f64; 3], [f64; 3]),
    eye: [f64; 3],
) -> Outcome {
    s.set_view_parameters(&ViewParameters {
        scale: Some(scale),
        proj: Some(proj),
        ..ViewParameters::default()
    })?;
    s.set_view_parameters(&ViewParameters {
        up: Some(up),
        at: Some(at),
        ..ViewParameters::default()
    })?;
    s.set_view_parameters(&ViewParameters {
        eye: Some(eye),
        ..ViewParameters::default()
    })?;
    Ok(())
}
