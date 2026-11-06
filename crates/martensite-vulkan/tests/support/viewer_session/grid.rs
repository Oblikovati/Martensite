//! The viewer's construction grid (rectangular or circular), drawn as lines or points.
use super::*;

/// Grid layout values; `None` keeps the viewer's current value.
#[derive(Clone, Copy, Debug, Default)]
pub struct GridLayout {
    /// Grid origin in the privileged plane.
    pub origin: Option<(f64, f64)>,
    /// Rectangular: (step x, step y). Circular: (radius step, division count).
    pub step: Option<(f64, f64)>,
    /// In-plane rotation in radians.
    pub rotation: Option<f64>,
}

/// One grid activation; `GridRequest::rectangular(layout)`.
#[derive(Clone, Copy, Debug)]
pub struct GridRequest {
    /// Rectangular or circular; `None` keeps the viewer's current type.
    pub kind: Option<AspectGridType>,
    /// Lines or points; `None` keeps the viewer's current mode.
    pub mode: Option<AspectGridDrawMode>,
    /// Origin, step and rotation.
    pub layout: GridLayout,
}

impl GridRequest {
    /// A rectangular grid; `GridRequest::rectangular(layout)`.
    pub fn rectangular(layout: GridLayout) -> Self {
        Self {
            kind: Some(AspectGridType::Rectangular),
            mode: None,
            layout,
        }
    }

    /// A circular grid; `GridRequest::circular(layout)`.
    pub fn circular(layout: GridLayout) -> Self {
        Self {
            kind: Some(AspectGridType::Circular),
            mode: None,
            layout,
        }
    }

    /// The same grid drawn as `mode`; `request.drawn_as(AspectGridDrawMode::Points)`.
    pub fn drawn_as(self, mode: AspectGridDrawMode) -> Self {
        Self {
            mode: Some(mode),
            ..self
        }
    }
}

impl ViewerSession {
    /// Set the viewer grid values and activate it; `session.show_grid(&request)?`.
    pub fn show_grid(&self, request: &GridRequest) -> Result<(), Failure> {
        let viewer = self.viewer.clone();
        let request = *request;
        self.with_update(move |_| {
            let kind = request.kind.unwrap_or_else(|| viewer.grid_type());
            let mode = match request.mode {
                Some(mode) => mode,
                None => viewer.grid_draw_mode()?,
            };
            match kind {
                AspectGridType::Circular => set_circular(&viewer, &request.layout)?,
                _ => set_rectangular(&viewer, &request.layout)?,
            }
            viewer.activate_grid(kind, mode)
        })
    }

    /// Erase the view's grid and deactivate the viewer grid; `session.hide_grid()?`.
    pub fn hide_grid(&self) -> Result<(), Failure> {
        let view = self.view().clone();
        let viewer = self.viewer.clone();
        self.with_update(move |_| {
            view.view_base().grid_erase();
            viewer.deactivate_grid()
        })
    }
}

fn set_rectangular(viewer: &V3dViewer, layout: &GridLayout) -> Result<(), Failure> {
    let (x, y, sx, sy, angle) = viewer.rectangular_grid_values()?;
    let (x, y) = layout.origin.unwrap_or((x, y));
    let (sx, sy) = layout.step.unwrap_or((sx, sy));
    viewer.set_rectangular_grid_values(x, y, sx, sy, layout.rotation.unwrap_or(angle))
}

fn set_circular(viewer: &V3dViewer, layout: &GridLayout) -> Result<(), Failure> {
    let (x, y, mut radius_step, mut divisions, angle) = viewer.circular_grid_values()?;
    let (x, y) = layout.origin.unwrap_or((x, y));
    if let Some((step, count)) = layout.step {
        radius_step = step;
        divisions = count as i32;
    }
    if divisions < 1 {
        return Err(martensite_tkernel::standard::exceptions::program_error(
            &format!("circular grid divisions {divisions}; expected at least one"),
        ));
    }
    viewer.set_circular_grid_values(
        x,
        y,
        radius_step,
        divisions,
        layout.rotation.unwrap_or(angle),
    )
}
