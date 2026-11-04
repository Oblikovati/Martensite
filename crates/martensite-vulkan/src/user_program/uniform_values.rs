//! The per-draw default uniform block of a user program, filled by member name from the scene
//! state (the native occ* matrices, lights, material, colour and clip planes, packed as
//! GlShaderManager pushes them) and from the program's own variables. Members the
//! renderer does not know stay zero, as unset GLSL uniforms read zero.
use super::spirv_reflect::{UniformMember, UniformScalar};
use martensite_tkernel::ncollection::Mat4;

/// Flat uniform values: elements, then columns, then rows (column-major).
#[derive(Clone, Debug, PartialEq)]
pub enum UserUniformValues {
    /// Float components (also matrices).
    Float(Vec<f32>),
    /// Integer components (also booleans).
    Int(Vec<i32>),
}

/// One light as the native occLightSources rows (colour+intensity, position, spot direction +
/// range, parameters) and its occLightSourcesTypes value.
pub type UserUniformLight = ([[f32; 4]; 4], i32);

/// The scene state a user program reads through the native occ* uniforms.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UserUniformScene {
    /// occModelWorldMatrix, column-major.
    pub model_world: [f32; 16],
    /// occWorldViewMatrix, column-major.
    pub world_view: [f32; 16],
    /// occProjectionMatrix, column-major.
    pub projection: [f32; 16],
    /// occLightAmbient.
    pub light_ambient: [f32; 4],
    /// Enabled non-ambient lights in native order.
    pub lights: Vec<UserUniformLight>,
    /// THE_MAX_LIGHTS of the program: the length of occLightSourcesTypes (unused entries -1).
    pub max_lights: usize,
    /// occCommonMaterial: front then back diffuse, emission, specular+shininess, ambient.
    pub material: [[f32; 4]; 8],
    /// occColor.
    pub color: [f32; 4],
    /// World clip plane equations with their native chain values (planes left in the chain).
    pub clip_planes: Vec<([f32; 4], i32)>,
    /// The program's custom variables (Graphic3dShaderProgram::Variables), by name.
    pub variables: Vec<(String, UserUniformValues)>,
    /// occTexTrsf2d: translation and scale, then rotation sine and cosine of the first texture
    /// (GlContext::SetTextureMatrix); zero without a texture.
    pub texture_trsf: [[f32; 4]; 2],
}

const MATRIX_SUFFIXES: [&str; 4] = ["", "Inverse", "Transpose", "InverseTranspose"];

impl UserUniformScene {
    /// The values of the uniform `name`, or `None` when the renderer does not supply it;
    /// `scene.values("occColor")`.
    pub fn values(&self, name: &str) -> Option<UserUniformValues> {
        let variable = self.variables.iter().find(|(known, _)| known == name);
        if let Some((_, values)) = variable {
            return Some(values.clone());
        }
        self.matrix_values(name).or_else(|| self.state_values(name))
    }
    fn matrix_values(&self, name: &str) -> Option<UserUniformValues> {
        let bases = [
            ("occModelWorldMatrix", &self.model_world),
            ("occWorldViewMatrix", &self.world_view),
            ("occProjectionMatrix", &self.projection),
        ];
        let (base, matrix) = bases.into_iter().find(|(base, _)| name.starts_with(base))?;
        let suffix = &name[base.len()..];
        MATRIX_SUFFIXES
            .contains(&suffix)
            .then(|| UserUniformValues::Float(matrix_variant(matrix, suffix).to_vec()))
    }
    fn state_values(&self, name: &str) -> Option<UserUniformValues> {
        let floats = |values: Vec<f32>| Some(UserUniformValues::Float(values));
        match name {
            "occLightAmbient" => floats(self.light_ambient.to_vec()),
            "occLightSources" => floats(self.lights.iter().flat_map(|l| l.0).flatten().collect()),
            "occLightSourcesCount" => Some(UserUniformValues::Int(vec![self.lights.len() as i32])),
            "occLightSourcesTypes" => Some(UserUniformValues::Int(self.light_types())),
            "occCommonMaterial" => floats(self.material.iter().flatten().copied().collect()),
            "occColor" => floats(self.color.to_vec()),
            "occTexTrsf2d" => floats(self.texture_trsf.concat()),
            "occClipPlaneEquations" => floats(self.clip_planes.iter().flat_map(|p| p.0).collect()),
            "occClipPlaneChains" => Some(UserUniformValues::Int(
                self.clip_planes.iter().map(|plane| plane.1).collect(),
            )),
            "occClipPlaneCount" => {
                Some(UserUniformValues::Int(vec![self.clip_planes.len() as i32]))
            }
            _ => None,
        }
    }
    /// The light types padded with -1 up to THE_MAX_LIGHTS (GlShaderManager).
    fn light_types(&self) -> Vec<i32> {
        let mut types: Vec<i32> = self.lights.iter().map(|light| light.1).collect();
        types.resize(self.max_lights.max(types.len()), -1);
        types
    }
}

