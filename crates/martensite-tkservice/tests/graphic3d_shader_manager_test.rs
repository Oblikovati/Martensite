//! Behaviour tests of `Graphic3d_ShaderManager`.
#![allow(non_snake_case)]
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectGraphicsLibrary;
use martensite_tkservice::graphic3d::{
    Graphic3dLightSet, Graphic3dPhongVariant, Graphic3dShaderFlags as Flags,
    Graphic3dShaderManager, Graphic3dShaderProgram, Graphic3dTypeOfShaderObject,
};

/// The test proxy: an OpenGL 3.2 manager whose protected generators the tests call.
fn proxy() -> Graphic3dShaderManager {
    let manager = Graphic3dShaderManager::new(AspectGraphicsLibrary::OpenGL);
    manager.set_gapi_version(3, 2);
    manager
}

/// Whether a stage of `kind` holds `snippet` (`Source().Search(snippet) > 0`).
fn hasShaderSource(
    program: &Handle<Graphic3dShaderProgram>,
    kind: Graphic3dTypeOfShaderObject,
    snippet: &str,
) -> bool {
    let program = program.get().expect("a generated program");
    program.shader_objects().iter().any(|shader| {
        shader.get().is_some_and(|shader| {
            shader.shader_type() == kind && shader.source().search(snippet) > 0
        })
    })
}

#[test]
fn Graphic3d_ShaderManagerTest__UnlitFrontOnlyVertexColor_UsesBackColor() {
    let aManager = proxy();
    let aDefaultProgram = aManager.get_std_program_unlit(Flags::VERT_COLOR.0, false);
    let aFrontOnlyProgram =
        aManager.get_std_program_unlit(Flags::VERT_COLOR.0 | Flags::VERT_COLOR_FRONT_ONLY.0, false);
    assert!(!aDefaultProgram.is_null());
    assert!(!aFrontOnlyProgram.is_null());
    assert_ne!(
        aDefaultProgram.get().unwrap().get_id(),
        aFrontOnlyProgram.get().unwrap().get_id()
    );
    assert!(hasShaderSource(
        &aFrontOnlyProgram,
        Graphic3dTypeOfShaderObject::Fragment,
        "gl_FrontFacing ? VertColor : getBackColor()"
    ));
    assert!(hasShaderSource(
        &aFrontOnlyProgram,
        Graphic3dTypeOfShaderObject::Fragment,
        "occBackColor"
    ));
}

#[test]
fn Graphic3d_ShaderManagerTest__GouraudFrontOnlyVertexColor_UsesSideAwareHelper() {
    let aManager = proxy();
    let aLights = Handle::new(Graphic3dLightSet::default());
    let aProgram = aManager
        .get_std_program_gouraud(
            &aLights,
            Flags::VERT_COLOR.0 | Flags::VERT_COLOR_FRONT_ONLY.0,
        )
        .expect("a light set is given");
    assert!(!aProgram.is_null());
    assert!(hasShaderSource(
        &aProgram,
        Graphic3dTypeOfShaderObject::Vertex,
        "theIsFront ? occVertColor : vec4(1.0)"
    ));
    assert!(hasShaderSource(
        &aProgram,
        Graphic3dTypeOfShaderObject::Vertex,
        "BackColor   = computeLighting (aNormal, aView, aPositionWorld, false)"
    ));
}

#[test]
fn Graphic3d_ShaderManagerTest__PhongFrontOnlyVertexColor_UsesSideAwareHelper() {
    let aManager = proxy();
    let aLights = Handle::new(Graphic3dLightSet::default());
    let aProgram = aManager
        .get_std_program_phong(
            &aLights,
            Flags::VERT_COLOR.0 | Flags::VERT_COLOR_FRONT_ONLY.0,
            Graphic3dPhongVariant::default(),
        )
        .expect("a light set is given");
    assert!(!aProgram.is_null());
    assert!(hasShaderSource(
        &aProgram,
        Graphic3dTypeOfShaderObject::Fragment,
        "theIsFront ? VertColor : vec4(1.0)"
    ));
    assert!(hasShaderSource(
        &aProgram,
        Graphic3dTypeOfShaderObject::Fragment,
        "getVertColor(theIsFront)"
    ));
}
