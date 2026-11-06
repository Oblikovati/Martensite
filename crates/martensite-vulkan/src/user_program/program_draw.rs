//! One draw of a registered user program: its SPIR-V stages and the scene state of its
//! default uniform block, attached to the face draws a program-bearing aspect lowers to. The
//! view's lights reach the draws after lowering (`retained_lighting::apply_view_lights`), so
//! the block is filled at upload.
use super::library::UserProgramModules;
use super::spirv_reflect::UniformMember;
use super::uniform_values::{
    UserUniformLight, UserUniformScene, fill_uniform_block, uniform_block_size,
};
use std::sync::Arc;

/// The program and uniform scene of one draw; `UserProgramDraw::new(modules, scene)`.
#[derive(Clone, Debug, PartialEq)]
pub struct UserProgramDraw {
    /// The registered stages.
    pub modules: Arc<UserProgramModules>,
    /// Everything but the lights, which come from the draw's lighting at upload.
    pub scene: UserUniformScene,
    /// occTexCoord per vertex: the primitive's own texture coordinates, untransformed (the
    /// native program reads the attribute array, not the built-in texture matrix); empty when
    /// the primitive has none, which reads GL's generic attribute (0, 0, 0, 1).
    pub tex_coords: Vec<[f32; 4]>,
    /// Whether the program's one colour reaches every attachment of a multi-attachment pass,
    /// as native `gl_FragColor` does (`broadcasts_frag_color`); false for declared outputs.
    pub broadcasts_color: bool,
}

impl PartialEq for UserProgramModules {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl UserProgramDraw {
    /// A draw of `modules` reading `scene`; `UserProgramDraw::new(modules, scene)`.
    pub fn new(modules: Arc<UserProgramModules>, scene: UserUniformScene) -> Self {
        Self {
            modules,
            scene,
            tex_coords: Vec::new(),
            broadcasts_color: false,
        }
    }
    /// The std140 default uniform block with the view's `lights` (none for an unlit draw);
    /// `program.uniform_bytes(draw.lighting.as_ref().map(|l| &l.lights))`.
    pub(crate) fn uniform_bytes(&self, lights: Option<&crate::RenderLightSet>) -> Vec<u8> {
        let mut scene = self.scene.clone();
        if let Some(lights) = lights {
            scene.light_ambient = lights.ambient;
            scene.lights = lights.sources.iter().map(packed_light).collect();
        }
        let members = block_members(&self.modules);
        fill_uniform_block(members, uniform_block_size(members), &scene)
    }
}

/// GlShaderLightParameters rows (Color, Position, Direction, Parameters) and the type.
fn packed_light(light: &crate::light_snapshot::PackedRenderLight) -> UserUniformLight {
    let rows = [light.color, light.position, light.direction, light.params];
    (rows, light.kind as i32)
}

/// The members of the program's default uniform block (one linked layout across stages).
fn block_members(modules: &UserProgramModules) -> &[UniformMember] {
    modules
        .stages
        .iter()
        .find_map(|(reflection, _)| reflection.uniform_block.as_ref())
        .map_or(&[], |(_, _, members)| members.as_slice())
}
