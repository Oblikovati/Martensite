//! The built-in PhongShading sample: Martensite's own
//! per-pixel Blinn-Phong GLSL (shaders/user/phong_shading.*.glsl), precompiled by
//! the recording tool from shaders/user/manifest.json and registered in every
//! driver's library, so the program of `phong_shading_program` draws without a GLSL compiler.
use super::library::RenderUserProgramLibrary;
use super::source_key::UserProgramKey;
use crate::RenderError;
use martensite_tkservice::graphic3d::{
    Graphic3dRenderTransparentMethod, Graphic3dShaderObject, Graphic3dShaderProgram,
    Graphic3dTypeOfShaderObject,
};

const PHONG_VERTEX: &str = include_str!("../../shaders/user/phong_shading.vert.glsl");
const PHONG_FRAGMENT: &str = include_str!("../../shaders/user/phong_shading.frag.glsl");
const PHONG_SPIRV: [&[u8]; 2] = [
    include_bytes!("../../shaders/user/spirv/fc9b4b4a9d14bfd5.vert.spv"),
    include_bytes!("../../shaders/user/spirv/fc9b4b4a9d14bfd5.frag.spv"),
];
/// The weighted-OIT output variant (the sample in a view using Graphic3dRTM_BLEND_OIT:
/// two fragment outputs and the weighted OIT output; manifest `phong-shading-weighted-oit`).
const PHONG_WEIGHTED_OIT_SPIRV: [&[u8]; 2] = [
    include_bytes!("../../shaders/user/spirv/56c5cf8c2c17de14.vert.spv"),
    include_bytes!("../../shaders/user/spirv/56c5cf8c2c17de14.frag.spv"),
];

/// A new PhongShading sample program with the default options, whose key has registered
/// SPIR-V in every driver; `aspect.set_shader_program(Handle::new(phong_shading_program()))`.
pub fn phong_shading_program() -> Graphic3dShaderProgram {
    let program = Graphic3dShaderProgram::default();
    for (kind, source) in [
        (Graphic3dTypeOfShaderObject::Vertex, PHONG_VERTEX),
        (Graphic3dTypeOfShaderObject::Fragment, PHONG_FRAGMENT),
    ] {
        program.attach_shader(&Graphic3dShaderObject::create_from_source(
            kind,
            source.into(),
        ));
    }
    program
}

/// Register the built-in samples; `register_samples(&library)?` at driver creation.
pub(crate) fn register_samples(library: &RenderUserProgramLibrary) -> Result<(), RenderError> {
    let modules = PHONG_SPIRV.iter().map(|bytes| spirv_words(bytes)).collect();
    library.insert(UserProgramKey::of(&phong_shading_program()), modules)?;
    let modules = PHONG_WEIGHTED_OIT_SPIRV
        .iter()
        .map(|bytes| spirv_words(bytes))
        .collect();
    library.insert(UserProgramKey::of(&weighted_oit_phong()), modules)
}

/// The sample as created for a weighted-OIT view.
fn weighted_oit_phong() -> Graphic3dShaderProgram {
    let program = phong_shading_program();
    program.set_nb_fragment_outputs(2);
    program.set_oit_output(Graphic3dRenderTransparentMethod::BlendOit);
    program
}

fn spirv_words(bytes: &[u8]) -> Vec<u32> {
    let words = bytes.as_chunks::<4>().0;
    words.iter().map(|word| u32::from_le_bytes(*word)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_sample_key_names_its_precompiled_modules() {
        let key = UserProgramKey::of(&phong_shading_program());
        assert_eq!(key.to_string(), "fc9b4b4a9d14bfd5");
        let library = RenderUserProgramLibrary::default();
        register_samples(&library).unwrap();
        assert!(library.contains(&phong_shading_program()));
        let weighted = weighted_oit_phong();
        assert_eq!(
            UserProgramKey::of(&weighted).to_string(),
            "56c5cf8c2c17de14"
        );
        assert!(library.contains(&weighted));
        // One output keeps the plain sample: native drops a weighted output without a second one.
        let single = phong_shading_program();
        single.set_oit_output(Graphic3dRenderTransparentMethod::BlendOit);
        assert_eq!(UserProgramKey::of(&single).to_string(), "fc9b4b4a9d14bfd5");
    }
}
