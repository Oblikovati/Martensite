//! A moved structure must leave neither layer's cached extents behind (#353).
use super::*;
use martensite_tkmath::bnd::BoundingBox3;

type RetainedLayerOwners = [Handle<dyn Graphic3dStructureSource>; 2];

fn bounds_target_scene() -> Scene {
    let scene = scene();
    scene
        .driver
        .insert_layer_after(1, &Graphic3dZLayerSettings::default(), 0)
        .expect("bounds target layer");
    scene
        .view
        .insert_layer_after(1, &Graphic3dZLayerSettings::default(), 0)
        .expect("view bounds target layer");
    assert!(!scene.view.layer(1).is_null());
    scene
}
fn moved_structure_with_cached_bounds() -> (Scene, RetainedLayerOwners) {
    let scene = bounds_target_scene();
    let seed = structure_with_box(&scene, [-0.25, -0.25, -0.25, 0.25, 0.25, 0.25]);
    seed.set_z_layer(1);
    seed.display();
    let moving = structure_with_box(&scene, [0., 0., -0.5, 0.5, 0.5, 0.]);
    moving.display();
    scene
        .view
        .min_max_values(false)
        .expect("populate bounds caches");
    moving.set_z_layer(1);
    // Layer entries borrow their structures, so keep both owners until each assertion ends.
    (scene, [seed, moving])
}
fn retained_bounds(scene: &Scene, layer: i32) -> BoundingBox3 {
    scene
        .view
        .layer(layer)
        .bounding_box(
            scene.view.identification(),
            &scene.view.camera(),
            [800, 600],
            false,
        )
        .expect("retained layer bounds")
}
#[test]
fn moving_a_structure_invalidates_the_source_layer_bounds() {
    let (scene, _owners) = moved_structure_with_cached_bounds();
    assert_eq!(scene.view.layer(0).nb_structures(), 0);
    assert!(retained_bounds(&scene, 0).is_void());
}
#[test]
fn moving_a_structure_invalidates_the_destination_layer_bounds() {
    let (scene, _owners) = moved_structure_with_cached_bounds();
    assert_eq!(scene.view.layer(1).nb_structures(), 2);
    assert_eq!(
        retained_bounds(&scene, 1)
            .corner_min()
            .expect("minimum")
            .z(),
        -0.5
    );
}
#[test]
fn an_explicit_refresh_confirms_the_moved_membership_bounds() {
    let (scene, _owners) = moved_structure_with_cached_bounds();
    scene.view.update(0);
    scene.view.update(1);
    assert!(retained_bounds(&scene, 0).is_void());
    assert_eq!(
        retained_bounds(&scene, 1)
            .corner_min()
            .expect("minimum")
            .z(),
        -0.5
    );
}
