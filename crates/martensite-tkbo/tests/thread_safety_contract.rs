//! Compile-time Send and Sync contract assertions for core public kernel types (#61).
use martensite_tkbo::bop_algo::{BOPAlgoBOP, BOPAlgoBuilder, BOPAlgoPaveFiller, Operation};
use martensite_tkbrep::topods::{Compound, Edge, Face, Shape, Shell, Solid, Vertex, Wire};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::{BaseAllocator, List, PackedMap, Sequence, SparseArray};
use martensite_tkernel::standard::Failure;
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dLine};
use martensite_tkg3d::geom::{
    GeomCircle, GeomCylindricalSurface, GeomLine, GeomPlane, GeomSphericalSurface,
};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};
use martensite_tkmath::gp::{
    Axis1, Axis2, Axis3, Direction2, Direction3, Matrix3, Plane3, Point2, Point3, Quaternion,
    Transform3, Vector2, Vector3, Xy, Xyz,
};

const fn assert_send<T: Send>() {}
const fn assert_sync<T: Sync>() {}
const fn assert_send_sync<T: Send + Sync>() {}

const _: () = {
    // Foundation (martensite-tkernel)
    assert_send_sync::<Failure>();
    assert_send_sync::<BaseAllocator>();
    assert_send::<ProgressRange>();
    assert_send_sync::<List<Shape>>();
    assert_send_sync::<Sequence<Shape>>();
    assert_send_sync::<PackedMap<u32>>();
    assert_send_sync::<SparseArray<i32>>();

    // Math primitives (martensite-tkmath)
    assert_send_sync::<Point2>();
    assert_send_sync::<Point3>();
    assert_send_sync::<Direction2>();
    assert_send_sync::<Direction3>();
    assert_send_sync::<Vector2>();
    assert_send_sync::<Vector3>();
    assert_send_sync::<Xy>();
    assert_send_sync::<Xyz>();
    assert_send_sync::<Axis1>();
    assert_send_sync::<Axis2>();
    assert_send_sync::<Axis3>();
    assert_send_sync::<Plane3>();
    assert_send_sync::<Matrix3>();
    assert_send_sync::<Transform3>();
    assert_send_sync::<Quaternion>();

    // Geometry (martensite-tkg2d, martensite-tkg3d)
    assert_send_sync::<Geom2dLine>();
    assert_send_sync::<Geom2dCircle>();
    assert_send_sync::<GeomLine>();
    assert_send_sync::<GeomCircle>();
    assert_send_sync::<GeomPlane>();
    assert_send_sync::<GeomCylindricalSurface>();
    assert_send_sync::<GeomSphericalSurface>();
    assert_send_sync::<ShapeEnum>();
    assert_send_sync::<Orientation>();
    assert_send_sync::<State>();

    // Topology (martensite-tkbrep)
    assert_send_sync::<Shape>();
    assert_send_sync::<Vertex>();
    assert_send_sync::<Edge>();
    assert_send_sync::<Wire>();
    assert_send_sync::<Face>();
    assert_send_sync::<Shell>();
    assert_send_sync::<Solid>();
    assert_send_sync::<Compound>();

    // Boolean operations (martensite-tkbo)
    assert_send_sync::<Operation>();
    assert_send::<BOPAlgoPaveFiller>();
    assert_send::<BOPAlgoBuilder>();
    assert_send::<BOPAlgoBOP>();
    assert_sync::<Operation>();
};

#[test]
fn compile_time_thread_safety_assertions_hold() {
    // Verified at compile time in the const block above.
}
