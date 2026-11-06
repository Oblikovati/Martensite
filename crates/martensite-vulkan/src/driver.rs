//! Graphic3d factory routes ordinary structure/view creation to Martensite rendering.
use crate::retained_texture_images::RetainedTextureImages;
use crate::{
    RenderBackend, RenderError, VulkanOptions, VulkanRenderer,
    driver_backend::{DriverBackend, SharedRenderDevice},
    driver_structure::Graphic3dRenderStructure,
    driver_view::Graphic3dRenderView,
    error::rejected,
};
use crate::{RenderPipelinePolicies, RenderRasterPolicy};
use martensite_tkernel::{
    standard::{Failure, Handle, SharedState, StandardTransient, WeakHandle},
    tcollection::AsciiString,
};
use martensite_tkservice::{aspect::AspectWindow, graphic3d::*};
use std::sync::Arc;
/// A Graphic3d driver with an injected device; `Graphic3dRenderDriver::new(VulkanOptions::default())?`.
pub struct Graphic3dRenderDriver {
    transient: StandardTransient,
    base: Graphic3dGraphicDriverBase,
    device: SharedRenderDevice,
    text_source: Arc<dyn crate::TextLayoutSource>,
    pub(crate) mesh_edge_pipeline: crate::RenderMeshEdgePipeline,
    pub(crate) unlit_face_color_policy: crate::RenderUnlitFaceColorPolicy,
    pub(crate) lighting_selection: crate::driver_lighting_capability::SharedLightingSelection,
    pub(crate) texture_images: Arc<crate::retained_texture_images::RetainedTextureImages>,
    /// The views and structures this driver created, shared with its views.
    pub(crate) objects: Arc<crate::driver_device_lost::DriverObjects>,
    limit: u32,
    pub(super) line_widths: crate::RenderLineWidthPolicy,
    gui_surface: crate::RenderGuiSurfacePolicy,
    /// Options shared with every view: GlCaps of the native driver.
    pub(crate) options: crate::driver_options::SharedDriverOptions,
    /// Unique ids of primitive arrays (GlGraphicDriver::myUIDGenerator).
    primitive_array_uids: std::sync::atomic::AtomicUsize,
    /// Precompiled SPIR-V of the user programs this driver can draw.
    user_programs: Arc<crate::RenderUserProgramLibrary>,
}
struct DriverBuildInputs {
    backend: DriverBackend,
    limit: u32,
    text_source: Arc<dyn crate::TextLayoutSource>,
    selection: RenderPipelinePolicies,
    line_widths: crate::RenderLineWidthPolicy,
}
martensite_tkernel::impl_transient!(Graphic3dRenderDriver, "Martensite_RenderDriver", StandardTransient, transient, interfaces:[dyn Graphic3dGraphicDriver]);
impl Graphic3dRenderDriver {
    /// Create a Vulkan 1.3 driver; `Graphic3dRenderDriver::new(options)?`.
    pub fn new(options: VulkanOptions) -> Result<Handle<Self>, RenderError> {
        Self::with_texture_coordinate_pipeline(
            options,
            crate::RenderTextureCoordinatePipeline::default(),
        )
    }
    /// Inject legacy coordinate semantics; `Graphic3dRenderDriver::with_texture_coordinate_pipeline(options, RenderTextureCoordinatePipeline::GeneratedCompatibility)?`.
    /// This changes texture coordinates, not the shader lighting family.
    pub fn with_texture_coordinate_pipeline(
        options: VulkanOptions,
        coordinate_pipeline: crate::RenderTextureCoordinatePipeline,
    ) -> Result<Handle<Self>, RenderError> {
        Self::with_surface_pipelines(
            options,
            coordinate_pipeline,
            crate::RenderMeshEdgePipeline::default(),
        )
    }
    /// Select native polygon edge compatibility; `Self::with_mesh_edge_pipeline(options, RenderMeshEdgePipeline::PolygonCompatibility)?`.
    pub fn with_mesh_edge_pipeline(
        options: VulkanOptions,
        pipeline: crate::RenderMeshEdgePipeline,
    ) -> Result<Handle<Self>, RenderError> {
        Self::with_surface_pipelines(
            options,
            crate::RenderTextureCoordinatePipeline::default(),
            pipeline,
        )
    }
    /// Inject independent legacy coordinate and edge policies; `Self::with_surface_pipelines(options, RenderTextureCoordinatePipeline::GeneratedCompatibility, RenderMeshEdgePipeline::PolygonCompatibility)?`.
    pub fn with_surface_pipelines(
        options: VulkanOptions,
        coordinates: crate::RenderTextureCoordinatePipeline,
        edges: crate::RenderMeshEdgePipeline,
    ) -> Result<Handle<Self>, RenderError> {
        Self::with_render_pipelines(
            options,
            coordinates,
            edges,
            crate::RenderLightingPipeline::default(),
        )
    }
    /// Inject lighting, coordinates and edges; `Self::with_render_pipelines(options, coordinates, edges, RenderLightingPipeline::FixedFunction)?`.
    pub fn with_render_pipelines(
        options: VulkanOptions,
        coordinates: crate::RenderTextureCoordinatePipeline,
        edges: crate::RenderMeshEdgePipeline,
        lighting: crate::RenderLightingPipeline,
    ) -> Result<Handle<Self>, RenderError> {
        Self::with_pipeline_policies(
            options,
            coordinates,
            edges,
            lighting,
            Default::default(),
            Default::default(),
        )
    }
    pub(super) fn with_pipeline_policies(
        options: VulkanOptions,
        coordinates: crate::RenderTextureCoordinatePipeline,
        edges: crate::RenderMeshEdgePipeline,
        lighting: crate::RenderLightingPipeline,
        unlit: crate::RenderUnlitFaceColorPolicy,
        gui_surface: crate::RenderGuiSurfacePolicy,
    ) -> Result<Handle<Self>, RenderError> {
        let selection = RenderPipelinePolicies {
            texture_coordinates: coordinates,
            mesh_edges: edges,
            lighting,
            unlit_faces: unlit,
            gui_surface,
            ..Default::default()
        };
        Self::with_pipeline_configuration(options, selection)
    }
    /// Compose independent policies; `Self::with_pipeline_configuration(options, selection)?`.
    pub fn with_pipeline_configuration(
        options: VulkanOptions,
        selection: RenderPipelinePolicies,
    ) -> Result<Handle<Self>, RenderError> {
        Self::with_coordinate_configuration(options, selection, Default::default())
    }
    /// Select mapping independently; `Self::with_coordinate_configuration(options, selection, coordinates)?`.
    pub fn with_coordinate_configuration(
        options: VulkanOptions,
        selection: RenderPipelinePolicies,
        coordinates: crate::RenderCoordinateConvention,
    ) -> Result<Handle<Self>, RenderError> {
        Self::with_line_width_configuration(options, selection, coordinates, Default::default())
    }

