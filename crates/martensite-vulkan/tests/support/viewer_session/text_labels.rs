//! Text labels: placed at a point, optionally on a plane, in a registered or system font.
use super::*;
#[path = "font_registration.rs"]
mod font_registration;
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkservice::font::{FontFontAspect, FontFontMgr, FontSystemFont};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObject as _;

/// A label's text and the options a label command sets; `None` keeps the label default.
/// Example: `TextLabelRequest { height: Some(20.), ..TextLabelRequest::new("Default") }`.
#[derive(Clone, Debug)]
pub struct TextLabelRequest<'a> {
    /// The text, lines separated by newlines.
    pub text: &'a str,
    /// The anchor point.
    pub position: [f64; 3],
    /// The text colour.
    pub color: Option<Color>,
    /// The horizontal alignment at the anchor.
    pub horizontal: Option<Graphic3dHorizontalTextAlignment>,
    /// The vertical alignment at the anchor.
    pub vertical: Option<Graphic3dVerticalTextAlignment>,
    /// The rotation in the screen plane, in degrees.
    pub angle_degrees: Option<f64>,
    /// Whether the text scales with the view.
    pub zoomable: Option<bool>,
    /// The font height in pixels.
    pub height: Option<f64>,
    /// The font aspect.
    pub aspect: Option<FontFontAspect>,
    /// The font family or alias.
    pub font: Option<&'a str>,
    /// The text plane as (normal, X direction) through `position`.
    pub plane: Option<([f64; 3], [f64; 3])>,
    /// Whether the text turns to stay readable from behind.
    pub flipping: Option<bool>,
}

impl<'a> TextLabelRequest<'a> {
    /// `text` at the origin with every other option left at the label default.
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            position: [0.; 3],
            color: None,
            horizontal: None,
            vertical: None,
            angle_degrees: None,
            zoomable: None,
            height: None,
            aspect: None,
            font: None,
            plane: None,
            flipping: None,
        }
    }

    fn apply(&self, label: &AisTextLabel) -> Result<(), Failure> {
        let [x, y, z] = self.position;
        // Multi-byte text for the label.
        label.set_text(&ExtendedString::from_utf16(
            &self.text.encode_utf16().collect::<Vec<_>>(),
        ));
        label.set_position(Point3::new(x, y, z));
        if let Some(color) = self.color {
            label.set_color(color)?;
        }
        self.apply_layout(label);
        self.apply_font(label);
        self.apply_plane(label)?;
        label.set_display_type(AspectTypeOfDisplayText::Normal);
        Ok(())
    }

    fn apply_layout(&self, label: &AisTextLabel) {
        if let Some(horizontal) = self.horizontal {
            label.set_h_justification(horizontal);
        }
        if let Some(vertical) = self.vertical {
            label.set_v_justification(vertical);
        }
        if let Some(degrees) = self.angle_degrees {
            label.set_angle(degrees * (std::f64::consts::PI / 180.0));
        }
        if let Some(zoomable) = self.zoomable {
            label.set_zoomable(zoomable);
        }
    }

    fn apply_font(&self, label: &AisTextLabel) {
        if let Some(height) = self.height {
            label.set_height(height);
        }
        if let Some(aspect) = self.aspect {
            label.set_font_aspect(aspect);
        }
        if let Some(font) = self.font {
            label.set_font(font);
        }
        if let Some(flipping) = self.flipping {
            label.set_flipping(flipping);
        }
    }

    fn apply_plane(&self, label: &AisTextLabel) -> Result<(), Failure> {
        let Some((normal, x_dir)) = self.plane else {
            return Ok(());
        };
        let [x, y, z] = self.position;
        let dir = |[a, b, c]: [f64; 3]| Direction3::new(a, b, c);
        let axes = Axis2::with_x_direction(Point3::new(x, y, z), dir(normal)?, dir(x_dir)?)?;
        label.set_orientation_3d(axes);
        Ok(())
    }
}

impl ViewerSession {
    /// Display a new text label `name`; `session.draw_text("t0", &request)?`.
    pub fn draw_text(&mut self, name: &str, request: &TextLabelRequest<'_>) -> Result<(), Failure> {
        let label = AisTextLabel::new();
        request.apply(&label)?;
        self.display_object(name, label.cast())
    }
}

/// Register the font file `path` for every aspect under the family `alias`, overriding a
/// registered family of that name; `register_font_file(path, "SansFont")?`.
pub fn register_font_file(path: &str, alias: &str) -> Result<(), Failure> {
    register_font_file_with(path, Some(alias), None)?;
    Ok(())
}

/// Register checked family properties in native command order; `register_font_file_with(path, None, None)?`.
pub fn register_font_file_with(
    path: &str,
    alias: Option<&str>,
    single_stroke: Option<bool>,
) -> Result<Handle<FontSystemFont>, Failure> {
    let manager = FontFontMgr::get_instance();
    let manager = manager
        .get()
        .expect("the font manager instance is never null");
    let found = manager.check_font(&AsciiString::new(path))?;
    if found.is_null() {
        return Err(martensite_tkernel::standard::exceptions::program_error(
            &format!("font file {path:?}; expected a readable font",),
        ));
    }
    let font = font_registration::command_font_record(&found, alias, single_stroke)?;
    manager.register_font(&font, true)?;
    Ok(font)
}
