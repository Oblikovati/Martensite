//! The layer origin moves from the model translation into the world view.
use super::LayerOrigin;
use martensite_tkernel::ncollection::Mat4;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Point3, Transform3, Vector3, Xyz};
use martensite_tkmath::toploc::Datum3;
use martensite_tkservice::graphic3d::{Graphic3dCamera, Graphic3dZLayerSettings};

const FAR: Xyz = Xyz::new(1.0e6, 0.0, 0.0);

fn far_camera() -> Graphic3dCamera {
    let camera = Graphic3dCamera::default();
    camera
        .set_eye_and_center(
            &Point3::new(1.0e6 + 0.5, -0.5, 0.5),
            &Point3::new(1.0e6, 0.0, 0.0),
        )
        .unwrap();
    camera
}

#[test]
fn the_default_layer_has_no_origin_and_a_moved_one_has() {
    let mut settings = Graphic3dZLayerSettings::default();
    assert_eq!(LayerOrigin::of(&settings), None);
    settings.set_origin(FAR);
    assert_eq!(LayerOrigin::of(&settings), Some(LayerOrigin(FAR)));
}

#[test]
fn the_local_world_view_is_the_world_view_translated_by_the_origin() {
    let camera = far_camera();
    let local = LayerOrigin(FAR).local_camera(&camera);
    let mut shift = Mat4::<f64>::identity();
    shift.set_column(
        3,
        martensite_tkernel::ncollection::Vec4::new(1.0e6, 0., 0., 1.),
    );
    let expected = camera.orientation_matrix() * shift;
    let actual = local.orientation_matrix();
    for (got, want) in actual.data().iter().zip(expected.data()) {
        assert!(
            (got - want).abs() < 1e-9,
            "local world view {got} expected {want}"
        );
    }
    assert_eq!(local.direction(), camera.direction());
    assert_eq!(local.distance(), camera.distance());
}

#[test]
fn the_local_model_translation_subtracts_the_origin() {
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(1.0e6 + 0.25, 2.0, -3.0));
    let local = LayerOrigin(FAR).local_transformation(&Handle::new(Datum3::new(transform)));
    let moved = local.get().unwrap().transformation().translation_part();
    assert_eq!(moved, Xyz::new(0.25, 2.0, -3.0));
    let untransformed = LayerOrigin(FAR).local_transformation(&Handle::default());
    let moved = untransformed
        .get()
        .unwrap()
        .transformation()
        .translation_part();
    assert_eq!(moved, Xyz::new(-1.0e6, 0.0, 0.0));
}
