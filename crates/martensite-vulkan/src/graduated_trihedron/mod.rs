//! The graduated trihedron of a view (GlGraduatedTrihedron and the GlView members
//! `myGraduatedTrihedron`, `myGTrihedronData`, `myToShowGradTrihedron`): a grid
//! box around the scene bounds with axes, arrows, tickmarks, names and values, drawn after the
//! scene layers and the shader grid with depth test and writes (GlView::renderTrihedron).
mod axis_draws;
mod grid_axes;
mod labels;
mod trihedron_frame;

use crate::{RenderError, RenderLayer, error::rejected, stencil_program::StencilProgram};
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::standard::{Handle, SharedState};
use martensite_tkservice::graphic3d::*;
use trihedron_frame::TrihedronFrame;

/// The retained structure whose groups lower the trihedron primitives and the axis labels (the
/// text path needs the driver's fonts); it is never displayed.
struct LabelGroups {
    structure: Handle<Graphic3dStructure>,
    groups: [Handle<dyn Graphic3dGroup>; 3],
    primitives: Handle<dyn Graphic3dGroup>,
}

impl LabelGroups {
    fn new(manager: &Handle<Graphic3dStructureManager>) -> Result<Self, RenderError> {
        let structure = Graphic3dStructure::new(manager, &Handle::default())
            .map_err(|error| rejected(error, "a trihedron label structure of the view manager"))?;
        let group = || {
            structure
                .new_group()
                .map_err(|error| rejected(error, "a trihedron label group"))
        };
        let groups = [group()?, group()?, group()?];
        let primitives = group()?;
        Ok(Self {
            structure,
            groups,
            primitives,
        })
    }
}

struct TrihedronState {
    /// `myData`: the parameters of the last `SetValues` (defaults at first).
    data: Graphic3dGraduatedTrihedron,
    /// `myMin`, `myMax`: the cubic axes bounds, (0, 0, 0) to (100, 100, 100) at first.
    min: Vec3<f32>,
    max: Vec3<f32>,
    /// The bounds the axis lines were initialized with; reset by `SetValues` and `Release`.
    line_bounds: Option<(Vec3<f32>, Vec3<f32>)>,
    labels: Option<LabelGroups>,
}

impl Default for TrihedronState {
    fn default() -> Self {
        Self {
            data: Graphic3dGraduatedTrihedron::default(),
            min: Vec3::new(0., 0., 0.),
            max: Vec3::new(100., 100., 100.),
            line_bounds: None,
            labels: None,
        }
    }
}

/// `GlGraduatedTrihedron`: the graduated trihedron element of a view
/// ([`crate::Graphic3dRenderView::graduated_trihedron`]); the view renders it after its scene
/// while `GraduatedTrihedronDisplay` shows it.
/// Example: `let trihedron = RenderGraduatedTrihedron::new; trihedron.set_values(&data);`.
#[derive(Default)]
pub struct RenderGraduatedTrihedron {
    state: SharedState<TrihedronState>,
}

/// `SetMinMaxValuesCallback`: the view's scene bounds become the trihedron bounds.
pub(crate) fn min_max_values_callback(view: &dyn Graphic3dCView) {
    let Ok(bounds) = view.min_max_values(false) else {
        return;
    };
    let (Ok(min), Ok(max)) = (bounds.corner_min(), bounds.corner_max()) else {
        return;
    };
    let narrow = |x: f64, y: f64, z: f64| Vec3::new(x as f32, y as f32, z as f32);
    view.graduated_trihedron_min_max_values(
        narrow(min.x(), min.y(), min.z()),
        narrow(max.x(), max.y(), max.z()),
    );
}

