//! Clipping-plane snapshots relative to a Z layer's local origin.
use super::*;
use martensite_tkernel::ncollection::Vec4;

fn plane(a: f64, b: f64, c: f64, d: f64) -> Handle<Graphic3dClipPlane> {
    Handle::new(Graphic3dClipPlane::from_equation(Vec4::new(a, b, c, d)).unwrap())
}

#[test]
fn layer_origin_recomputes_w_from_the_plane_location_in_double() {
    let far = plane(1., 0., 0., -(1.0e6 + 1.));
    assert_eq!(
        snapshot_chain(far.clone()).unwrap(),
        vec![[1., 0., 0., -1_000_001.]]
    );
    let local = snapshot_chain_around(far, Some([1.0e6, 0., 0.])).unwrap();
    assert_eq!(local, vec![[1., 0., 0., -1.]]);
}

#[test]
fn layer_origin_applies_to_every_plane_of_a_chain() {
    let first = plane(0., 1., 0., -3.);
    Graphic3dClipPlane::set_chain_next_plane(&first, plane(0., 0., 1., 2.)).unwrap();
    let local = snapshot_chain_around(first, Some([10., 1., -1.])).unwrap();
    assert_eq!(local, vec![[0., 1., 0., -2.], [0., 0., 1., 1.]]);
}

#[test]
fn public_local_plane_w_matches_the_layer_snapshot() {
    let far = plane(1., 0., 0., -(1.0e6 + 1.));
    assert_eq!(RenderClipping::local_plane_w(&far, [1.0e6, 0., 0.]), -1.);
    assert_eq!(RenderClipping::local_plane_w(&far, [0.; 3]), -1_000_001.);
}
