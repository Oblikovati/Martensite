//! The render context a user element draws with, as the element reaches it. A Vulkan frame
//! keeps no current GL state; this value carries what the context answers natively: the device
//! limits, the viewport, the resolution and render scale of the frame, the model-world,
//! world-view and projection matrix stacks of the element's structure, and the per-draw state
//! (`GlWorkspace::ApplyAspects`: face culling, polygon offset, alpha-to-coverage) the
//! element's draws get with the current aspects.
use crate::{RenderDeviceLimits, RenderMatrixState};
use martensite_tkernel::standard::{Handle, dump::DumpStream};
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::json_dump::{class_begin, field_dumped, field_integer, field_real};
use std::io;

/// The context state of one element call; `context.context_state().viewport()`.
///
/// `RenderContextState::default()` is the unbound native context: no device, the null-context
/// limits and an empty viewport.
#[derive(Clone, Debug, PartialEq)]
pub struct RenderContextState {
    pub(crate) limits: Option<RenderDeviceLimits>,
    pub(crate) viewport: [i32; 4],
    pub(crate) virtual_viewport: [i32; 4],
    pub(crate) polygon_offset: Graphic3dPolygonOffset,
    pub(crate) face_culling: Graphic3dTypeOfBackfacingModel,
    /// The view's backfacing model, which `ApplyAspects` reads before the aspects.
    pub(crate) view_backfacing: Graphic3dTypeOfBackfacingModel,
    pub(crate) allow_alpha_to_coverage: bool,
    pub(crate) alpha_to_coverage: bool,
    pub(crate) resolution: u32,
    pub(crate) resolution_ratio: f32,
    pub(crate) line_width_scale: f32,
    pub(crate) line_feather: f32,
    pub(crate) render_scale: f32,
    pub(crate) render_scale_inv: f32,
    pub(crate) model_world: RenderMatrixState<f32>,
    pub(crate) world_view: RenderMatrixState<f32>,
    pub(crate) projection: RenderMatrixState<f32>,
    /// The frame camera (the view camera, or the light camera of a shadow pass).
    pub(crate) camera: Handle<Graphic3dCamera>,
    pub(crate) window_size: [i32; 2],
    pub(crate) local_origin: [f64; 3],
    /// The view model after the PBR fallback (`GlView::render` sets it each frame).
    pub(crate) shading_model: Graphic3dTypeOfShadingModel,
    pub(crate) has_pbr: bool,
    /// The frame renders into sRGB attachments (every pipeline but the fixed-function one).
    pub(crate) srgb: bool,
    /// Preserve live FFP capability before texture extraction discards the supplied Q.
    pub(crate) fixed_function_lighting: bool,
}

impl Default for RenderContextState {
    fn default() -> Self {
        Self {
            limits: None,
            viewport: [0; 4],
            virtual_viewport: [0; 4],
            polygon_offset: Graphic3dPolygonOffset::default(),
            face_culling: Graphic3dTypeOfBackfacingModel::DoubleSided,
            view_backfacing: Graphic3dTypeOfBackfacingModel::Auto,
            allow_alpha_to_coverage: false,
            alpha_to_coverage: false,
            resolution: 72,
            resolution_ratio: 1.,
            line_width_scale: 1.,
            line_feather: 1.,
            render_scale: 1.,
            render_scale_inv: 1.,
            model_world: RenderMatrixState::default(),
            world_view: RenderMatrixState::default(),
            projection: RenderMatrixState::default(),
            camera: Handle::default(),
            window_size: [0; 2],
            local_origin: [0.; 3],
            // GlShaderManager starts with Gouraud until a view sets its model.
            shading_model: Graphic3dTypeOfShadingModel::Gouraud,
            has_pbr: false,
            srgb: false,
            fixed_function_lighting: false,
        }
    }
}

