//! Runtime ownership keeps the loader, instance and device alive until all resources drop.
use super::validation::{ValidationMessages, ValidationMessenger};
use crate::error::{RenderError, rejected};
use ash::{Device, Entry, Instance, vk};
use std::{ffi::CStr, sync::Arc};
/// Device creation policy; `VulkanOptions { validation: true, ..Default::default() }`.
#[derive(Clone, Copy, Debug, Default)]
pub struct VulkanOptions {
    /// Require the Khronos validation layer when enabled.
    pub validation: bool,
    /// Select a physical device by enumeration index, or the first capable device.
    pub device_index: Option<usize>,
}
pub(super) struct VulkanRuntime {
    pub(super) entry: Entry,
    pub(super) lifecycle: super::device_lifecycle::DeviceLifecycle,
    pub window_support: bool,
    pub displays: martensite_tkernel::standard::SharedState<
        Vec<martensite_tkservice::aspect::AspectDisplayConnectionLease>,
    >,
    pub queues: Vec<vk::Queue>,
    pub instance: Instance,
    messenger: Option<ValidationMessenger>,
    device: Option<Device>,
    pub validation: Arc<ValidationMessages>,
    pub physical: vk::PhysicalDevice,
    pub queue: vk::Queue,
    pub queue_family: u32,
    pub memory: vk::PhysicalDeviceMemoryProperties,
    pub limits: vk::PhysicalDeviceLimits,
    pub depth_clamp: bool,
    /// 24-bit depth and OpenGL clip-depth mapping where the device offers them.
    pub(super) depth: super::depth_equivalence::DepthEquivalence,
    #[cfg(all(test, target_os = "linux"))]
    pub(super) owned_v2_original_depth: Option<super::depth_equivalence::DepthEquivalence>,
    /// `VK_EXT_non_seamless_cube_map`: cube samplers filter per face, as OpenGL does.
    pub(super) non_seamless_cube: bool,
    /// `VK_EXT_sample_locations`: the sample counts placed at OpenGL's mirrored positions.
    pub(super) mirrored_samples: vk::SampleCountFlags,
    /// `shaderImageFloat32AtomicAdd` (backend/image_float_atomics.rs), enabled when offered.
    pub(crate) image_float_atomic_add: bool,
    /// Non-uniform sampler indexing for textured ray-traced materials
    /// (backend/raytrace_texture_indexing.rs), enabled when the driver reports the flag.
    pub(crate) raytrace_textures: bool,
    /// `fragmentStoresAndAtomics`: adaptive sampling writes storage images from fragments.
    pub(crate) fragment_stores: bool,
    #[cfg(feature = "raytrace-hit-diagnostics")]
    pub(super) ray_hit_control: Option<super::raytrace_hit_control::RayHitControl>,
    /// Test control: keep the standard positions although the device could mirror them.
    #[cfg(test)]
    pub(super) standard_samples: bool,
    pub logic_op: bool,
    pub geometry_shader: bool,
    /// `tessellationShader`: user programs with tessellation stages (Graphic3dTOS_TESS_*).
    pub tessellation_shader: bool,
    pub polygon_lines: bool,
    pub subset_point_polygons: Option<bool>,
    pub large_points: bool,
    pub sampler_anisotropy: bool,
    pub sample_rate_shading: bool,
    pub depth_resolve_modes: vk::ResolveModeFlags,
    pub stencil_resolve_modes: vk::ResolveModeFlags,
    pub lines: super::line_device::LineCapabilities,
    pub line_stipple: Option<super::line_device::LineStippleDevice>,
    pub(super) point_raster: crate::RenderPointRasterBehavior,
    pub(super) thin_axis_coverage: crate::RenderThinAxisCoverage,
    pub(super) thin_line_coverage: crate::RenderThinLineCoverage,
    #[cfg(feature = "vertex-output-diagnostics")]
    pub(super) vertex_output: Option<Arc<crate::VertexOutputRequest>>,
    #[cfg(feature = "lit-only-command-diagnostics")]
    pub(super) lit_only_commands: super::lit_only_commands::LitOnlyCommands,
    pub(super) line_clipping: crate::RenderLineClippingBehavior,
    pub(super) light_dot_behavior: crate::RenderLightDotBehavior,
    pub(super) phong_interpolation: crate::RenderPhongInterpolationBehavior,
    pub(super) triangle_coverage: crate::RenderTriangleCoverageBehavior,
    pub(super) text_interpolation: crate::RenderTextInterpolationBehavior,
    pub(super) depth_bias_behavior: crate::RenderDepthBiasBehavior,
    pub(super) projection_behavior: crate::RenderProjectionBehavior,
    pub(super) window_depth_behavior: crate::RenderWindowDepthBehavior,
    pub(super) depth_bias_command: Option<ash::ext::depth_bias_control::Device>,
    pub(super) line_clip: super::line_clip_device::LineClipDevice,
    pub(super) world_clipping: crate::RenderWorldClippingBehavior,
    pub(super) world_clip_interpolation: crate::RenderWorldClipInterpolationBehavior,
    pub(super) world_clip: super::world_clip_device::WorldClipDevice,
    pub name: String,
    pub quad_diagonal: crate::quad_assembly::QuadDiagonal,
    pub raster_policy: crate::RenderRasterPolicy,
    #[cfg(test)]
    pub(super) executable_profile: Option<super::diagnostic_executable::CaptureProfile>,
    pub(super) attachment_coordinates: super::attachment_coordinates::AttachmentCoordinates,
    pub(super) image_memory: super::image_memory::ImageMemoryLedger,
    pub(super) memory_arena: super::memory_arena::MemoryArena,
}
impl VulkanRuntime {
    pub(super) fn admit_world_clip_interpolation(
        &self,
        draw: &crate::PrimitiveDraw,
    ) -> Result<bool, RenderError> {
        let selected = super::world_clip_interpolation_upload::admit(
            self.world_clip_interpolation,
            self.world_clipping,
            self.triangle_coverage,
            self.phong_interpolation,
            draw,
        )?;
        #[cfg(feature = "vertex-output-diagnostics")]
        if selected && self.vertex_output.is_some() {
            return Err(
                crate::RenderWorldClipInterpolationFailure::UnsupportedDraw {
                    actual: "concurrent vertex-output diagnostic capture".into(),
                }
                .into(),
            );
        }
        Ok(selected)
    }
    pub(super) fn uses_grid12_world_clipping(&self, draw: &crate::PrimitiveDraw) -> bool {
        super::world_clip_interpolation_upload::selects_grid12_fragment(
            draw,
            self.world_clip_interpolation,
        )
    }
    pub(super) fn admit_world_clipping(
        &self,
        draw: &crate::PrimitiveDraw,
    ) -> Result<bool, RenderError> {
        let compatible = self.world_clip_policies_are_supported(draw);
        self.world_clip
            .admit_draw(self.effective_world_clipping(draw), draw, compatible)
    }
    fn world_clip_policies_are_supported(&self, draw: &crate::PrimitiveDraw) -> bool {
        let legacy_phong = self.phong_interpolation
            == crate::RenderPhongInterpolationBehavior::HardwareV1
            && self.light_dot_behavior == crate::RenderLightDotBehavior::DriverNativeV1
            && self.triangle_coverage == crate::RenderTriangleCoverageBehavior::HardwareV1;
        let grid12_phong = super::world_clip_interpolation_upload::supports_composition(
            self.world_clip_interpolation,
            self.world_clipping,
            self.triangle_coverage,
            self.phong_interpolation,
        ) && super::triangle_grid_coverage_upload::selected_phong(draw);
        let phong = legacy_phong || grid12_phong;
        if self.world_clipping != crate::RenderWorldClippingBehavior::RasterClipDistanceV3
            || draw.topology() != crate::PrimitiveTopology::Lines
        {
            return phong;
        }
        self.line_clipping == crate::RenderLineClippingBehavior::VulkanDriverV1
            && self.thin_line_coverage == crate::RenderThinLineCoverage::HardwareV1
            && self.thin_axis_coverage == crate::RenderThinAxisCoverage::HardwareV1
            && self.lines.bresenham
            && native_line_width_supported(self, draw)
    }
    pub(super) fn uses_raster_world_clipping(&self, draw: &crate::PrimitiveDraw) -> bool {
        matches!(
            self.effective_world_clipping(draw),
            crate::RenderWorldClippingBehavior::RasterClipDistanceV2
                | crate::RenderWorldClippingBehavior::RasterClipDistanceV3
                | crate::RenderWorldClippingBehavior::RasterFixedFunctionV4
                | crate::RenderWorldClippingBehavior::RasterFixedFunctionClipSpaceV5
        ) && draw
            .clipping
            .as_ref()
            .is_some_and(|clip| clip.chains().iter().any(|chain| !chain.is_empty()))
    }
    pub(super) fn uses_world_clip_line_v3(&self, draw: &crate::PrimitiveDraw) -> bool {
        self.world_clipping == crate::RenderWorldClippingBehavior::RasterClipDistanceV3
            && draw.topology() == crate::PrimitiveTopology::Lines
    }
    pub fn create_with_raster_policy(
        options: VulkanOptions,
        policy: crate::RenderRasterPolicy,
    ) -> Result<Arc<Self>, RenderError> {
        Self::create_with_conventions(options, policy, Default::default())
    }
    /// Publish one admitted immutable selection; `Self::create_with_conventions(opts, raster, coordinates)?`.
    pub fn create_with_conventions(
        options: VulkanOptions,
        policy: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
    ) -> Result<Arc<Self>, RenderError> {
        Self::create_with_line_clipping(options, policy, coordinates, Default::default())
    }
    pub fn create_with_line_clipping(
        options: VulkanOptions,
        policy: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        behavior: crate::RenderLineClippingBehavior,
    ) -> Result<Arc<Self>, RenderError> {
        let mut runtime = Self::configured_runtime(options, policy, coordinates)?;
        runtime.line_clipping = behavior;
        runtime.initialize_device(options.device_index)?;
        Ok(Arc::new(runtime))
    }
    pub fn admit_line_clipping(&self, registered_program: bool) -> Result<bool, RenderError> {
        self.line_clip.admit(registered_program).map_err(|reason| {
            self.line_clip
                .refusal(
                    self.line_clipping,
                    super::driver_identity::query_driver_identity(&self.instance, self.physical),
                    reason,
                )
                .into()
        })
    }
    pub(super) fn create_with_light_dots(
        options: VulkanOptions,
        policy: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        clipping: crate::RenderLineClippingBehavior,
        light_dots: crate::RenderLightDotBehavior,
    ) -> Result<Arc<Self>, RenderError> {
        let mut runtime = Self::configured_runtime(options, policy, coordinates)?;
        runtime.line_clipping = clipping;
        runtime.light_dot_behavior = light_dots;
        runtime.initialize_device(options.device_index)?;
        Ok(Arc::new(runtime))
    }
    pub(super) fn configured_runtime(
        options: VulkanOptions,
        policy: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
    ) -> Result<Self, RenderError> {
        let mut runtime = Self::load(options.validation)?;
        // Publication happens only after admission and complete device initialization.
        runtime.raster_policy = policy;
        runtime.attachment_coordinates = coordinates;
        if options.validation {
            runtime.messenger = Some(
                ValidationMessenger::new(&runtime.entry, &runtime.instance, &runtime.validation)
                    .map_err(|e| vk_error("create validation messenger", e))?,
            );
        }
        Ok(runtime)
    }
    /// Test control: a top-left runtime keeping the standard sample positions.
    /// Example: `VulkanRuntime::create_with_standard_samples(options)?`.
    #[cfg(test)]
    pub(super) fn create_with_standard_samples(
        options: VulkanOptions,
    ) -> Result<Arc<Self>, RenderError> {
        let mut runtime =
            Self::configured_runtime(options, Default::default(), Default::default())?;
        runtime.standard_samples = true;
        runtime.initialize_device(options.device_index)?;
        Ok(Arc::new(runtime))
    }
    #[cfg(test)]
    /// Admit one private capture context; `Self::create_for_executable(options)?`.
    pub(super) fn create_for_executable(
        options: VulkanOptions,
    ) -> Result<Arc<Self>, super::diagnostic_executable::ExecutableGap> {
        let mut runtime =
            Self::configured_runtime(options, Default::default(), Default::default())?;
        let (physical, family) = choose_device(&runtime.instance, options.device_index)?;
        runtime.executable_profile = Some(super::diagnostic_executable::admit_profile(
            &runtime.instance,
            physical,
        )?);
        runtime.initialize_selected_device(physical, family)?;
        Ok(Arc::new(runtime))
    }
    fn load(enable_validation: bool) -> Result<Self, RenderError> {
        #[cfg(feature = "raytrace-hit-diagnostics")]
        let ray_hit_control = super::raytrace_hit_control::RayHitControl::from_environment()?;
        #[cfg(feature = "lit-only-command-diagnostics")]
        let lit_only_commands = super::lit_only_commands::LitOnlyCommands::from_environment()?;
        let entry =
            // SAFETY: the Vulkan loader library is the system's, and loading it runs only its own initializers.
            unsafe { Entry::load() }.map_err(|e| rejected(e, "a loadable Vulkan 1.3 runtime"))?;
        let (instance, window_support) = create_instance(&entry, enable_validation)?;
        Ok(Self {
            entry,
            lifecycle: Default::default(),
            window_support,
            queues: vec![],
            displays: Default::default(),
            instance,
            messenger: None,
            device: None,
            validation: Arc::new(ValidationMessages::new(enable_validation)),
            physical: vk::PhysicalDevice::null(),
            queue: vk::Queue::null(),
            queue_family: 0,
            memory: Default::default(),
            limits: Default::default(),
            depth_clamp: false,
            depth: Default::default(),
            #[cfg(all(test, target_os = "linux"))]
            owned_v2_original_depth: None,
            non_seamless_cube: false,
            mirrored_samples: vk::SampleCountFlags::empty(),
            image_float_atomic_add: false,
            raytrace_textures: false,
            fragment_stores: false,
            #[cfg(feature = "raytrace-hit-diagnostics")]
            ray_hit_control,
            #[cfg(test)]
            standard_samples: false,
            logic_op: false,
            geometry_shader: false,
            tessellation_shader: false,
            polygon_lines: false,
            subset_point_polygons: None,
            large_points: false,
            sampler_anisotropy: false,
            sample_rate_shading: false,
            depth_resolve_modes: vk::ResolveModeFlags::empty(),
            stencil_resolve_modes: vk::ResolveModeFlags::empty(),
            lines: Default::default(),
            line_stipple: None,
            point_raster: Default::default(),
            thin_axis_coverage: Default::default(),
            thin_line_coverage: Default::default(),
            #[cfg(feature = "vertex-output-diagnostics")]
            vertex_output: None,
            #[cfg(feature = "lit-only-command-diagnostics")]
            lit_only_commands,
            line_clipping: Default::default(),
            light_dot_behavior: Default::default(),
            phong_interpolation: Default::default(),
            triangle_coverage: Default::default(),
            text_interpolation: Default::default(),
            depth_bias_behavior: Default::default(),
            projection_behavior: Default::default(),
            window_depth_behavior: Default::default(),
            depth_bias_command: None,
            line_clip: Default::default(),
            world_clipping: Default::default(),
            world_clip_interpolation: Default::default(),
            world_clip: Default::default(),
            name: String::new(),
            quad_diagonal: Default::default(),
            raster_policy: Default::default(),
            attachment_coordinates: Default::default(),
            image_memory: Default::default(),
            memory_arena: Default::default(),
            #[cfg(test)]
            executable_profile: None,
        })
    }
    pub fn device(&self) -> &Device {
        self.device.as_ref().expect("initialized Vulkan device")
    }
    pub(super) fn initialize_device(&mut self, index: Option<usize>) -> Result<(), RenderError> {
        let (physical, family) = choose_device(&self.instance, index)?;
        self.initialize_selected_device(physical, family)
    }
    fn initialize_selected_device(
        &mut self,
        physical: vk::PhysicalDevice,
        family: u32,
    ) -> Result<(), RenderError> {
        #[cfg(feature = "vertex-output-diagnostics")]
        if let Some(request) = &self.vertex_output {
            super::vertex_output::admit(&self.instance, physical, request)?;
        }
        super::depth_bias_device::admit(&self.instance, physical, self.depth_bias_behavior)?;
        super::window_depth_device::admit(&self.instance, physical, self.window_depth_behavior)?;
        #[cfg(feature = "vertex-output-diagnostics")]
        if super::window_depth_device::selected(self.window_depth_behavior)
            && self.vertex_output.is_some()
        {
            return Err(crate::RenderWindowDepthFailure::UnsupportedInterface {
                actual:
                    "vertex-output capture without the selected geometry-stage output interface"
                        .into(),
            }
            .into());
        }
        self.admit_phong_interpolation(physical)?;
        self.admit_triangle_coverage(physical)?;
        super::text_grid_interpolation_device::admit(
            &self.instance,
            physical,
            self.text_interpolation,
        )?;
        #[cfg(feature = "vertex-output-diagnostics")]
        if let Some(request) = &self.vertex_output {
            super::text_grid_interpolation_device::admit_text_feedback(
                self.text_interpolation,
                request.format(),
            )?;
        }
        self.initialize_device_properties(physical);
        self.initialize_core_features(physical);
        self.initialize_format_features(physical)?;
        self.world_clip
            .admit_device(self.world_clip_device_behavior())?;
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        self.memory = unsafe {
            self.instance
                .get_physical_device_memory_properties(physical)
        };
        self.window_support &= super::window_support::device_supported(&self.instance, physical)?;
        self.install_selected_device(physical, family)
    }
    fn admit_triangle_coverage(&self, physical: vk::PhysicalDevice) -> Result<(), RenderError> {
        #[cfg(feature = "vertex-output-diagnostics")]
        if self.triangle_coverage != crate::RenderTriangleCoverageBehavior::HardwareV1
            && self.vertex_output.is_some()
        {
            return Err(crate::RenderTriangleCoverageFailure::UnsupportedInterface {
                actual: "vertex-output diagnostic shader".into(),
            }
            .into());
        }
        super::triangle_grid_coverage_device::admit(
            &self.instance,
            physical,
            self.triangle_coverage,
            self.phong_interpolation,
        )
    }
    fn initialize_device_properties(&mut self, physical: vk::PhysicalDevice) {
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let properties = unsafe { self.instance.get_physical_device_properties(physical) };
        // SAFETY: the pointer names a live NUL-terminated buffer that outlives this view.
        self.name = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        self.limits = properties.limits;
        self.quad_diagonal = crate::quad_assembly::QuadDiagonal::for_vendor(properties.vendor_id);
    }
    fn initialize_core_features(&mut self, physical: vk::PhysicalDevice) {
        self.depth_clamp =
            // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
            unsafe { self.instance.get_physical_device_features(physical) }.depth_clamp == vk::TRUE;
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        self.large_points = unsafe { self.instance.get_physical_device_features(physical) }
            .large_points
            == vk::TRUE;
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        self.sampler_anisotropy = unsafe { self.instance.get_physical_device_features(physical) }
            .sampler_anisotropy
            == vk::TRUE;
        self.logic_op =
            // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
            unsafe { self.instance.get_physical_device_features(physical) }.logic_op == vk::TRUE;
    }
    fn initialize_format_features(
        &mut self,
        physical: vk::PhysicalDevice,
    ) -> Result<(), RenderError> {
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let native_features = unsafe { self.instance.get_physical_device_features(physical) };
        self.sample_rate_shading = native_features.sample_rate_shading == vk::TRUE;
        self.fragment_stores = native_features.fragment_stores_and_atomics == vk::TRUE;
        self.initialize_resolve_modes(physical);
        self.geometry_shader = native_features.geometry_shader == vk::TRUE;
        self.tessellation_shader = native_features.tessellation_shader == vk::TRUE;
        self.polygon_lines = native_features.fill_mode_non_solid == vk::TRUE;
        let subset = super::portability_runtime::query_device(&self.instance, physical)?;
        self.subset_point_polygons = subset.extension.map(|_| subset.point_polygons);
        self.lines = super::line_device::LineCapabilities::query(&self.instance, physical)?;
        self.line_clip = super::line_clip_device::LineClipDevice::query(
            &self.instance,
            physical,
            self.line_clipping,
        );
        self.world_clip = super::world_clip_device::WorldClipDevice::query(
            &self.instance,
            physical,
            self.world_clip_device_behavior(),
        );
        self.depth = super::depth_equivalence::DepthEquivalence::probe(&self.instance, physical);
        self.non_seamless_cube =
            super::cube_seams::non_seamless_cube_supported(&self.instance, physical);
        self.image_float_atomic_add = self.fragment_stores
            && super::image_float_atomics::image_float_atomic_add_supported(
                &self.instance,
                physical,
            );
        self.raytrace_textures = super::raytrace_texture_indexing::ray_tracing_textures_supported(
            &self.instance,
            physical,
        );
        self.mirrored_samples = super::mirrored_sample_locations::frame_counts(
            self.attachment_coordinates,
            super::mirrored_sample_locations::probe(&self.entry, &self.instance, physical),
        );
        #[cfg(test)]
        if self.standard_samples {
            self.mirrored_samples = vk::SampleCountFlags::empty();
        }
        Ok(())
    }
    fn initialize_resolve_modes(&mut self, physical: vk::PhysicalDevice) {
        let mut resolve = vk::PhysicalDeviceDepthStencilResolveProperties::default();
        let mut query = vk::PhysicalDeviceProperties2::default().push_next(&mut resolve);
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        unsafe {
            self.instance
                .get_physical_device_properties2(physical, &mut query);
        }
        self.depth_resolve_modes = resolve.supported_depth_resolve_modes;
        self.stencil_resolve_modes = resolve.supported_stencil_resolve_modes;
    }
    /// Keep child destruction behind actual idle/loss; `runtime.retire_device_blocking()` may block.
    pub fn retire_device_blocking(&self) {
        let mut calls = super::device_retirement::DeviceIdleCalls::new(self.device());
        let outcome = super::device_retirement::retire_until_stopped(&mut calls);
        let _ = self.lifecycle.observe(outcome);
    }
    pub fn memory_type(
        &self,
        bits: u32,
        flags: vk::MemoryPropertyFlags,
    ) -> Result<u32, RenderError> {
        super::memory_selection::compatible_memory_type(&self.memory, bits, flags)
            .ok_or_else(|| rejected((bits, flags), "a compatible Vulkan memory type"))
    }
}

