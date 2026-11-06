//! The lookup key of a user program: FNV-1a 64 over its header, its stages (type and source
//! text, in attach order) and the options that change the compiled wrapping. The offline
//! compiler the recording tool (`program_key`) names its SPIR-V outputs by the same
//! key, so a program built from the same sources finds its precompiled modules.
use martensite_tkservice::graphic3d::{
    Graphic3dRenderTransparentMethod, Graphic3dShaderProgram, Graphic3dTypeOfShaderObject,
};

const KEY_SCHEMA: &str = "martensite-user-program-v1";

/// A program lookup key; `UserProgramKey::of(&program).to_string()` is 16 hex digits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserProgramKey(pub u64);

impl std::fmt::Display for UserProgramKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:016x}", self.0)
    }
}

/// FNV-1a 64 of `bytes`; `fnv1a64(b"") == 0xcbf2_9ce4_8422_2325`.
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

impl UserProgramKey {
    /// The key of `program`'s current header, stages and options.
    pub fn of(program: &Graphic3dShaderProgram) -> Self {
        let mut parts = vec![
            KEY_SCHEMA.as_bytes().to_vec(),
            program.header().as_bytes().to_vec(),
        ];
        for shader in program.shader_objects() {
            let source = shader.source().as_bytes().to_vec();
            parts.push(stage_bits(shader.shader_type()).to_string().into_bytes());
            parts.push(source.len().to_string().into_bytes());
            parts.push(source);
        }
        parts.extend(option_lines(program).into_iter().map(String::into_bytes));
        Self(fnv1a64(&parts.join(&b'\n')))
    }
}

/// The native Graphic3dTypeOfShaderObject bit of a stage.
fn stage_bits(kind: Graphic3dTypeOfShaderObject) -> i32 {
    kind as i32
}

/// `name=value` per wrapping option, sorted by name as the tool writes them; a weighted OIT
/// output adds `oit_output=1` last, so the keys of programs without one are unchanged.
fn option_lines(program: &Graphic3dShaderProgram) -> Vec<String> {
    let mut lines = plain_option_lines(program);
    if writes_weighted_oit(program) {
        lines.push("oit_output=1".to_owned());
    }
    lines
}

/// Native GlShaderProgram keeps a Graphic3dRTM_BLEND_OIT output only with at least two
/// fragment outputs (otherwise the output falls back to unordered blending);
/// `writes_weighted_oit(&program)` for a program in a weighted-OIT view.
pub fn writes_weighted_oit(program: &Graphic3dShaderProgram) -> bool {
    program.oit_output() == Graphic3dRenderTransparentMethod::BlendOit
        && program.nb_fragment_outputs() >= 2
}

fn plain_option_lines(program: &Graphic3dShaderProgram) -> Vec<String> {
    [
        ("alpha_test", i32::from(program.has_alpha_test())),
        ("default_sampler", i32::from(program.has_default_sampler())),
        ("nb_clip_planes_max", program.nb_clip_planes_max()),
        ("nb_frag_outputs", program.nb_fragment_outputs()),
        ("nb_lights_max", program.nb_lights_max()),
    ]
    .iter()
    .map(|(name, value)| format!("{name}={value}"))
    .collect()
}
