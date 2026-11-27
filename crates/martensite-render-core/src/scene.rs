//! Immutable submissions separate geometry validation from device operations.
#[cfg(feature = "command-state-diagnostics")]
#[path = "replay_capture_diagnostic.rs"]
pub mod replay_diagnostic;
use crate::error::{RenderError, rejected};
use crate::{PrimitiveDraw, RenderLayer, RenderLayerSpan, render_layers::flatten_layers_at};
/// Position and linear RGBA color; `RenderVertex::new([0.,0.,0.],[1.,0.,0.,1.])`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderVertex {
    /// Object-space coordinates.
    pub position: [f32; 3],
    /// Homogeneous weight; zero represents a direction at infinity.
    pub weight: f32,
    /// Linear, unpremultiplied RGBA.
    pub color: [f32; 4],
}
impl RenderVertex {
    /// Construct a vertex; validation occurs at submission, e.g. `RenderVertex::new([0.;3],[1.;4])`.
    pub fn new(position: [f32; 3], color: [f32; 4]) -> Self {
        Self {
            position,
            weight: 1.,
            color,
        }
    }
    /// Preserve a full object-space position, e.g. `RenderVertex::homogeneous([1.,0.,0.,0.],[1.;4])`.
    pub fn homogeneous(position: [f32; 4], color: [f32; 4]) -> Self {
        let [x, y, z, weight] = position;
        Self {
            position: [x, y, z],
            weight,
            color,
        }
    }
}
/// OpenGL clip identity; `let transform = identity_clip()`.
pub fn identity_clip() -> [f32; 16] {
    [
        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
    ]
}
/// Immutable frame input; `RenderFrame::new(64,64,[0.,0.,0.,1.],draws)?`.
#[derive(Clone, Debug)]
pub struct RenderFrame {
    /// A camera-dependent presentation mesh was authored, even if this extent uses the primary.
    pub uses_mesh_lod: bool,
    pub assembly_upload_budget: Option<u64>,
    pub geometry_batch: Option<std::sync::Arc<dyn crate::device_backend::RenderGeometryBatch>>,
    #[cfg(feature = "raytrace-input-diagnostics")]
    pub raytrace_observation:
        Option<std::sync::Arc<crate::raytrace_input_observation::RayInputObservation>>,
    pub width: u32,
    pub height: u32,
    pub clear_color: [f32; 4],
    pub draws: Vec<PrimitiveDraw>,
    pub layers: Vec<RenderLayerSpan>,
    pub gui_draw_start: Option<usize>,
    pub raster: crate::transparency::RenderRasterSettings,
    pub raster_feedback: crate::raster_feedback::RasterFailureFeedback,
    pub stencil_clear_value: u8,
    pub stencil_context: Option<crate::stencil_feedback::StencilSnapshot>,
    pub fragment_alpha_context: Option<crate::fragment_alpha_feedback::FragmentAlphaSnapshot>,
    /// The view's shader grid, drawn before the span at its index (end of the main pass).
    pub shader_grid: Option<ShaderGridPass>,
    /// Present the frame upside down (GlView::ToFlipOutput reaching its target).
    pub flip_output: bool,
    /// The view's cubemap background and PBR environment (GlView::Render).
    pub environment: Option<crate::environment_frame::FrameEnvironment>,
    /// The view's corner gradient background (getColoredQuadProgram), drawn first.
    pub colored_quad: Option<crate::colored_quad_background::ColoredQuadBackground>,
    /// The view's rendered shadow maps (GlView::renderShadowMap), sampled by lit fragments.
    pub shadow_maps: Option<std::sync::Arc<crate::shadow_maps::RenderShadowMaps>>,
    /// The stereo pair composed over the frame (GlView::drawStereoPair), drawn first.
    pub stereo_pair: Option<std::sync::Arc<crate::stereo_pair::StereoPairComposite>>,
    /// The ray-tracing pass (GlView::raytrace) after the ray-traced layers' raster passes.
    pub raytrace: Option<std::sync::Arc<crate::raytrace::RaytraceFrame>>,
    /// The active subviews' colour copies (GlView::blitSubviews), applied after the scene
    /// and before the flipped output: on the GPU for a presented frame, on the host capture.
    pub subview_blits: Vec<crate::subview_composition::RenderSubviewBlit>,
    pub immediate_start: Option<usize>,
    pub immediate_main: Option<crate::immediate_main::ImmediateMainPlan>,
}
/// One frame's shader grid: its position among the layer spans and its shader block.
#[derive(Clone, Debug, PartialEq)]
pub struct ShaderGridPass {
    pub span: usize,
    pub uniforms: crate::shader_grid::RenderShaderGridBlock,
}
impl RenderFrame {
    /// Set an explicit low-level attachment clear; `frame.with_stencil_clear_value(1)`.
    /// Retained-view test adaptors use this to prove comparisons against independently seeded buffers.
    pub fn with_stencil_clear_value(mut self, value: u8) -> Self {
        self.stencil_clear_value = value;
        self
    }
    /// Read target dimensions; `frame.extent()`.
    pub fn extent(&self) -> [u32; 2] {
        [self.width, self.height]
    }
    /// Read the linear clear color; `frame.clear_color()`.
    pub fn clear_color(&self) -> [f32; 4] {
        self.clear_color
    }
    /// Read validated draws for an injected backend; `frame.draws()`.
    pub fn draws(&self) -> &[PrimitiveDraw] {
        &self.draws
    }
    /// Validate extent and clear color; `RenderFrame::new(64,64,[0.;4],vec![])?`.
    pub fn new(
        width: u32,
        height: u32,
        clear_color: [f32; 4],
        draws: Vec<PrimitiveDraw>,
    ) -> Result<Self, RenderError> {
        Self::with_layers(
            width,
            height,
            clear_color,
            vec![RenderLayer::new(draws, true, true, true)],
        )
    }
    /// Read ordered depth policies; `frame.layers()`.
    pub fn layers(&self) -> &[RenderLayerSpan] {
        &self.layers
    }
    /// Build an ordered frame; an empty layer list still clears its attachments.
    /// Example: `RenderFrame::with_layers(64, 64, [0.;4], layers)?`.
    pub fn with_layers(
        width: u32,
        height: u32,
        clear_color: [f32; 4],
        layers: Vec<RenderLayer>,
    ) -> Result<Self, RenderError> {
        Self::with_layers_and_grid(
            width,
            height,
            clear_color,
            layers,
            None,
            crate::mesh_lod::FrameMeshLod::Direct,
        )
    }
    /// Build a frame whose first `main` layers form the main pass, followed by the shader
    /// grid (GlView::render draws it after the main layers, before immediate ones).
    /// Example: `RenderFrame::with_layers_and_grid(64, 64, clear, layers, grid, lod)?`.
    pub fn with_layers_and_grid(
        width: u32,
        height: u32,
        clear_color: [f32; 4],
        mut layers: Vec<RenderLayer>,
        grid: Option<(usize, crate::shader_grid::RenderShaderGridBlock)>,
        mesh_lod: crate::mesh_lod::FrameMeshLod,
    ) -> Result<Self, RenderError> {
        if layers.is_empty() {
            layers.push(RenderLayer::new(vec![], true, true, true));
        }
        for layer in &layers {
            crate::capping_pass::validate_capping_sequence(&layer.draws)?;
        }
        let boundary = grid.map(|(main, _)| main);
        let (mut draws, layers, grid_span) = flatten_layers_at(layers, boundary);
        let shader_grid = grid.map(|(_, uniforms)| ShaderGridPass {
            span: grid_span,
            uniforms,
        });
        let uses_mesh_lod = crate::mesh_lod::select_levels(&mut draws, [width, height], mesh_lod);
        crate::texture_coordinate_frame::resolve_ordered_texture_coordinates(&mut draws)?;
        crate::retained_mesh_edges::resolve_edge_background(&mut draws, clear_color);
        crate::retained_hollow::resolve_ordered_line_widths(&mut draws);
        validate_extent(width, height)?;
        crate::line_guard_band::clip_far_lines(&mut draws, [width, height]);
        crate::projected_segments::validate_prepared_draws(&draws, [width, height])?;
        if !clear_color.iter().all(|v| v.is_finite()) {
            return Err(rejected(clear_color, "finite clear color"));
        }
        Ok(Self {
            uses_mesh_lod,
            assembly_upload_budget: None,
            geometry_batch: None,
            width,
            height,
            clear_color,
            draws,
            layers,
            gui_draw_start: None,
            raster: Default::default(),
            raster_feedback: Default::default(),
            fragment_alpha_context: None,
            flip_output: false,
            environment: None,
            colored_quad: None,
            stereo_pair: None,
            raytrace: None,
            #[cfg(feature = "raytrace-input-diagnostics")]
            raytrace_observation: None,
            shadow_maps: None,
            subview_blits: Vec::new(),
            immediate_start: None,
            immediate_main: None,
            stencil_context: None,
            stencil_clear_value: 0,
            shader_grid,
        })
    }
}
/// Host capture with top-left origin; `capture.pixel(0,0)`.
#[derive(Clone, Debug)]
pub struct FrameCapture {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// Tightly packed sRGB color and linear alpha bytes, row-major from top left.
    pub rgba: Vec<u8>,
    /// Vulkan depth values in `[0,1]`, same pixel order as color.
    pub depth: Vec<f32>,
}
impl FrameCapture {
    /// Reverse the row order of color and depth together; `capture.reverse_rows()`.
    pub fn reverse_rows(&mut self) {
        let width = self.width as usize;
        reverse_chunks(&mut self.rgba, width * 4);
        reverse_chunks(&mut self.depth, width);
    }
    /// Read a checked pixel; `capture.pixel(10,20)`.
    pub fn pixel(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let offset = (y as usize * self.width as usize + x as usize) * 4;
        self.rgba.get(offset..offset + 4)?.try_into().ok()
    }
}
fn reverse_chunks<T: Copy>(values: &mut [T], row: usize) {
    if row == 0 {
        return;
    }
    let reversed: Vec<T> = values.rchunks_exact(row).flatten().copied().collect();
    values[..reversed.len()].copy_from_slice(&reversed);
}
/// Device seam: tests inject a named recording backend; `backend.render(&frame)?`.
pub trait RenderBackend {
    /// Execute a complete immutable frame and return its capture; `backend.render(&frame)?`.
    fn render(&mut self, frame: &RenderFrame) -> Result<FrameCapture, RenderError>;
}

