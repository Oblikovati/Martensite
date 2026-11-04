//! Buffers are created by each callback, then owned by that frame's submission.
use martensite_tkernel::standard::{Handle, SharedState, WeakHandle};
use martensite_tkmath::gp::Point3;
use martensite_tkservice::{aspect::AspectTypeOfLine, graphic3d::*};
use martensite_vulkan::{RenderElement, RenderElementContext, RenderError};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[derive(Default)]
pub(super) struct CallbackTrace {
    pub calls: AtomicUsize,
    pub buffers: SharedState<Vec<WeakHandle<dyn Graphic3dBufferSource>>>,
}
pub(super) struct UserDrawPayload {
    pub coordinates: SharedState<[f32; 6]>,
    pub trace: CallbackTrace,
}
impl Default for UserDrawPayload {
    fn default() -> Self {
        Self {
            coordinates: SharedState::new([-10., -20., -30., 10., 20., 30.]),
            trace: CallbackTrace::default(),
        }
    }
}
pub(super) fn user_draw_positions(coordinates: [f32; 6]) -> [Point3; 4] {
    let [x0, y0, z0, x1, y1, z1] = coordinates.map(f64::from);
    [
        Point3::new(x0, y0, z0),
        Point3::new(x1, y1, z0),
        Point3::new(x1, y1, z1),
        Point3::new(x0, y0, z1),
    ]
}
// The presentation owns the element; retaining its parent object would create a cycle.
// Selection still uses the actual interactive object, while rendering owns its coordinates.
pub(super) struct UserDrawElement(pub Arc<UserDrawPayload>);
impl RenderElement for UserDrawElement {
    fn render(&self, context: &mut RenderElementContext<'_>) -> Result<(), RenderError> {
        let _marker = context.aspects().marker_type();
        let line = user_draw_line_aspect(context)?;
        let array = self.fresh_array()?;
        let previous = context.set_aspects(line.cast());
        context.add_primitive_array(
            array.primitive_type(),
            array.indices_ref(),
            array.attributes_ref(),
            array.bounds_ref(),
        );
        context.set_aspects(previous);
        self.0.trace.calls.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}
impl UserDrawElement {
    fn fresh_array(&self) -> Result<Graphic3dArrayOfPolylines, RenderError> {
        let array = Graphic3dArrayOfPolylines::new(4, 0, 5, 0).map_err(user_draw_buffer_failure)?;
        for point in user_draw_positions(*self.0.coordinates.read()) {
            array
                .add_vertex_pnt(&point)
                .map_err(user_draw_buffer_failure)?;
        }
        for vertex in [1, 2, 3, 4, 1] {
            array.add_edge(vertex).map_err(user_draw_buffer_failure)?;
        }
        self.0
            .trace
            .buffers
            .write()
            .push(array.attributes().downgrade());
        Ok(array)
    }
}
fn user_draw_line_aspect(
    context: &RenderElementContext<'_>,
) -> Result<Handle<Graphic3dAspectLine3d>, RenderError> {
    let line =
        Graphic3dAspectLine3d::new(*context.interior_color().rgb(), AspectTypeOfLine::Solid, 1.)
            .map_err(user_draw_buffer_failure)?;
    line.set_interior_color_rgba(context.interior_color());
    line.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    line.set_alpha_mode(Graphic3dAlphaMode::Opaque, 0.5);
    Ok(Handle::new(line))
}
fn user_draw_buffer_failure(cause: martensite_tkernel::standard::Failure) -> RenderError {
    RenderError::rejected(
        cause.to_string(),
        "valid fresh four-position closed user-draw buffer",
    )
}
