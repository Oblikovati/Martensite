//! Exercise actual registered custom stages without a Vulkan device or fake shader words.
use super::*;
use crate::user_program::{RenderUserProgramLibrary, UserProgramKey};
use martensite_tkservice::graphic3d::Graphic3dShaderProgram;
fn registered_custom_modules() -> Arc<UserProgramModules> {
    let program = Graphic3dShaderProgram::default();
    let library = RenderUserProgramLibrary::default();
    let inputs: [&[u8]; 2] = [
        include_bytes!("../../shaders/user/spirv/fc9b4b4a9d14bfd5.vert.spv"),
        include_bytes!("../../shaders/user/spirv/fc9b4b4a9d14bfd5.frag.spv"),
    ];
    library
        .insert(
            UserProgramKey::of(&program),
            inputs.into_iter().map(shader_words).collect(),
        )
        .unwrap();
    library.modules(&program).unwrap()
}
fn shader_words(bytes: &[u8]) -> Vec<u32> {
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|word| u32::from_le_bytes(*word))
        .collect()
}
fn assert_unapplied(behavior: crate::RenderLightDotBehavior, bytes: &[u8]) {
    use super::super::light_dot_shader::{selected_fragment, selected_vertex};
    for select in [selected_vertex, selected_fragment] {
        assert!(select(behavior, false, bytes).unwrap().is_none());
    }
}
fn assert_custom_stages_unchanged(
    behavior: crate::RenderLightDotBehavior,
    modules: &UserProgramModules,
) {
    let mut kind = PipelineVariant::from_index(0, false);
    kind.lighting = true;
    let fixed = UserPipelineState::single(kind, 0);
    for (reflection, words) in &modules.stages {
        let original: Vec<u32> = words.to_vec();
        let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_le_bytes()).collect();
        assert_unapplied(behavior, &bytes);
        assert_eq!(stage_words(reflection, words, fixed).unwrap(), original);
        assert_eq!(words.as_ref(), original);
    }
}
#[test]
fn numeric_builtin_policy_leaves_actual_custom_stage_words_unapplied() {
    let modules = registered_custom_modules();
    for behavior in [
        crate::RenderLightDotBehavior::DriverNativeV1,
        crate::RenderLightDotBehavior::NumericMaxZeroV2,
    ] {
        assert_custom_stages_unchanged(behavior, &modules);
    }
}
