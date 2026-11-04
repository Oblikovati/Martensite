use super::*;
use crate::case_view::SourceViewOutput;
#[test]
fn full_procedural_hole_face_is_repaired_and_triangulated() {
    let shape = geometry::healed_face().unwrap();
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    display(&mut session, &shape).unwrap();
    assert!(session.object("result").is_some());
    let mesh = crate::shape_mesh::TriangulationSummary::of(&shape).unwrap();
    assert!(
        mesh.triangles > 0,
        "actual triangles{}; expected nonempty original face mesh",
        mesh.triangles
    );
    session.close();
}
