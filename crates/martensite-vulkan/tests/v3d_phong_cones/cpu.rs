//! Source preparation inspects actual objects and material program state on the Null driver.
use super::*;
use martensite_tkservice::graphic3d::Graphic3dAspectsRenderMembers;

#[test]
fn full_source_keeps_both_meshing_defaults_and_only_the_second_program() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    prepare_source(&mut session).unwrap();
    let drawer = session.context().default_drawer();
    assert_eq!(
        drawer.type_of_deflection(),
        AspectTypeOfDeflection::Relative
    );
    assert_eq!(drawer.maximal_chordial_deviation(), 10.);
    assert_eq!(drawer.deviation_coefficient(), 0.1);
    assert_object_programs(&session);
}

fn assert_object_programs(session: &ViewerSession) {
    let gouraud = session.object("p_gouraud").unwrap();
    let phong = session.object("p_phong").unwrap();
    assert_ne!(gouraud, phong);
    assert_eq!((gouraud.display_mode(), phong.display_mode()), (1, 1));
    assert!(
        gouraud
            .attributes()
            .shading_aspect()
            .aspect()
            .shader_program()
            .is_null()
    );
    assert_phong_key(phong);
}

fn assert_phong_key(phong: &Handle<dyn martensite_tkv3d::ais::AisInteractiveObject>) {
    let actual = phong
        .attributes()
        .shading_aspect()
        .aspect()
        .shader_program();
    assert!(!actual.is_null());
    let expected = martensite_vulkan::phong_shading_program();
    assert_eq!(
        martensite_vulkan::UserProgramKey::of(&actual),
        martensite_vulkan::UserProgramKey::of(&expected)
    );
}
