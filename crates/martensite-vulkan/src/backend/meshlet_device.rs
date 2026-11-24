//! Mesh shader qualification stays optional on every Vulkan device (#594).
use ash::{Instance, vk};

pub(super) fn meshlet_limit(instance: &Instance, physical: vk::PhysicalDevice) -> u32 {
    // SAFETY: instance and physical were selected together and enumeration returns owned names.
    let extensions = unsafe { instance.enumerate_device_extension_properties(physical) };
    let Ok(extensions) = extensions else { return 0 };
    if !extensions
        .iter()
        .any(|extension| extension.extension_name_as_c_str() == Ok(ash::ext::mesh_shader::NAME))
    {
        return 0;
    }
    let mut mesh = vk::PhysicalDeviceMeshShaderFeaturesEXT::default();
    let mut features = vk::PhysicalDeviceFeatures2::default().push_next(&mut mesh);
    // SAFETY: every linked feature node is initialized and lives through the query.
    unsafe { instance.get_physical_device_features2(physical, &mut features) };
    if mesh.mesh_shader != vk::TRUE {
        return 0;
    }
    meshlet_workgroup_limit(instance, physical)
}

fn meshlet_workgroup_limit(instance: &Instance, physical: vk::PhysicalDevice) -> u32 {
    let mut mesh = vk::PhysicalDeviceMeshShaderPropertiesEXT::default();
    let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut mesh);
    // SAFETY: the mesh property structure remains live through this synchronous query.
    unsafe { instance.get_physical_device_properties2(physical, &mut properties) };
    if !descriptor_limits(&properties.properties.limits) {
        return 0;
    }
    qualified_limit(&mesh)
}

fn descriptor_limits(limits: &vk::PhysicalDeviceLimits) -> bool {
    limits.max_bound_descriptor_sets >= 8
        && limits.max_per_stage_descriptor_storage_buffers >= 2
        && limits.max_descriptor_set_storage_buffers >= 2
}

fn qualified_limit(mesh: &vk::PhysicalDeviceMeshShaderPropertiesEXT<'_>) -> u32 {
    let vertex_granularity = u64::from(mesh.mesh_output_per_vertex_granularity.max(1));
    let primitive_granularity = u64::from(mesh.mesh_output_per_primitive_granularity.max(1));
    let vertices = 64_u64.next_multiple_of(vertex_granularity);
    let primitives = 96_u64.next_multiple_of(primitive_granularity);
    if mesh.max_mesh_output_vertices < 64
        || mesh.max_mesh_output_primitives < 96
        || mesh.max_mesh_work_group_invocations < 64
        || mesh.max_mesh_work_group_size[0] < 64
        || mesh.max_mesh_output_components < 76
        || u64::from(mesh.max_mesh_output_memory_size) < vertices * 42 * 4 + primitives * 3 * 4
    {
        return 0;
    }
    mesh.max_mesh_work_group_count[0].min(mesh.max_mesh_work_group_total_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparse_meshlet_layout_checks_total_sets_and_per_stage_storage() {
        let mut limits = vk::PhysicalDeviceLimits {
            max_bound_descriptor_sets: 8,
            max_per_stage_descriptor_storage_buffers: 2,
            max_descriptor_set_storage_buffers: 2,
            ..Default::default()
        };
        assert!(descriptor_limits(&limits));
        limits.max_bound_descriptor_sets = 7;
        assert!(!descriptor_limits(&limits));
        limits.max_bound_descriptor_sets = 8;
        limits.max_per_stage_descriptor_storage_buffers = 1;
        assert!(!descriptor_limits(&limits));
    }

    #[test]
    fn mesh_cap_limits_workgroups_and_rejects_small_outputs() {
        let mut mesh = vk::PhysicalDeviceMeshShaderPropertiesEXT {
            max_mesh_output_vertices: 64,
            max_mesh_output_primitives: 96,
            max_mesh_work_group_invocations: 64,
            max_mesh_work_group_size: [64, 1, 1],
            max_mesh_output_components: 128,
            max_mesh_output_memory_size: 32_768,
            max_mesh_work_group_count: [1024, 1, 1],
            max_mesh_work_group_total_count: 500,
            ..Default::default()
        };
        assert_eq!(qualified_limit(&mesh), 500);
        mesh.max_mesh_output_primitives = 95;
        assert_eq!(qualified_limit(&mesh), 0);
    }
}
