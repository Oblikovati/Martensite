//! IGES files to and from documents: colors, names and layers with the shapes.
mod reader;
mod writer;
mod writer_attributes;

pub use reader::IgesCafControlReader;
pub use writer::IgesCafControlWriter;

use martensite_tkernel::quantity::{Color, ColorSpace, NamedColor};

/// The color of an IGES color number; `decode_color(2)`.
pub fn decode_color(color: i32) -> Color {
    Color::named(match color {
        1 => NamedColor::BLACK,
        2 => NamedColor::RED,
        3 => NamedColor::GREEN,
        4 => NamedColor::BLUE1,
        5 => NamedColor::YELLOW,
        6 => NamedColor::MAGENTA1,
        7 => NamedColor::CYAN1,
        _ => NamedColor::WHITE,
    })
}

/// The IGES color number of a pure color, 0 for another one; `encode_color(&color)`.
pub fn encode_color(color: &Color) -> i32 {
    let epsilon = Color::epsilon();
    let channel = |value: f64| -> Option<bool> {
        if (value - 1.0).abs() <= epsilon {
            Some(true)
        } else if value.abs() > epsilon {
            None
        } else {
            Some(false)
        }
    };
    let [red, green, blue] = color.values(ColorSpace::Rgb);
    match (channel(red), channel(green), channel(blue)) {
        (Some(false), Some(false), Some(false)) => 1,
        (Some(true), Some(false), Some(false)) => 2,
        (Some(false), Some(true), Some(false)) => 3,
        (Some(false), Some(false), Some(true)) => 4,
        (Some(true), Some(true), Some(false)) => 5,
        (Some(true), Some(false), Some(true)) => 6,
        (Some(false), Some(true), Some(true)) => 7,
        (Some(true), Some(true), Some(true)) => 8,
        _ => 0,
    }
}
