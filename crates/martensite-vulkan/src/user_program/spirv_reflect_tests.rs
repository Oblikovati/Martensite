//! Reflection and key of the vertex/fragment uniforms test program,
//! compiled by the recording tool into tests/fixtures/user_programs/spirv.
use super::source_key::UserProgramKey;
use super::spirv_reflect::{UniformScalar, UserStage, reflect_stage};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::graphic3d::*;

const VERTEX: &[u8] =
    include_bytes!("../../tests/fixtures/user_programs/spirv/ced4d8ee5ebda03a.vert.spv");
const FRAGMENT: &[u8] =
    include_bytes!("../../tests/fixtures/user_programs/spirv/ced4d8ee5ebda03a.frag.spv");

fn words(bytes: &[u8]) -> Vec<u32> {
    let (words, rest) = bytes.as_chunks::<4>();
    assert!(
        rest.is_empty(),
        "SPIR-V of {} bytes; expected whole words",
        bytes.len()
    );
    words.iter().map(|&word| u32::from_le_bytes(word)).collect()
}

#[test]
fn the_uniforms_program_reflects_one_linked_block_and_matching_varyings() {
    let vertex = reflect_stage(&words(VERTEX)).unwrap();
    let fragment = reflect_stage(&words(FRAGMENT)).unwrap();
    assert_eq!(
        (vertex.stage, fragment.stage),
        (UserStage::Vertex, UserStage::Fragment)
    );
    assert_eq!(vertex.inputs["occVertex"], 0);
    assert_eq!(vertex.inputs["occNormal"], 1);
    assert_eq!(vertex.outputs["vColor"], fragment.inputs["vColor"]);
    assert_eq!(vertex.outputs["vNormal"], fragment.inputs["vNormal"]);
    assert_eq!(fragment.outputs["occFragColor"], 0);
    let (set, binding, members) = vertex.uniform_block.clone().unwrap();
    assert_eq!((set, binding), (1, 0));
    assert_eq!(fragment.uniform_block, vertex.uniform_block);
    let member = |name: &str| members.iter().find(|m| m.name == name).unwrap().clone();
    assert_eq!(member("occWorldViewMatrix").offset, 0);
    assert_eq!(member("occWorldViewMatrix").shape.columns, 4);
    assert_eq!(member("occWorldViewMatrix").shape.matrix_stride, 16);
    assert_eq!(member("uNormTrsf").shape.columns, 3);
    assert_eq!(member("occLightSources").array, Some((32, 16)));
    assert_eq!(
        member("occLightSourcesTypes").shape.scalar,
        UniformScalar::Int
    );
    assert_eq!(fragment.samplers.len(), 1);
    assert_eq!(
        (fragment.samplers[0].set, fragment.samplers[0].binding),
        (0, 0)
    );
    assert_eq!(fragment.samplers[0].dim, 1, "a sampler2D");
}

#[test]
fn malformed_modules_are_refused_with_their_shape() {
    assert!(
        reflect_stage(&[0, 1, 2])
            .unwrap_err()
            .to_string()
            .contains("0x07230203")
    );
    let mut truncated = words(VERTEX);
    truncated.truncate(7);
    assert!(reflect_stage(&truncated).is_err());
}

/// The run-time key equals the key the offline tool printed for the same sources.
#[test]
fn the_program_key_matches_the_offline_compiler() {
    let program = uniforms_program();
    assert_eq!(UserProgramKey::of(&program).to_string(), "ced4d8ee5ebda03a");
    program.set_nb_lights_max(4);
    assert_ne!(UserProgramKey::of(&program).to_string(), "ced4d8ee5ebda03a");
}

fn uniforms_program() -> Graphic3dShaderProgram {
    let program = Graphic3dShaderProgram::default();
    program.set_header(AsciiString::from("#version 110"));
    for (kind, source) in [
        (
            Graphic3dTypeOfShaderObject::Vertex,
            include_str!("../../tests/fixtures/user_programs/uniforms.vert.glsl"),
        ),
        (
            Graphic3dTypeOfShaderObject::Fragment,
            include_str!("../../tests/fixtures/user_programs/uniforms.frag.glsl"),
        ),
    ] {
        let shader = Graphic3dShaderObject::create_from_source(kind, AsciiString::from(source));
        assert!(program.attach_shader(&shader));
    }
    program
}

/// GLSL source alone is refused with its key; registered SPIR-V is found by the same key.
#[test]
fn the_library_refuses_glsl_source_and_finds_registered_spirv() {
    let library = crate::RenderUserProgramLibrary::default();
    let program = uniforms_program();
    let key = UserProgramKey::of(&program);
    let refused = library.modules(&program).unwrap_err();
    assert_eq!(
        refused.user_program_failure(),
        Some(&crate::RenderUserProgramFailure::GlslSourceUnsupported { key })
    );
    library
        .insert(key, vec![words(FRAGMENT), words(VERTEX)])
        .unwrap();
    let modules = library.modules(&program).unwrap();
    assert_eq!(modules.stages.len(), 2);
    assert!(modules.stage(UserStage::Vertex).is_some() && library.contains(&program));
    let lone = library.insert(key, vec![words(FRAGMENT)]).unwrap_err();
    assert!(matches!(
        lone.user_program_failure(),
        Some(crate::RenderUserProgramFailure::InvalidModules { .. })
    ));
}