    /// Select raster behavior with other defaults; `Self::with_raster_policy(options, policy)?`.
    pub fn with_raster_policy(
        options: VulkanOptions,
        policy: RenderRasterPolicy,
    ) -> Result<Handle<Self>, RenderError> {
        let selection = RenderPipelinePolicies {
            raster: policy,
            ..Default::default()
        };
        Self::with_pipeline_configuration(options, selection)
    }
    /// Read admitted raster behavior; injected devices return `None`.
    /// Example: `driver.raster_policy()` records the effective replay selection.
    pub fn raster_policy(&self) -> Option<RenderRasterPolicy> {
        let backend = self.device.read();
        backend.raster_policy()
    }
    /// Inspect real attachment mapping; an injected backend returns `None`.
    /// Example: `driver.coordinate_convention()` never guesses a fake device's behavior.
    pub fn coordinate_convention(&self) -> Option<crate::RenderCoordinateConvention> {
        let backend = self.device.read();
        backend.coordinate_convention()
    }
    /// Inject a named backend and its extent limit; `Graphic3dRenderDriver::with_backend(recorder, 4096)?`.
    pub fn with_backend(
        backend: impl RenderBackend + Send + Sync + 'static,
        limit: u32,
    ) -> Result<Handle<Self>, RenderError> {
        Self::with_backend_and_fonts(
            backend,
            limit,
            Arc::new(crate::FreeTypeTextSource::default()),
        )
    }
    /// Inject both graphics and font I/O; `Graphic3dRenderDriver::with_backend_and_fonts(recorder, 4096, fonts)?`.
    pub fn with_backend_and_fonts(
        backend: impl RenderBackend + Send + Sync + 'static,
        limit: u32,
        fonts: Arc<dyn crate::TextLayoutSource>,
    ) -> Result<Handle<Self>, RenderError> {
        Self::with_backend_and_texture_coordinates(
            backend,
            limit,
            fonts,
            crate::RenderTextureCoordinatePipeline::default(),
        )
    }
    /// Inject device/font I/O and coordinate policy; `Self::with_backend_and_texture_coordinates(recorder, 4096, fonts, policy)?`.
    pub fn with_backend_and_texture_coordinates(
        backend: impl RenderBackend + Send + Sync + 'static,
        limit: u32,
        fonts: Arc<dyn crate::TextLayoutSource>,
        coordinate_pipeline: crate::RenderTextureCoordinatePipeline,
    ) -> Result<Handle<Self>, RenderError> {
        Self::with_injected_policies(
            backend,
            limit,
            fonts,
            coordinate_pipeline,
            Default::default(),
            Default::default(),
        )
    }
    pub(super) fn with_injected_policies(
        backend: impl RenderBackend + Send + Sync + 'static,
        limit: u32,
        fonts: Arc<dyn crate::TextLayoutSource>,
        coordinates: crate::RenderTextureCoordinatePipeline,
        unlit: crate::RenderUnlitFaceColorPolicy,
        gui_surface: crate::RenderGuiSurfacePolicy,
    ) -> Result<Handle<Self>, RenderError> {
        let selection = RenderPipelinePolicies {
            texture_coordinates: coordinates,
            unlit_faces: unlit,
            gui_surface,
            ..Default::default()
        };
        Self::with_injected_selection(backend, limit, fonts, selection)
    }
    fn with_injected_selection(
        backend: impl RenderBackend + Send + Sync + 'static,
        limit: u32,
        fonts: Arc<dyn crate::TextLayoutSource>,
        selection: RenderPipelinePolicies,
    ) -> Result<Handle<Self>, RenderError> {
        Self::with_backend_and_line_width_configuration(
            backend,
            limit,
            fonts,
            selection,
            Default::default(),
        )
    }

