//! Parsed arrays own their shading before presentation; later material/color edits keep that handle.
use super::*;
use crate::viewer_objects::PrimitiveArrayObject;
use martensite_tkservice::graphic3d::{
    Graphic3dArrayOfQuadrangles, Graphic3dAspectsMembers as _, Graphic3dAspectsSource,
    Graphic3dMaterialAspect, Graphic3dStructureMembers as _,
};
use martensite_tkv3d::ais::PrimitiveArrayPresentation;

fn displayed_parsed_quad() -> ViewerSession {
    let mut session = cpu_session();
    objects::display_array(&mut session, "quad", objects::quad_array().unwrap()).unwrap();
    session
}

fn displayed_aspect(session: &ViewerSession, name: &str) -> Handle<dyn Graphic3dAspectsSource> {
    let object = named(session, name).unwrap();
    let prs = session
        .context()
        .main_prs_mgr()
        .presentation(&object.cast(), 0, false, &Handle::default())
        .unwrap();
    let groups = prs.groups();
    assert_eq!(groups.len(), 1);
    groups[0].aspects()
}

fn owned_aspect(session: &ViewerSession, name: &str) -> Handle<dyn Graphic3dAspectsSource> {
    named(session, name)
        .unwrap()
        .attributes()
        .shading_aspect()
        .aspect()
        .cast()
}

fn assert_command_material(aspect: &dyn Graphic3dAspectsSource) {
    let material = aspect.front_material();
    let half = Color::from_vec3(Vec3::new(0.5, 0.5, 0.5)).unwrap();
    assert_eq!(aspect.interior_color(), Color::named(NamedColor::WHITE));
    assert_eq!(material.ambient_color(), half);
    assert_eq!(material.diffuse_color(), half);
    assert_eq!(material.specular_color(), Color::named(NamedColor::BLACK));
    assert_eq!(material.emissive_color(), Color::named(NamedColor::BLACK));
}

#[test]
fn parsed_colored_array_owns_shading_before_initial_presentation() {
    let session = displayed_parsed_quad();
    let object = named(&session, "quad").unwrap();
    assert!(object.attributes().has_own_shading_aspect());
    let group = displayed_aspect(&session, "quad");
    assert_eq!(
        group.object_address(),
        owned_aspect(&session, "quad").object_address()
    );
    assert_command_material(&*group);
}

#[test]
fn parsed_group_retains_its_material_color_handle_after_source_edits() {
    let session = displayed_parsed_quad();
    let group = displayed_aspect(&session, "quad");
    session.set_material("quad", commands::PLASTIC).unwrap();
    assert_eq!(
        group.front_material(),
        Graphic3dMaterialAspect::new(commands::PLASTIC)
    );
    session
        .set_color("quad", Color::named(NamedColor::PALEGREEN2))
        .unwrap();
    let own = owned_aspect(&session, "quad");
    assert_eq!(group.object_address(), own.object_address());
    assert_eq!(group.front_material(), own.front_material());
    assert_eq!(group.back_material(), own.back_material());
    assert_eq!(group.interior_color(), Color::named(NamedColor::PALEGREEN2));
}

fn displayed_raw_quad() -> ViewerSession {
    let mut session = cpu_session();
    let object = PrimitiveArrayObject::new(objects::quad_array().unwrap());
    session.display_object("raw", object.cast()).unwrap();
    session
}

fn assert_raw_group_stays_linked(
    session: &ViewerSession,
    before: &Handle<dyn Graphic3dAspectsSource>,
) {
    let group = displayed_aspect(session, "raw");
    let owned = owned_aspect(session, "raw");
    assert_eq!(group.object_address(), before.object_address());
    assert_ne!(group.object_address(), owned.object_address());
    assert_ne!(group.front_material(), owned.front_material());
}

#[test]
fn legacy_raw_array_keeps_linked_group_when_material_creates_owned_aspect() {
    let session = displayed_raw_quad();
    let group = displayed_aspect(&session, "raw");
    assert!(
        !named(&session, "raw")
            .unwrap()
            .attributes()
            .has_own_shading_aspect()
    );
    session.set_material("raw", commands::PLASTIC).unwrap();
    session
        .set_color("raw", Color::named(NamedColor::PALEGREEN2))
        .unwrap();
    assert_raw_group_stays_linked(&session, &group);
}

fn uncolored_quad() -> Handle<martensite_tkservice::graphic3d::Graphic3dArrayOfPrimitives> {
    let array =
        Handle::new(Graphic3dArrayOfQuadrangles::with_options(4, 0, false, false, false).unwrap());
    for point in objects::CORNERS {
        let point = martensite_tkmath::gp::Point3::new(point[0], point[1], point[2]);
        array.add_vertex_pnt(&point).unwrap();
    }
    array.cast()
}

#[test]
fn parsed_uncolored_array_keeps_linked_shading_without_color_initialization() {
    let mut session = cpu_session();
    objects::display_array(&mut session, "plain", uncolored_quad()).unwrap();
    let object = named(&session, "plain").unwrap();
    assert!(!object.attributes().has_own_shading_aspect());
    assert_eq!(
        displayed_aspect(&session, "plain").object_address(),
        owned_aspect(&session, "plain").object_address()
    );
}

fn assert_quad_sensitive(session: &ViewerSession, name: &str) {
    let object = named(session, name).unwrap();
    let entity = sensitive(session, name);
    assert!(!entity.cast::<Select3DSensitivePrimitiveArray>().is_null());
    assert_eq!(entity.sensitivity_factor(), 8);
    assert_eq!(entity.nb_sub_elements(), 4);
    assert_eq!(
        entity.owner_id().selectable().object_address(),
        object.object_address()
    );
}

#[test]
fn parsed_and_raw_quad_keep_primitive_channels_and_real_selection() {
    let mut session = displayed_parsed_quad();
    let object = named(&session, "quad")
        .unwrap()
        .cast::<PrimitiveArrayPresentation>();
    let array = object.primitive_array();
    assert_eq!(array.vertex_number(), 4);
    assert!(!array.has_vertex_normals());
    assert_eq!(array.vertex_color_rgb(4).unwrap(), [0., 0., 1.]);
    assert_eq!(array.vertice(1).unwrap().x(), 175.);
    let raw = PrimitiveArrayObject::new(objects::quad_array().unwrap());
    session.display_object("raw", raw.cast()).unwrap();
    assert_quad_sensitive(&session, "quad");
    assert_quad_sensitive(&session, "raw");
}

#[test]
fn parsed_and_raw_persistent_quad_pick_corners_but_keep_interior_unselectable() {
    for raw in [false, true] {
        let (mut session, name) = if raw {
            (displayed_raw_quad(), "raw")
        } else {
            (displayed_parsed_quad(), "quad")
        };
        commands::persistent_existing(&session, name, Some(1)).unwrap();
        assert_named_detection(&mut session, (376, 100), name);
        session.move_cursor(300, 200).unwrap();
        assert!(session.context().detected_owner().is_null());
    }
}
