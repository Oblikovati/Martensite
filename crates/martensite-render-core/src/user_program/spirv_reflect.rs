//! Reflection of one SPIR-V stage module of a user program: interface locations, the default
//! uniform block (glslang `-R`: loose GLSL uniforms gathered into `gl_DefaultUniformBlock`) with
//! its std140 member offsets, and the combined image samplers. Only what the renderer binds is
//! read; any other shape is a typed rejection.
use crate::RenderError;
use crate::error::rejected;
use std::collections::BTreeMap;

const MAGIC: u32 = 0x0723_0203;
const OP_NAME: u16 = 5;
const OP_MEMBER_NAME: u16 = 6;
const OP_ENTRY_POINT: u16 = 15;
const OP_TYPE_BOOL: u16 = 20;
const OP_TYPE_INT: u16 = 21;
const OP_TYPE_FLOAT: u16 = 22;
const OP_TYPE_VECTOR: u16 = 23;
const OP_TYPE_MATRIX: u16 = 24;
const OP_TYPE_IMAGE: u16 = 25;
const OP_TYPE_SAMPLED_IMAGE: u16 = 27;
const OP_TYPE_ARRAY: u16 = 28;
const OP_TYPE_STRUCT: u16 = 30;
const OP_TYPE_POINTER: u16 = 32;
const OP_CONSTANT: u16 = 43;
const OP_VARIABLE: u16 = 59;
const OP_DECORATE: u16 = 71;
const OP_MEMBER_DECORATE: u16 = 72;
const DECORATION_ARRAY_STRIDE: u32 = 6;
const DECORATION_MATRIX_STRIDE: u32 = 7;
const DECORATION_BUILT_IN: u32 = 11;
const DECORATION_LOCATION: u32 = 30;
const DECORATION_BINDING: u32 = 33;
const DECORATION_DESCRIPTOR_SET: u32 = 34;
const DECORATION_OFFSET: u32 = 35;
const STORAGE_UNIFORM_CONSTANT: u32 = 0;
const STORAGE_INPUT: u32 = 1;
const STORAGE_UNIFORM: u32 = 2;
const STORAGE_OUTPUT: u32 = 3;

/// The shader stage of a module (SPIR-V ExecutionModel).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UserStage {
    /// Vertex stage (`Graphic3dTOS_VERTEX`).
    Vertex,
    /// Tessellation control stage.
    TessControl,
    /// Tessellation evaluation stage.
    TessEvaluation,
    /// Geometry stage.
    Geometry,
    /// Fragment stage (`Graphic3dTOS_FRAGMENT`).
    Fragment,
}

/// The scalar component of a uniform value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UniformScalar {
    /// 32-bit float.
    Float,
    /// 32-bit signed integer.
    Int,
    /// 32-bit unsigned integer.
    UInt,
    /// Boolean (32 bits in a uniform block).
    Bool,
}

/// A uniform value shape: `columns` vectors of `rows` scalars (a scalar is 1x1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniformShape {
    /// The component type.
    pub scalar: UniformScalar,
    /// Components per vector (or per matrix column).
    pub rows: u32,
    /// Matrix columns (1 for vectors and scalars).
    pub columns: u32,
    /// Byte distance between matrix columns (0 for vectors and scalars).
    pub matrix_stride: u32,
}

/// One member of the default uniform block; arrays keep their element count and stride.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UniformMember {
    /// The GLSL uniform name.
    pub name: String,
    /// Byte offset in the block (std140).
    pub offset: u32,
    /// Value shape of one element.
    pub shape: UniformShape,
    /// Element count and byte stride; `None` for a non-array member.
    pub array: Option<(u32, u32)>,
}

/// A combined image sampler: name, descriptor set, binding and image dimensionality.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SamplerBinding {
    /// The GLSL sampler name.
    pub name: String,
    /// Descriptor set.
    pub set: u32,
    /// Binding in the set.
    pub binding: u32,
    /// The SPIR-V `Dim` of the sampled image: 0 1D, 1 2D, 2 3D, 3 Cube.
    pub dim: u32,
}

