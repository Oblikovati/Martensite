//! Complete anonymous-object QA command regressions.
mod angle_arrows;
mod color_scale;
mod default_dump;
mod frame_control;
pub(crate) mod selected_removal;
pub(crate) use angle_arrows::angle_arrows;
pub(crate) use color_scale::anonymous_color_scale;
pub(crate) use default_dump::default_dump;
pub(crate) use selected_removal::selected_removal;
type Outcome = Result<(), Box<dyn std::error::Error>>;
