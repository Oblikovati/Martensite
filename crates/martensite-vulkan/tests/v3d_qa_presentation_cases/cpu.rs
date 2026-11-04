//! CPU scene invariants grant no pixel or distinct Vulkan buffer-path coverage.
use super::*;
use martensite_tkernel::quantity::Color;
use martensite_tkg3d::geom::GeomAxisPlacement as _;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::{graphic3d::Graphic3dNameOfMaterial, null_driver::NullGraphicDriver};
use martensite_tkv3d::{
    ais::AisTrihedronSource as _,
    prs_mgr::{PrsMgrPresentableObject as _, PrsMgrPresentableObjectMembers as _},
};

fn null_session() -> ViewerSession {
    let driver = NullGraphicDriver::new_handle();
    ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap()
}

#[test]
fn datum_has_authored_placement_size_and_colour() {
    let datum = ordered::source_datum().unwrap();
    let position = datum.component().location();
    assert_eq!([position.x(), position.y(), position.z()], [100., 100., 0.]);
    assert_eq!(datum.size(), 100.);
    assert_eq!(datum.color(), Color::from_name("LIGHTSTEELBLUE4").unwrap());
}

#[test]
fn shaded_box_has_source_mode_material_and_colour_before_display() {
    let object = ordered::source_box().unwrap();
    assert_eq!(object.display_mode(), 1);
    assert_eq!(object.material(), Graphic3dNameOfMaterial::Plastified);
    assert_eq!(object.color(), Color::from_name("RED").unwrap());
}

#[test]
fn actual_context_relocation_preserves_the_source_translation() {
    let session = null_session();
    let object = ordered::ordered_objects(&session).unwrap();
    let translation = object.local_transformation().translation_part();
    assert_eq!(
        [translation.x(), translation.y(), translation.z()],
        [20., 20., 0.]
    );
    assert!(session.context().is_displayed(&object.cast()));
    assert_eq!(session.context().nb_selected(), 0);
}

#[test]
fn authored_camera_scale_and_target_are_applied_to_actual_view() {
    let session = null_session();
    ordered::source_camera(&session).unwrap();
    let camera = session.view().camera();
    let target = camera.center();
    // The view parameters scale the default 1000-unit view, not the camera extent directly.
    assert_eq!(camera.scale(), 1000. / 4.9487928);
    let requested = Point3::new(32.4556665273951, 52.9347942181675, -5.1270029887922);
    // Center is reconstructed from eye, direction and distance. This owned-fixture
    // rounding allowance is not a certified bound or an original pixel tolerance.
    let rounding_allowance = 16. * f64::EPSILON * camera.distance();
    assert!(target.distance(&requested) <= rounding_allowance);
}

#[test]
fn polyline_contains_all_four_vertices_and_one_red_bound() {
    let array = polyline::red_bound_polyline().unwrap();
    assert_eq!(array.vertex_number(), 4);
    assert_eq!(array.bound_number(), 1);
    assert_eq!(array.bound(1).unwrap(), 4);
    assert_eq!(array.bound_color_rgb(1).unwrap(), [1., 0., 0.]);
    let corner = array.vertice(3).unwrap();
    assert_eq!([corner.x(), corner.y(), corner.z()], [0., 10., 10.]);
    assert_eq!(array.vertice(1).unwrap(), array.vertice(4).unwrap());
}

#[test]
fn both_literal_buffer_requests_keep_the_red_array_after_clear() {
    let mut session = null_session();
    let first = polyline::bound_stage(&mut session, false).unwrap();
    session.fit().unwrap();
    session.clear().unwrap();
    let second = polyline::bound_stage(&mut session, true).unwrap();
    assert_ne!(first.object_address(), second.object_address());
    assert_eq!(second.bound_color_rgb(1).unwrap(), [1., 0., 0.]);
    assert!(
        session
            .context()
            .is_displayed(session.object("pl01").unwrap())
    );
}
