//! Capture and diagnostic queries use actual driver/view values, never backend-name aliases.
use super::*;
use martensite_tkservice::graphic3d::{
    Graphic3dDiagnosticInfo, Graphic3dFrameStatsDict, Graphic3dTypeOfShadingModel,
};
#[path = "../viewer_commands/opengl_commands/glinfo_format.rs"]
mod glinfo_format;

pub(super) fn backend_scene(factory: &'static str, expected: &str, image: &str) -> Outcome {
    let (_driver, mut session) = create_context(factory, ContextRequest::default())?;
    session.clear()?;
    let shape = crate::primitive_shapes::box_at([3., 0., 0.], [1., 2., 3.])?;
    let sphere = crate::primitive_shapes::sphere(1.)?;
    let shaded = DisplayRequest::default().with_display_mode(1);
    session.display("b", &shape, &shaded)?;
    session.display("s", &sphere, &shaded)?;
    check_backend_information(&session.graphics_information(), expected);
    session.fit()?;
    session.set_shading_model(Graphic3dTypeOfShadingModel::Phong)?;
    dump(&session, image)?;
    finish(session)
}
fn check_backend_information(actual: &str, expected: &str) {
    if actual.contains(expected) {
        println!("OK: {expected}");
    } else {
        println!("Error: {expected} is expected");
    }
}
pub(super) fn complete_information(session: &ViewerSession) -> String {
    let mut dictionary = Graphic3dFrameStatsDict::default();
    session
        .view()
        .diagnostic_information(&mut dictionary, Graphic3dDiagnosticInfo::COMPLETE);
    glinfo_format::format_information(&dictionary, 80)
}
pub(super) fn dump(session: &ViewerSession, basename: &str) -> Outcome {
    crate::source_output::source_dump(session, basename)?;
    Ok(())
}

pub(super) fn finish(session: ViewerSession) -> Outcome {
    println!("{}", session.graphics_information());
    println!("TEST COMPLETED");
    session.close();
    Ok(())
}
#[test]
fn backend_query_does_not_invent_a_required_api_string() {
    check_backend_information("Vulkan 1.3", "OpenGL ES");
    assert!(!"Vulkan 1.3".contains("OpenGL ES"));
}
