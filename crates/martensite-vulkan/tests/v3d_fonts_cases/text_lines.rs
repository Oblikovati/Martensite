//! Three lines of text at sizes 12, 18 and 24, each as a label and as a B-Rep shape below a
//! point marker.
use super::prelude::*;

/// The font and display options of one text-lines program.
struct TextLines {
    font: &'static str,
    composite: bool,
    own_shaded_mode: bool,
}

const TEXT: &str = "The quick brown fox\njumps over the lazy dog!";

/// Draw the three lines (label and shape per size), then fit; `draw_lines(&mut run, &lines)?`.
fn draw_lines(run: &mut SessionRun, lines: &TextLines) -> Outcome {
    let mut line = 0.0;
    for (id, size) in [12.0_f64, 18., 24.].into_iter().enumerate() {
        let text = format!("[{size}] {TEXT}");
        display_point(run, &format!("aPnt_{size}"), [0., line, 0.])?;
        run.session()
            .draw_text(&format!("Line_{id}"), &label(&text, line, size, lines.font))?;
        let options = BRepText {
            font: lines.font,
            height: size,
            aspect: FontFontAspect::BoldItalic,
            composite: lines.composite,
            position: [0., line, 0.],
            vertical: VAlign::Topfirstline,
            ..Default::default()
        };
        let shape = text_shape(&text, &options)?;
        let request = if lines.own_shaded_mode {
            DisplayRequest::default().with_display_mode(1)
        } else {
            DisplayRequest::default()
        };
        run.session()
            .display(&format!("aBText_{size}"), &shape, &request)?;
        line -= 4.0 * size;
    }
    run.session().fit()?;
    Ok(())
}

/// The cyan bold-italic label of one line, left/top at the line anchor.
fn label<'a>(text: &'a str, line: f64, size: f64, font: &'a str) -> TextLabelRequest<'a> {
    TextLabelRequest {
        position: [0., line, 0.],
        color: Some(Color::named(NamedColor::CYAN)),
        horizontal: Some(HAlign::Left),
        vertical: Some(VAlign::Top),
        angle_degrees: Some(0.),
        zoomable: Some(true),
        height: Some(size),
        aspect: Some(FontFontAspect::BoldItalic),
        font: Some(font),
        ..TextLabelRequest::new(text)
    }
}

/// A shaded top view of the three lines in `lines`, dumped to `image`.
fn text_lines(lines: &TextLines, image: &'static str) -> Outcome {
    let mut run = SessionRun::new()?;
    if !lines.own_shaded_mode {
        run.session().set_display_mode(1)?;
    }
    run.session()
        .set_standard_projection(martensite_tkv3d::v3d::V3dTypeOfOrientation::ZUP_TOP)?;
    enable_font_trace();
    draw_lines(&mut run, lines)?;
    println!("{}", run.session().graphics_information());
    run.dump(image)?;
    run.finish()
}

/// `B1`: the system sans-serif font, separate contours.
pub(crate) fn system_font_lines() -> Outcome {
    banner(&[
        "============",
        "OCC24181 Render text as BRep",
        "============",
        "",
    ]);
    text_lines(
        &TextLines {
            font: "sans-serif",
            composite: false,
            own_shaded_mode: false,
        },
        "B1.png",
    )
}

/// `B2`: the system sans-serif font, composite contours.
pub(crate) fn composite_system_lines() -> Outcome {
    banner(&[
        "============",
        "OCC24181 Render text as BRep (composite curve)",
        "============",
        "",
    ]);
    text_lines(
        &TextLines {
            font: "sans-serif",
            composite: true,
            own_shaded_mode: false,
        },
        "B2.png",
    )
}

/// `B7`: a registered font with only a regular style; bold italic is synthesized.
pub(crate) fn registered_font_lines() -> Outcome {
    banner(&[
        "============",
        "0030663: Visualization - synthesize italic style for a font having no italic style",
        "============",
        "",
    ]);
    register_sans_font()?;
    text_lines(
        &TextLines {
            font: "SansFont",
            composite: false,
            own_shaded_mode: true,
        },
        "B7.png",
    )
}

// B1/B2/B7 enable message trace before asking the font manager to trace aliases.
fn enable_font_trace() {
    use martensite_tkernel::message::{Gravity, Message};
    let messenger = Message::default_messenger();
    if messenger.printers().is_empty() {
        println!("Error: no printers registered in default Messenger");
    }
    for printer in messenger.printers().iter() {
        printer.set_trace_level(Gravity::Trace);
    }
    martensite_tkservice::font::FontFontMgr::get_instance().set_trace_aliases(true);
}

#[test]
fn authored_font_trace_commands_update_the_actual_message_and_font_services() {
    use martensite_tkernel::message::{Gravity, Message};
    let manager = martensite_tkservice::font::FontFontMgr::get_instance();
    let previous_aliases = manager.to_trace_aliases();
    let printers = Message::default_messenger().printers().clone();
    let previous_levels: Vec<Gravity> = printers.iter().map(|p| p.trace_level()).collect();
    enable_font_trace();
    assert!(manager.to_trace_aliases());
    assert!(printers.iter().all(|p| p.trace_level() == Gravity::Trace));
    manager.set_trace_aliases(previous_aliases);
    for (printer, level) in printers.iter().zip(previous_levels) {
        printer.set_trace_level(level);
    }
}