impl RenderContextState {
    /// The selected device limits; `None` outside a Vulkan device (`IsValid` false).
    pub fn limits(&self) -> Option<RenderDeviceLimits> {
        self.limits
    }
    /// Whether a device backs the context (`GlContext::IsValid`); `state.is_valid`.
    pub fn is_valid(&self) -> bool {
        self.limits.is_some()
    }
    /// Vulkan views render no quad-buffered stereo (`HasStereoBuffers`); always `false`.
    pub fn has_stereo_buffers(&self) -> bool {
        false
    }
    /// The scene framebuffer viewport `[x, y, width, height]` (`Viewport`).
    pub fn viewport(&self) -> [i32; 4] {
        self.viewport
    }
    /// The viewport divided by the render scale (`VirtualViewport`); `state.virtual_viewport()[2]`.
    pub fn virtual_viewport(&self) -> [i32; 4] {
        self.virtual_viewport
    }
    /// The polygon offset the element's surface draws get (`PolygonOffset`).
    pub fn polygon_offset(&self) -> Graphic3dPolygonOffset {
        self.polygon_offset
    }
    /// The resolved face culling of the element's draws (`FaceCulling`): never `Auto`.
    pub fn face_culling(&self) -> Graphic3dTypeOfBackfacingModel {
        self.face_culling
    }
    /// Whether alpha-to-coverage may be used this frame (`AllowSampleAlphaToCoverage`).
    pub fn allow_sample_alpha_to_coverage(&self) -> bool {
        self.allow_alpha_to_coverage
    }
    /// Whether the element's masked draws use alpha-to-coverage (`SampleAlphaToCoverage`).
    pub fn sample_alpha_to_coverage(&self) -> bool {
        self.alpha_to_coverage
    }
    /// Pixels per inch the frame renders text with, scaled by the render scale (`Resolution`).
    pub fn resolution(&self) -> u32 {
        self.resolution
    }
    /// Resolution over 72 times the render scale (`ResolutionRatio`); `state.resolution_ratio()`.
    pub fn resolution_ratio(&self) -> f32 {
        self.resolution_ratio
    }
    /// Line widths are multiplied by `max(1, round(ratio))` (`LineWidthScale`).
    pub fn line_width_scale(&self) -> f32 {
        self.line_width_scale
    }
    /// The mesh-edge feather width in pixels (`LineFeather`); `state.line_feather()`.
    pub fn line_feather(&self) -> f32 {
        self.line_feather
    }
    /// The rendering resolution scale of the frame (`RenderScale`), 1 without one.
    pub fn render_scale(&self) -> f32 {
        self.render_scale
    }
    /// `1 / render_scale()` (`RenderScaleInv`); `state.render_scale_inv()`.
    pub fn render_scale_inv(&self) -> f32 {
        self.render_scale_inv
    }
    /// Whether the frame renders at a scale other than 1 (`HasRenderScale`).
    pub fn has_render_scale(&self) -> bool {
        self.render_scale != 1.
    }
    /// The model-world stack of the element's structure (`ModelWorldState`).
    pub fn model_world_state(&self) -> &RenderMatrixState<f32> {
        &self.model_world
    }
    /// The world-view stack, after transform persistence (`WorldViewState`).
    pub fn world_view_state(&self) -> &RenderMatrixState<f32> {
        &self.world_view
    }
    /// The projection stack (`ProjectionState`); `state.projection_state().current()`.
    pub fn projection_state(&self) -> &RenderMatrixState<f32> {
        &self.projection
    }

    /// `GlContext::SetResolution`: the resolution, ratio and line-width scale for a frame
    /// rendered at `scale`.
    pub(crate) fn set_resolution(&mut self, resolution: u32, ratio: f32, scale: f32) {
        self.resolution = (scale * resolution as f32 + 0.5) as u32;
        self.render_scale = scale;
        self.render_scale_inv = 1. / scale;
        self.resolution_ratio = ratio * scale;
        self.line_width_scale = (self.resolution_ratio + 0.5).floor().max(1.);
    }

    /// `GlContext::ResizeViewport` of a scene framebuffer of `extent` pixels.
    pub(crate) fn resize_viewport(&mut self, extent: [u32; 2]) {
        self.viewport = [0, 0, extent[0] as i32, extent[1] as i32];
        self.virtual_viewport = match self.has_render_scale() {
            true => self
                .viewport
                .map(|side| (side as f32 * self.render_scale_inv) as i32),
            false => self.viewport,
        };
    }

    /// The face culling, polygon offset and alpha-to-coverage `ApplyAspects` and the primitive
    /// draw set for `aspects`.
    pub(crate) fn apply_aspects(
        &mut self,
        aspects: &Handle<dyn Graphic3dAspectsSource>,
        allow_face_culling: bool,
    ) {
        let Some(aspect) = aspects.get() else {
            return;
        };
        self.face_culling = applied_culling(self.view_backfacing, aspect, allow_face_culling);
        let offset = aspect.polygon_offset();
        let none = martensite_tkservice::aspect::AspectPolygonOffsetMode::NONE.0;
        if offset.mode.0 & none != none {
            self.polygon_offset = offset;
        }
        let masked = matches!(
            aspect.alpha_mode(),
            Graphic3dAlphaMode::Mask | Graphic3dAlphaMode::MaskBlend
        );
        self.alpha_to_coverage = self.allow_alpha_to_coverage && masked;
    }
}

/// `GlWorkspace::ApplyAspects` culling: the view's model, then the aspect's, then the
/// automatic rule for opaque surfaces of groups allowing culling.
fn applied_culling(
    view: Graphic3dTypeOfBackfacingModel,
    aspect: &dyn Graphic3dAspectsSource,
    allow_face_culling: bool,
) -> Graphic3dTypeOfBackfacingModel {
    use Graphic3dTypeOfBackfacingModel::*;
    let mode = match view {
        Auto => aspect.face_culling(),
        other => other,
    };
    if mode != Auto {
        return mode;
    }
    match allow_face_culling && crate::face_culling::allow_surface_auto_culling(aspect) {
        true => BackCulled,
        false => DoubleSided,
    }
}

