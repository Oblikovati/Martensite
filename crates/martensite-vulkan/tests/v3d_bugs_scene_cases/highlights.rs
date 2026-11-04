//! Dynamic highlight styles, the highlighting of hidden-line presentations and of wide
//! outlines over a shaded shape, and hidden-line removal in a perspective view.
use super::prelude::*;
use crate::case_view::viewer_session::HighlightStyle;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::graphic3d::{
    Graphic3dCameraProjection, Graphic3dNameOfMaterial, Graphic3dRenderTransparentMethod,
};
use martensite_tkv3d::prs3d::{Prs3dTypeOfHighlight, Prs3dTypeOfHlr};

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

/// A cleared view showing `shape` shaded as `name`, fitted.
fn shaded_view(name: &str, shape: &Shape) -> Result<SessionRun, Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().display(name, shape, &shaded)?;
    run.session().fit()?;
    Ok(run)
}

/// `bug25695`: a half-transparent plastic dynamic highlight in the object's own mode.
pub(crate) fn plastic_dynamic_highlight() -> Result<(), Box<dyn Error>> {
    banner(&[
        "===========",
        "OCC25695",
        "Visualization, AisInteractiveContext - define default HilightMode",
    ]);
    banner(&["===========", ""]);
    let mut run = shaded_view("s", &sphere(1.)?)?;
    let style = HighlightStyle {
        display_mode: Some(-1),
        transparency: Some(0.5),
        material: Some(Graphic3dNameOfMaterial::Plastified),
        ..HighlightStyle::default()
    };
    run.session()
        .set_highlight_style(Prs3dTypeOfHighlight::Dynamic, &style)?;
    run.session().move_cursor(250, 250)?;
    expect_named(
        &mut run,
        (250, 250),
        "GRAY58",
        "Error: wrong highlighting color",
    )?;
    run.dump("bug25695.png")?;
    run.finish()
}

/// `bug29051`: a transparent red dynamic highlight keeps its colour with OIT off and on.
pub(crate) fn transparent_highlight_with_oit() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "0029051: Visualization, Renderer - wrong color of transparent dynamic highlight with OIT tuned ON",
    ]);
    banner(&["========", ""]);
    let mut run = shaded_view("b", &box_sized([1., 2., 3.])?)?;
    let style = HighlightStyle {
        display_mode: Some(1),
        color: Some(Color::named(NamedColor::RED)),
        transparency: Some(0.2),
        ..HighlightStyle::default()
    };
    run.session()
        .set_highlight_style(Prs3dTypeOfHighlight::Dynamic, &style)?;
    run.session().move_cursor(250, 250)?;
    let unordered = Graphic3dRenderTransparentMethod::BlendUnordered;
    run.session().set_transparency_method(unordered, None)?;
    let off = "Error: wrong highlight color with OIT turned OFF";
    expect_named(&mut run, (250, 250), "ORANGERED2", off)?;
    run.dump("bug29051_oit_off.png")?;
    let weighted = Graphic3dRenderTransparentMethod::BlendOit;
    run.session().set_transparency_method(weighted, Some(0.))?;
    let on = "Error: wrong highlight color with OIT turned ON";
    expect_named(&mut run, (250, 250), "ORANGERED2", on)?;
    run.dump("bug29051_oit_on.png")?;
    run.finish()
}

/// `bug28061`: a wide wireframe box drawn with hidden lines removed is highlighted.
pub(crate) fn hidden_line_box_highlighted() -> Result<(), Box<dyn Error>> {
    banner(&[
        "===========",
        "OCC28061 - Highlighting of any object with HLR option leads to crash",
    ]);
    banner(&["===========", ""]);
    let b = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let wireframe = DisplayRequest::default().with_display_mode(0);
    run.session().display("b", &b, &wireframe)?;
    run.session().set_line_width("b", 4.)?;
    run.session().fit()?;
    run.session().set_hidden_line_removal(true, None)?;
    run.session().move_cursor(250, 250)?;
    expect_named(
        &mut run,
        (321, 150),
        "CYAN",
        "Error: the box is not highlighted",
    )?;
    run.dump("bug28061.png")?;
    run.finish()
}

/// `bug31652`: highlighting a shaded sphere computed by the polygonal hidden-line algorithm.
pub(crate) fn poly_hidden_line_sphere_highlighted() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "0031652: Visualization - crash on highlighting HLR Computed presentation displayed with Shaded display mode",
    ]);
    banner(&["============", ""]);
    let s = sphere(1.)?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().display("s", &s, &shaded)?;
    run.session().set_axonometric()?;
    run.session().fit()?;
    run.session()
        .set_hidden_line_removal(true, Some(Prs3dTypeOfHlr::PolyAlgo))?;
    run.session().move_cursor(351, 64)?;
    run.dump("bug31652.png")?;
    run.finish()
}

/// `bug24867`: the dynamic highlight of the wide outlines of shaded `occ/Top.brep` is drawn in
/// front of the surface (polygon offsets), then the object is selected.
pub(crate) fn wide_outline_highlight() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC24867: Visualization - polygon offsets look broken",
        "============",
        "",
    ]);
    let b = crate::dataset_shape::read_dataset_shape("Top.brep")?;
    let mut run = SessionRun::new()?;
    show(&mut run, "b", &b)?;
    run.session().fit()?;
    run.session().set_display_mode(1)?;
    run.session().set_line_width("b", 5.)?;
    run.session().move_cursor(200, 200)?;
    let message = "Error : Highlighting of dimension with flipping in local context failed.";
    expect_named(&mut run, (264, 288), "CYAN", message)?;
    expect_named(&mut run, (251, 271), "CYAN", message)?;
    run.dump("bug24867_highlighted.png")?;
    run.session().select_point(200, 200)?;
    run.dump_and_finish("bug24867_selected.png", false)
}

/// The pixels of `bug349` that show the wide visible edges, then those that stay background.
const BUG349_YELLOW: [(i32, i32); 9] = [
    (135, 119),
    (387, 33),
    (172, 144),
    (28, 190),
    (212, 272),
    (60, 343),
    (26, 255),
    (389, 113),
    (60, 276),
];
const BUG349_BLACK: [(i32, i32); 3] = [(160, 257), (365, 150), (353, 99)];

/// `bug349`: hidden-line removal of a box in a perspective view keeps only its visible edges.
pub(crate) fn perspective_hidden_lines() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "OCC349 - Hidden line removal algorithm works incorrect in the perspective view",
        "========",
        "",
    ]);
    let b = box_sized([100., 900., 300.])?;
    let mut run = SessionRun::new()?;
    run.session()
        .set_projection(Graphic3dCameraProjection::Perspective)?;
    show(&mut run, "b", &b)?;
    run.session().set_hidden_line_removal(true, None)?;
    run.session().fit()?;
    run.session().set_line_width("b", 5.)?;
    for (x, y) in BUG349_YELLOW {
        expect_named(
            &mut run,
            (x, y),
            "YELLOW",
            &format!("Error : {x} {y} is not yellow"),
        )?;
    }
    for (x, y) in BUG349_BLACK {
        expect_named(
            &mut run,
            (x, y),
            "BLACK",
            &format!("Error : {x} {y} is not black"),
        )?;
    }
    run.dump_and_finish("bug349.png", false)
}
