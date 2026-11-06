//! Lexical feature owners keep every pNext node alive through device creation.
use super::runtime::{VulkanRuntime, vk_error};
use crate::{RenderError, RenderRasterPolicy};
use ash::{Device, vk};

/// Create only after capability admission; `create_admitted_device(runtime, physical, policy)?`.
pub(super) fn create_admitted_device(
    runtime: &VulkanRuntime,
    physical: vk::PhysicalDevice,
    policy: RenderRasterPolicy,
) -> Result<Device, RenderError> {
    let portability = super::portability_runtime::query_device(&runtime.instance, physical)?;
    let queues = device_queue_infos(runtime, physical);
    let core = requested_core_features(runtime, physical);
    let mut extensions = device_extension_names(runtime, policy);
    extensions.extend(portability.extension.map(|name| name.as_ptr()));
    let mut subset = portability.requested_features();
    let mut chain = DeviceExtensionChain::new(runtime, policy);
    let info = chain.description(&queues, &core, &extensions);
    let info = super::portability_runtime::link_subset(info, &mut subset);
    // The borrowed queues, features, extension names and chain outlive this call.
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe { runtime.instance.create_device(physical, &info, None) }
        .map_err(|error| vk_error("create device", error))
}
fn device_queue_infos(
    runtime: &VulkanRuntime,
    physical: vk::PhysicalDevice,
) -> Vec<vk::DeviceQueueCreateInfo<'static>> {
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let families = unsafe {
        runtime
            .instance
            .get_physical_device_queue_family_properties(physical)
    };
    (0..families.len())
        .map(|index| {
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(index as u32)
                .queue_priorities(&[1.0])
        })
        .collect()
}
fn requested_core_features(
    runtime: &VulkanRuntime,
    physical: vk::PhysicalDevice,
) -> vk::PhysicalDeviceFeatures {
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let supported = unsafe { runtime.instance.get_physical_device_features(physical) };
    vk::PhysicalDeviceFeatures::default()
        .sample_rate_shading(supported.sample_rate_shading == vk::TRUE)
        .sampler_anisotropy(supported.sampler_anisotropy == vk::TRUE)
        // Adaptive path tracing stores into images from fragment shaders.
        .fragment_stores_and_atomics(supported.fragment_stores_and_atomics == vk::TRUE)
        .texture_compression_bc(supported.texture_compression_bc == vk::TRUE)
        .depth_clamp(runtime.depth_clamp)
        .large_points(runtime.large_points)
        .wide_lines(runtime.lines.wide)
        .shader_clip_distance(runtime.line_clip.enabled() || runtime.world_clip.enabled())
        .logic_op(runtime.logic_op)
        .geometry_shader(supported.geometry_shader == vk::TRUE)
        .vertex_pipeline_stores_and_atomics(super::window_depth_device::fragment_selected(
            runtime.window_depth_behavior,
        ))
        .tessellation_shader(supported.tessellation_shader == vk::TRUE)
        .fill_mode_non_solid(supported.fill_mode_non_solid == vk::TRUE)
        .shader_float64(super::window_depth_device::float64_selected(
            runtime.window_depth_behavior,
        ))
        .shader_int64(
            supported.shader_int64 == vk::TRUE
                && (runtime.triangle_coverage != crate::RenderTriangleCoverageBehavior::HardwareV1
                    || super::window_depth_device::selected(runtime.window_depth_behavior)),
        )
}
fn device_extension_names(
    runtime: &VulkanRuntime,
    policy: RenderRasterPolicy,
) -> Vec<*const std::ffi::c_char> {
    let mut extensions = Vec::new();
    if let Some(name) = runtime.lines.extension.filter(|_| runtime.lines.bresenham) {
        extensions.push(name.as_ptr());
    }
    append_window_extensions(runtime, &mut extensions);
    if policy.requires_last_vertex() {
        extensions.push(ash::ext::provoking_vertex::NAME.as_ptr());
    }
    if runtime.depth.clip_control {
        extensions.push(ash::ext::depth_clip_control::NAME.as_ptr());
    }
    if runtime.depth_bias_behavior == crate::RenderDepthBiasBehavior::Native24UnitsV2 {
        extensions.push(ash::ext::depth_bias_control::NAME.as_ptr());
    }
    if runtime.non_seamless_cube {
        extensions.push(ash::ext::non_seamless_cube_map::NAME.as_ptr());
    }
    if !runtime.mirrored_samples.is_empty() {
        extensions.push(ash::ext::sample_locations::NAME.as_ptr());
    }
    if runtime.image_float_atomic_add {
        extensions.push(ash::ext::shader_atomic_float::NAME.as_ptr());
    }
    #[cfg(test)]
    if runtime.executable_profile.is_some() {
        extensions.push(ash::khr::pipeline_executable_properties::NAME.as_ptr());
    }
    #[cfg(feature = "vertex-output-diagnostics")]
    if runtime.vertex_output.is_some() {
        extensions.push(ash::ext::transform_feedback::NAME.as_ptr());
        if !policy.requires_last_vertex() {
            extensions.push(ash::ext::provoking_vertex::NAME.as_ptr());
        }
    }
    if runtime.phong_interpolation != crate::RenderPhongInterpolationBehavior::HardwareV1
        || super::text_grid_interpolation_device::requires_barycentric(runtime.text_interpolation)
    {
        extensions.push(ash::khr::fragment_shader_barycentric::NAME.as_ptr());
    }
    if runtime.triangle_coverage != crate::RenderTriangleCoverageBehavior::HardwareV1 {
        extensions.push(ash::ext::conservative_rasterization::NAME.as_ptr());
    }
    extensions
}
struct DeviceExtensionChain {
    depth_bias: vk::PhysicalDeviceDepthBiasControlFeaturesEXT<'static>,
    interpolation: vk::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'static>,
    #[cfg(feature = "vertex-output-diagnostics")]
    transform_feedback: vk::PhysicalDeviceTransformFeedbackFeaturesEXT<'static>,
    #[cfg(test)]
    executable: vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'static>,
    core13: vk::PhysicalDeviceVulkan13Features<'static>,
    core12: vk::PhysicalDeviceVulkan12Features<'static>,
    window: vk::PhysicalDeviceSwapchainMaintenance1FeaturesEXT<'static>,
    lines: vk::PhysicalDeviceLineRasterizationFeaturesKHR<'static>,
    provoking: vk::PhysicalDeviceProvokingVertexFeaturesEXT<'static>,
    clip_control: vk::PhysicalDeviceDepthClipControlFeaturesEXT<'static>,
    cube_seams: vk::PhysicalDeviceNonSeamlessCubeMapFeaturesEXT<'static>,
    float_atomics: vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT<'static>,
}
impl DeviceExtensionChain {
    fn new(runtime: &VulkanRuntime, policy: RenderRasterPolicy) -> Self {
        Self {
            depth_bias: super::depth_bias_device::feature(runtime.depth_bias_behavior),
            interpolation: vk::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR::default()
                .fragment_shader_barycentric(
                    runtime.phong_interpolation
                        != crate::RenderPhongInterpolationBehavior::HardwareV1
                        || super::text_grid_interpolation_device::requires_barycentric(
                            runtime.text_interpolation,
                        ),
                ),
            #[cfg(feature = "vertex-output-diagnostics")]
            transform_feedback: vk::PhysicalDeviceTransformFeedbackFeaturesEXT::default()
                .transform_feedback(runtime.vertex_output.is_some()),
            #[cfg(test)]
            executable: vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR::default()
                .pipeline_executable_info(runtime.executable_profile.is_some()),
            core13: vk::PhysicalDeviceVulkan13Features::default()
                .dynamic_rendering(true)
                .synchronization2(true)
                // Required by Vulkan 1.3; the shader grid keeps OpenGL discard derivatives.
                .shader_demote_to_helper_invocation(true),
            core12: super::raytrace_texture_indexing::ray_tracing_textures_feature(
                runtime.raytrace_textures,
            ),
            window: vk::PhysicalDeviceSwapchainMaintenance1FeaturesEXT::default()
                .swapchain_maintenance1(runtime.window_support),
            lines: vk::PhysicalDeviceLineRasterizationFeaturesKHR::default()
                .bresenham_lines(runtime.lines.bresenham)
                .stippled_bresenham_lines(runtime.lines.stippled)
                .rectangular_lines(runtime.lines.rectangular)
                .stippled_rectangular_lines(runtime.lines.stippled_rectangular),
            provoking: vk::PhysicalDeviceProvokingVertexFeaturesEXT::default()
                .provoking_vertex_last(policy.requires_last_vertex()),
            clip_control: vk::PhysicalDeviceDepthClipControlFeaturesEXT::default()
                .depth_clip_control(runtime.depth.clip_control),
            cube_seams: vk::PhysicalDeviceNonSeamlessCubeMapFeaturesEXT::default()
                .non_seamless_cube_map(runtime.non_seamless_cube),
            float_atomics: super::image_float_atomics::image_float_atomic_add_feature(
                runtime.image_float_atomic_add,
            ),
        }
    }
    fn description<'a>(
        &'a mut self,
        queues: &'a [vk::DeviceQueueCreateInfo<'a>],
        core: &'a vk::PhysicalDeviceFeatures,
        extensions: &'a [*const std::ffi::c_char],
    ) -> vk::DeviceCreateInfo<'a> {
        let mut info =
            device_create_description(queues, core, extensions).push_next(&mut self.core13);
        if self.core12.shader_sampled_image_array_non_uniform_indexing == vk::TRUE {
            info = info.push_next(&mut self.core12);
        }
        if self.interpolation.fragment_shader_barycentric == vk::TRUE {
            info = info.push_next(&mut self.interpolation);
        }
        if self.depth_bias.depth_bias_control == vk::TRUE {
            info = info.push_next(&mut self.depth_bias);
        }
        info = link_window_feature(info, &mut self.window);
        if self.lines.bresenham_lines == vk::TRUE {
            info = info.push_next(&mut self.lines);
        }
        #[cfg(feature = "vertex-output-diagnostics")]
        if self.transform_feedback.transform_feedback == vk::TRUE {
            self.provoking.transform_feedback_preserves_provoking_vertex = vk::TRUE;
            info = info.push_next(&mut self.transform_feedback);
        }
        info = link_provoking_feature(info, &mut self.provoking);
        if self.clip_control.depth_clip_control == vk::TRUE {
            info = info.push_next(&mut self.clip_control);
        }
        if self.cube_seams.non_seamless_cube_map == vk::TRUE {
            info = info.push_next(&mut self.cube_seams);
        }
        if self.float_atomics.shader_image_float32_atomic_add == vk::TRUE {
            info = info.push_next(&mut self.float_atomics);
        }
        #[cfg(test)]
        if self.executable.pipeline_executable_info == vk::TRUE {
            info = info.push_next(&mut self.executable);
        }
        info
    }
}

