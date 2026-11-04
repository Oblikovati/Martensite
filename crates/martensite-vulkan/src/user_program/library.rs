//! The SPIR-V library of user programs, keyed by `UserProgramKey`. The renderer takes user
//! programs only as precompiled SPIR-V (user decision 2026-10-28): a Graphic3dShaderProgram
//! whose key has no entry is refused with `RenderUserProgramFailure::GlslSourceUnsupported`.
use super::source_key::UserProgramKey;
use super::spirv_reflect::{StageReflection, UserStage, reflect_stage};
use crate::RenderError;
use martensite_tkservice::graphic3d::Graphic3dShaderProgram;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

/// Why a user program cannot be drawn.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RenderUserProgramFailure {
    /// GLSL source without precompiled SPIR-V: no runtime GLSL compiler (user decision
    /// 2026-10-28); compile it with the recording tool and register the modules.
    GlslSourceUnsupported {
        /// The key of the refused program's sources.
        key: UserProgramKey,
    },
    /// The registered modules do not form a drawable program; `reason` names the mismatch.
    InvalidModules {
        /// The key the modules were registered under.
        key: UserProgramKey,
        /// The reflected shape that does not fit, with the expected one.
        reason: String,
    },
}

/// The registered stages of one program and their reflection, in pipeline stage order.
#[derive(Debug)]
pub struct UserProgramModules {
    /// The key the stages are registered under.
    pub key: UserProgramKey,
    /// Reflection and SPIR-V words per stage, vertex first, fragment last.
    pub stages: Vec<(StageReflection, Arc<[u32]>)>,
}

impl UserProgramModules {
    /// The reflection of `stage`, when the program has one; `modules.stage(UserStage::Vertex)`.
    pub fn stage(&self, stage: UserStage) -> Option<&StageReflection> {
        let found = self
            .stages
            .iter()
            .find(|(reflection, _)| reflection.stage == stage);
        found.map(|(reflection, _)| reflection)
    }
}

/// Precompiled user programs of one driver; `driver.user_programs().insert(key, stages)?`.
#[derive(Default)]
pub struct RenderUserProgramLibrary {
    programs: RwLock<BTreeMap<UserProgramKey, Arc<UserProgramModules>>>,
}

impl RenderUserProgramLibrary {
    /// Register the SPIR-V words of every stage of the program with `key`, replacing an
    /// earlier entry; the modules must reflect as one vertex and one fragment stage (plus
    /// optional tessellation and geometry stages) sharing one default uniform block layout.
    ///
    /// Example: `library.insert(UserProgramKey::of(&program), vec![vertex, fragment])?`.
    pub fn insert(&self, key: UserProgramKey, modules: Vec<Vec<u32>>) -> Result<(), RenderError> {
        let mut stages = Vec::new();
        for words in modules {
            let reflection =
                reflect_stage(&words).map_err(|error| invalid(key, error.to_string()))?;
            stages.push((reflection, Arc::from(words)));
        }
        stages.sort_by_key(|(reflection, _)| reflection.stage);
        validate_stages(key, &stages)?;
        let entry = Arc::new(UserProgramModules { key, stages });
        self.programs.write().unwrap().insert(key, entry);
        Ok(())
    }
    /// Whether `program`'s key has registered modules; `library.contains(&program)`.
    pub fn contains(&self, program: &Graphic3dShaderProgram) -> bool {
        self.modules(program).is_ok()
    }
    /// The modules registered for `program`'s key, or the typed refusal of its GLSL source;
    /// `library.modules(&program)?.stage(UserStage::Fragment)`.
    pub fn modules(
        &self,
        program: &Graphic3dShaderProgram,
    ) -> Result<Arc<UserProgramModules>, RenderError> {
        let key = UserProgramKey::of(program);
        let found = self.programs.read().unwrap().get(&key).cloned();
        found.ok_or_else(|| {
            user_program_error(RenderUserProgramFailure::GlslSourceUnsupported { key })
        })
    }
}

fn invalid(key: UserProgramKey, reason: String) -> RenderError {
    user_program_error(RenderUserProgramFailure::InvalidModules { key, reason })
}

fn user_program_error(failure: RenderUserProgramFailure) -> RenderError {
    RenderError::user_program(failure)
}

fn validate_stages(
    key: UserProgramKey,
    stages: &[(StageReflection, Arc<[u32]>)],
) -> Result<(), RenderError> {
    let kinds: Vec<UserStage> = stages
        .iter()
        .map(|(reflection, _)| reflection.stage)
        .collect();
    let mut unique = kinds.clone();
    unique.dedup();
    if unique.len() != kinds.len()
        || kinds.first() != Some(&UserStage::Vertex)
        || kinds.last() != Some(&UserStage::Fragment)
    {
        return Err(invalid(
            key,
            format!("stages {kinds:?}; expected one vertex .. fragment chain"),
        ));
    }
    let blocks: Vec<_> = stages
        .iter()
        .filter_map(|(r, _)| r.uniform_block.as_ref())
        .collect();
    if blocks.windows(2).any(|pair| pair[0] != pair[1]) {
        return Err(invalid(
            key,
            "differing default uniform blocks; expected one linked layout".into(),
        ));
    }
    Ok(())
}

/// The typed refusal of an aspect's user program: drawing user programs needs their
/// precompiled SPIR-V (`RenderUserProgramLibrary`), and the GLSL source alone is unsupported;
/// `refuse_user_program(&aspect.shader_program())`.
pub(crate) fn refuse_user_program(program: &Graphic3dShaderProgram) -> RenderError {
    let key = UserProgramKey::of(program);
    user_program_error(RenderUserProgramFailure::GlslSourceUnsupported { key })
}