fn native_line_width_supported(runtime: &VulkanRuntime, draw: &crate::PrimitiveDraw) -> bool {
    let Some(style) = draw.line_style else {
        return false;
    };
    let width = style.raster_width();
    width <= runtime.limits.line_width_range[1] && (width == 1. || runtime.lines.wide)
}
impl Drop for VulkanRuntime {
    fn drop(&mut self) {
        if let Some(device) = self.device.take() {
            // Explicit close may be incomplete; last-owner teardown cannot abandon execution.
            if self.lifecycle.state() != crate::RenderDeviceState::Closed {
                let mut calls = super::device_retirement::DeviceIdleCalls::new(&device);
                let _ = super::device_retirement::retire_until_stopped(&mut calls);
            }
            // Every lease holds this runtime, so all resources are gone; free the shared blocks.
            self.memory_arena.free_all(&device);
            // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
            unsafe {
                device.destroy_device(None);
            }
        }
        self.messenger.take();
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe { self.instance.destroy_instance(None) };
    }
}
/// Create the instance and report window admission; `create_instance(&entry, validation)?`.
fn create_instance(entry: &Entry, validation: bool) -> Result<(Instance, bool), RenderError> {
    let app = vk::ApplicationInfo::default()
        .application_name(c"Martensite")
        .api_version(vk::API_VERSION_1_3);
    let mut info = vk::InstanceCreateInfo::default().application_info(&app);
    let layers = [super::validation_settings::VALIDATION_LAYER.as_ptr()];
    let surfaces = super::surface_platform::host_factory();
    let plan = super::portability_runtime::instance_extensions(entry, validation, surfaces)?;
    let extensions = plan.names;
    info = info.flags(plan.flags).enabled_extension_names(&extensions);
    let settings = super::validation_settings::synchronization_settings();
    let mut layer_settings = vk::LayerSettingsCreateInfoEXT::default().settings(&settings);
    if validation {
        info = info
            .enabled_layer_names(&layers)
            .enabled_extension_names(&extensions)
            .push_next(&mut layer_settings);
    }
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    let instance = unsafe { entry.create_instance(&info, None) }
        .map_err(|e| vk_error("create Vulkan 1.3 instance", e))?;
    Ok((instance, plan.window_support))
}
fn choose_device(
    instance: &Instance,
    selected: Option<usize>,
) -> Result<(vk::PhysicalDevice, u32), RenderError> {
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let devices = unsafe { instance.enumerate_physical_devices() }
        .map_err(|e| vk_error("enumerate devices", e))?;
    for (index, physical) in devices.iter().enumerate() {
        if selected.is_some_and(|wanted| wanted != index) {
            continue;
        }
        if let Some(family) = capable_queue(instance, *physical) {
            return Ok((*physical, family));
        }
    }
    Err(rejected(
        selected,
        "a Vulkan 1.3 graphics device with dynamic rendering and synchronization2",
    ))
}
fn capable_queue(instance: &Instance, physical: vk::PhysicalDevice) -> Option<u32> {
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let properties = unsafe { instance.get_physical_device_properties(physical) };
    if properties.api_version < vk::API_VERSION_1_3 {
        return None;
    }
    let mut features = vk::PhysicalDeviceVulkan13Features::default();
    let mut query = vk::PhysicalDeviceFeatures2::default().push_next(&mut features);
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    unsafe { instance.get_physical_device_features2(physical, &mut query) };
    if features.dynamic_rendering == vk::FALSE || features.synchronization2 == vk::FALSE {
        return None;
    }
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    unsafe { instance.get_physical_device_queue_family_properties(physical) }
        .iter()
        .position(|family| {
            family.queue_count > 0 && family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
        })
        .map(|index| index as u32)
}
pub(super) fn vk_error(operation: &str, error: vk::Result) -> RenderError {
    let message = rejected(format!("{operation}: {error:?}"), "VK_SUCCESS").to_string();
    RenderError::device_backend(operation, error.as_raw(), message)
}
fn create_device(
    runtime: &VulkanRuntime,
    physical: vk::PhysicalDevice,
) -> Result<Device, RenderError> {
    let source = super::raster_policy_device::SelectedRasterCapabilitySource::new(
        &runtime.instance,
        physical,
    );
    crate::raster_policy_admission::create_after_raster_admission(
        runtime.raster_policy,
        &source,
        |admitted| super::device_configuration::create_admitted_device(runtime, physical, admitted),
    )
}

#[cfg(all(test, target_os = "linux"))]
#[path = "owned_sample_runtime.rs"]
mod owned_sample_carrier;

#[path = "runtime_render_behavior.rs"]
mod render_behavior;

#[path = "runtime_device_install.rs"]
mod device_install;
