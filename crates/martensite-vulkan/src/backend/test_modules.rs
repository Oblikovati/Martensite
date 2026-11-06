// Test declarations retain backend scope and existing exact test names.
#[cfg(test)]
mod coordinate_convention_device_tests;
#[cfg(test)]
mod depth_bias_gpu_tests;
#[cfg(test)]
mod depth_plane_operand_gpu_tests;
#[cfg(test)]
mod mirrored_sample_locations_device_tests;
#[cfg(test)]
mod oit_msaa_composite_tests;
#[cfg(test)]
mod oit_resource_tests;
#[cfg(all(test, target_os = "linux"))]
mod owned_sample_gpu_tests;
#[cfg(all(test, target_os = "linux"))]
mod owned_sample_inputs;
#[cfg(all(test, target_os = "linux"))]
mod owned_sample_json;
#[cfg(all(test, target_os = "linux"))]
mod owned_sample_metadata;
#[cfg(all(test, target_os = "linux"))]
mod owned_sample_readback;
#[cfg(test)]
mod persistent_retirement_tests;
#[cfg(test)]
mod projected_transport_tests;
#[cfg(test)]
mod resource_lifecycle_gpu_tests;
#[cfg(test)]
mod resource_lifecycle_test_support;
#[cfg(test)]
mod resource_lifecycle_upload_gpu_tests;
#[cfg(test)]
mod text_flat_corners_gpu_tests;
#[cfg(test)]
mod text_flat_corners_reference;
#[cfg(test)]
mod text_grid_interpolation_gpu_tests;
#[cfg(test)]
mod triangle_grid_coverage_gpu_tests;
#[cfg(test)]
mod triangle_grid_facing_gpu_tests;
#[cfg(test)]
mod viewport_lighting_tests;
#[cfg(test)]
mod window_depth_float64_gpu_tests;
#[cfg(test)]
mod window_depth_gpu_tests;
#[cfg(test)]
mod window_gui_contract_tests;
#[cfg(all(test, target_os = "linux"))]
mod window_gui_native_tests;

#[cfg(test)]
#[path = "encoded_resolve_gpu_tests.rs"]
mod encoded_resolve_gpu_tests;

#[cfg(test)]
mod colorless_resolve_gpu_tests;
#[cfg(all(test, target_os = "linux"))]
mod raytrace_pixel_control;
