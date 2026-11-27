//! Typed backend traits and opaque memo handles separating Graphic3d lowering from GPU APIs (#633).
use crate::driver_options::WindowFrameQueue;
use crate::frame_work_ledger::FrameWorkLedger;
use crate::view_diagnostics::RenderDiagnosticFacts;
use crate::{
    FrameCapture, PrimitiveDraw, PrimitiveTopology, RenderAssemblyStreamingProgress,
    RenderBehaviorConfiguration, RenderCoordinateConvention, RenderDeviceLimits,
    RenderDeviceMemoryReport, RenderDriverIdentity, RenderError, RenderFrame,
    RenderGuiSurfacePolicy, RenderImageMemory, RenderLighting, RenderLineClippingBehavior,
    RenderMemoryQueryError, RenderRasterPolicy, RenderShading, RenderUserProgramLibrary,
    RenderValidationObservationFailure,
};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectWindow;
use martensite_tkservice::graphic3d::Graphic3dTypeOfLightSource;
use std::any::Any;
use std::fmt;
use std::sync::{Arc, Mutex, PoisonError};

/// The attachments copied by a deferred readback; colour dumps skip the depth plane (#594).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapturePlanes {
    /// Colour and depth planes.
    ColorAndDepth,
    /// Colour plane only; depth stays empty.
    Color,
}

/// Minimum vertices to amortize a specialized directional-Phong resident batch (#594).
pub const MIN_PHONG_BATCH_VERTICES: usize = (4 << 20) / 44;

/// Whether `draw` is a plain opaque triangle surface compatible with directional-Phong batching.
pub fn directional_phong_draw(draw: &PrimitiveDraw) -> bool {
    ordinary_surface(draw) && draw.lighting.as_ref().is_some_and(directional_material)
}

/// Whether `draw` is an ordinary triangle surface without clipping, textures, edges, or capping.
pub fn ordinary_surface(draw: &PrimitiveDraw) -> bool {
    draw.topology() == PrimitiveTopology::Triangles
        && !draw.annotation
        && draw.clipping.is_none()
        && draw.hatch_rows.is_none()
        && draw.surface_texture.is_none()
        && draw.surface_texture_unavailable.is_none()
        && draw.surface_cutoff.is_none()
        && draw.unlit_back_color.is_none()
        && !draw.surface_blend
        && !draw.surface_forced_blend
        && draw.mesh_edges.is_none()
        && draw.polygon_edge_style.is_none()
        && !draw.polygon_points
        && draw.textured_style.is_none()
        && draw.user_program.is_none()
        && draw.capping_pass == crate::RenderCappingPass::None
        && draw.depth_pass == Default::default()
        && !draw.fixed_function_lighting
}

fn directional_material(state: &RenderLighting) -> bool {
    state.shading == RenderShading::Phong
        && state.fixed.is_none()
        && !state.pbr
        && !state.vertex_colors
        && state.eye[3] == 0.
        && directional_lights(&state.lights)
}

/// Whether every light in `lights` is a shadowless directional light.
pub fn directional_lights(lights: &crate::RenderLightSet) -> bool {
    lights.sources.iter().all(|source| {
        source.kind == Graphic3dTypeOfLightSource::Directional as u32 && !source.cast_shadows
    })
}

/// Opaque per-`DrawBody` backend cache slot (pipeline or geometry upload memo, #594).
///
/// Cloning starts empty, and two memos always compare equal so caching never alters draw equality.
#[derive(Default)]
pub struct OpaqueMemo(Mutex<Option<Box<dyn Any + Send + Sync>>>);

impl OpaqueMemo {
    /// Inspect the cached value with `read` under the memo lock.
    pub fn with_ref<T: Any + Send + Sync, R>(&self, read: impl FnOnce(&T) -> R) -> Option<R> {
        let slot = self.0.lock().ok()?;
        let value = slot.as_deref()?.downcast_ref::<T>()?;
        Some(read(value))
    }

    /// Store a backend-owned memo value.
    pub fn store<T: Any + Send + Sync>(&self, value: T) {
        if let Ok(mut slot) = self.0.lock() {
            *slot = Some(Box::new(value));
        }
    }

    /// Return whether a cached value is currently held.
    pub fn is_kept(&self) -> bool {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .is_some()
    }

    /// Clear any cached entry because the owning `DrawBody` is being mutated in place.
    pub fn clear(&mut self) {
        if let Ok(slot) = self.0.get_mut() {
            *slot = None;
        }
    }
}

