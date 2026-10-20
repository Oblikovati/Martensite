use super::version::round_up_max_light_sources;
use super::*;
use crate::graphic3d::{Graphic3dShaderFlags as Flags, Graphic3dTypeOfShaderObject};

fn manager(gapi: AspectGraphicsLibrary, version: (i32, i32)) -> Graphic3dShaderManager {
    let manager = Graphic3dShaderManager::new(gapi);
    manager.set_gapi_version(version.0, version.1);
    manager
}

#[test]
fn light_arrays_round_up_by_doubling_from_thirty_two() {
    assert_eq!(round_up_max_light_sources(0), 32);
    assert_eq!(round_up_max_light_sources(32), 32);
    assert_eq!(round_up_max_light_sources(33), 64);
    assert_eq!(round_up_max_light_sources(129), 256);
}

#[test]
fn constructor_defaults_follow_the_graphics_api() {
    let desktop = Graphic3dShaderManager::new(AspectGraphicsLibrary::OpenGL);
    assert!(desktop.has_flat_shading() && desktop.to_emulate_depth_clamp());
    assert!(!desktop.to_reverse_dfdx_sign() && !desktop.use_red_alpha());
    assert!(!desktop.fields().read().set_point_size);
    let embedded = Graphic3dShaderManager::new(AspectGraphicsLibrary::OpenGLES);
    assert!(embedded.fields().read().set_point_size);
    assert!(!embedded.has_glsl_extension(Graphic3dGlslExtension::GlExtFragDepth));
    embedded.enable_glsl_extension(Graphic3dGlslExtension::GlExtFragDepth, true);
    assert!(embedded.has_glsl_extension(Graphic3dGlslExtension::GlExtFragDepth));
}

#[test]
fn version_comparison_orders_major_then_minor() {
    let manager = manager(AspectGraphicsLibrary::OpenGL, (3, 2));
    assert!(manager.is_gapi_greater_equal(3, 2) && manager.is_gapi_greater_equal(2, 9));
    assert!(!manager.is_gapi_greater_equal(3, 3) && !manager.is_gapi_greater_equal(4, 0));
    assert_eq!(
        (manager.gapi_version_major(), manager.gapi_version_minor()),
        (3, 2)
    );
}

#[test]
fn program_ids_carry_the_supported_bits_in_hex() {
    let desktop = manager(AspectGraphicsLibrary::OpenGL, (3, 2));
    let program = desktop.get_std_program_unlit(Flags::VERT_COLOR.0 | Flags::WRITE_OIT.0, false);
    assert_eq!(
        program.get().unwrap().get_id().as_bytes(),
        b"occt_unlit0801"
    );
    assert_eq!(program.get().unwrap().header().as_bytes(), b"#version 150");
    let es2 = manager(AspectGraphicsLibrary::OpenGLES, (2, 0));
    let program = es2.get_std_program_unlit(Flags::WRITE_OIT.0 | Flags::VERT_COLOR.0, false);
    assert_eq!(
        program.get().unwrap().get_id().as_bytes(),
        b"occt_unlit0001"
    );
}

#[test]
fn geometry_stage_exists_only_for_mesh_edges() {
    let manager = manager(AspectGraphicsLibrary::OpenGL, (3, 2));
    let kinds = |bits: i32| -> Vec<Graphic3dTypeOfShaderObject> {
        let program = manager.get_std_program_unlit(bits, false);
        let program = program.get().unwrap();
        program
            .shader_objects()
            .iter()
            .map(|s| s.get().unwrap().shader_type())
            .collect()
    };
    assert_eq!(kinds(0).len(), 2);
    assert_eq!(
        kinds(Flags::MESH_EDGES.0)[1],
        Graphic3dTypeOfShaderObject::Geometry
    );
}

#[test]
fn baking_index_outside_zero_to_two_is_a_program_error() {
    let manager = manager(AspectGraphicsLibrary::OpenGL, (3, 2));
    let Err(failure) = manager.get_pbr_env_baking_program(3) else {
        panic!("index 3 must be rejected");
    };
    assert_eq!(failure.exception_type(), "Standard_ProgramError");
    assert!(manager.get_pbr_env_baking_program(2).is_ok());
}
