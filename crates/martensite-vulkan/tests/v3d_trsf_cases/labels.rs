//! Text labels kept at their anchor, and flipped labels picked from behind.
use super::prelude::*;
use crate::case_view::viewer_session::{TextLabelRequest, ViewParameters, register_font_file};
use crate::session_run::ImageControl;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::font::FontFontAspect;
use martensite_tkservice::graphic3d::{
    Graphic3dHorizontalTextAlignment as H, Graphic3dVerticalTextAlignment as V,
};
use martensite_tkv3d::v3d::V3dTypeOfOrientation as Orientation;

/// The public DejaVu Sans file the cases locate as a data file.
pub(super) const DEJAVU_SANS: &str = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";

/// White 20-pixel zoomable regular label in the registered `SansFont` at the origin.
fn sans_label(text: &str, horizontal: H) -> TextLabelRequest<'_> {
    TextLabelRequest {
        color: Some(Color::named(NamedColor::WHITE)),
        horizontal: Some(horizontal),
        vertical: Some(V::Bottom),
        angle_degrees: Some(0.),
        zoomable: Some(true),
        height: Some(20.),
        aspect: Some(FontFontAspect::Regular),
        font: Some("SansFont"),
        ..TextLabelRequest::new(text)
    }
}

/// `bug23652`: left- and right-aligned zoomable labels stay at their anchor.
// The camera literals are the original's, digit for digit.
#[allow(clippy::excessive_precision)]
pub(crate) fn aligned_zoomable_labels() -> Outcome {
    banner(&[
        "========",
        "OCC23652: Non zoomable text with alignment slides away when zooming view",
        "========",
    ]);
    register_font_file(DEJAVU_SANS, "SansFont")?;
    let mut run = SessionRun::new()?;
    run.session().display_trihedron("tri")?;
    run.session()
        .draw_text("text0", &sans_label("Default", H::Left))?;
    run.session()
        .draw_text("text1", &sans_label("Right_Align", H::Right))?;
    let (proj, up) = (0.57735025882720947, 0.40824827551841736);
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(3.1783114563761763),
        proj: Some([proj, -proj, proj]),
        up: Some([-up, up, 0.81649655103683472]),
        at: Some([0., 0., 0.]),
        ..ViewParameters::default()
    })?;
    run.session().check_color(210, 204, [0.6, 0.6, 0.6])?;
    run.dump("bug23652.png")?;
    run.finish()
}

/// A 40-pixel centred label on the XY plane with X along Y, flipped to stay readable.
fn flipped_label(text: &str, color: NamedColor) -> TextLabelRequest<'_> {
    TextLabelRequest {
        color: Some(Color::named(color)),
        height: Some(40.),
        plane: Some(([0., 1., 0.], [1., 0., 0.])),
        flipping: Some(true),
        vertical: Some(V::Center),
        horizontal: Some(H::Center),
        ..TextLabelRequest::new(text)
    }
}

/// Hover and pick the view centre, report the selected count and dump.
fn pick_centre(run: &mut SessionRun) -> Result<i32, Box<dyn Error>> {
    run.session().move_cursor(205, 205)?;
    run.session().select_point(205, 205)?;
    Ok(run.session().selected_count())
}