/// The SPIR-V `Dim` of a `sampler3D`; `binding.dim == SPIRV_DIM_3D`.
pub const SPIRV_DIM_3D: u32 = 2;

/// The reflected interface of one stage module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StageReflection {
    /// The stage of the module.
    pub stage: UserStage,
    /// Non-built-in stage inputs by name: location.
    pub inputs: BTreeMap<String, u32>,
    /// Non-built-in stage outputs by name: location.
    pub outputs: BTreeMap<String, u32>,
    /// The default uniform block: set, binding and members, when the stage reads uniforms.
    pub uniform_block: Option<(u32, u32, Vec<UniformMember>)>,
    /// Combined image samplers.
    pub samplers: Vec<SamplerBinding>,
}

#[derive(Default)]
struct Module {
    names: BTreeMap<u32, String>,
    member_names: BTreeMap<(u32, u32), String>,
    decorations: BTreeMap<(u32, u32), u32>,
    member_decorations: BTreeMap<(u32, u32, u32), u32>,
    types: BTreeMap<u32, (u16, Vec<u32>)>,
    constants: BTreeMap<u32, u32>,
    variables: Vec<(u32, u32, u32)>,
    stage: Option<UserStage>,
}

/// Reflect `words`, one SPIR-V module with a single entry point;
/// `reflect_stage(&words)?.uniform_block`.
pub fn reflect_stage(words: &[u32]) -> Result<StageReflection, RenderError> {
    let module = parse_module(words)?;
    let stage = module.stage.ok_or_else(|| {
        rejected(
            "no OpEntryPoint",
            "one vertex, tessellation, geometry or fragment entry point",
        )
    })?;
    Ok(StageReflection {
        stage,
        inputs: module.interface(STORAGE_INPUT),
        outputs: module.interface(STORAGE_OUTPUT),
        uniform_block: module.uniform_block()?,
        samplers: module.samplers(),
    })
}

fn parse_module(words: &[u32]) -> Result<Module, RenderError> {
    if words.len() < 5 || words[0] != MAGIC {
        return Err(rejected(
            words.first(),
            "a SPIR-V module starting with 0x07230203",
        ));
    }
    let mut module = Module::default();
    let mut cursor = 5;
    while cursor < words.len() {
        let count = (words[cursor] >> 16) as usize;
        let end = cursor + count;
        if count == 0 || end > words.len() {
            return Err(rejected(
                (cursor, count),
                "SPIR-V instructions inside the module",
            ));
        }
        module.record(words[cursor] as u16, &words[cursor + 1..end])?;
        cursor = end;
    }
    Ok(module)
}

/// A nul-terminated literal string starting at `words[0]`.
fn literal_string(words: &[u32]) -> String {
    let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_le_bytes()).collect();
    let end = bytes
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

fn execution_model(model: u32) -> Option<UserStage> {
    match model {
        0 => Some(UserStage::Vertex),
        1 => Some(UserStage::TessControl),
        2 => Some(UserStage::TessEvaluation),
        3 => Some(UserStage::Geometry),
        4 => Some(UserStage::Fragment),
        _ => None,
    }
}

