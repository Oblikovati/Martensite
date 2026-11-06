//! Text rendered as B-Rep shapes next to text labels: system and registered fonts, styles,
//! composite contours, alignment, and a circled symbol.
mod aligned_text;
mod brep_text;
mod single_line_inputs;
mod single_line_text;
mod text_lines;
pub(crate) use aligned_text::{aligned_bold_text, aligned_regular_text};
pub(crate) use brep_text::{alphabet_shapes, circled_symbol};
pub(crate) use single_line_text::single_line_font_shapes;
pub(crate) use text_lines::{composite_system_lines, registered_font_lines, system_font_lines};

/// What every font program uses.
mod prelude {
    pub(super) use super::brep_text::{BRepText, text_shape};
    pub(super) use crate::case_view::viewer_session::{DisplayRequest, TextLabelRequest};
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use martensite_tkernel::quantity::{Color, NamedColor};
    pub(super) use martensite_tkernel::standard::Handle;
    pub(super) use martensite_tkservice::font::FontFontAspect;
    pub(super) use martensite_tkservice::graphic3d::{
        Graphic3dHorizontalTextAlignment as HAlign, Graphic3dVerticalTextAlignment as VAlign,
    };
    pub(super) use std::error::Error;

    pub(super) type Outcome = Result<(), Box<dyn Error>>;

    /// `DejaVuSans.ttf` registered as `SansFont`; the data-file lookup of the cases
    /// resolves it to the system copy.
    pub(super) fn register_sans_font() -> Outcome {
        let path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";
        crate::case_view::viewer_session::register_font_file(path, "SansFont")?;
        Ok(())
    }

    /// A point marker `name` at `position`, as the point command displays it.
    pub(super) fn display_point(run: &mut SessionRun, name: &str, [x, y, z]: [f64; 3]) -> Outcome {
        use martensite_tkg3d::geom::GeomCartesianPoint;
        use martensite_tkv3d::ais::AisPoint;
        let point = Handle::new(GeomCartesianPoint::new(x, y, z));
        run.session()
            .display_object(name, AisPoint::new(&point.cast())?.cast())?;
        Ok(())
    }
}
