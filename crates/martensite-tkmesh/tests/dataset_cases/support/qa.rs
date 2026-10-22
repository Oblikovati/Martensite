//! The QAcommands the TKMesh cases call (Draw/TKQADraw/QABugs/QABugs_11.cxx at
//! 3d097a0328e71b826377d4814ab05ec3c3d23871).
use super::draw::{Draw, Reply, Tcl};
use martensite_tkbrep::brep_tools;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Point3;
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon};

/// `OCC20627 MaxNbr`: `MaxNbr` times, build the closed 20x100 rectangle wire, its planar
/// face, mesh the face with deflection 1 (BRepMesh_IncrementalMesh) and clean the mesh
/// again (BRepTools::Clean); `occ20627(&mut draw, "10")?`.
pub fn occ20627(draw: &mut Draw, max_nbr: &str) -> Tcl {
    draw.command(&["OCC20627", max_nbr], |_| {
        // Draw::Atoi: the leading integer of the word.
        let count: i32 = max_nbr.parse().unwrap_or(0);
        for _ in 0..count {
            mesh_and_clean_rectangle()?;
        }
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

fn mesh_and_clean_rectangle() -> Result<(), Failure> {
    let mut wire = BRepBuilderAPIMakePolygon::from_four_points(
        Point3::new(0., 0., 0.),
        Point3::new(0., 100., 0.),
        Point3::new(20., 100., 0.),
        Point3::new(20., 0., 0.),
        false,
    )?;
    wire.close()?;
    let face_builder = BRepBuilderAPIMakeFace::from_wire(&wire.wire()?, false)?;
    let face = face_builder.face()?;
    // `BRepMesh_IncrementalMesh im(f, 1)`: relative false, angle 0.5, not in parallel.
    BRepMeshIncrementalMesh::with_shape(&face.clone().into(), 1., false, 0.5, false)?;
    brep_tools::clean(&face.into(), false)
}
