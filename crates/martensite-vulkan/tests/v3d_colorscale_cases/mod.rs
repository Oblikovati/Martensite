//! Colour-scale regressions: ranges, logarithmic and custom intervals, labels, titles and
//! placement of the colour-scale overlay, each scale configured as one command would.
mod custom;
mod layout;
mod ranges;
mod scenes;
pub(crate) use custom::{customized_scales, find_color_custom, hue_and_free_labels};
pub(crate) use layout::{face_culled_scale, multiple_views, position_and_height};
pub(crate) use ranges::{logarithmic_ranges, uniform_lightness};
pub(crate) use scenes::{transparent_sphere, wide_view_overlay};

/// What every colour-scale regression program uses.
mod prelude {
    pub(super) use super::scale::{
        ColorScaleSetting, color_scale, demo, expect_pixels, linear_rgb,
    };
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use martensite_tkernel::quantity::{Color, NamedColor};
    pub(super) use martensite_tkernel::tcollection::ExtendedString;
    pub(super) use martensite_tkservice::aspect::{
        AspectTypeOfColorScaleData as Data, AspectTypeOfColorScalePosition as Pos,
    };
    pub(super) use martensite_tkv3d::ais::AisColorScale;
    pub(super) use std::error::Error;
    /// A regression program's result.
    pub(super) type Outcome = Result<(), Box<dyn Error>>;
}

mod scale;
