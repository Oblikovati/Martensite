use ash::vk;

pub(super) fn shader_stages(
    shaders: &[vk::ShaderModule],
) -> Vec<vk::PipelineShaderStageCreateInfo<'static>> {
    let mut stages = vec![
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(shaders[0])
            .name(c"main"),
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(shaders[1])
            .name(c"main"),
    ];
    if shaders.len() == 3 {
        stages.push(
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::GEOMETRY)
                .module(shaders[2])
                .name(c"main"),
        );
    }
    stages
}
