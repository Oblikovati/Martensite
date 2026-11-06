//! GlPBREnvironment::Bake: processSpecIBLMap renders every specular level from the last
//! (roughest) to level 0, six sides each, then processDiffIBLMap renders the nine spherical
//! harmonics coefficients, all from the source cube map with PBREnvBaking.fs.
use super::super::environment_cube::EnvironmentCube;
use super::super::environment_cube::upload::cube_barrier;
use super::super::environment_pipeline::{
    EnvironmentPipeline, EnvironmentPipelineSpec, begin_texture_pass,
};
use super::super::runtime::VulkanRuntime;
use super::image::{EnvironmentImage, FaceView};
use crate::RenderError;
use crate::environment_frame::PbrBakingSamples;
use ash::vk;
use martensite_tkservice::graphic3d::Graphic3dPbrMaterial;
use std::sync::Arc;

const BAKING_VERTEX: &[u8] = include_bytes!("../../../shaders/pbr_env_baking.vert.spv");
const BAKING_DIFFUSE: &[u8] = include_bytes!("../../../shaders/pbr_env_baking.diffuse.frag.spv");
const BAKING_SPECULAR: &[u8] = include_bytes!("../../../shaders/pbr_env_baking.specular.frag.spv");

const BAKING_STAGES: vk::ShaderStageFlags = vk::ShaderStageFlags::from_raw(
    vk::ShaderStageFlags::VERTEX.as_raw() | vk::ShaderStageFlags::FRAGMENT.as_raw(),
);

/// The diffuse and specular programs of Graphic3dShaderManager::getPBREnvBakingProgram.
pub(in super::super) struct BakingPipelines {
    diffuse: EnvironmentPipeline,
    specular: EnvironmentPipeline,
}

impl BakingPipelines {
    /// `BakingPipelines::new(&runtime, cube_layout)?`.
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        layout: vk::DescriptorSetLayout,
    ) -> Result<Self, RenderError> {
        let spec = |fragment, color| EnvironmentPipelineSpec {
            shaders: [BAKING_VERTEX, fragment],
            topology: vk::PrimitiveTopology::TRIANGLE_STRIP,
            color,
            depth: vk::Format::UNDEFINED,
            stencil: vk::Format::UNDEFINED,
            samples: vk::SampleCountFlags::TYPE_1,
            push: vk::PushConstantRange::default()
                .stage_flags(BAKING_STAGES)
                .size(28),
            set_layout: Some(layout),
        };
        Ok(Self {
            diffuse: EnvironmentPipeline::new(
                runtime,
                spec(BAKING_DIFFUSE, vk::Format::R32G32B32A32_SFLOAT),
            )?,
            specular: EnvironmentPipeline::new(
                runtime,
                spec(BAKING_SPECULAR, vk::Format::R8G8B8A8_UNORM),
            )?,
        })
    }
}

/// The PbrEnvBakingConstants block (uCurrentSide, uYCoeff, uZCoeff, uSamplesNum,
/// uCurrentLevel, occNbSpecIBLLevels, uEnvSolidAngleSource).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct BakingConstants {
    pub side: i32,
    pub coefficients: [i32; 2],
    pub samples: i32,
    pub level: i32,
    pub levels: i32,
    pub solid_angle_source: f32,
}

impl BakingConstants {
    fn bytes(&self) -> [u8; 28] {
        let words = [
            self.side.to_ne_bytes(),
            self.coefficients[0].to_ne_bytes(),
            self.coefficients[1].to_ne_bytes(),
            self.samples.to_ne_bytes(),
            self.level.to_ne_bytes(),
            self.levels.to_ne_bytes(),
            self.solid_angle_source.to_ne_bytes(),
        ];
        super::super::environment_pipeline::push_constant_bytes(words)
    }
}

/// `float(4.0 * M_PI / (6.0 * float(EnvMapSize * EnvMapSize)))` of processSpecIBLMap.
pub(super) fn solid_angle_source(env_map_size: u32) -> f32 {
    let texels = (u64::from(env_map_size) * u64::from(env_map_size)) as f32;
    (4.0 * std::f64::consts::PI / (6.0 * f64::from(texels))) as f32
}

/// `int(SpecIBLMapSamplesFactor(Probability, level / float(levels - 1)) * NbSpecSamples)`.
pub(super) fn specular_samples(samples: &PbrBakingSamples, level: u32, levels: u32) -> i32 {
    let roughness = level as f32 / (levels - 1) as f32;
    (Graphic3dPbrMaterial::spec_ibl_map_samples_factor(samples.probability, roughness)
        * samples.specular as f32) as i32
}

