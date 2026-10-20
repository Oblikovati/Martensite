//! Face fields serialized after a surface callback reflect the updated source.
use super::regularity_dump_callbacks::dump_surface::CallbackDumpPlane;
use martensite_tkbrep::{brep::BRepTFace, topods::TShape};
use martensite_tkernel::standard::{Handle, dump::DumpStream};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkmath::{
    gp::{Transform3, Vector3},
    poly::Triangulation,
    toploc::Location,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

fn face_fixture() -> (Handle<BRepTFace>, Arc<AtomicUsize>) {
    let face = Handle::new(BRepTFace::new());
    let plane = Handle::new(CallbackDumpPlane::new());
    let surface: Handle<dyn GeomSurface> = plane.cast();
    face.set_surface(&surface);
    let old = Handle::new(Triangulation::new());
    old.set_deflection(11.);
    face.set_triangulation(&old, true);
    let count = Arc::new(AtomicUsize::new(0));
    let calls = count.clone();
    let owner = face.downgrade();
    plane.set_dump_hook(Arc::new(move || {
        calls.fetch_add(1, Ordering::SeqCst);
        mutate_face(&owner.upgrade());
    }));
    (face, count)
}

fn mutate_face(face: &BRepTFace) {
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(8., -4., 2.));
    face.set_location(&Location::new(transform));
    face.set_tolerance(0.25);
    face.set_natural_restriction(true);
    let mesh = Handle::new(Triangulation::new());
    mesh.set_deflection(23.);
    face.set_triangulation(&mesh, true);
}

fn compact_dump(face: &BRepTFace) -> String {
    let mut stream = DumpStream::default();
    face.dump_json(&mut stream, -1).unwrap();
    String::from_utf8(stream.bytes().to_vec())
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

#[test]
fn face_dump_late_scalars_observe_surface_callback() {
    let (face, count) = face_fixture();
    let text = compact_dump(&face);
    assert_eq!(count.load(Ordering::SeqCst), 1);
    assert_eq!(face.tolerance(), 0.25);
    assert!(face.natural_restriction());
    assert!(text.contains("\"Tolerance\":0.25"), "dump={text}");
    assert!(text.contains("\"NaturalRestriction\":1"), "dump={text}");
    assert!(text.contains("\"Location\":[8,-4,2]"), "dump={text}");
}

#[test]
fn face_dump_keeps_prior_active_mesh_but_reads_current_mesh_list() {
    let (face, count) = face_fixture();
    let text = compact_dump(&face);
    assert_eq!(count.load(Ordering::SeqCst), 1);
    assert_eq!(face.active_triangulation().deflection(), 23.);
    assert_eq!(text.matches("\"Deflection\":11").count(), 1, "dump={text}");
    assert_eq!(text.matches("\"Deflection\":23").count(), 1, "dump={text}");
}

#[test]
fn shallow_face_dump_skips_surface_callback() {
    let (face, count) = face_fixture();
    face.dump_json(&mut DumpStream::default(), 0).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 0);
    assert!(!face.natural_restriction());
    assert_eq!(face.active_triangulation().deflection(), 11.);
}
