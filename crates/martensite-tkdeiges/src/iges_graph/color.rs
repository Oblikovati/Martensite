//! IgesGraphColor.
#![allow(
    unused_mut,
    unused_variables,
    unused_assignments,
    clippy::needless_late_init,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::collapsible_else_if,
    clippy::needless_return,
    reason = "the members keep the order and the locals of the record they read"
)]
use crate::iges_data::IgesDataColorEntity;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// defines IGESColor, Type `314` Form `0`
///
/// The Color Definition Entity is used to communicate the
/// relationship of primary colors to the intensity level of
/// the respective graphics devices as a percent of full
/// intensity range.
IgesGraphColor, IgesGraphColorState, "IgesGraphColor", kind dyn IgesDataColorEntity, {
the_red: f64,
the_green: f64,
the_blue: f64,
the_color_name: Handle<HAsciiString>,
}
}

impl IgesDataColorEntity for IgesGraphColor {}

impl IgesGraphColor {
    iges_methods! {
    get color_name = the_color_name: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class Color
    /// - red        : Red   color intensity (range 0.0 to 100.0)
    /// - green      : Green color intensity (range 0.0 to 100.0)
    /// - blue       : Blue  color intensity (range 0.0 to 100.0)
    /// - aColorName : Name of the color (optional)
    pub fn init(&self, red: f64, green: f64, blue: f64, a_color_name: &Handle<HAsciiString>) {
        {
            let mut state = self.state.write();
            state.the_red = red;
            state.the_green = green;
            state.the_blue = blue;
            state.the_color_name = a_color_name.clone();
        }
        self.init_type_and_form(314, 0);
    }
    /// `rgb_intensity`.
    pub fn rgb_intensity(&self, red: &mut f64, green: &mut f64, blue: &mut f64) {
        *red = self.state.read().the_red;
        *green = self.state.read().the_green;
        *blue = self.state.read().the_blue;
    }
    /// `cmy_intensity`.
    pub fn cmy_intensity(&self, cyan: &mut f64, magenta: &mut f64, yellow: &mut f64) {
        *cyan = 100.0 - (self.state.read().the_red);
        *magenta = 100.0 - (self.state.read().the_green);
        *yellow = 100.0 - (self.state.read().the_blue);
    }
    /// `hls_percentage`.
    pub fn hls_percentage(&self, hue: &mut f64, lightness: &mut f64, saturation: &mut f64) {
        *hue = (1.0 / (2.0 * (std::f64::consts::PI)))
            * (martensite_cmath::atan(
                (((2.0 * (self.state.read().the_red)) - (self.state.read().the_green))
                    - (self.state.read().the_blue))
                    / (1.732_050_807_568_877_2
                        * ((self.state.read().the_green) - (self.state.read().the_blue))),
            ));
        *lightness = (1.0 / 3.0)
            * (((self.state.read().the_red) + (self.state.read().the_green))
                + (self.state.read().the_blue));
        *saturation = (((((((self.state.read().the_red) * (self.state.read().the_red))
            + ((self.state.read().the_green) * (self.state.read().the_green)))
            + ((self.state.read().the_blue) * (self.state.read().the_blue)))
            - ((self.state.read().the_red) * (self.state.read().the_green)))
            - ((self.state.read().the_red) * (self.state.read().the_blue)))
            - ((self.state.read().the_blue) * (self.state.read().the_green)))
            .sqrt();
    }
    /// returns True if optional character string is assigned,
    /// False otherwise.
    pub fn has_color_name(&self) -> bool {
        !(self.state.read().the_color_name.is_null())
    }
}
