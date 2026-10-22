//! Postprocessing updates only faces whose boundary wires were visited.
use martensite_tkbrep::{brep::BRepBuilder, topods::Face};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
use martensite_tkmath::gp::{Axis3, Plane3};
use martensite_tkshhealing::shape_upgrade::ShapeUpgradeShapeConvertToBezier;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

fn wireless_plane_face() -> Face {
    let support: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).cast();
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_on_surface(&mut face, &support, 1e-7)
        .unwrap();
    face
}

#[test]
fn public_conversion_preserves_unchecked_wireless_plane_face() {
    let face = wireless_plane_face();
    face.set_checked(false).unwrap();
    let converter = ShapeUpgradeShapeConvertToBezier::new(face.clone().into_shape());
    converter.set_3d_conversion(true);
    assert!(!converter.perform(true).unwrap());
    assert!(!face.checked().unwrap());
    assert!(!converter.result().checked().unwrap());
    assert!(converter.result().is_same(&face.into_shape()));
}

#[test]
fn public_conversion_updates_bounded_result_without_checking_source() {
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., 2., 0., 3.))
        .unwrap()
        .face()
        .unwrap();
    face.set_checked(false).unwrap();
    let converter = ShapeUpgradeShapeConvertToBezier::new(face.clone().into_shape());
    converter.set_3d_conversion(true);
    assert!(converter.perform(true).unwrap());
    assert!(!face.checked().unwrap());
    assert!(converter.result().checked().unwrap());
    assert!(!converter.result().is_same(&face.into_shape()));
}
