//! Constructor profiles and exact polynomial expectations are independent of pixels.
use super::{
    bezier_api::{Planar, Spatial},
    planar, spatial,
};
#[test]
fn bezier_defaults_keep_dimension_specific_sampling_and_shared_geometry() {
    let space = Spatial::new(spatial());
    let plane = Planar::new(planar());
    assert!(space.poles().shown() && plane.poles().shown());
    assert_eq!(super::bezier_api::settings_space(&space), (16, 0.05, 1));
    assert_eq!(super::bezier_api::settings_plane(&plane), (50, 0.01, 0));
    let point = space.curve_part().curve().get().unwrap().d0(0.5).unwrap();
    assert_eq!((point.x(), point.y(), point.z()), (0., 2.5, 0.));
    let point = plane.curve_part().curve().get().unwrap().d0(0.5).unwrap();
    assert_eq!((point.x(), point.y()), (0., 2.5));
    assert_send::<Spatial>();
    assert_send::<Planar>();
}
fn assert_send<T: Send>() {}
