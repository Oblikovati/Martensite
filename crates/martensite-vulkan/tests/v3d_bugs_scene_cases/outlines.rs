//! Face boundaries drawn over shaded shapes, with and without an existing triangulation.
use super::prelude::*;
use crate::case_view::viewer_session::FaceBoundary;
use martensite_tkernel::quantity::{Color, ColorSpace, NamedColor};
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkservice::aspect::AspectTypeOfLine;

/// `bug23407_2`: wide dashed red face boundaries of a shaded box stay red after every line of
/// the box is widened.
pub(crate) fn dashed_face_boundaries() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR23407: Draw face outlines for XDE objects",
    ]);
    banner(&["============", ""]);
    let b = box_sized([10., 10., 10.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    show(&mut run, "b", &b)?;
    run.session().set_display_mode(1)?;
    let style = FaceBoundary {
        color: Some(Color::named(NamedColor::RED)),
        width: Some(5.),
        line_type: Some(AspectTypeOfLine::Dash),
    };
    run.session().draw_face_boundaries("b", &style)?;
    run.session().fit()?;
    run.session().set_line_width("b", 10.)?;
    if run.session().pixel_color_name(314, 344)? != "RED"
        || run.session().pixel_color_name(26, 267)? != "RED"
    {
        println!("Error : Boundary of face is not changed");
    }
    run.dump("bug23407_2.png")?;
    run.finish()
}

/// Draw olive face boundaries on `c`, then select at (200, 200)
/// and the dump `image`, with its faces-only control (`<image>.uncovered.png`): the AMD one-pixel-line snap tie of the
/// selected wireframe is graded against it.
fn olive_boundaries_selected(
    run: &mut SessionRun,
    image: &'static str,
    c: &Shape,
) -> Result<(), Box<dyn Error>> {
    let olive = Color::new(0.25, 0.25, 0., ColorSpace::Rgb)?;
    let style = FaceBoundary {
        color: Some(olive),
        ..FaceBoundary::default()
    };
    run.session().draw_face_boundaries("c", &style)?;
    run.session().select_point(200, 200)?;
    run.dump_with_faces_only_control(image, "c", c, 1)
}

/// `bug25071`: the selected wireframe of a shaded cone follows the same boundary
/// discretization as its face outlines, before and after the cone is triangulated.
pub(crate) fn cone_outlines_selected() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR25071: Check that the selected wireframe representation uses exactly the same",
    ]);
    banner(&["discretization of face boundaries as the face outlines shown in shading mode"]);
    banner(&["============", ""]);
    let c = cone(0., 5., 10.)?;
    let mut run = SessionRun::new()?;
    show(&mut run, "c", &c)?;
    run.session().fit()?;
    run.session().set_object_display_mode("c", 1)?;
    olive_boundaries_selected(&mut run, "bug25071_wf_no_triang.png", &c)?;
    run.session().remove(&["c"])?;
    run.session().reopen_view()?;
    run.session().set_display_mode(1)?;
    show(&mut run, "c", &c)?;
    run.session().fit()?;
    olive_boundaries_selected(&mut run, "bug25071_wf_with_triang.png", &c)?;
    run.finish()
}

/// `bug29787`: red face boundaries on two shaded spheres beside a wireframe one; the second
/// sphere keeps only boundaries up to C2, which drops its seam.
pub(crate) fn boundaries_up_to_continuity() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "0029787: Visualization - Avoid in presentation edges of certain continuity class",
        "========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .set_background_color(Color::named(NamedColor::WHITE))?;
    let spheres = [0., 4., 8.].map(|x| translated(&sphere(1.)?, [x, 0., 0.]));
    for ((name, shape), mode) in ["s1", "s2", "s3"].iter().zip(spheres).zip([0, 1, 1]) {
        let request = DisplayRequest {
            display_mode: Some(mode),
            ..DisplayRequest::default()
        };
        run.session().display(name, &shape?, &request)?;
    }
    run.session().fit()?;
    let red = FaceBoundary {
        color: Some(Color::named(NamedColor::RED)),
        ..FaceBoundary::default()
    };
    run.session().draw_face_boundaries("s2", &red)?;
    run.session().draw_face_boundaries("s3", &red)?;
    run.session()
        .set_face_boundary_continuity("s3", Continuity::C2)?;
    run.dump_and_finish("bug29787.png", false)
}