/// `bug_flipped_selection`: a flipped label is picked from the front, from behind and when
/// its host is moved.
pub(crate) fn flipped_label_selection() -> Outcome {
    banner(&[
        "========",
        "Selection must work on a group-flipped text label after the camera is rotated 180 deg.",
        "A Graphic3d_Flipper is stored on the group and on the sensitive entity; the selection",
        "pipeline applies its matrix so the sensitive frustum matches the rendered geometry.",
        "========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .draw_text("t0", &flipped_label("PICK ME", NamedColor::RED))?;
    run.session()
        .set_standard_projection(Orientation::ZUP_FRONT)?;
    run.session().fit()?;
    let front = pick_centre(&mut run)?;
    println!("Selected count (front view, unflipped label): {front}");
    if front < 1 {
        println!("Error : sanity check failed - label is not selectable even in front view");
    }
    run.dump("bug_flipped_selection_front_selected.png")?;
    run.session().select_point(0, 0)?;
    run.session()
        .set_standard_projection(Orientation::ZUP_BACK)?;
    run.session().fit()?;
    run.dump("bug_flipped_selection_back.png")?;
    let back = pick_centre(&mut run)?;
    println!("Selected count (back view, flipped label): {back}");
    report_pick(
        back,
        "flipped text label",
        "flipped text label selection works.",
    );
    run.dump("bug_flipped_selection_back_selected.png")?;
    run.session().clear()?;
    let rotated = flipped_label("ROTATED FLIPPED", NamedColor::BLUE);
    run.session().draw_text("t2", &rotated)?;
    // The old six-number form keeps only the last translation it reads.
    run.session().set_translation("t2", [0., 45., 0.])?;
    run.session()
        .set_standard_projection(Orientation::ZUP_BACK)?;
    run.session().fit()?;
    let moved = pick_centre(&mut run)?;
    println!("Selected count (rotated host, flipped label): {moved}");
    report_pick(
        moved,
        "rotated + flipped label",
        "rotated + flipped label selection works.",
    );
    run.dump("bug_flipped_selection_rotated_selected.png")?;
    run.finish()
}

/// The pick verdict lines; `report_pick(1, "flipped text label", "... works.")`.
fn report_pick(count: i32, what: &str, works: &str) {
    if count < 1 {
        let expected = if what == "flipped text label" {
            format!("Error : {what} is not selectable (expected >=1, got {count})")
        } else {
            format!("Error : {what} not selectable (expected >=1, got {count})")
        };
        println!("{expected}");
    } else {
        println!("OK : {works}");
    }
}

/// A 30-pixel label on the YZ plane (X direction along Y) that flips to stay readable.
fn yoz_label(
    text: &str,
    position: [f64; 3],
    color: NamedColor,
    align: (H, V),
) -> TextLabelRequest<'_> {
    TextLabelRequest {
        position,
        color: Some(Color::named(color)),
        height: Some(30.),
        plane: Some(([1., 0., 0.], [0., 1., 0.])),
        flipping: Some(true),
        horizontal: Some(align.0),
        vertical: Some(align.1),
        font: Some("SansFont"),
        ..TextLabelRequest::new(text)
    }
}

/// `bug27359`: flipped labels on the YZ plane stay readable when the camera turns round.
pub(crate) fn flipped_plane_labels() -> Outcome {
    banner(&[
        "========",
        "OCC27359: Visualization - add support of flipping for textured text",
        "========",
        "",
    ]);
    register_font_file(DEJAVU_SANS, "SansFont")?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .set_standard_projection(Orientation::ZUP_RIGHT)?;
    let b = box_sized([10., 50., 30.])?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().display_trihedron("t")?;
    let top_left = yoz_label(
        "top left text\n on yoz",
        [10., 0., 0.],
        NamedColor::GREEN,
        (H::Left, V::Top),
    );
    run.session().draw_text("t0", &top_left)?;
    let centre = yoz_label(
        "center text",
        [0., 25., 15.],
        NamedColor::RED,
        (H::Center, V::Center),
    );
    run.session().draw_text("t2", &centre)?;
    run.session().fit()?;
    run.session()
        .set_standard_projection(Orientation::ZUP_RIGHT)?;
    run.dump("bug27359_1.png")?;
    // The original's "3,14" reads as 3 radians: the command host's number parser stops at the comma.
    run.session().rotate(3., 0., 0.)?;
    run.session().fit()?;
    run.dump_with_controls("bug27359_2.png", &BOX_AND_AXES)?;
    run.session()
        .set_standard_projection(Orientation::ZUP_RIGHT)?;
    run.session().rotate(0., 3., 0.)?;
    run.session().fit()?;
    run.dump_with_controls("bug27359_3.png", &BOX_AND_AXES)?;
    run.finish()
}

/// The box edges and the trihedron axes along them are coincident one-pixel lines at equal
/// depth in the turned views: the undefined-locus controls show each without the other.
const BOX_AND_AXES: [ImageControl<'static>; 2] = [
    ImageControl {
        suffix: "locus-1",
        erased: &["t"],
    },
    ImageControl {
        suffix: "locus-2",
        erased: &["b"],
    },
];