/// `matrix` or its float inverse, transpose or inverse transpose, as GlShaderManager
/// derives them (NCollectionMat4<float>::Inverted; a singular matrix keeps zeros).
fn matrix_variant(matrix: &[f32; 16], suffix: &str) -> [f32; 16] {
    let source = Mat4::<f32>::from_data(*matrix);
    let inverse = || source.inverted().unwrap_or_else(|_| Mat4::zero());
    let variant = match suffix {
        "Inverse" => inverse(),
        "Transpose" => source.transposed(),
        "InverseTranspose" => inverse().transposed(),
        _ => source,
    };
    variant.data()
}

/// The std140 bytes of a default uniform block of `size` bytes with `members` filled from
/// `scene`; `fill_uniform_block(&members, 256, &scene)`.
pub fn fill_uniform_block(
    members: &[UniformMember],
    size: usize,
    scene: &UserUniformScene,
) -> Vec<u8> {
    let mut bytes = vec![0; size];
    for member in members {
        if let Some(values) = scene.values(&member.name) {
            write_member(&mut bytes, member, &values);
        }
    }
    bytes
}

/// The byte size of the block: the end of its last member, rounded up to 16 (std140).
pub fn uniform_block_size(members: &[UniformMember]) -> usize {
    let end = members.iter().map(member_end).max().unwrap_or(0);
    end.div_ceil(16) * 16
}

fn member_end(member: &UniformMember) -> usize {
    let (count, stride) = member.array.unwrap_or((1, 0));
    let shape = member.shape;
    let column = (shape.columns.saturating_sub(1) * shape.matrix_stride + shape.rows * 4) as usize;
    member.offset as usize + (count.saturating_sub(1) * stride) as usize + column
}

/// Write the scalars of `values` into the element/column/row slots of `member`; surplus
/// values are dropped and missing ones stay zero.
fn write_member(bytes: &mut [u8], member: &UniformMember, values: &UserUniformValues) {
    let words = scalar_words(values, member.shape.scalar);
    for (index, word) in words.into_iter().enumerate() {
        let Some(at) = scalar_offset(member, index) else {
            return;
        };
        bytes[at..at + 4].copy_from_slice(&word.to_ne_bytes());
    }
}

/// The 32-bit words of `values` converted to the member's scalar type.
fn scalar_words(values: &UserUniformValues, scalar: UniformScalar) -> Vec<u32> {
    match (values, scalar) {
        (UserUniformValues::Float(v), UniformScalar::Float) => {
            v.iter().map(|f| f.to_bits()).collect()
        }
        (UserUniformValues::Float(v), _) => v.iter().map(|f| *f as i32 as u32).collect(),
        (UserUniformValues::Int(v), UniformScalar::Float) => {
            v.iter().map(|i| (*i as f32).to_bits()).collect()
        }
        (UserUniformValues::Int(v), _) => v.iter().map(|i| *i as u32).collect(),
    }
}

/// The byte offset of flat scalar `index` of `member`, or `None` past its last element.
fn scalar_offset(member: &UniformMember, index: usize) -> Option<usize> {
    let shape = member.shape;
    let per_element = (shape.rows * shape.columns) as usize;
    let (element, within) = (index / per_element, index % per_element);
    let (count, stride) = member.array.unwrap_or((1, 0));
    if element >= count as usize {
        return None;
    }
    let (column, row) = (within / shape.rows as usize, within % shape.rows as usize);
    let column_stride = shape.matrix_stride as usize;
    Some(member.offset as usize + element * stride as usize + column * column_stride + row * 4)
}
