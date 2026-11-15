use super::*;
use martensite_tkservice::graphic3d::Graphic3dAspectsMembers;
use martensite_tkservice::null_driver::NullGraphicDriver;
fn null_session() -> ViewerSession {
    ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409).unwrap()
}
#[test]
fn actual_null_driver_query_keeps_actual_dictionary_without_a_fake_device() {
    let session = null_session();
    let response = session.graphics_information();
    assert_eq!(response, "OpenGL info:\n");
}
#[test]
fn source_raytrace_and_msaa_commands_preserve_unrequested_ray_options() {
    let session = null_session();
    let before = session.view().rendering_params();
    session.set_sample_count(8).unwrap();
    assert_eq!(session.view().rendering_params().nb_msaa_samples, 8);
    session.select_ray_tracing().unwrap();
    session.set_sample_count(0).unwrap();
    let after = session.view().rendering_params();
    assert_eq!(after.nb_msaa_samples, 0);
    assert_eq!(after.method, Graphic3dRenderingMode::Raytracing);
    assert_eq!(
        after.ray_tracing.is_reflection_enabled,
        before.ray_tracing.is_reflection_enabled
    );
    assert_eq!(
        after.ray_tracing.is_antialiasing_enabled,
        before.ray_tracing.is_antialiasing_enabled
    );
}
fn two_objects() -> ViewerSession {
    let mut session = null_session();
    let mut builder =
        martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::from_sizes(1., 1., 1.).unwrap();
    let cube = builder.shape().unwrap();
    session
        .display("color", cube, &DisplayRequest::default())
        .unwrap();
    session
        .display("other", cube, &DisplayRequest::default())
        .unwrap();
    session
}
fn shading(session: &ViewerSession, name: &str) -> Graphic3dTypeOfShadingModel {
    session
        .object(name)
        .unwrap()
        .attributes()
        .shading_aspect()
        .aspect()
        .shading_model()
}
#[test]
fn own_unlit_shading_keeps_context_defaults_and_other_objects() {
    let session = two_objects();
    let defaults = session.context().default_drawer().shading_aspect().aspect();
    let before = defaults.shading_model();
    let other = shading(&session, "other");
    session
        .set_object_shading("color", Graphic3dTypeOfShadingModel::Unlit)
        .unwrap();
    assert_eq!(defaults.shading_model(), before);
    assert_eq!(
        shading(&session, "color"),
        Graphic3dTypeOfShadingModel::Unlit
    );
    assert_eq!(shading(&session, "other"), other);
    assert!(
        session
            .set_object_shading("absent", Graphic3dTypeOfShadingModel::Unlit)
            .is_err()
    );
}