impl Clone for OpaqueMemo {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl PartialEq for OpaqueMemo {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl fmt::Debug for OpaqueMemo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("OpaqueMemo")
    }
}

/// Opaque resident geometry batch prepared by a GPU backend.
pub trait RenderGeometryBatch: Any + fmt::Debug + Send + Sync {
    /// Downcast handle for the owning backend.
    fn as_any(&self) -> &dyn Any;
    /// Downcast shared ownership handle for the owning backend.
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

/// Opaque HiZ occlusion history owned by a placement batch.
pub trait RenderHiZHistory: Any + fmt::Debug + Send + Sync {
    /// Downcast handle for the owning backend.
    fn as_any(&self) -> &dyn Any;
    /// Downcast shared ownership handle for the owning backend.
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

/// Opaque deferred device-resident frame ticket returned by a GPU backend redraw.
pub trait RenderDeferredFrame: Any + Send + Sync {
    /// Logical width and height of the rendered frame.
    fn logical_size(&self) -> [u32; 2];
    /// Optional assembly-streaming progress reported by the redraw.
    fn assembly_streaming(&self) -> Option<RenderAssemblyStreamingProgress> {
        None
    }
    /// Downcast handle for the owning backend.
    fn as_any(&self) -> &dyn Any;
}

/// Opaque window swapchain/surface presenter owned by a view.
pub trait RenderWindowPresenter: Any + Send + Sync {
    /// Observe the authoritative backing extent of the native/browser surface.
    fn observed_extent(&self) -> Result<[u32; 2], RenderError>;
    /// Cancel any pending progressive assembly-streaming work bound to this presenter.
    fn cancel_assembly_streaming(&mut self) {}
    /// Downcast handle for the owning backend.
    fn as_any(&self) -> &dyn Any;
    /// Mutable downcast handle for the owning backend.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Result of one redraw: host pixels or a deferred device-resident ticket (#594).
pub enum RenderedFrame {
    /// Pixels already resident on the host.
    Ready(FrameCapture),
    /// Device-resident images read back on demand.
    Deferred(Box<dyn RenderDeferredFrame>),
}

/// Typed backend contract implemented by `VulkanRenderer`, `WebGpuRenderer`, and custom backends.
pub trait RenderDeviceBackend: Any + Send + Sync {
    /// Maximum capture dimension supported by this device.
    fn capture_limit(&self) -> u32;

    /// Register any backend-provided built-in user shader samples into `library`.
    fn register_builtin_user_programs(
        &self,
        _library: &RenderUserProgramLibrary,
    ) -> Result<(), RenderError> {
        Ok(())
    }

    /// Prepare a resident merged geometry batch when supported by the backend.
    fn prepare_batch(
        &self,
        _draws: Vec<PrimitiveDraw>,
    ) -> Result<Option<Arc<dyn RenderGeometryBatch>>, RenderError> {
        Ok(None)
    }

    /// Allocate a fresh HiZ history container for a placement batch when supported.
    fn create_hiz_history(&self) -> Option<Arc<dyn RenderHiZHistory>> {
        None
    }

    /// Whether the backend supports hierarchical-Z occlusion culling.
    fn supports_hiz(&self) -> bool {
        false
    }

    /// Whether the backend supports GPU placement visibility compaction.
    fn supports_placement_compaction(&self) -> bool {
        false
    }

    /// Whether the backend can draw `templates` across `placement_count` placements.
    fn supports_placements(
        &self,
        _templates: &[PrimitiveDraw],
        _placement_count: usize,
    ) -> Result<bool, RenderError> {
        Ok(false)
    }

    /// Whether the backend supports specialized directional-Phong fused batches.
    fn supports_phong_batching(&self) -> bool {
        false
    }

    /// Whether the backend supports mesh LOD selection.
    fn supports_mesh_lod(&self) -> bool {
        true
    }

    /// Whether the backend can retain and restore main-layer images for immediate redraws.
    fn allows_main_copy(&self) -> bool {
        false
    }

    /// Query device capacities.
    fn device_limits(&self) -> Option<RenderDeviceLimits> {
        None
    }

    /// Query diagnostic dictionary facts.
    fn diagnostic_facts(&self) -> Option<RenderDiagnosticFacts> {
        None
    }

    /// Query GPU sampled image memory counters.
    fn image_memory(&self) -> RenderImageMemory {
        RenderImageMemory::default()
    }

    /// Shared device work ledger for frame profiling.
    fn work_ledger(&self) -> Option<Arc<FrameWorkLedger>> {
        None
    }

    /// Render a complete frame and return host pixels synchronously.
    fn render_frame(&mut self, frame: &RenderFrame) -> Result<FrameCapture, RenderError>;

