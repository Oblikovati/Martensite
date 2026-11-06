#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_geom::*;

#[test]
fn init_round_trip_step_visual_view_volume() {
    let a_projection_type = StepVisualCentralOrParallel::default();
    let a_projection_point = Handle::new(StepGeomCartesianPoint::new());
    let a_view_plane_distance = 2.5;
    let a_front_plane_distance = 2.5;
    let a_front_plane_clipping = true;
    let a_back_plane_distance = 2.5;
    let a_back_plane_clipping = true;
    let a_view_volume_sides_clipping = true;
    let a_view_window = Handle::new(StepVisualPlanarBox::new());
    let object = StepVisualViewVolume::new();
    object.init(
        a_projection_type,
        &a_projection_point,
        a_view_plane_distance,
        a_front_plane_distance,
        a_front_plane_clipping,
        a_back_plane_distance,
        a_back_plane_clipping,
        a_view_volume_sides_clipping,
        &a_view_window,
    );
    assert!(object.projection_type() == a_projection_type);
    assert!(object.projection_point() == a_projection_point);
    assert!(object.view_plane_distance() == a_view_plane_distance);
    assert!(object.front_plane_distance() == a_front_plane_distance);
    assert!(object.front_plane_clipping() == a_front_plane_clipping);
    assert!(object.back_plane_distance() == a_back_plane_distance);
    assert!(object.back_plane_clipping() == a_back_plane_clipping);
    assert!(object.view_volume_sides_clipping() == a_view_volume_sides_clipping);
    assert!(object.view_window() == a_view_window);
}
