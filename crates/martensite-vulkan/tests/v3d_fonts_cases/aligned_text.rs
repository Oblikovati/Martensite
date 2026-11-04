//! Nine multi-line texts in a front plane, one per horizontal and vertical alignment.
use super::prelude::*;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

/// The alignment rows (top, centre, bottom) with their Z and colour, and the columns.
const ROWS: [(&str, VAlign, f64, NamedColor); 3] = [
    ("Top", VAlign::Top, 600., NamedColor::CYAN),
    ("Center", VAlign::Center, -100., NamedColor::WHITE),
    ("Bottom", VAlign::Bottom, -700., NamedColor::YELLOW),
];
const COLUMNS: [(&str, HAlign, f64); 3] = [
    ("Left", HAlign::Left, -700.),
    ("Center", HAlign::Center, 0.),
    ("Right", HAlign::Right, 700.),
];

/// The nine texts made by `text_of(row, column)`, then coloured by row and fitted.
fn aligned_texts(
    image: &'static str,
    font: &'static str,
    aspect: FontFontAspect,
    text_of: fn(&str, &str) -> String,
) -> Outcome {
    let mut run = SessionRun::new()?;
    run.session().display_trihedron("trihedr")?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_FRONT)?;
    run.session().set_display_mode(1)?;
    let mut shapes = Vec::new();
    for (row, vertical, z, _) in ROWS {
        for (column, horizontal, x) in COLUMNS {
            let id = shapes.len();
            super::prelude::display_point(
                &mut run,
                &format!("p{}{}", &row[..1], &column[..1]),
                [x, 100., z],
            )?;
            let options = BRepText {
                font,
                height: 50.,
                aspect,
                position: [x, 100., z],
                plane: ([0., -1., 0.], [1., 0., 0.]),
                horizontal,
                vertical,
                ..Default::default()
            };
            shapes.push((
                format!("Text{id}"),
                text_shape(&text_of(row, column), &options)?,
            ));
        }
    }
    for (name, shape) in &shapes {
        run.session()
            .display(name, shape, &DisplayRequest::default())?;
    }
    for (index, (_, _, _, colour)) in ROWS.into_iter().enumerate() {
        for (name, _) in &shapes[index * 3..index * 3 + 3] {
            run.session().set_color(name, Color::named(colour))?;
        }
    }
    run.session().fit()?;
    run.dump(image)?;
    run.finish()
}

/// `B4`: regular Times-Roman texts.
pub(crate) fn aligned_regular_text() -> Outcome {
    banner(&[
        "============",
        "OCC24272 Provide basic text formatting routines for Font_BRepFont",
        "Test case prints 3D brep text with different text alignment styles",
        "============",
        "",
    ]);
    aligned_texts(
        "B4.png",
        "Times-Roman",
        FontFontAspect::Regular,
        |row, column| format!("{row}-{column}\nFirst line\nLion The Second\n3rd"),
    )
}

/// `B5`: bold Arial texts with leading, trailing and repeated spaces.
pub(crate) fn aligned_bold_text() -> Outcome {
    banner(&[
        "============",
        "OCC24272 Provide basic text formatting routines for Font_BRepFont",
        "Test case prints 3D brep text with different text alignment styles",
        "============",
        "",
    ]);
    aligned_texts("B5.png", "Arial", FontFontAspect::Bold, |row, column| {
        format!("  {row}-{column}\nFirst line  \nLion   The Second\n  3rd  ")
    })
}