impl Module {
    fn record(&mut self, opcode: u16, operands: &[u32]) -> Result<(), RenderError> {
        let operand = |index: usize| {
            operands
                .get(index)
                .copied()
                .ok_or_else(|| rejected((opcode, operands.len()), "a complete SPIR-V instruction"))
        };
        match opcode {
            OP_NAME => {
                self.names
                    .insert(operand(0)?, literal_string(&operands[1..]));
            }
            OP_MEMBER_NAME => {
                let key = (operand(0)?, operand(1)?);
                self.member_names
                    .insert(key, literal_string(&operands[2..]));
            }
            OP_ENTRY_POINT => self.record_entry_point(operand(0)?)?,
            OP_DECORATE => self.record_decoration(operands),
            OP_MEMBER_DECORATE => self.record_member_decoration(operands),
            OP_CONSTANT => {
                self.constants.insert(operand(1)?, operand(2)?);
            }
            OP_VARIABLE => self.variables.push((operand(0)?, operand(1)?, operand(2)?)),
            OP_TYPE_BOOL..=OP_TYPE_POINTER => {
                self.types
                    .insert(operand(0)?, (opcode, operands[1..].to_vec()));
            }
            _ => {}
        }
        Ok(())
    }
    fn record_entry_point(&mut self, model: u32) -> Result<(), RenderError> {
        if self.stage.is_some() {
            return Err(rejected(
                "a second OpEntryPoint",
                "one entry point per stage module",
            ));
        }
        self.stage = Some(execution_model(model).ok_or_else(|| {
            rejected(model, "a vertex, tessellation, geometry or fragment model")
        })?);
        Ok(())
    }
    fn record_decoration(&mut self, operands: &[u32]) {
        if let [target, decoration, value, ..] = operands {
            self.decorations.insert((*target, *decoration), *value);
        } else if let [target, decoration] = operands {
            self.decorations.insert((*target, *decoration), 0);
        }
    }
    fn record_member_decoration(&mut self, operands: &[u32]) {
        if let [target, member, decoration, rest @ ..] = operands {
            let value = rest.first().copied().unwrap_or(0);
            self.member_decorations
                .insert((*target, *member, *decoration), value);
        }
    }
    /// The pointee type of pointer type `pointer`.
    fn pointee(&self, pointer: u32) -> Option<u32> {
        match self.types.get(&pointer) {
            Some((OP_TYPE_POINTER, operands)) => operands.get(1).copied(),
            _ => None,
        }
    }
    fn interface(&self, storage: u32) -> BTreeMap<String, u32> {
        self.variables
            .iter()
            .filter(|&&(_, _, class)| class == storage)
            .filter(|(_, id, _)| !self.decorations.contains_key(&(*id, DECORATION_BUILT_IN)))
            .filter_map(|(_, id, _)| {
                let location = self.decorations.get(&(*id, DECORATION_LOCATION))?;
                Some((self.names.get(id)?.clone(), *location))
            })
            .collect()
    }
    fn binding_of(&self, id: u32) -> (u32, u32) {
        let set = self.decorations.get(&(id, DECORATION_DESCRIPTOR_SET));
        let binding = self.decorations.get(&(id, DECORATION_BINDING));
        (set.copied().unwrap_or(0), binding.copied().unwrap_or(0))
    }
    fn samplers(&self) -> Vec<SamplerBinding> {
        self.variables
            .iter()
            .filter(|&&(_, _, class)| class == STORAGE_UNIFORM_CONSTANT)
            .filter_map(|&(pointer, id, _)| {
                let dim = self.sampled_image_dim(pointer)?;
                let (set, binding) = self.binding_of(id);
                let name = self.names.get(&id).cloned().unwrap_or_default();
                Some(SamplerBinding {
                    name,
                    set,
                    binding,
                    dim,
                })
            })
            .collect()
    }
    /// The `Dim` of the image of a pointer to a sampled image, `None` for another pointee.
    fn sampled_image_dim(&self, pointer: u32) -> Option<u32> {
        let pointee = self.pointee(pointer).and_then(|ty| self.types.get(&ty));
        let Some((OP_TYPE_SAMPLED_IMAGE, operands)) = pointee else {
            return None;
        };
        match self.types.get(operands.first()?) {
            // OpTypeImage operands after the result: sampled type, Dim, ...
            Some((OP_TYPE_IMAGE, image)) => image.get(1).copied(),
            _ => None,
        }
    }
    fn uniform_block(&self) -> Result<Option<(u32, u32, Vec<UniformMember>)>, RenderError> {
        let blocks: Vec<_> = self
            .variables
            .iter()
            .filter(|&&(_, _, class)| class == STORAGE_UNIFORM)
            .collect();
        let Some(&&(pointer, id, _)) = blocks.first() else {
            return Ok(None);
        };
        if blocks.len() > 1 {
            return Err(rejected(
                blocks.len(),
                "at most one uniform block (the default uniform block)",
            ));
        }
        let block = self
            .pointee(pointer)
            .ok_or_else(|| rejected(pointer, "a uniform block pointer type"))?;
        let (set, binding) = self.binding_of(id);
        Ok(Some((set, binding, self.block_members(block)?)))
    }
    fn block_members(&self, block: u32) -> Result<Vec<UniformMember>, RenderError> {
        let Some((OP_TYPE_STRUCT, members)) = self.types.get(&block) else {
            return Err(rejected(block, "a struct uniform block type"));
        };
        members
            .iter()
            .enumerate()
            .map(|(index, &ty)| self.block_member(block, index as u32, ty))
            .collect()
    }
    fn block_member(&self, block: u32, index: u32, ty: u32) -> Result<UniformMember, RenderError> {
        let decoration = |kind| self.member_decorations.get(&(block, index, kind)).copied();
        let name = self
            .member_names
            .get(&(block, index))
            .cloned()
            .unwrap_or_default();
        let offset = decoration(DECORATION_OFFSET)
            .ok_or_else(|| rejected(&name, "an explicit Offset on every uniform member"))?;
        let (element, array) = self.array_of(ty)?;
        let mut shape = self.shape_of(element, &name)?;
        shape.matrix_stride = decoration(DECORATION_MATRIX_STRIDE).unwrap_or(0);
        Ok(UniformMember {
            name,
            offset,
            shape,
            array,
        })
    }
    /// The element type and `(count, stride)` of array type `ty`, or `ty` itself.
    fn array_of(&self, ty: u32) -> Result<(u32, Option<(u32, u32)>), RenderError> {
        let Some((OP_TYPE_ARRAY, operands)) = self.types.get(&ty) else {
            return Ok((ty, None));
        };
        let (element, length) = (operands[0], operands[1]);
        let count = self.constants.get(&length).copied();
        let stride = self
            .decorations
            .get(&(ty, DECORATION_ARRAY_STRIDE))
            .copied();
        match (count, stride) {
            (Some(count), Some(stride)) => Ok((element, Some((count, stride)))),
            _ => Err(rejected(
                ty,
                "a uniform array with a constant length and an ArrayStride",
            )),
        }
    }
    fn shape_of(&self, ty: u32, name: &str) -> Result<UniformShape, RenderError> {
        let scalar = |scalar| UniformShape {
            scalar,
            rows: 1,
            columns: 1,
            matrix_stride: 0,
        };
        match self.types.get(&ty) {
            Some((OP_TYPE_FLOAT, _)) => Ok(scalar(UniformScalar::Float)),
            Some((OP_TYPE_INT, operands)) if operands.get(1) == Some(&1) => {
                Ok(scalar(UniformScalar::Int))
            }
            Some((OP_TYPE_INT, _)) => Ok(scalar(UniformScalar::UInt)),
            Some((OP_TYPE_BOOL, _)) => Ok(scalar(UniformScalar::Bool)),
            Some((OP_TYPE_VECTOR, operands)) => Ok(UniformShape {
                rows: operands[1],
                ..self.shape_of(operands[0], name)?
            }),
            Some((OP_TYPE_MATRIX, operands)) => {
                let column = self.shape_of(operands[0], name)?;
                Ok(UniformShape {
                    columns: operands[1],
                    ..column
                })
            }
            other => Err(rejected(
                (name, other.map(|(opcode, _)| *opcode), OP_TYPE_IMAGE),
                "a scalar, vector or matrix uniform (no struct members)",
            )),
        }
    }
}
