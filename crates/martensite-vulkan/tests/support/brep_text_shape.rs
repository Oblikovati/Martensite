//! Shared text-to-BRep factory; original command defaults stay unchanged.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::String8;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::{Axis3, Direction3, Point3};
use martensite_tkservice::font::FontStrictLevel;
use martensite_tkv3d::std_prs::{StdPrsBRepFont, StdPrsBRepFontTextOptions};

use martensite_tkservice::font::FontFontAspect;
use martensite_tkservice::graphic3d::{
    Graphic3dHorizontalTextAlignment as HAlign, Graphic3dVerticalTextAlignment as VAlign,
};
use std::error::Error;

/// The options of one text-to-shape conversion, with the defaults of the conversion command
/// (Courier, height 16, regular, separate contours, XY plane at the origin, left/bottom).
#[derive(Clone, Copy, Debug)]
pub(crate) struct BRepText<'a> {
    pub font: &'a str,
    pub height: f64,
    pub aspect: FontFontAspect,
    pub composite: bool,
    pub position: [f64; 3],
    pub plane: ([f64; 3], [f64; 3]),
    pub horizontal: HAlign,
    pub vertical: VAlign,
}

impl Default for BRepText<'_> {
    fn default() -> Self {
        Self {
            font: "Courier",
            height: 16.,
            aspect: FontFontAspect::Regular,
            composite: false,
            position: [0.; 3],
            plane: ([0., 0., 1.], [1., 0., 0.]),
            horizontal: HAlign::Left,
            vertical: VAlign::Bottom,
        }
    }
}

/// The shape of `text` (UTF-8) with `options`; `text_shape("A", &BRepText::default())?`.
pub(crate) fn text_shape(text: &str, options: &BRepText) -> Result<Shape, Box<dyn Error>> {
    let font = StdPrsBRepFont::new();
    let name = AsciiString::from(options.font);
    if !font.find_and_init(&name, options.aspect, options.height, FontStrictLevel::Any) {
        return Err(format!(
            "font {:?} not loaded; expected an installed or registered font",
            options.font
        )
        .into());
    }
    let ([nx, ny, nz], [dx, dy, dz]) = options.plane;
    let [x, y, z] = options.position;
    let pen = Axis3::new(
        Point3::new(x, y, z),
        Direction3::new(nx, ny, nz)?,
        Direction3::new(dx, dy, dz)?,
    )?;
    let text_options = StdPrsBRepFontTextOptions {
        pen,
        horizontal_alignment: options.horizontal,
        vertical_alignment: options.vertical,
        to_concatenate_contours: options.composite,
    };
    let shape = font.render_text_with(
        &String8::from_unicode_units(text.as_bytes(), -1),
        &text_options,
    );
    if shape.is_null() {
        return Err(format!("text {text:?} produced no shape; expected text contours").into());
    }
    Ok(shape)
}
