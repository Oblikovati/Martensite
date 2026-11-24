//! A cloned draw shares its body; the per-frame fields change without copying it, and a body
//! write copies it first, so the other owner keeps what it had (#594).
use crate::{PrimitiveDraw, RenderVertex};
use std::sync::Arc;

fn triangle() -> PrimitiveDraw {
    let vertex = |x: f32| RenderVertex::new([x, 0., 0.], [1.; 4]);
    PrimitiveDraw::new(
        vec![vertex(0.), vertex(1.), vertex(0.5)],
        vec![0, 1, 2],
        crate::identity_clip(),
    )
    .unwrap()
}

#[test]
fn per_frame_fields_change_without_copying_the_shared_body() {
    let recorded = triangle();
    let mut frame = recorded.clone();
    frame.clip_transform[12] = 3.;
    frame.depth_clamp = true;
    assert!(Arc::ptr_eq(&recorded.body, &frame.body));
    assert_eq!(recorded.clip_transform[12], 0.);
}

#[test]
fn a_body_write_copies_the_body_and_leaves_the_other_owner_unchanged() {
    let recorded = triangle();
    let mut frame = recorded.clone();
    frame.surface_blend = true;
    assert!(!Arc::ptr_eq(&recorded.body, &frame.body));
    assert!(!recorded.surface_blend && frame.surface_blend);
}

#[test]
fn a_lighting_view_change_keeps_the_lighting_body_shared() {
    let mut recorded = triangle();
    let lighting = crate::RenderLighting::new(
        vec![[0., 0., 1.]; 3],
        Default::default(),
        crate::identity_clip(),
        crate::identity_clip(),
        false,
        crate::RenderShading::Phong,
    )
    .unwrap();
    recorded.lighting = Some(lighting);
    let mut frame = recorded.clone();
    frame.lighting.as_mut().unwrap().eye = [1., 0., 0., 0.];
    let (a, b) = (recorded.lighting.unwrap(), frame.lighting.unwrap());
    assert!(Arc::ptr_eq(&a.body, &b.body));
    assert_ne!(a.eye, b.eye);
}