/// `GlContext::DumpJson`. The keys whose meaning holds keep the native
/// names and order. The GL-only fields are not written: the texture clamp and point-sprite
/// origin enumerations, the GL version, GL_NORMALIZE, the render and polygon modes as GL
/// enumerations, the read buffer, the default VAO, the colour-mask pointer and the GL debug
/// context. The four ray-tracing flags are the selected device's (all 0 without a device).
impl RenderContextState {
    /// Write the context dump; `state.dump_json(&mut stream, -1)?`.
    pub fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        class_begin(stream, "GlContext")?;
        self.dump_limits(stream)?;
        field_integer(stream, "myIsInitialized", i32::from(self.is_valid()))?;
        field_integer(stream, "myIsStereoBuffers", 0)?;
        for (flag, value) in self.ray_tracing_flags() {
            field_integer(stream, flag, i32::from(value))?;
        }
        for value in self.viewport {
            field_integer(stream, "myViewport[i]", value)?;
        }
        for value in self.virtual_viewport {
            field_integer(stream, "myViewportVirt[i]", value)?;
        }
        self.dump_draw_state(stream, depth)?;
        self.dump_matrices(stream, depth)
    }
    /// `myHasRayTracing*`: what the selected device supports, as GlContext::init sets them.
    fn ray_tracing_flags(&self) -> [(&'static str, bool); 4] {
        let flag = |query: fn(&RenderDeviceLimits) -> bool| self.limits.as_ref().is_some_and(query);
        [
            ("myHasRayTracing", flag(RenderDeviceLimits::has_ray_tracing)),
            (
                "myHasRayTracingTextures",
                flag(RenderDeviceLimits::has_ray_tracing_textures),
            ),
            (
                "myHasRayTracingAdaptiveSampling",
                flag(RenderDeviceLimits::has_ray_tracing_adaptive_sampling),
            ),
            (
                "myHasRayTracingAdaptiveSamplingAtomic",
                flag(RenderDeviceLimits::has_ray_tracing_adaptive_sampling_atomic),
            ),
        ]
    }
    /// The native limit fields; without a device, the unbound context's initial values.
    fn dump_limits(&self, stream: &mut DumpStream) -> io::Result<()> {
        let values = match self.limits {
            Some(limits) => [
                limits.max_degree_of_anisotropy(),
                limits.max_texture_size(),
                limits.max_combined_texture_units(),
                limits.max_dump_size_x(),
                limits.max_dump_size_y(),
                limits.max_clip_planes(),
                limits.max_msaa_samples(),
                limits.max_draw_buffers(),
                limits.max_color_attachments(),
            ],
            None => [1, 1024, 1, 1024, 1024, 6, 0, 1, 1],
        };
        let keys = [
            "myAnisoMax",
            "myMaxTexDim",
            "myMaxTexCombined",
            "myMaxDumpSizeX",
            "myMaxDumpSizeY",
            "myMaxClipPlanes",
            "myMaxMsaaSamples",
            "myMaxDrawBuffers",
            "myMaxColorAttachments",
        ];
        for (key, value) in keys.into_iter().zip(values) {
            field_integer(stream, key, value)?;
        }
        Ok(())
    }
    fn dump_draw_state(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        let offset = self.polygon_offset;
        field_dumped(stream, depth, "&myPolygonOffset", &|o, d| {
            offset.dump_json(o, d)
        })?;
        field_integer(stream, "myFaceCulling", self.face_culling as i32)?;
        let allow = i32::from(self.allow_alpha_to_coverage);
        field_integer(stream, "myAllowAlphaToCov", allow)?;
        field_integer(
            stream,
            "myAlphaToCoverage",
            i32::from(self.alpha_to_coverage),
        )?;
        field_integer(stream, "myResolution", self.resolution as i32)?;
        field_real(
            stream,
            "myResolutionRatio",
            f64::from(self.resolution_ratio),
        )?;
        field_real(stream, "myLineWidthScale", f64::from(self.line_width_scale))?;
        field_real(stream, "myLineFeather", f64::from(self.line_feather))?;
        field_real(stream, "myRenderScale", f64::from(self.render_scale))?;
        field_real(stream, "myRenderScaleInv", f64::from(self.render_scale_inv))
    }
    fn dump_matrices(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        for (key, state) in [
            ("&ModelWorldState", &self.model_world),
            ("&WorldViewState", &self.world_view),
            ("&ProjectionState", &self.projection),
        ] {
            field_dumped(stream, depth, key, &|o, d| state.dump_json(o, d))?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "render_context_state_tests.rs"]
mod tests;
