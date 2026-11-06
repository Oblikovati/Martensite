//! Plain style values can cross threads; drawable caches and report/pick contexts cannot.
use martensite_geometry_raster::{HlrSettings, ShapeColors, TopologyStyle, edge_face::FaceIso};

fn send_sync<T: Send + Sync>() {}

#[test]
fn immutable_style_values_are_send_and_sync() {
    send_sync::<martensite_geometry_raster::CurveLook>();
    send_sync::<martensite_geometry_raster::curve_properties::CurvatureDisplay>();
    send_sync::<ShapeColors>();
    send_sync::<HlrSettings>();
    send_sync::<TopologyStyle>();
    send_sync::<FaceIso>();
}

fn movable<T: Send>() {}
#[test]
fn curve_records_and_retained_objects_can_move_between_threads() {
    movable::<martensite_geometry_raster::SpatialCurveRendering>();
    movable::<martensite_geometry_raster::PlanarCurveRendering>();
    movable::<martensite_geometry_raster::SpatialCurveDrawable>();
    movable::<martensite_geometry_raster::PlanarCurveDrawable>();
}
