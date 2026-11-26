// SPDX-License-Identifier: GPL-2.0-only
//! Concurrent lock-free read and copy-on-write builder tests for published geometry (#65).
use martensite_tkg2d::published::PublishedCurve2d;
use martensite_tkg3d::published::{PublishedCurve3d, PublishedSurface};
use martensite_tkmath::gp::{
    Axis2, Axis3, Circle3, Cylinder3, Direction3, Point3, Transform3, Vector3,
};
use std::sync::Barrier;
use std::thread;

#[test]
fn published_geometry_types_are_send_sync_without_locks() {
    fn assert_send_sync<T: Send + Sync + 'static>() {}
    assert_send_sync::<PublishedCurve2d>();
    assert_send_sync::<PublishedCurve3d>();
    assert_send_sync::<PublishedSurface>();
}

#[test]
fn concurrent_readers_see_consistent_values_while_writer_builds_edited_copy() {
    let circle = PublishedCurve3d::circle(
        Circle3::new(
            Axis2::new(Point3::new(0.0, 0.0, 0.0), Direction3::Z).unwrap(),
            10.0,
        )
        .unwrap(),
    );
    let cylinder = PublishedSurface::cylinder(Cylinder3::new(Axis3::default(), 5.0).unwrap());

    let barrier = std::sync::Arc::new(Barrier::new(5));
    let mut readers = Vec::new();
    for _ in 0..4 {
        let curve = circle.clone();
        let surf = cylinder.clone();
        let start = std::sync::Arc::clone(&barrier);
        readers.push(thread::spawn(move || {
            start.wait();
            for step in 0..2_000 {
                let u = (step as f64) * 0.003;
                let p = curve.d0(u);
                let radius = (p.x() * p.x() + p.y() * p.y()).sqrt();
                assert!((radius - 10.0).abs() < 1e-12);
                assert!(p.z().abs() < 1e-12);

                let s = surf.d0(u, 2.0);
                let sr = (s.x() * s.x() + s.y() * s.y()).sqrt();
                assert!((sr - 5.0).abs() < 1e-12);
                assert!((s.z() - 2.0).abs() < 1e-12);
            }
        }));
    }

    let writer_curve = circle.clone();
    let writer_surf = cylinder.clone();
    let start = std::sync::Arc::clone(&barrier);
    let writer = thread::spawn(move || {
        start.wait();
        let mut latest_curve = writer_curve;
        let mut latest_surf = writer_surf;
        for step in 1..=20 {
            let mut trsf = Transform3::default();
            trsf.set_translation(Vector3::new(0.0, 0.0, 1.0));
            let mut cb = latest_curve.to_builder();
            cb.transform(&trsf).unwrap();
            latest_curve = cb.publish();

            let mut sb = latest_surf.to_builder();
            sb.transform(&trsf);
            latest_surf = sb.publish();

            assert!((latest_curve.d0(0.0).z() - step as f64).abs() < 1e-12);
            assert!((latest_surf.d0(0.0, 0.0).z() - step as f64).abs() < 1e-12);
        }
        (latest_curve, latest_surf)
    });

    for reader in readers {
        reader.join().unwrap();
    }
    let (edited_curve, edited_surf) = writer.join().unwrap();
    // Original published handles remain completely unchanged.
    assert!(circle.d0(0.0).z().abs() < 1e-12);
    assert!(cylinder.d0(0.0, 0.0).z().abs() < 1e-12);
    // Edited published handles reflect all 20 copy-on-write edits.
    assert!((edited_curve.d0(0.0).z() - 20.0).abs() < 1e-12);
    assert!((edited_surf.d0(0.0, 0.0).z() - 20.0).abs() < 1e-12);
}
