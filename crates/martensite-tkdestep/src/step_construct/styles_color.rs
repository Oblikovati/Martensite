//! The colours of `StepConstructStyles`: STEP colour entities to `Color` and back.
use super::styles::StepConstructStyles;
use crate::step_entity::step_live;
use crate::step_visual::{
    StepVisualColour, StepVisualColourRgb, StepVisualDraughtingPreDefinedColour,
    StepVisualPreDefinedColour, StepVisualPreDefinedItem,
};
use martensite_tkernel::quantity::{Color, ColorSpace, NamedColor};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use std::collections::HashMap;

const WHERE: &str = "StepConstructStyles";

/// The colours predefined in STEP, in the order the tests them.
const PREDEFINED: [(NamedColor, &str); 8] = [
    (NamedColor::GREEN, "green"),
    (NamedColor::RED, "red"),
    (NamedColor::BLUE1, "blue"),
    (NamedColor::YELLOW, "yellow"),
    (NamedColor::MAGENTA1, "magenta"),
    (NamedColor::CYAN1, "cyan"),
    (NamedColor::BLACK, "black"),
    (NamedColor::WHITE, "white"),
];

/// The name of a colour predefined in STEP, if it is one.
fn predefined_name(color: &Color) -> Option<&'static str> {
    PREDEFINED
        .iter()
        .find(|(named, _)| *color == Color::named(*named))
        .map(|(_, name)| *name)
}

fn new_predefined(name: &str) -> Handle<StepVisualDraughtingPreDefinedColour> {
    let colour = Handle::new(StepVisualDraughtingPreDefinedColour::new());
    let item = Handle::new(StepVisualPreDefinedItem::new());
    item.init(&Handle::new(HAsciiString::new(name)));
    colour.set_pre_defined_item(&item);
    colour
}

fn new_rgb([red, green, blue]: [f64; 3]) -> Handle<StepVisualColourRgb> {
    let colour = Handle::new(StepVisualColourRgb::new());
    colour.init(&Handle::new(HAsciiString::new("")), red, green, blue);
    colour
}

/// The key of the map of RGB colours: the sRGB values, compared exactly as the point key.
fn rgb_key(values: [f64; 3]) -> [u64; 3] {
    values.map(f64::to_bits)
}

impl StepConstructStyles {
    /// `encode_color(Col)`: a predefined colour entity when the colour is one of the colours STEP
    /// predefines, otherwise an RGB one.
    pub fn encode_color(color: &Color) -> Result<Handle<StepVisualColour>, Failure> {
        // detect if color corresponds to one of pre-defined colors
        if let Some(name) = predefined_name(color) {
            return Ok(new_predefined(name).cast());
        }
        Ok(new_rgb(color.values(ColorSpace::Srgb)).cast())
    }
    /// `encode_color(Col, DPDCs, ColRGBs)`: as `encode_color`, sharing one entity per colour through the
    /// two maps.
    pub fn encode_color_shared(
        color: &Color,
        dpdcs: &mut HashMap<String, Handle<StandardTransient>>,
        col_rgbs: &mut HashMap<[u64; 3], Handle<StandardTransient>>,
    ) -> Result<Handle<StepVisualColour>, Failure> {
        // detect if color corresponds to one of pre-defined colors
        if let Some(name) = predefined_name(color) {
            let known = dpdcs
                .get(name)
                .map(|bound| bound.cast::<StepVisualDraughtingPreDefinedColour>());
            if let Some(known) = known.filter(Handle::is_some) {
                return Ok(known.cast());
            }
            let colour = new_predefined(name);
            dpdcs.insert(name.to_owned(), colour.cast());
            return Ok(colour.cast());
        }
        let values = color.values(ColorSpace::Srgb);
        let key = rgb_key(values);
        let known = col_rgbs
            .get(&key)
            .map(|bound| bound.cast::<StepVisualColourRgb>());
        if let Some(known) = known.filter(Handle::is_some) {
            return Ok(known.cast());
        }
        let colour = new_rgb(values);
        col_rgbs.insert(key, colour.cast());
        Ok(colour.cast())
    }
    /// `decode_color`: fills `color` from a STEP colour; false when not recognized.
    pub fn decode_color(
        colour: &Handle<StepVisualColour>,
        color: &mut Color,
    ) -> Result<bool, Failure> {
        step_live(colour, WHERE)?;
        if let Some(rgb) = colour.cast::<StepVisualColourRgb>().get() {
            let (red, green, blue) = (rgb.red(), rgb.green(), rgb.blue());
            if red > 1. || green > 1. || blue > 1. {
                let norm = red.max(green).max(blue);
                color.set_values(red / norm, green / norm, blue / norm, ColorSpace::Srgb)?;
            } else {
                color.set_values(red, green, blue, ColorSpace::Srgb)?;
            }
            return Ok(true);
        }
        let predefined = colour.cast::<StepVisualPreDefinedColour>();
        let Some(predefined) = predefined.get() else {
            return Ok(false);
        };
        let item = predefined.get_pre_defined_item();
        let name = step_live(&item, WHERE)?.name();
        let name = step_live(&name, WHERE)?.string();
        let found = PREDEFINED
            .iter()
            .find(|(_, text)| name.as_bytes() == text.as_bytes());
        let Some((named, _)) = found else {
            return Ok(false);
        };
        color.set_named(*named);
        Ok(true)
    }
}