impl RenderGraduatedTrihedron {
    /// `GlGraduatedTrihedron`: default parameters and bounds (0, 0, 0) to (100, 100, 100).
    /// Example: `RenderGraduatedTrihedron::new`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `DumpJson`: the class and the element base (the address);
    /// `trihedron.dump_json(&mut stream, -1)?`.
    pub fn dump_json(
        &self,
        stream: &mut martensite_tkernel::standard::dump::DumpStream,
        depth: i32,
    ) -> std::io::Result<()> {
        let address = crate::driver_dump::address_of(self);
        crate::driver_dump::element_begin(stream, depth, "GlGraduatedTrihedron", address)
    }
    /// `SetValues`: new parameters; the next render rebuilds the axes.
    /// Example: `trihedron.set_values(&Graphic3dGraduatedTrihedron::default())`.
    pub fn set_values(&self, data: &Graphic3dGraduatedTrihedron) {
        let mut state = self.state.write();
        state.data = data.clone();
        state.line_bounds = None;
    }
    /// `SetMinMax`: the bounds of the next render, kept until the cubic axes callback (if the
    /// parameters have one) replaces them. Example: `trihedron.set_min_max(min, max)`.
    pub fn set_min_max(&self, min: Vec3<f32>, max: Vec3<f32>) {
        let mut state = self.state.write();
        state.min = min;
        state.max = max;
    }
    /// `Release`: drops the axis lines and labels; the next render rebuilds them.
    /// Example: `trihedron.release()`.
    pub fn release(&self) {
        let mut state = self.state.write();
        state.line_bounds = None;
        state.labels = None;
    }
    /// `Render`: runs the cubic axes callback on `view`, then emits the grid, axes, tickmarks
    /// and labels for a viewport of `extent` pixels, in native order.
    /// Example: `let draws = trihedron.render(&*view, [800, 600])?;`.
    pub fn render(
        &self,
        view: &dyn Graphic3dCView,
        extent: [u32; 2],
    ) -> Result<Vec<crate::PrimitiveDraw>, RenderError> {
        let data = self.state.read().data.clone();
        // The callback re-enters the view (set_min_max), so no state lock is held across it.
        let refreshed = data.run_cubic_axes_callback(view);
        let mut state = self.state.write();
        if refreshed {
            refresh_line_bounds(&mut state);
        }
        if state.labels.is_none() {
            state.labels = Some(LabelGroups::new(&view.structure_manager())?);
        }
        frame_draws(view, extent, &state, &data)
    }
    /// The view layer of `render`: depth-tested with depth writes, after the scene.
    pub(crate) fn layer(
        &self,
        view: &dyn Graphic3dCView,
        extent: [u32; 2],
    ) -> Result<RenderLayer, RenderError> {
        let program = StencilProgram {
            draws: self.render(view, extent)?,
            boundaries: Vec::new(),
        };
        Ok(RenderLayer::from_stencil_program(
            program, true, true, false,
        ))
    }
}

/// `Render`: re-initialize the axis lines when missing or when the bounds moved by more than
/// `Precision::Confusion()`.
fn refresh_line_bounds(state: &mut TrihedronState) {
    let confusion = martensite_tkernel::precision::confusion() as f32;
    let moved = |(min, max): (Vec3<f32>, Vec3<f32>)| {
        (min - state.min).modulus() > confusion || (max - state.max).modulus() > confusion
    };
    if state.line_bounds.is_none_or(moved) {
        state.line_bounds = Some((state.min, state.max));
    }
}

fn frame_draws(
    view: &dyn Graphic3dCView,
    extent: [u32; 2],
    state: &TrihedronState,
    data: &Graphic3dGraduatedTrihedron,
) -> Result<Vec<crate::PrimitiveDraw>, RenderError> {
    let labels = state.labels.as_ref().expect("created before the frame");
    let camera = crate::view_render_camera::owned_render_camera(view);
    let rendering = view.rendering_params();
    let fields = labels
        .structure
        .cstructure()
        .cstructure_base()
        .fields
        .read()
        .clone();
    let primitives = labels.primitives.cast::<crate::Graphic3dDrawGroup>();
    let primitives = primitives
        .get()
        .ok_or_else(|| rejected(primitives.object_address(), "a Martensite trihedron group"))?;
    let mut frame = TrihedronFrame {
        primitives,
        projection: camera.projection_matrix_f(),
        world_view: camera.orientation_matrix_f(),
        camera,
        extent,
        rendering: &rendering,
        fields,
        data,
        min: state.min,
        max: state.max,
        lines: state.line_bounds.map(|(min, max)| max - min),
        draws: Vec::new(),
    };
    frame.render(&labels.groups)?;
    Ok(frame.draws)
}
