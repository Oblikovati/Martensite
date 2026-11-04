//! The view's shader grid: an infinite or bounded rectangular/polar grid drawn per pixel.
use super::*;
use martensite_tkmath::gp::Point3;

/// Shader grid settings; `None`/`false` keeps the viewer-derived default.
#[derive(Clone, Copy, Debug, Default)]
pub struct ShaderGridRequest {
    /// Circular when set to `Circular`, or when a radius or an arc is given.
    pub kind: Option<AspectGridType>,
    /// Lines (default) or points.
    pub mode: Option<AspectGridDrawMode>,
    /// Origin, step and rotation, as for the viewer grid.
    pub layout: GridLayout,
    /// Rectangular bound (x, y).
    pub size: Option<(f64, f64)>,
    /// Circular bound.
    pub radius: Option<f64>,
    /// Offset along the plane normal.
    pub z_offset: Option<f64>,
    /// Angular range (start, end) of a polar grid.
    pub arc: Option<(f64, f64)>,
    /// Main line colour.
    pub color: Option<Color>,
    /// Draw behind the scene.
    pub background: Option<bool>,
    /// Draw the plane axes.
    pub draw_axis: Option<bool>,
    /// Adapt the cell spacing to the camera scale.
    pub view_adaptive: Option<bool>,
    /// Line thickness in grid units.
    pub line_thickness: Option<f64>,
}

impl ShaderGridRequest {
    fn is_circular(&self) -> bool {
        self.kind == Some(AspectGridType::Circular) || self.radius.is_some() || self.arc.is_some()
    }

    fn parsed_params(&self) -> Result<AspectGridParams, Failure> {
        let mut params = AspectGridParams::default();
        if let Some(color) = self.color {
            params.set_color(color);
        }
        if let Some((start, end)) = self.arc {
            params.set_arc_range(start, end);
        }
        if let Some(thickness) = self.line_thickness {
            params.set_line_thickness(thickness)?;
        }
        if let Some(value) = self.background {
            params.set_is_background(value);
        }
        if let Some(value) = self.draw_axis {
            params.set_is_draw_axis(value);
        }
        if let Some(value) = self.view_adaptive {
            params.set_is_view_adaptive(value);
        }
        Ok(params)
    }
}

impl ViewerSession {
    /// Display the shader grid on the active view; `session.show_shader_grid(&request)?`.
    pub fn show_shader_grid(&self, request: &ShaderGridRequest) -> Result<(), Failure> {
        let viewer = self.viewer.clone();
        let view = self.view().clone();
        let request = *request;
        self.with_update(move |_| {
            let mut params = request.parsed_params()?;
            let (origin, rotation) = match request.is_circular() {
                true => circular_params(&viewer, &request, &mut params)?,
                false => rectangular_params(&viewer, &request, &mut params)?,
            };
            finish_params(&request, &mut params, origin, rotation)?;
            view.view_base().grid_display(&params);
            Ok(())
        })
    }
}

fn finish_params(
    request: &ShaderGridRequest,
    params: &mut AspectGridParams,
    origin: (f64, f64),
    rotation: f64,
) -> Result<(), Failure> {
    params.set_origin(Point3::new(-origin.0, -origin.1, 0.0));
    params.set_draw_mode(request.mode.unwrap_or(AspectGridDrawMode::Lines));
    params.set_rotation_angle(rotation);
    if let Some((x, y)) = request.size.filter(|_| !request.is_circular()) {
        params.set_size_x(x)?;
        params.set_size_y(y)?;
    }
    if let Some(radius) = request.radius {
        params.set_radius(radius)?;
    }
    if let Some(offset) = request.z_offset {
        params.set_z_offset(offset);
    }
    Ok(())
}

fn circular_params(
    viewer: &V3dViewer,
    request: &ShaderGridRequest,
    params: &mut AspectGridParams,
) -> Result<((f64, f64), f64), Failure> {
    let (x, y, mut radius_step, mut divisions, rotation) = viewer.circular_grid_values()?;
    let layout = &request.layout;
    if let Some((step, count)) = layout.step {
        radius_step = step;
        divisions = count as i32;
    }
    if radius_step <= 0.0 {
        radius_step = 1.0;
    }
    if divisions <= 0 {
        divisions = 16;
    }
    params.set_scale(1.0 / radius_step)?;
    params.set_scale_y(0.0)?;
    params.set_angular_divisions(divisions);
    Ok((
        layout.origin.unwrap_or((x, y)),
        layout.rotation.unwrap_or(rotation),
    ))
}

fn rectangular_params(
    viewer: &V3dViewer,
    request: &ShaderGridRequest,
    params: &mut AspectGridParams,
) -> Result<((f64, f64), f64), Failure> {
    let (x, y, step_x, step_y, rotation) = viewer.rectangular_grid_values()?;
    let layout = &request.layout;
    let (sx, sy) = layout.step.unwrap_or((
        if step_x > 0.0 { step_x } else { 1.0 },
        if step_y > 0.0 { step_y } else { 1.0 },
    ));
    params.set_scale(1.0 / sx)?;
    params.set_scale_y(1.0 / sy)?;
    params.set_angular_divisions(0);
    Ok((
        layout.origin.unwrap_or((x, y)),
        layout.rotation.unwrap_or(rotation),
    ))
}
