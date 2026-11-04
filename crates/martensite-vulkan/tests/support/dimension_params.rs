//! Changes to a displayed dimension: its flyout, arrows, label, units and value (the dimension
//! parameter command) and the direction of a length (the length parameter command), each followed by
//! a validity check and a redisplay.
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tkmath::gp::Direction3;
use martensite_tkv3d::prs_dim::{
    PrsDimDimensionMembers as _, PrsDimDimensionSource, PrsDimLengthDimension,
};
use martensite_tkv3d::prs3d::{
    Prs3dDimensionArrowOrientation as Orientation, Prs3dDimensionAspect,
    Prs3dDimensionTextHorizontalPosition as HPos, Prs3dDimensionTextVerticalPosition as VPos,
};
use std::error::Error;

/// The aspect options of the dimension commands: text, label,
/// arrow, arrow length, zoomable arrow and units; `None` keeps the current value.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct DimensionStyle {
    pub arrow_length: Option<f64>,
    pub text_3d: Option<bool>,
    pub color: Option<martensite_tkernel::quantity::Color>,
    pub arrow: Option<Orientation>,
    pub label: Option<HPos>,
    pub label_vertical: Option<VPos>,
    pub zoomable_arrow: Option<bool>,
    pub text_height: Option<f64>,
    pub text_shaded: Option<bool>,
    pub show_units: Option<bool>,
}

impl DimensionStyle {
    /// Apply the set options to `aspect`; `style.apply(&aspect)`.
    pub(crate) fn apply(&self, aspect: &Prs3dDimensionAspect) {
        self.apply_parts(aspect);
        self.apply_layout(aspect);
        self.apply_visibility(aspect);
        self.apply_text_and_color(aspect);
    }

    fn apply_layout(&self, aspect: &Prs3dDimensionAspect) {
        if let Some(orientation) = self.arrow {
            aspect.set_arrow_orientation(orientation);
        }
        if let Some(position) = self.label {
            aspect.set_text_horizontal_position(position);
        }
        if let Some(position) = self.label_vertical {
            aspect.set_text_vertical_position(position);
        }
    }

    fn apply_visibility(&self, aspect: &Prs3dDimensionAspect) {
        if let Some(shaded) = self.text_shaded {
            aspect.make_text_shaded(shaded);
        }
        if let Some(show) = self.show_units {
            aspect.make_units_displayed(show);
        }
    }

    fn apply_text_and_color(&self, aspect: &Prs3dDimensionAspect) {
        if let Some(enabled) = self.text_3d {
            aspect.make_text_3d(enabled);
        }
        if let Some(color) = self.color {
            aspect.set_common_color(color);
        }
    }

    /// The options held by the aspect's arrow and text aspects.
    fn apply_parts(&self, aspect: &Prs3dDimensionAspect) {
        if let (Some(length), Some(arrow)) = (self.arrow_length, aspect.arrow_aspect().get()) {
            arrow.set_length(length);
        }
        if let (Some(zoomable), Some(arrow)) = (self.zoomable_arrow, aspect.arrow_aspect().get()) {
            arrow.set_zoomable(zoomable);
        }
        if let (Some(height), Some(text)) = (self.text_height, aspect.text_aspect().get()) {
            text.set_height(height);
        }
    }
}

/// The dimension parameter options; `None` keeps the current value. The values
/// apply in this order: flyout, computed value, custom value, custom text.
#[derive(Clone, Debug, Default)]
pub(crate) struct DimensionParams {
    pub flyout: Option<f64>,
    pub style: DimensionStyle,
    pub auto_value: bool,
    pub arrow_angle: Option<f64>,
    pub value: Option<f64>,
    pub text_value: Option<&'static str>,
}

impl DimensionParams {
    fn apply_arrow_angle(&self, value: &dyn PrsDimDimensionSource) -> Result<(), Box<dyn Error>> {
        let Some(angle) = self.arrow_angle else {
            return Ok(());
        };
        let aspect = value.dimension_aspect();
        let aspect = aspect
            .get()
            .ok_or("arrow angle requires a live dimension aspect")?;
        let arrow = aspect.arrow_aspect();
        arrow
            .get()
            .ok_or("arrow angle requires a live arrow aspect")?
            .set_angle(angle)?;
        Ok(())
    }

    /// Apply requested value changes in command order; `params.apply_value(dimension)`.
    pub(crate) fn apply_value(&self, value: &dyn PrsDimDimensionSource) {
        if let Some(flyout) = self.flyout {
            value.set_flyout(flyout);
        }
        if self.auto_value {
            value.set_computed_value();
        }
        if let Some(custom) = self.value {
            value.set_custom_value(custom);
        }
        if let Some(text) = self.text_value {
            value.set_custom_value_text(&ExtendedString::from_utf8(text));
        }
    }
}

/// The displayed dimension `name`; `named_dimension(&session, "dim1")?`.
fn named_dimension(
    session: &ViewerSession,
    name: &str,
) -> Result<Handle<dyn PrsDimDimensionSource>, Box<dyn Error>> {
    let object = session
        .object(name)
        .ok_or_else(|| format!("Error: no object with name '{name}'"))?;
    let dim = object.cast::<dyn PrsDimDimensionSource>();
    if dim.is_null() {
        return Err(format!("Error: '{name}' is not a dimension").into());
    }
    Ok(dim)
}

/// Check the dimension and redisplay it when displayed; `finish_change(&session, &dim)?`.
pub(crate) fn finish_change(
    session: &ViewerSession,
    dim: &Handle<dyn PrsDimDimensionSource>,
) -> Result<(), Box<dyn Error>> {
    if !dim.get().ok_or("a dimension")?.is_valid() {
        return Err("Error: Dimension geometry or plane is not valid".into());
    }
    let object = dim.clone().cast();
    if session.context().is_displayed(&object) {
        session.context().redisplay(&object, true, false)?;
    }
    Ok(())
}

/// Apply `params` to the dimension `name`'s own aspect; `set_dimension_params(&s, "d", &p)?`.
pub(crate) fn set_dimension_params(
    session: &ViewerSession,
    name: &str,
    params: &DimensionParams,
) -> Result<(), Box<dyn Error>> {
    let dim = named_dimension(session, name)?;
    let value = dim.get().ok_or("a dimension")?;
    let aspect = value.dimension_aspect();
    params
        .style
        .apply(aspect.get().ok_or("a dimension aspect")?);
    params.apply_arrow_angle(value)?;
    params.apply_value(value);
    finish_change(session, &dim)
}

#[allow(dead_code)] // Only the length-direction regressions measure along a direction.
/// Measure the length `name` along `direction`, or automatically for `None`.
/// Example: `set_length_direction(&session, "dim3", Some(dx()))?`.
pub(crate) fn set_length_direction(
    session: &ViewerSession,
    name: &str,
    direction: Option<Direction3>,
) -> Result<(), Box<dyn Error>> {
    let object = session
        .object(name)
        .ok_or_else(|| format!("Syntax error: no object with name '{name}'"))?;
    let length = object.cast::<PrsDimLengthDimension>();
    let value = length
        .get()
        .ok_or_else(|| format!("Syntax error: no length dimension with name '{name}'"))?;
    value.set_direction(&direction.unwrap_or_default(), direction.is_some());
    finish_change(session, &length.cast())
}

#[cfg(test)]
mod dimension_cpu_contract {
    include!("../support/dimension_params_contract.rs");
}

#[cfg(test)]
#[path = "dimension_style_contract.rs"]
mod dimension_style_contract;