    fn build(inputs: DriverBuildInputs) -> Handle<Self> {
        let retained =
            RetainedTextureImages::with_coordinate_pipeline(inputs.selection.texture_coordinates);
        let texture_images = Arc::new(retained);
        Handle::new(Self {
            transient: StandardTransient::default(),
            base: Graphic3dGraphicDriverBase::new(&Handle::default()),
            device: Arc::new(SharedState::new(inputs.backend)),
            text_source: inputs.text_source,
            mesh_edge_pipeline: inputs.selection.mesh_edges,
            lighting_selection: Arc::new(SharedState::new(
                crate::driver_lighting_capability::LightingSelection::new(
                    inputs.selection.lighting,
                ),
            )),
            unlit_face_color_policy: inputs.selection.unlit_faces,
            texture_images,
            objects: Arc::default(),
            limit: inputs.limit,
            line_widths: inputs.line_widths,
            gui_surface: inputs.selection.gui_surface,
            // Native GlCaps::swapInterval starts at 1, so IsVerticalSync is true by default.
            options: Arc::default(),
            primitive_array_uids: Default::default(),
            user_programs: sample_user_programs(),
        })
    }
    pub(super) fn publish_renderer(
        renderer: VulkanRenderer,
        selection: RenderPipelinePolicies,
        line_widths: crate::RenderLineWidthPolicy,
    ) -> Handle<Self> {
        Self::build(DriverBuildInputs {
            limit: renderer.capture_limit(),
            backend: DriverBackend::Vulkan(Box::new(renderer)),
            text_source: Arc::new(crate::FreeTypeTextSource::default()),
            selection,
            line_widths,
        })
    }
    /// The next unique primitive-array id, starting at 1 (GlGraphicDriver::
    /// GetNextPrimitiveArrayUID); every group array and `RenderPrimitiveArray` takes one.
    /// Example: `let uid = driver.next_primitive_array_uid;`.
    pub fn next_primitive_array_uid(&self) -> usize {
        // GlStateCounter::Increment returns the incremented value.
        1 + self
            .primitive_array_uids
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
    /// Require complete Vulkan observations; `driver.validation_errors()?` preserves disabled/poisoned failures.
    pub fn validation_errors(
        &self,
    ) -> Result<Vec<String>, crate::RenderValidationObservationFailure> {
        self.device.read().validation_errors()
    }
    /// Testing only, not product API: the raw `VkInstance` handle of the Vulkan device, None for
    /// an injected backend. RenderDoc's in-application API keys a headless frame capture on the
    /// instance's dispatch pointer (`RENDERDOC_DEVICEPOINTER_FROM_VKINSTANCE`); see
    /// tests/support/renderdoc_capture.rs. Example: `driver.raw_vulkan_instance()`.
    #[doc(hidden)]
    pub fn raw_vulkan_instance(&self) -> Option<u64> {
        self.device.read().raw_instance()
    }
    /// The SPIR-V library of user programs; `driver.user_programs().insert(key, modules)?`.
    pub fn user_programs(&self) -> &Arc<crate::RenderUserProgramLibrary> {
        &self.user_programs
    }
    /// Identify the actual rendering device; `driver.device_name()`.
    pub fn device_name(&self) -> String {
        self.device.read().name().to_owned()
    }
    /// Read exact selected Vulkan provenance, absent for injected backends; `driver.driver_identity()`.
    pub fn driver_identity(&self) -> Option<crate::RenderDriverIdentity> {
        self.device.read().driver_identity()
    }
    /// Sample owned physical capacity and optional process-budget estimates; driver.memory_report()?.heaps().
    /// Actual global free memory remains unknown. Injected backends report unavailable information.
    pub fn memory_report(
        &self,
    ) -> Result<crate::RenderDeviceMemoryReport, crate::RenderMemoryQueryError> {
        // Keep the existing backend owner alive throughout the query; reports retain no device borrow.
        self.device.read().memory_report()
    }
    /// Read the selected device capacities behind `inquire_limit`; `None` for injected backends.
    /// Example: `driver.device_limits().map(|limits| limits.max_clip_planes())`.
    pub fn device_limits(&self) -> Option<crate::RenderDeviceLimits> {
        self.device.read().device_limits()
    }
    fn live_views(&self) -> Vec<Handle<dyn Graphic3dCView>> {
        self.objects
            .views
            .read()
            .iter()
            .map(WeakHandle::upgrade)
            .filter(|v| !v.is_null() && !v.is_removed())
            .collect()
    }
}
impl Graphic3dGraphicDriver for Graphic3dRenderDriver {
    fn graphic_driver_base(&self) -> &Graphic3dGraphicDriverBase {
        &self.base
    }
    fn inquire_limit(&self, limit: Graphic3dTypeOfLimit) -> i32 {
        crate::driver_limits::inquire(limit, self.limit, self.device_limits())
    }
    fn create_structure(
        &self,
        manager: &Handle<Graphic3dStructureManager>,
    ) -> Result<Handle<dyn Graphic3dCStructure>, Failure> {
        let structure: Handle<dyn Graphic3dCStructure> =
            Graphic3dRenderStructure::new(manager, self.text_source.clone())?.cast();
        let mut structures = self.objects.structures.write();
        structures.retain(|weak| !weak.upgrade().is_null());
        structures.push(structure.downgrade());
        Ok(structure)
    }
    fn remove_structure(&self, structure: &mut Handle<dyn Graphic3dCStructure>) {
        self.objects
            .structures
            .write()
            .retain(|weak| weak.upgrade() != *structure && !weak.upgrade().is_null());
        if let Some(value) = structure.get() {
            value.clear();
        }
        *structure = Handle::default();
    }
    fn create_view(
        &self,
        manager: &Handle<Graphic3dStructureManager>,
    ) -> Result<Handle<dyn Graphic3dCView>, Failure> {
        let view: Handle<dyn Graphic3dCView> = Graphic3dRenderView::create(
            manager,
            &self.layers(),
            self.device.clone(),
            self.limit,
            (
                self.options.clone(),
                self.objects.clone(),
                self.line_widths,
                self.lighting_selection.clone(),
            ),
            self.gui_surface,
        )?
        .cast();
        self.objects.views.write().push(view.downgrade());
        Ok(view)
    }
    fn remove_view(&self, view: &Handle<dyn Graphic3dCView>) {
        self.objects
            .views
            .write()
            .retain(|weak| weak.upgrade() != *view && !weak.upgrade().is_null());
    }
    // Vulkan buffer uploads are mandatory; this legacy GL implementation switch has no effect.
    fn enable_vbo(&self, _: bool) {}
    fn is_vertical_sync(&self) -> bool {
        self.options.read().vertical_sync
    }
    fn set_vertical_sync(&self, enabled: bool) {
        self.options.write().vertical_sync = enabled;
    }
    fn memory_info(&self, free_bytes: &mut usize, info: &mut AsciiString) -> bool {
        // Native failure leaves caller outputs untouched; unknown global free is zero only on success.
        crate::memory_report::legacy::write_result(self.memory_report(), free_bytes, info)
    }
    fn default_text_height(&self) -> f32 {
        16.
    }
    fn text_size(&self, view: &Handle<dyn Graphic3dCView>, text: &str, height: f32) -> [f32; 3] {
        crate::driver_text::text_size(view, text, height)
    }
    fn insert_layer_before(
        &self,
        id: Graphic3dZLayerId,
        settings: &Graphic3dZLayerSettings,
        after: Graphic3dZLayerId,
    ) -> Result<(), Failure> {
        self.base.insert_layer_before(id, settings, after)?;
        for view in self.live_views() {
            view.insert_layer_before(id, settings, after)?;
        }
        Ok(())
    }
    fn insert_layer_after(
        &self,
        id: Graphic3dZLayerId,
        settings: &Graphic3dZLayerSettings,
        before: Graphic3dZLayerId,
    ) -> Result<(), Failure> {
        self.base.insert_layer_after(id, settings, before)?;
        for view in self.live_views() {
            view.insert_layer_after(id, settings, before)?;
        }
        Ok(())
    }
    fn remove_z_layer(&self, id: Graphic3dZLayerId) -> Result<(), Failure> {
        self.base.remove_z_layer(id)?;
        for view in self.live_views() {
            view.remove_z_layer(id)?;
        }
        Ok(())
    }
    fn set_z_layer_settings(
        &self,
        id: Graphic3dZLayerId,
        settings: &Graphic3dZLayerSettings,
    ) -> Result<(), Failure> {
        self.base.set_z_layer_settings(id, settings)?;
        for view in self.live_views() {
            view.set_z_layer_settings(id, settings)?;
        }
        Ok(())
    }
    fn view_exists(
        &self,
        window: &Handle<dyn AspectWindow>,
        output: &mut Handle<dyn Graphic3dCView>,
    ) -> bool {
        *output = self
            .live_views()
            .into_iter()
            .find(|view| view.window() == *window)
            .unwrap_or_default();
        !output.is_null()
    }
}

fn validated_renderer(
    options: VulkanOptions,
    selection: &RenderPipelinePolicies,
    coordinates: crate::RenderCoordinateConvention,
) -> Result<(VulkanRenderer, u32), RenderError> {
    selection.unlit_faces.validate(selection.lighting)?;
    let renderer = VulkanRenderer::new_with_conventions(options, selection.raster, coordinates)?;
    let limit = renderer.capture_limit();
    Ok((renderer, limit))
}

fn validate_capture_limit(limit: u32) -> Result<(), RenderError> {
    if limit == 0 || limit > i32::MAX as u32 {
        return Err(rejected(limit, "a positive 31-bit capture limit"));
    }
    Ok(())
}

#[cfg(test)]
#[path = "driver_constructor_contract.rs"]
mod constructor_contract;

#[cfg(test)]
#[path = "raster_policy_driver_tests.rs"]
mod raster_policy_contract;

/// A library holding the built-in sample programs; their SPIR-V is part of the crate, so a
/// failed registration is a build defect.
fn sample_user_programs() -> Arc<crate::RenderUserProgramLibrary> {
    let library = crate::RenderUserProgramLibrary::default();
    crate::user_program::register_samples(&library).expect("valid built-in sample programs");
    Arc::new(library)
}

#[path = "driver_light_dot.rs"]
mod light_dot;

#[path = "driver_line_clipping.rs"]
mod line_clipping_configuration;
#[path = "driver_line_width.rs"]
mod line_width;

#[path = "driver_phong_interpolation.rs"]
mod phong_interpolation_configuration;
#[path = "driver_point_raster.rs"]
mod point_raster_configuration;
