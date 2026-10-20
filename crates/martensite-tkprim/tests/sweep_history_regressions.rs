//! Native sweep/history contracts from bug30346_1, bug30346_2 and bug30433.
use martensite_tkbrep::{
    brep_tools::{History, HistoryAlgorithm},
    topods::Shape,
};
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
    storage::IoStream,
};
use martensite_tkg3d::geom::{GeomCurve, GeomEllipse, GeomSphericalSurface, GeomSurface};
use martensite_tkmath::gp::{Axis1, Axis2, Axis3, Direction3, Plane3, Point3, Vector3};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakePrism, BRepPrimAPIMakeRevol};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};
use std::{
    f64::consts::PI,
    io::{Read, Seek, SeekFrom},
};

// This operation-local adapter exercises the existing history constructor, so no
// case session or global last-operation state participates in native Rust tests.
struct SweepRelations<'a>(&'a mut dyn BRepBuilderAPIMakeShape);
impl HistoryAlgorithm for SweepRelations<'_> {
    fn is_deleted(&mut self, shape: &Shape) -> Result<bool, Failure> {
        self.0.is_deleted(shape)
    }
    fn modified(&mut self, shape: &Shape) -> Result<Vec<Shape>, Failure> {
        Ok(self.0.modified(shape)?.iter().cloned().collect())
    }
    fn generated(&mut self, shape: &Shape) -> Result<Vec<Shape>, Failure> {
        Ok(self.0.generated(shape)?.iter().cloned().collect())
    }
}

fn assert_history_counts(
    source: &Shape,
    maker: &mut dyn BRepBuilderAPIMakeShape,
    deleted: i32,
    generated: i32,
) {
    assert!(maker.is_done());
    assert!(!maker.shape().unwrap().is_null());
    let history =
        History::from_algorithm(std::slice::from_ref(source), &mut SweepRelations(maker)).unwrap();
    let text = history_text(&history);
    assert!(
        text.contains(&format!(" - {deleted} Deleted shapes;")),
        "{text}"
    );
    assert!(
        text.contains(&format!(" - {generated} Generated shapes.")),
        "{text}"
    );
}

fn history_text(history: &History) -> String {
    let mut stream = IoStream::from_bytes(Vec::new());
    history.dump(&mut stream);
    let mut device = stream.into_device().unwrap();
    device.seek(SeekFrom::Start(0)).unwrap();
    let mut text = String::new();
    device.read_to_string(&mut text).unwrap();
    text
}

fn rectangular_plane() -> Shape {
    let plane = Plane3::new(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 1., 0.).unwrap(),
    )
    .unwrap();
    BRepBuilderAPIMakeFace::from_plane_bounds(plane, (0., 1., 0., 1.))
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}

fn rotation_axis(x: f64, y: f64, z: f64) -> Axis1 {
    Axis1::new(Point3::new(0., 0., 0.), Direction3::new(x, y, z).unwrap())
}

#[test]
fn partial_and_full_plane_revolutions_preserve_original_history_counts() {
    crate::isolated_primitive_case::run_primitive_case(
        "sweep_history_regressions::partial_and_full_plane_revolutions_preserve_original_history_counts",
        partial_and_full_plane_revolutions_preserve_original_history_counts_body,
    );
}

fn partial_and_full_plane_revolutions_preserve_original_history_counts_body() {
    let source = rectangular_plane();
    let axis = rotation_axis(0., 0., 1.);
    for (angle, deleted, generated) in [(60., 0, 6), (360., 6, 3)] {
        let mut maker =
            BRepPrimAPIMakeRevol::new(&source, axis, angle * (PI / 180.), false).unwrap();
        assert_history_counts(&source, &mut maker, deleted, generated);
    }
}

#[test]
fn full_and_trimmed_spherical_prisms_preserve_original_history_counts() {
    crate::isolated_primitive_case::run_primitive_case(
        "sweep_history_regressions::full_and_trimmed_spherical_prisms_preserve_original_history_counts",
        full_and_trimmed_spherical_prisms_preserve_original_history_counts_body,
    );
}

fn full_and_trimmed_spherical_prisms_preserve_original_history_counts_body() {
    let sphere: Handle<dyn GeomSurface> =
        Handle::new(GeomSphericalSurface::new(Axis3::default(), 1.).unwrap()).cast();
    for (upper_u, generated) in [(2. * PI, 3), (5., 7)] {
        let source = spherical_patch(&sphere, upper_u);
        let mut maker =
            BRepPrimAPIMakePrism::new(&source, Vector3::new(0., 0., 2.), false, true).unwrap();
        assert_history_counts(&source, &mut maker, 0, generated);
    }
}

// Keep the exact decimal endpoints from bug30346_2, including its rounded pole.
#[allow(clippy::approx_constant)]
fn spherical_patch(sphere: &Handle<dyn GeomSurface>, upper_u: f64) -> Shape {
    BRepBuilderAPIMakeFace::from_surface_bounds(
        sphere,
        (0., upper_u, 0.785398163397448, 1.5707963267949),
        precision::confusion(),
    )
    .unwrap()
    .face()
    .unwrap()
    .into_shape()
}

#[test]
fn axis_crossing_elliptical_face_revolution_stays_not_done() {
    crate::isolated_primitive_case::run_primitive_case(
        "sweep_history_regressions::axis_crossing_elliptical_face_revolution_stays_not_done",
        axis_crossing_elliptical_face_revolution_stays_not_done_body,
    );
}

fn axis_crossing_elliptical_face_revolution_stays_not_done_body() {
    let source = elliptical_face();
    let mut maker =
        BRepPrimAPIMakeRevol::new(&source, rotation_axis(1., 0., 0.), 180. * (PI / 180.), true)
            .unwrap();
    assert!(!maker.is_done());
    assert_eq!(
        maker.shape().unwrap_err().exception_type(),
        "StdFail_NotDone"
    );
}

fn elliptical_face() -> Shape {
    let ellipse: Handle<dyn GeomCurve> =
        Handle::new(GeomEllipse::new(Axis2::default(), 200., 100.).unwrap()).cast();
    let edge = BRepBuilderAPIMakeEdge::from_curve(&ellipse)
        .unwrap()
        .edge()
        .unwrap();
    let wire = BRepBuilderAPIMakeWire::from_edge(&edge)
        .unwrap()
        .wire()
        .unwrap();
    BRepBuilderAPIMakeFace::from_plane_wire(Plane3::default(), &wire, true)
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}
