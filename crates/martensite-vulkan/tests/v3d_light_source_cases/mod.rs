//! Light sources shown as interactive presentations: headlights, display, toggling and
//! dragging.
mod display_all;
mod dyn_highlight;
mod headlight;
mod manipulator;
mod scene;
pub(crate) use display_all::every_light_kind;
pub(crate) use dyn_highlight::sphere_detection;
pub(crate) use headlight::headlight_spot_and_positional;
pub(crate) use manipulator::light_manipulator;