pub fn validate_components(
    vertices: &[RenderVertex],
    clip_transform: &[f32; 16],
) -> Result<(), RenderError> {
    if !clip_transform.iter().all(|v| v.is_finite())
        || vertices.iter().any(|v| {
            !v.position
                .iter()
                .chain(std::iter::once(&v.weight))
                .chain(v.color.iter())
                .all(|c| c.is_finite())
        })
    {
        return Err(rejected(
            (vertices, clip_transform),
            "finite vertex components and clip transform",
        ));
    }
    if let Some(vertex) = vertices.iter().find(|v| !(0.0..=1.0).contains(&v.color[3])) {
        return Err(rejected(vertex.color, "vertex alpha in 0..=1"));
    }
    Ok(())
}

fn validate_extent(width: u32, height: u32) -> Result<(), RenderError> {
    if width == 0
        || height == 0
        || width
            .checked_mul(height)
            .and_then(|v| v.checked_mul(4))
            .is_none()
    {
        return Err(rejected(
            [width, height],
            "nonzero extent fitting a 32-bit RGBA byte count",
        ));
    }
    Ok(())
}

pub fn validate_annotation_opacity(vertices: &[RenderVertex]) -> Result<(), RenderError> {
    if let Some(vertex) = vertices.iter().find(|v| v.color[3] != 1.) {
        return Err(rejected(
            vertex.color,
            "opaque point, text and highlight colors",
        ));
    }
    Ok(())
}