    /// Render a view frame, optionally keeping images on the device until capture.
    fn render_deferred(&mut self, frame: &RenderFrame) -> Result<RenderedFrame, RenderError> {
        self.render_frame(frame).map(RenderedFrame::Ready)
    }

    /// Wait for any in-flight asynchronous GPU frames to retire.
    fn finish_frames(&mut self) -> Result<(), RenderError> {
        Ok(())
    }

    /// Stream RGB rows of a deferred frame through `write_row`.
    fn read_frame_rgb(
        &mut self,
        _frame: &dyn RenderDeferredFrame,
        _flip: bool,
        _write_row: &mut dyn FnMut(usize, &[u8]) -> bool,
    ) -> Result<bool, RenderError> {
        Err(crate::error::rejected(
            self.device_name(),
            "a backend supporting deferred RGB readback",
        ))
    }

    /// Copy the chosen planes of a deferred frame to host memory.
    fn read_frame(
        &mut self,
        _frame: &dyn RenderDeferredFrame,
        _planes: CapturePlanes,
    ) -> Result<FrameCapture, RenderError> {
        Err(crate::error::rejected(
            self.device_name(),
            "a backend supporting synchronous deferred frame readback",
        ))
    }

    /// Optional testing-only raw native instance handle (e.g., `VkInstance`).
    fn raw_instance(&self) -> Option<u64> {
        None
    }

    /// Return collected API validation errors or an explicit unavailable error.
    fn validation_errors(&self) -> Result<Vec<String>, RenderValidationObservationFailure> {
        Err(RenderValidationObservationFailure::unavailable())
    }

    /// Return selected GPU driver provenance.
    fn driver_identity(&self) -> Option<RenderDriverIdentity> {
        None
    }

    /// Query GPU memory heaps and optional budget estimates.
    fn memory_report(&self) -> Result<RenderDeviceMemoryReport, RenderMemoryQueryError> {
        Ok(RenderDeviceMemoryReport::unavailable())
    }

    /// Return the effective raster policy admitted by the device.
    fn raster_policy(&self) -> Option<RenderRasterPolicy> {
        None
    }

    /// Return the effective attachment coordinate convention admitted by the device.
    fn coordinate_convention(&self) -> Option<RenderCoordinateConvention> {
        None
    }

    /// Human-readable device name.
    fn device_name(&self) -> &str;

    /// Create a window presenter bound to `window`.
    fn window_presenter(
        &self,
        _window: &Handle<dyn AspectWindow>,
    ) -> Result<Box<dyn RenderWindowPresenter>, RenderError> {
        Err(crate::error::rejected(
            self.device_name(),
            "a backend supporting window presentation",
        ))
    }

    /// Render and present a frame to `target`.
    fn render_window(
        &mut self,
        _frame: &RenderFrame,
        _target: &mut dyn RenderWindowPresenter,
        _frame_queue: WindowFrameQueue,
        _policy: RenderGuiSurfacePolicy,
    ) -> Result<RenderedFrame, RenderError> {
        Err(crate::error::rejected(
            self.device_name(),
            "a backend supporting window presentation",
        ))
    }

    /// Downcast handle for backend-specific extensions (e.g., async WebGPU readback).
    fn as_any(&self) -> &dyn Any;

    /// Mutable downcast handle for backend-specific extensions.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Options that can synchronously construct a [`RenderDeviceBackend`] for [`crate::Graphic3dRenderDriver`].
pub trait RenderDeviceOptions: Sized {
    /// Construct a device backend with the requested policies and versioned behavior.
    fn create_device(
        self,
        raster: RenderRasterPolicy,
        coordinates: RenderCoordinateConvention,
        clipping: RenderLineClippingBehavior,
        configuration: RenderBehaviorConfiguration,
    ) -> Result<Box<dyn RenderDeviceBackend>, RenderError>;

    /// Construct a device backend with vertex-output diagnostic observation enabled.
    #[cfg(feature = "vertex-output-diagnostics")]
    fn create_device_with_vertex_output(
        self,
        _raster: RenderRasterPolicy,
        _coordinates: RenderCoordinateConvention,
        _clipping: RenderLineClippingBehavior,
        _configuration: RenderBehaviorConfiguration,
        _request: crate::VertexOutputRequest,
    ) -> Result<Box<dyn RenderDeviceBackend>, RenderError> {
        Err(crate::error::rejected(
            "vertex output diagnostics",
            "a backend implementing vertex output observation",
        ))
    }
}
