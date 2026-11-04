//! Explicit camera parameters of the active view: scale, projection direction, up and centre.
use super::*;

/// Camera values to set; `None` keeps the view's current value.
/// Example: `ViewParameters { scale: Some(6.06), ..ViewParameters::default() }`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ViewParameters {
    /// The view scale.
    pub scale: Option<f64>,
    /// The eye point.
    pub eye: Option<[f64; 3]>,
    /// The centre of interest.
    pub at: Option<[f64; 3]>,
    /// The direction from the centre of interest towards the eye.
    pub proj: Option<[f64; 3]>,
    /// The up direction.
    pub up: Option<[f64; 3]>,
}

impl ViewerSession {
    /// Set the given camera values in scale, eye, at, projection, up order, then redraw.
    /// Example: `session.set_view_parameters(&ViewParameters { scale: Some(2.), ..Default::default() })?`.
    pub fn set_view_parameters(&self, parameters: &ViewParameters) -> Result<(), Failure> {
        let view = self.view();
        with_source_view_update(
            &**view,
            || apply_view_parameters(view.view_base(), parameters),
            || view.redraw(),
        )
    }
}

fn apply_view_parameters(view: &V3dView, parameters: &ViewParameters) -> Result<(), Failure> {
    if let Some(scale) = parameters.scale {
        view.set_scale(scale)?;
    }
    if let Some([x, y, z]) = parameters.eye {
        view.set_eye(x, y, z)?;
    }
    if let Some([x, y, z]) = parameters.at {
        view.set_at(x, y, z)?;
    }
    if let Some([x, y, z]) = parameters.proj {
        view.set_proj(x, y, z)?;
    }
    if let Some([x, y, z]) = parameters.up {
        view.set_up(x, y, z)?;
    }
    Ok(())
}
