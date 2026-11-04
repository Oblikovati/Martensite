//! Text converted to B-Rep shapes with a font, a pen frame and an alignment.
use super::prelude::*;
use martensite_tkbrep::topods::Shape;
#[path = "../support/brep_text_shape.rs"]
mod brep_text_shape;
pub(crate) use brep_text_shape::{BRepText, text_shape};

/// Report a shape check as the shape check command does.
fn check_shape(shape: &Shape) -> Outcome {
    crate::shape_checks::check_shape_valid(shape)?;
    Ok(())
}

/// The keyboard alphabet text of `B3`, exactly as the literal reads.
const ALPHABET: &str = "\n`1234567890-=\n~!@#$%^&*()_+\nqwertyuiop[]\\\\\nQWERTYUIOP{}|\nasdfghjkl;'\nASDFGHJKL:\"\nzxcvbnm,./\nZXCVBNM<>?\u{a7}\nAVATAR Y.\n";

/// `B3`: the whole alphabet as separate and as composite contours, side by side.
pub(crate) fn alphabet_shapes() -> Outcome {
    banner(&[
        "============",
        "OCC24181 Render text as BRep (check alphabet)",
        "============",
        "",
    ]);
    register_sans_font()?;
    let separate = BRepText {
        font: "SansFont",
        height: 24.,
        ..Default::default()
    };
    let text_n = text_shape(ALPHABET, &separate)?;
    let text_c = text_shape(
        ALPHABET,
        &BRepText {
            composite: true,
            ..separate
        },
    )?;
    check_shape(&text_n)?;
    check_shape(&text_c)?;
    let text_c = crate::primitive_shapes::translated(&text_c, [220., 0., 0.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .set_standard_projection(martensite_tkv3d::v3d::V3dTypeOfOrientation::ZUP_TOP)?;
    let request = DisplayRequest::default().with_display_mode(1);
    run.session().display("aBTextN", &text_n, &request)?;
    run.session().display("aBTextC", &text_c, &request)?;
    run.session().fit()?;
    run.dump("B3.png")?;
    run.finish()
}

/// `bug25852`: the circled registered sign converts to valid faces.
pub(crate) fn circled_symbol() -> Outcome {
    banner(&[
        "============",
        "CR25852: Text to BRep algorithm produces bad faces for circled symbols",
        "============",
        "",
    ]);
    // The positional font and size are not options of the conversion command: it warns and
    // keeps its defaults.
    eprintln!("Warning! Unknown argument 'courier'");
    eprintln!("Warning! Unknown argument '10'");
    let shape = text_shape("\u{ae}", &BRepText::default())?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .set_standard_projection(martensite_tkv3d::v3d::V3dTypeOfOrientation::ZUP_TOP)?;
    run.session().set_display_mode(1)?;
    run.session()
        .display("t", &shape, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().move_cursor(250, 250)?;
    check_shape(&shape)?;
    run.dump("bug25852.png")?;
    run.finish()
}
