//! A real Graphic3d view owns capture results while TKService owns common scene bookkeeping.
use crate::{
    FrameCapture, RenderError,
    driver_backend::SharedRenderDevice,
    error::rejected,
    view_capture::{dump_capture, report_capture_failure},
    view_forward::retained_view_methods,
    view_scene::frame_for_view,
    view_target::{RenderFrameBuffer, capture_extent},
};
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::{aspect::*, graphic3d::*, image::ImagePixMap};
use std::sync::Arc;
/// Renderer-backed Graphic3d view; `view.cast::<Graphic3dRenderView>().capture()?`.
pub struct Graphic3dRenderView {
    transient: StandardTransient,
    pub(crate) retained: crate::view_statistics::RenderRetainedView,
    pub(crate) this: SharedState<martensite_tkernel::standard::WeakHandle<Self>>,
    pub(crate) device: SharedRenderDevice,
    pub(crate) limit: u32,
    gui_surface: crate::RenderGuiSurfacePolicy,
    line_widths: crate::RenderLineWidthPolicy,
    pub(crate) options: crate::driver_options::SharedDriverOptions,
    pub(crate) lighting_selection: crate::driver_lighting_capability::SharedLightingSelection,
    presenter: SharedState<Option<crate::backend::WindowPresenter>>,
    window_error: SharedState<Option<RenderError>>,
    pub(crate) outcome: SharedState<Option<Result<FrameCapture, RenderError>>>,
    pub(crate) gui_layers: SharedState<crate::view_gui_layer::GuiLayerSlot>,
    raster_feedback: crate::raster_feedback::RasterFailureFeedback,
    stencil_feedback: crate::stencil_feedback::StencilFeedback,
    fragment_alpha_feedback: crate::fragment_alpha_feedback::FragmentAlphaFeedback,
    pub(crate) frame_hooks: SharedState<
        Option<std::sync::Arc<dyn crate::render_view_frame_hooks::RenderViewFrameHooks>>,
    >,
    /// Path-tracing accumulation (GlView's myAccumFrames and myRNG).
    pub(crate) raytrace_accumulation: SharedState<crate::raytrace::RaytraceAccumulation>,
    /// The bottom-level ray-tracing trees of the last frame (`raytrace::RaytraceBvhCache`).
    pub(crate) raytrace_bvh: SharedState<crate::raytrace::RaytraceBvhCache>,
    /// The views and structures of the driver (GlGraphicDriver::setDeviceLost).
    pub(crate) driver_objects: Arc<crate::driver_device_lost::DriverObjects>,
    /// `myWasRedrawnGL`: a Redraw or RedrawImmediate completed.
    was_redrawn: std::sync::atomic::AtomicBool,
    redraw_serial: std::sync::atomic::AtomicU64,
    #[cfg(feature = "raytrace-input-diagnostics")]
    raytrace_observation: SharedState<crate::raytrace_input_observation::ViewRayInputs>,
    redraw_target: SharedState<Option<redraw_completion::RedrawPublicationTarget>>,
}
martensite_tkernel::impl_transient!(Graphic3dRenderView, "Martensite_RenderView", StandardTransient, transient, interfaces:[dyn Graphic3dCView, dyn Graphic3dDataStructureManager]);
type RenderViewCreationInputs = (
    StandardTransient,
    crate::view_output_modes::OutputModeRetainedView,
    SharedRenderDevice,
    u32,
    (
        crate::driver_options::SharedDriverOptions,
        Arc<crate::driver_device_lost::DriverObjects>,
        crate::RenderLineWidthPolicy,
        crate::driver_lighting_capability::SharedLightingSelection,
    ),
    crate::RenderGuiSurfacePolicy,
);
impl Graphic3dRenderView {
    pub(crate) fn create(
        manager: &Handle<Graphic3dStructureManager>,
        layers: &[Handle<Graphic3dLayer>],
        device: SharedRenderDevice,
        limit: u32,
        options: (
            crate::driver_options::SharedDriverOptions,
            Arc<crate::driver_device_lost::DriverObjects>,
            crate::RenderLineWidthPolicy,
            crate::driver_lighting_capability::SharedLightingSelection,
        ),
        gui_surface: crate::RenderGuiSurfacePolicy,
    ) -> Result<Handle<Self>, Failure> {
        let inputs = (
            StandardTransient::default(),
            crate::view_output_modes::OutputModeRetainedView::new(Graphic3dRetainedView::new(
                manager, layers,
            )?),
            device,
            limit,
            options,
            gui_surface,
        );
        let view = Handle::new(Self::initialized_gui_view(inputs));
        *view.this.write() = view.downgrade();
        graphic3d_cview_register(&view.cast())?;
        Ok(view)
    }
    fn initialized_gui_view(inputs: RenderViewCreationInputs) -> Self {
        let (
            transient,
            retained,
            device,
            limit,
            (options, driver_objects, line_widths, lighting_selection),
            gui_surface,
        ) = inputs;
        Self {
            transient,
            retained: crate::view_statistics::RenderRetainedView::new(retained),
            this: SharedState::default(),
            device,
            limit,
            options,
            lighting_selection,
            gui_surface,
            line_widths,
            presenter: SharedState::default(),
            window_error: SharedState::default(),
            outcome: SharedState::default(),
            gui_layers: SharedState::default(),
            raster_feedback: Default::default(),
            stencil_feedback: Default::default(),
            fragment_alpha_feedback: Default::default(),
            frame_hooks: SharedState::default(),
            raytrace_accumulation: SharedState::default(),
            raytrace_bvh: SharedState::default(),
            driver_objects,
            was_redrawn: Default::default(),
            redraw_serial: Default::default(),
            #[cfg(feature = "raytrace-input-diagnostics")]
            raytrace_observation: SharedState::default(),
            redraw_target: Default::default(),
        }
    }
    /// `myWasRedrawnGL`: whether a Redraw or RedrawImmediate of this view completed.
    pub(crate) fn was_redrawn(&self) -> bool {
        self.was_redrawn.load(std::sync::atomic::Ordering::Relaxed)
    }
    fn finish_redraw(&self, outcome: Result<(), RenderError>) {
        if outcome.is_ok() {
            self.was_redrawn
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
    }
    /// Inspect this view's completed stencil context; `render_view.stencil_context_state()`.
    pub fn stencil_context_state(&self) -> crate::RenderStencilState {
        self.stencil_feedback.current()
    }
    fn render_current(&self) -> Result<FrameCapture, RenderError> {
        #[cfg(feature = "raytrace-input-diagnostics")]
        let observation = self.begin_raytrace_observation()?;
        self.with_frame_stats(|| {
            let (mut frame, scaled) = self.prepare_current_frame()?;
            frame.flip_output = self.retained.flips_output(&self.rendering_params());
            // redrawImmediate blits the subviews before the swap and the flipped output blit.
            frame.subview_blits = self.subview_blits(frame.extent());
            #[cfg(feature = "raytrace-input-diagnostics")]
            self.attach_raytrace_observation(&mut frame, observation.as_ref(), scaled.is_some());
            let mut capture = self.render_target(&frame)?;
            #[cfg(feature = "command-state-diagnostics")]
            let observed_transforms = [
                usize::from(scaled.is_some()),
                frame.subview_blits.len(),
                usize::from(frame.flip_output),
                usize::from(frame.gui_draw_start.is_some()),
            ];
            if let Some(scaled) = scaled {
                capture.depth =
                    crate::render_scale::blit_depth(&scaled, [frame.width, frame.height]);
            }
            crate::subview_composition::apply_subview_blits(&mut capture, &frame.subview_blits);
            if frame.flip_output {
                capture.reverse_rows();
            }
            #[cfg(feature = "raytrace-input-diagnostics")]
            self.finish_raytrace_observation(observation.as_ref(), &capture);
            #[cfg(feature = "command-state-diagnostics")]
            crate::scene::replay_diagnostic::rendered(
                self as *const Self as usize,
                &capture,
                observed_transforms,
            );
            Ok(capture)
        })
    }
    /// The frame to render on the target, and the scaled scene capture it blits, if any.
    fn prepare_current_frame(
        &self,
    ) -> Result<(crate::RenderFrame, Option<FrameCapture>), RenderError> {
        self.require_native_target(AspectNativeWindowOperation::Redraw)?;
        if !self.is_defined() || self.is_removed() {
            return Err(rejected("undefined or removed view", "a defined live view"));
        }
        if let Some(error) = self.window_error.read().as_ref() {
            return Err(error.clone());
        }
        let extent = capture_extent(self, self.limit)?;
        let packet = if self.checked_gui_policy() {
            self.checked_gui_snapshot(extent)?
        } else {
            self.gui_layers.read().snapshot(extent)?
        };
        let (mut frame, scaled) = self.composed_scene(extent)?;
        if let Some(packet) = packet {
            crate::view_gui_composition::append_gui_packet(&mut frame, packet)?;
        }
        Ok((frame.with_line_width_policy(self.line_widths), scaled))
    }
    /// The scene frame with the view's feedback attached.
    fn fed_scene_frame(&self, extent: [u32; 2]) -> Result<crate::RenderFrame, RenderError> {
        let mut frame = self.scene_frame(extent)?;
        frame.attach_raster_feedback(&self.raster_feedback);
        frame.attach_stencil_context(&self.stencil_feedback);
        frame.attach_fragment_alpha(&self.fragment_alpha_feedback);
        Ok(frame)
    }
    /// Under a render scale, render the scene offscreen and blit it over the target extent.
    fn blit_scaled_scene(
        &self,
        scene: crate::RenderFrame,
        extent: [u32; 2],
    ) -> Result<(crate::RenderFrame, Option<FrameCapture>), RenderError> {
        if crate::render_scale::active_render_scale(&self.rendering_params()).is_none() {
            return Ok((scene, None));
        }
        let scaled = self.device.write().render(&scene)?;
        let frame = crate::render_scale::blit_frame(&scaled, extent)?;
        Ok((frame, Some(scaled)))
    }
    fn scene_frame(&self, extent: [u32; 2]) -> Result<crate::RenderFrame, RenderError> {
        let stats = self.frame_stats();
        let clipping = self.retained.render_clip_planes();
        let extras = crate::view_scene::ViewFrameExtras {
            stats: stats.get(),
            overlay: Some(self.frame_stats_overlay()),
            trihedron: self.shown_graduated_trihedron(),
            grid: Some(self.shader_grid_state()),
            frame_steps: Some(self),
            limits: self.device.read().device_limits(),
        };
        // GlView::Redraw renders the shadow maps before the scene.
        let shadow_maps = self.shadow_maps(extent)?;
        let mut frame = frame_for_view(self, extent, clipping, extras)?;
        frame.shadow_maps = shadow_maps;
        Ok(frame.with_line_width_policy(self.line_widths))
    }
}
#[path = "driver_view_stereo.rs"]
mod stereo;
impl Graphic3dRenderView {
    fn render_target(&self, frame: &crate::RenderFrame) -> Result<FrameCapture, RenderError> {
        let window = self.window();
        let mut device = self.device.write();
        if !self.fbo().is_null() || window.is_virtual() {
            return device.render(frame);
        }
        if window.native_handle() == 0 {
            return Err(rejected(
                0,
                "a nonzero native window handle or a virtual window",
            ));
        }
        // Native buffersNoSwap draws the frame but never swaps; dumps read the rendered frame.
        if !window.is_mapped() || self.options.read().buffers_no_swap {
            return device.render(frame);
        }
        self.render_mapped_target(frame, &mut device)
    }
    fn render_mapped_target(
        &self,
        frame: &crate::RenderFrame,
        device: &mut crate::driver_backend::DriverBackend,
    ) -> Result<FrameCapture, RenderError> {
        let mut presenter = self.presenter.write();
        if presenter.is_none() {
            *presenter = Some(device.window_presenter(&self.window())?);
        }
        let result = device.render_window(
            frame,
            presenter.as_mut().unwrap(),
            self.options.read().vertical_sync,
            self.gui_surface,
        );
        if result.is_err() {
            presenter.take();
        }
        result
    }
    pub(crate) fn release_presenter(&self) {
        let _device = self.device.write();
        self.presenter.write().take();
    }
}
impl Drop for Graphic3dRenderView {
    fn drop(&mut self) {
        self.release_presenter();
    }
}
impl Graphic3dCView for Graphic3dRenderView {
    retained_view_methods!();
    fn cview_base(&self) -> &Graphic3dCViewBase {
        self.retained.cview_base()
    }
    fn to_flip_output(&self) -> bool {
        self.retained.to_flip_output()
    }
    fn graduated_trihedron_display(&self, data: &Graphic3dGraduatedTrihedron) {
        self.display_graduated_trihedron(data);
    }
    fn graduated_trihedron_erase(&self) {
        self.erase_graduated_trihedron();
    }
    fn graduated_trihedron_min_max_values(
        &self,
        min: martensite_tkernel::ncollection::Vec3<f32>,
        max: martensite_tkernel::ncollection::Vec3<f32>,
    ) {
        self.graduated_trihedron_bounds(min, max);
    }
    fn set_to_flip_output(&self, flip: bool) {
        self.retained.set_to_flip_output(flip);
    }
    fn remove(&self) {
        self.release_presenter();
        let retired = self.gui_layers.write().retire_removed();
        drop(retired);
        graphic3d_cview_remove(self);
    }
    fn redraw(&self) {
        self.keep_ray_traced_arrays();
        self.finish_redraw(self.try_redraw());
    }
    fn redraw_immediate(&self) {
        self.finish_redraw(self.try_redraw());
    }
    fn resized(&self) {
        if self.reject_closed_resize() {
            return;
        }
        if self.gui_layers.write().resized() {
            self.invalidate_gui_capture();
        }
        graphic3d_cview_resized(self);
        self.invalidate();
    }
    fn set_window(
        &self,
        parent: &Handle<dyn Graphic3dCView>,
        window: &Handle<dyn AspectWindow>,
        context: AspectRenderingContext,
    ) {
        self.release_presenter();
        // A subview shares its parent's device and renders offscreen (blit_subviews composes
        // it); only an external GL context stays unsupported.
        *self.window_error.write() = (context != 0).then(|| {
            rejected(
                (parent.object_address(), context),
                "a view without an external GL context",
            )
        });
        self.link_parent_view(parent, window);
        self.retained.set_window(parent, window, context);
        self.invalidate_window_binding();
    }
    fn buffer_dump(&self, image: &ImagePixMap, kind: &Graphic3dBufferType) -> bool {
        #[cfg(any(
            feature = "raytrace-input-diagnostics",
            feature = "command-state-diagnostics"
        ))]
        let observed_serial = self
            .redraw_serial
            .load(std::sync::atomic::Ordering::Relaxed);
        let capture = match self.capture() {
            Ok(capture) => capture,
            Err(error) => {
                report_capture_failure(&error);
                return false;
            }
        };
        let copied = dump_capture(image, kind, &capture);
        #[cfg(feature = "raytrace-input-diagnostics")]
        self.observe_raytrace_buffer_dump(observed_serial, kind, &capture, copied);
        #[cfg(feature = "command-state-diagnostics")]
        crate::scene::replay_diagnostic::dumped(
            self as *const Self as usize,
            [
                observed_serial,
                self.redraw_serial
                    .load(std::sync::atomic::Ordering::Relaxed),
            ],
            &format!("{kind:?}"),
            copied,
            &capture,
        );
        copied
    }
    fn fbo_create(&self, width: i32, height: i32) -> Handle<StandardTransient> {
        RenderFrameBuffer::create(width, height, self.limit)
    }
    fn fbo_release(&self, fbo: &mut Handle<StandardTransient>) {
        // Native FBORelease ignores a handle that is not a framebuffer of the driver.
        if !fbo.cast::<RenderFrameBuffer>().is_null() {
            *fbo = Handle::default();
        }
    }
    fn fbo_get_dimensions(&self, fbo: &Handle<StandardTransient>) -> [i32; 4] {
        fbo.cast::<RenderFrameBuffer>()
            .get()
            .map_or([0; 4], RenderFrameBuffer::dimensions)
    }
    fn fbo_change_viewport(&self, fbo: &Handle<StandardTransient>, width: i32, height: i32) {
        if let Some(target) = fbo.cast::<RenderFrameBuffer>().get() {
            target.change_viewport(width, height);
        }
    }
    fn grid_display(&self, params: &AspectGridParams, plane: &martensite_tkmath::gp::Axis3) {
        self.display_shader_grid(params, plane);
    }
    fn grid_erase(&self) {
        self.erase_shader_grid();
    }
    fn shader_grid_echo(&self, x: i32, y: i32, point: &mut Graphic3dVertex) -> bool {
        let mut display = Graphic3dVertex::default();
        self.shader_grid_echo_display(x, y, point, &mut display)
    }
    fn shader_grid_echo_display(
        &self,
        x: i32,
        y: i32,
        point: &mut Graphic3dVertex,
        display: &mut Graphic3dVertex,
    ) -> bool {
        let Some((snapped, shown)) = self.echo_shader_grid(x, y) else {
            return false;
        };
        *point = crate::view_shader_grid::vertex_of(snapped);
        *display = crate::view_shader_grid::vertex_of(shown);
        true
    }
    fn shader_grid_snap_point(
        &self,
        point: &Graphic3dVertex,
        grid_point: &mut Graphic3dVertex,
    ) -> bool {
        let Some(snapped) = self.snap_to_shader_grid(point) else {
            return false;
        };
        *grid_point = crate::view_shader_grid::vertex_of(snapped);
        true
    }
    fn z_fit_all_bounds(
        &self,
    ) -> Result<
        (
            martensite_tkmath::bnd::BoundingBox3,
            martensite_tkmath::bnd::BoundingBox3,
        ),
        Failure,
    > {
        self.z_fit_bounds_with_grid()
    }
    fn diagnostic_information(
        &self,
        dict: &mut Graphic3dFrameStatsDict,
        flags: Graphic3dDiagnosticInfo,
    ) {
        graphic3d_cview_diagnostic_information(self, dict, flags);
        self.diagnostics().fill(dict, flags);
        dict.add("Martensite device".into(), self.device.read().name().into());
        if let Some(Err(error)) = self.outcome.read().as_ref() {
            dict.add(
                "Martensite render error".into(),
                error.to_string().as_str().into(),
            );
        }
    }
}

#[cfg(test)]
#[path = "view_binding_tests.rs"]
mod view_binding_tests;

#[path = "view_frame_publication.rs"]
mod frame_publication;
#[path = "view_lighting_publication.rs"]
mod lighting_publication;

#[path = "view_gui_checked.rs"]
mod gui_checked;

#[cfg(test)]
#[path = "gui_closure_tests.rs"]
mod gui_closure_tests;

#[path = "redraw_completion.rs"]
mod redraw_completion;
pub use redraw_completion::{RenderRedrawCursor, RenderRedrawFailure};

#[cfg(test)]
#[path = "view_lighting_transition_tests.rs"]
mod lighting_transition_tests;

#[cfg(test)]
#[path = "view_lighting_concurrent_tests.rs"]
mod lighting_concurrent_tests;

#[cfg(test)]
#[path = "view_lighting_frames_tests.rs"]
mod lighting_frames_tests;

#[cfg(feature = "raytrace-input-diagnostics")]
#[path = "driver_view_raytrace_inputs.rs"]
mod raytrace_inputs;
