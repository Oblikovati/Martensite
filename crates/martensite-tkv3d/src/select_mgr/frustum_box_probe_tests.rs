//! The probe answers every box test as the live volume does, and only for that volume.
use super::*;
use crate::select_basics::SelectBasicsSelectingVolumeManager;
use martensite_tkernel::{ncollection::Vec3, standard::Handle};
use martensite_tkmath::gp::{Point2, Point3, Xy};
use martensite_tkservice::graphic3d::{Graphic3dCamera, Graphic3dCameraProjection};

fn camera(projection: Graphic3dCameraProjection) -> Handle<Graphic3dCamera> {
    let camera = Handle::new(Graphic3dCamera::default());
    camera.set_projection_type(projection);
    camera
        .set_eye_and_center(&Point3::new(0., 0., 10.), &Point3::default())
        .expect("eye and center");
    camera.set_z_range(0.1, 100.).expect("z range");
    camera.set_scale(20.);
    camera
}

fn point_volume(projection: Graphic3dCameraProjection) -> SelectMgrSelectingVolumeManager {
    let mut volume = SelectMgrSelectingVolumeManager::new();
    volume.init_point_selecting_volume(Point2::from(Xy::new(400., 300.)));
    volume.set_camera(&camera(projection)).expect("camera");
    volume.set_window_size(800, 600).expect("window size");
    volume.set_pixel_tolerance(2).expect("tolerance");
    volume.set_viewport(0., 0., 1., 1.).expect("viewport");
    volume.build_selecting_volume().expect("build");
    volume
}

/// Boxes around, beside, behind and across the picking ray.
fn boxes() -> Vec<Graphic3dBndBox3d> {
    let cube = |x: f64, y: f64, z: f64, half: f64| {
        Graphic3dBndBox3d::from_corners(
            Vec3::new(x - half, y - half, z - half),
            Vec3::new(x + half, y + half, z + half),
        )
    };
    vec![
        cube(0., 0., 0., 0.5),
        cube(3., 0., 0., 0.5),
        cube(0., -4., 2., 1.),
        cube(0.05, 0.05, -50., 0.01),
        cube(0., 0., 200., 1.),
        cube(0., 0., 0., 100.),
    ]
}

#[test]
fn the_probe_answers_each_box_as_the_live_volume() {
    for projection in [
        Graphic3dCameraProjection::Orthographic,
        Graphic3dCameraProjection::Perspective,
    ] {
        let volume = point_volume(projection);
        let probe = volume.box_probe().expect("a built rectangular volume");
        for bounds in boxes() {
            let live = volume.overlaps_box_inside(bounds.corner_min(), bounds.corner_max(), None);
            assert_eq!(
                probe.overlaps(&bounds).ok(),
                live.ok(),
                "{projection:?} {bounds:?}"
            );
        }
    }
}

#[test]
fn a_probe_reads_only_its_own_intersector() {
    let volume = point_volume(Graphic3dCameraProjection::Orthographic);
    let probe = volume.box_probe().expect("a built rectangular volume");
    assert!(
        probe.reads(&volume.clone()),
        "a copy shares the intersector"
    );
    let other = point_volume(Graphic3dCameraProjection::Orthographic);
    assert!(
        !probe.reads(&other),
        "another volume has its own intersector"
    );
}

#[test]
fn volumes_without_a_ready_rectangle_have_no_probe() {
    assert!(
        SelectMgrSelectingVolumeManager::new().box_probe().is_none(),
        "no volume"
    );
    let mut unset = SelectMgrSelectingVolumeManager::new();
    unset.init_selecting_volume(&Handle::new(SelectMgrRectangularFrustum::new()).cast());
    assert!(
        unset.box_probe().is_none(),
        "a rectangle without a selection type"
    );
}