/// Inputs of one bake recording.
pub(super) struct BakeTargets<'a> {
    pub specular: &'a EnvironmentImage,
    pub specular_faces: &'a [FaceView],
    pub diffuse: &'a EnvironmentImage,
    pub source: &'a EnvironmentCube,
    pub coefficients: [i32; 2],
    pub samples: PbrBakingSamples,
}

/// Record processSpecIBLMap then processDiffIBLMap; both maps end shader-readable.
/// Example: `record_bake(device, command, &pipelines, &targets)`.
pub(super) fn record_bake(
    device: &ash::Device,
    command: vk::CommandBuffer,
    pipelines: &BakingPipelines,
    targets: &BakeTargets<'_>,
) {
    let levels = targets.specular.spec.levels;
    let read = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
    let attachment = vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL;
    cube_barrier(
        device,
        command,
        targets.specular,
        0..levels,
        read,
        attachment,
    );
    cube_barrier(device, command, targets.diffuse, 0..1, read, attachment);
    let base = BakingConstants {
        coefficients: targets.coefficients,
        levels: levels as i32,
        solid_angle_source: solid_angle_source(targets.source.size()),
        ..Default::default()
    };
    for level in (0..levels).rev() {
        let constants = BakingConstants {
            samples: specular_samples(&targets.samples, level, levels),
            level: level as i32,
            ..base
        };
        record_specular_level(device, command, pipelines, targets, constants);
    }
    let diffuse = BakingConstants {
        samples: targets.samples.diffuse as i32,
        ..base
    };
    record_pass(
        device,
        command,
        &pipelines.diffuse,
        targets,
        (targets.diffuse.view, [9, 1]),
        diffuse,
    );
    cube_barrier(
        device,
        command,
        targets.specular,
        0..levels,
        attachment,
        read,
    );
    cube_barrier(device, command, targets.diffuse, 0..1, attachment, read);
}

fn record_specular_level(
    device: &ash::Device,
    command: vk::CommandBuffer,
    pipelines: &BakingPipelines,
    targets: &BakeTargets<'_>,
    constants: BakingConstants,
) {
    for side in 0..6 {
        let face = &targets.specular_faces[constants.level as usize * 6 + side];
        let constants = BakingConstants {
            side: side as i32,
            ..constants
        };
        let target = (face.handle, [face.extent; 2]);
        record_pass(
            device,
            command,
            &pipelines.specular,
            targets,
            target,
            constants,
        );
    }
}

fn record_pass(
    device: &ash::Device,
    command: vk::CommandBuffer,
    pipeline: &EnvironmentPipeline,
    targets: &BakeTargets<'_>,
    (view, extent): (vk::ImageView, [u32; 2]),
    constants: BakingConstants,
) {
    begin_texture_pass(device, command, view, extent);
    pipeline.bind(
        device,
        command,
        Some(targets.source.set),
        BAKING_STAGES,
        &constants.bytes(),
    );
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        device.cmd_draw(command, 4, 1, 0, 0);
        device.cmd_end_rendering(command);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solid_angle_follows_the_native_double_expression() {
        let expected = (4.0 * std::f64::consts::PI / (6.0 * 512.0 * 512.0)) as f32;
        assert_eq!(solid_angle_source(512), expected);
    }

    #[test]
    fn specular_samples_scale_with_the_native_factor() {
        let samples = PbrBakingSamples {
            diffuse: 1024,
            specular: 256,
            probability: 0.99,
        };
        let top = specular_samples(&samples, 5, 6);
        let factor = Graphic3dPbrMaterial::spec_ibl_map_samples_factor(0.99, 1.0);
        assert_eq!(top, (factor * 256.0) as i32);
        assert!(specular_samples(&samples, 1, 6) <= top);
    }

    #[test]
    fn constants_follow_the_block_layout() {
        let constants = BakingConstants {
            side: 3,
            coefficients: [-1, 1],
            samples: 64,
            level: 2,
            levels: 6,
            solid_angle_source: 0.5,
        };
        let bytes = constants.bytes();
        let int = |i: usize| i32::from_ne_bytes(bytes[4 * i..4 * i + 4].try_into().unwrap());
        assert_eq!(
            [int(0), int(1), int(2), int(3), int(4), int(5)],
            [3, -1, 1, 64, 2, 6]
        );
        assert_eq!(f32::from_ne_bytes(bytes[24..28].try_into().unwrap()), 0.5);
    }
}
