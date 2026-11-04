//! The complete single-line-font original: three registrations, three shapes and one image.
use super::{prelude::*, single_line_inputs::B6FontPaths};
use crate::case_view::viewer_session::register_font_file_with;
use martensite_tkbrep::topods::Shape;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
use std::path::Path;

/// Run B6 with its required exact inputs; `single_line_font_shapes()?`.
pub(crate) fn single_line_font_shapes() -> Outcome {
    banner(&[
        "============",
        "0029122: Visualization - improve Font_BRepFont to handle one-line-fonts",
        "============",
        "",
    ]);
    let paths = B6FontPaths::resolve()?;
    single_line_font_shapes_with_inputs(&paths)
}

fn single_line_font_shapes_with_inputs(paths: &B6FontPaths) -> Outcome {
    register_required_font(&paths.olf, None, None)?;
    register_required_font(&paths.machine_tool, Some("machtgth"), Some(true))?;
    register_required_font(&paths.sans, Some("SansFont"), None)?;
    let shapes = single_line_shapes()?;
    display_single_line_shapes(&shapes)
}

fn register_required_font(
    path: &Path,
    alias: Option<&str>,
    single_stroke: Option<bool>,
) -> Outcome {
    let utf8_path = path.to_str().ok_or_else(|| {
        format!("B6 font path {path:?}; expected a UTF-8 path to the required exact font")
    })?;
    let font = register_font_file_with(utf8_path, alias, single_stroke)?;
    println!("{}", std::str::from_utf8(font.to_string_desc().as_bytes())?);
    Ok(())
}

fn single_line_shapes() -> Result<[Shape; 3], Box<dyn Error>> {
    Ok([
        single_line_shape("OLF TestFont", [0., 0., 0.])?,
        single_line_shape("machtgth", [0., 50., 0.])?,
        single_line_shape("SansFont", [0., 100., 0.])?,
    ])
}

fn single_line_shape(font: &str, position: [f64; 3]) -> Result<Shape, Box<dyn Error>> {
    text_shape(
        "ABCDabcd123",
        &BRepText {
            font,
            height: 48.,
            position,
            ..Default::default()
        },
    )
}

fn display_single_line_shapes(shapes: &[Shape; 3]) -> Outcome {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    let request = DisplayRequest::default().with_display_mode(1);
    for (name, shape) in ["s1", "s2", "s3"].into_iter().zip(shapes) {
        run.session().display(name, shape, &request)?;
    }
    run.session().fit()?;
    run.dump("B6.png")?;
    run.finish()
}