fn device_create_description<'a>(
    queues: &'a [vk::DeviceQueueCreateInfo<'a>],
    core: &'a vk::PhysicalDeviceFeatures,
    extensions: &'a [*const std::ffi::c_char],
) -> vk::DeviceCreateInfo<'a> {
    vk::DeviceCreateInfo::default()
        .enabled_features(core)
        .queue_create_infos(queues)
        .enabled_extension_names(extensions)
}

fn append_window_extensions(
    runtime: &VulkanRuntime,
    extensions: &mut Vec<*const std::ffi::c_char>,
) {
    if runtime.window_support {
        extensions.extend([
            ash::khr::swapchain::NAME.as_ptr(),
            ash::ext::swapchain_maintenance1::NAME.as_ptr(),
        ]);
    }
}
fn link_window_feature<'a>(
    info: vk::DeviceCreateInfo<'a>,
    feature: &'a mut vk::PhysicalDeviceSwapchainMaintenance1FeaturesEXT<'static>,
) -> vk::DeviceCreateInfo<'a> {
    if feature.swapchain_maintenance1 == vk::TRUE {
        return info.push_next(feature);
    }
    info
}
fn link_provoking_feature<'a>(
    info: vk::DeviceCreateInfo<'a>,
    feature: &'a mut vk::PhysicalDeviceProvokingVertexFeaturesEXT<'static>,
) -> vk::DeviceCreateInfo<'a> {
    if feature.provoking_vertex_last == vk::TRUE
        || feature.transform_feedback_preserves_provoking_vertex == vk::TRUE
    {
        return info.push_next(feature);
    }
    info
}

#[cfg(all(test, feature = "vertex-output-diagnostics"))]
mod vertex_output_feature_tests {
    use super::*;
    #[test]
    fn vertex_output_preservation_is_linked_without_last_vertex_policy_change() {
        let mut feature = vk::PhysicalDeviceProvokingVertexFeaturesEXT::default()
            .transform_feedback_preserves_provoking_vertex(true)
            .provoking_vertex_last(false);
        let info = link_provoking_feature(vk::DeviceCreateInfo::default(), &mut feature);
        assert!(!info.p_next.is_null());
        assert_eq!(feature.provoking_vertex_last, vk::FALSE);
        assert_eq!(
            feature.transform_feedback_preserves_provoking_vertex,
            vk::TRUE
        );
    }
}
