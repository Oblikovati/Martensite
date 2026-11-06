//! A kernel-free terminal packet owns evaluable results after worker and source changes.
use super::{FaceFace, FaceFacePostprocessing};
use martensite_tkbrep::{brep::BRepBuilder, topods::Face};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{
    adaptor3d::Adaptor3dSurface,
    geom::{GeomPlane, GeomSurface},
};
use martensite_tkmath::{
    gp::{Axis3, Direction3, Point3, Transform3},
    toploc::Location,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

#[test]
fn terminal_packet_owns_surfaces_and_curves_after_kernel_context_and_inputs_drop() {
    static_assertions::assert_impl_all!(FaceFacePostprocessing: Send, Sync);
    let faces = intersecting_planes();
    let worker_faces = faces.clone();
    let packet = std::thread::spawn(move || intersect_in_worker(&worker_faces))
        .join()
        .unwrap();
    replace_source_surface(&faces[0]);
    replace_source_surface(&faces[1]);
    assert_terminal_geometry(&packet);
    drop(faces);
    let mut intersector = packet.into_intersector();
    intersector.prepare_lines_3d(false).unwrap();
    intersector
        .apply_result_transform(Transform3::default())
        .unwrap();
    assert_curve_sample(&intersector);
}

fn intersect_in_worker(faces: &[Face; 2]) -> FaceFacePostprocessing {
    let mut intersector = FaceFace::default();
    intersector.perform(&faces[0], &faces[1], false).unwrap();
    assert!(intersector.is_done());
    intersector.into_postprocessing()
}

fn assert_terminal_geometry(packet: &FaceFacePostprocessing) {
    assert!(packet.done);
    assert_eq!(packet.curves.len(), 1);
    let first = packet.surfaces[0].value(0.0, 0.0).unwrap();
    let second = packet.surfaces[1].value(0.0, 0.0).unwrap();
    assert_eq!(first, Point3::default());
    assert_eq!(second, Point3::new(0.5, 0.0, 0.0));
}

fn assert_curve_sample(intersector: &FaceFace) {
    let curve = intersector.lines().unwrap()[0].curve();
    let point = curve.get().unwrap().eval_d0(0.0).unwrap();
    assert!(
        point.distance(&Point3::new(0.5, 0.0, 0.0)) < 1e-12,
        "terminal curve sample {point:?}, expected intersection (.5, 0, 0)"
    );
}

fn intersecting_planes() -> [Face; 2] {
    let first = Axis3::default();
    let second = Axis3::new(Point3::new(0.5, 0.0, 0.0), Direction3::X, Direction3::Y).unwrap();
    [plane_face(first), plane_face(second)]
}

fn plane_face(frame: Axis3) -> Face {
    let surface: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(frame)).cast();
    BRepBuilderAPIMakeFace::from_surface_bounds(&surface, (-2.0, 2.0, -2.0, 2.0), 1e-7)
        .unwrap()
        .face()
        .unwrap()
}

fn replace_source_surface(face: &Face) {
    let frame = Axis3::new(Point3::new(20.0, 30.0, 40.0), Direction3::Z, Direction3::X).unwrap();
    let surface: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(frame)).cast();
    BRepBuilder::default()
        .update_face(face, &surface, &Location::default(), 1e-7)
        .unwrap();
}

#[test]
fn unfinished_packet_retains_not_done_and_unguarded_points_contracts() {
    let packet = FaceFace::default().into_postprocessing();
    let intersector = packet.into_intersector();
    assert!(!intersector.is_done());
    assert!(intersector.points().is_empty());
    assert_eq!(
        intersector
            .lines()
            .err()
            .map(|cause| cause.exception_type().to_owned()),
        Some("StdFail_NotDone".to_owned())
    );
}
