//! Retained getter aliases observe actual source slot changes without retaining targets.
use martensite_tkernel::standard::{Handle, HandleReference};
use martensite_tkservice::{graphic3d::Graphic3dCamera, null_driver::NullGraphicDriver};
use martensite_tkv3d::{ais::*, v3d::*};

type GetterViewRow = [i32; 9];
struct GetterViewScene {
    _viewer: Handle<V3dViewer>,
    first: Handle<dyn V3dViewSource>,
    second: Handle<dyn V3dViewSource>,
    first_count: i32,
    second_count: i32,
}
impl GetterViewScene {
    fn new() -> Self {
        let viewer = V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
        let first = viewer.create_view().unwrap();
        let second = viewer.create_view().unwrap();
        Self {
            _viewer: viewer,
            first_count: first.ref_count(),
            second_count: second.ref_count(),
            first,
            second,
        }
    }
}

#[test]
fn concrete_target_getter_preserves_live_slot_and_retention() {
    let animation = AisAnimationCamera::new("getters", &Handle::default());
    let rows = retained_target_rows(&animation);
    assert_target_retention_rows(&rows);
    println!("{{\"getter_kind\":\"concrete_view\",\"rows\":{rows:?}}}");
}

#[test]
fn derived_target_getter_preserves_live_slot_and_retention() {
    let animation = super::ais_animation_camera_source::protocol_camera(&Handle::default());
    let _: &dyn Send = animation.get().unwrap();
    let _: &dyn Sync = animation.get().unwrap();
    let source = animation.cast::<dyn AisAnimationCameraSource>();
    let rows = retained_target_rows(source.get().unwrap());
    assert_target_retention_rows(&rows);
    println!("{{\"getter_kind\":\"derived_view\",\"rows\":{rows:?}}}");
}

fn retained_target_rows(source: &dyn AisAnimationCameraSource) -> Vec<GetterViewRow> {
    let scene = GetterViewScene::new();
    let alias = source.target_view_ref();
    let mut rows = vec![target_observation(0, source, &alias, &scene)];
    source.set_view(&scene.first);
    rows.push(target_observation(1, source, &alias, &scene));
    source.set_view(&scene.first);
    rows.push(target_observation(2, source, &alias, &scene));
    let copied = alias.clone_handle();
    rows.push(target_observation(3, source, &alias, &scene));
    drop(copied);
    rows.push(target_observation(4, source, &alias, &scene));
    source.set_view(&scene.second);
    rows.push(target_observation(5, source, &alias, &scene));
    source.set_view(&Handle::default());
    rows.push(target_observation(6, source, &alias, &scene));
    rows
}

fn target_observation(
    frame: i32,
    source: &dyn AisAnimationCameraSource,
    alias: &impl HandleReference<dyn V3dViewSource>,
    scene: &GetterViewScene,
) -> GetterViewRow {
    [
        frame,
        i32::from(alias.is_null()),
        i32::from(alias.object_address() == scene.first.object_address()),
        i32::from(alias.object_address() == scene.second.object_address()),
        i32::from(source.target_view_ref().reference_address() == alias.reference_address()),
        scene.first.ref_count() - scene.first_count,
        scene.second.ref_count() - scene.second_count,
        scene.first.ref_count(),
        scene.second.ref_count(),
    ]
}

fn assert_target_retention_rows(rows: &[GetterViewRow]) {
    let expected = [
        [0, 1, 0, 0, 1, 0, 0],
        [1, 0, 1, 0, 1, 1, 0],
        [2, 0, 1, 0, 1, 1, 0],
        [3, 0, 1, 0, 1, 2, 0],
        [4, 0, 1, 0, 1, 1, 0],
        [5, 0, 0, 1, 1, 0, 1],
        [6, 1, 0, 0, 1, 0, 0],
    ];
    assert_eq!(rows.len(), expected.len());
    for (observed, expected) in rows.iter().zip(expected) {
        assert_eq!(observed[..7], expected);
    }
}

#[test]
fn concrete_endpoint_getters_preserve_live_slots_and_exact_counts() {
    let animation = AisAnimationCamera::new("getters", &Handle::default());
    let rows = retained_endpoint_rows(&animation);
    assert_endpoint_retention_rows(&rows);
    println!("{{\"getter_kind\":\"concrete_endpoints\",\"rows\":{rows:?}}}");
}

#[test]
fn derived_endpoint_getters_preserve_live_slots_and_exact_counts() {
    let animation = super::ais_animation_camera_source::protocol_camera(&Handle::default());
    let _: &dyn Send = animation.get().unwrap();
    let _: &dyn Sync = animation.get().unwrap();
    let source = animation.cast::<dyn AisAnimationCameraSource>();
    let rows = retained_endpoint_rows(source.get().unwrap());
    assert_endpoint_retention_rows(&rows);
    println!("{{\"getter_kind\":\"derived_endpoints\",\"rows\":{rows:?}}}");
}

fn retained_endpoint_rows(source: &dyn AisAnimationCameraSource) -> Vec<[i32; 7]> {
    let first = Handle::new(Graphic3dCamera::default());
    let second = Handle::new(Graphic3dCamera::default());
    let aliases = (source.camera_start_ref(), source.camera_end_ref());
    let mut rows = vec![endpoint_observation(0, source, &aliases, &first, &second)];
    assign_same_camera(source, &first);
    rows.push(endpoint_observation(1, source, &aliases, &first, &second));
    assign_same_camera(source, &first);
    rows.push(endpoint_observation(2, source, &aliases, &first, &second));
    source.set_camera_start(&second);
    rows.push(endpoint_observation(3, source, &aliases, &first, &second));
    source.set_camera_end(&Handle::default());
    rows.push(endpoint_observation(4, source, &aliases, &first, &second));
    source.set_camera_start(&Handle::default());
    rows.push(endpoint_observation(5, source, &aliases, &first, &second));
    rows
}

fn assign_same_camera(source: &dyn AisAnimationCameraSource, camera: &Handle<Graphic3dCamera>) {
    source.set_camera_start(camera);
    source.set_camera_end(camera);
}

fn endpoint_observation(
    frame: i32,
    source: &dyn AisAnimationCameraSource,
    aliases: &(
        impl HandleReference<Graphic3dCamera>,
        impl HandleReference<Graphic3dCamera>,
    ),
    first: &Handle<Graphic3dCamera>,
    second: &Handle<Graphic3dCamera>,
) -> [i32; 7] {
    [
        frame,
        i32::from(aliases.0.is_null()),
        i32::from(aliases.1.is_null()),
        first.ref_count(),
        second.ref_count(),
        i32::from(aliases.0.reference_address() == source.camera_start_ref().reference_address()),
        i32::from(aliases.1.reference_address() == source.camera_end_ref().reference_address()),
    ]
}

fn assert_endpoint_retention_rows(rows: &[[i32; 7]]) {
    let expected = [
        [0, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 3, 1, 1, 1],
        [2, 0, 0, 3, 1, 1, 1],
        [3, 0, 0, 2, 2, 1, 1],
        [4, 0, 1, 1, 2, 1, 1],
        [5, 1, 1, 1, 1, 1, 1],
    ];
    // The external two handles account for the final count of one, not an alias retain.
    assert_eq!(rows, expected);
}
