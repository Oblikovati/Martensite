//! `wavefront` of Draw/TKTopTest/MeshTest/MeshTest.cxx at
//! 3d097a0328e71b826377d4814ab05ec3c3d23871: mesh a shape and write its triangulations as
//! a Wavefront OBJ file.
use super::draw::{Draw, Reply, Tcl};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Face, Shape};
use martensite_tkernel::{precision, standard::Failure};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::{bnd::BoundingBox3, cslib, gp::Direction3, poly::mesh_purpose};
use martensite_tkmath::{poly::Triangulation, toploc::Location};
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
use std::fmt::Write as _;

/// `wavefront name [file]`: meshes the shape with 0.4% of its largest bounding-box side
/// and writes `<file>.obj` (`wave.obj` without a file); status 1 for a missing argument
/// or a null shape; `wavefront(&mut draw, &["b", "/tmp/out"])?`.
pub fn wavefront(draw: &mut Draw, args: &[&str]) -> Tcl {
    let words: Vec<&str> = std::iter::once("wavefront")
        .chain(args.iter().copied())
        .collect();
    draw.command(&words, |draw| {
        let Some(name) = args.first() else {
            return Ok(Reply::error(""));
        };
        let shape = draw.shape(name);
        if shape.is_null() {
            return Ok(Reply::error(""));
        }
        let file = args
            .get(1)
            .map_or_else(|| "wave.obj".to_string(), |file| format!("{file}.obj"));
        write_obj(&shape, &file)?;
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// The mesh and the OBJ text of `shape`, written to `file`.
fn write_obj(shape: &Shape, file: &str) -> Result<(), Failure> {
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add(shape, &mut bounds, true)?;
    let [xmin, ymin, zmin, xmax, ymax, zmax] = bounds.get()?;
    // MAX3 of the three box sides.
    let deflection = (xmax - xmin).max(ymax - ymin).max(zmax - zmin) * 0.004;
    BRepMeshIncrementalMesh::with_shape(shape, deflection, false, 0.5, false)?;
    let mut out = format!("# CASCADE     MATRA DATAVISION\n# {file}\n\n");
    let mut counts = ObjCounts::default();
    for face in Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)? {
        counts.faces += 1;
        let face = topods::face(&face)?;
        let (triangulation, location) = BRepTool::triangulation(&face, mesh_purpose::NONE)?;
        if let Some(triangulation) = triangulation.get() {
            write_face(&mut out, &face, triangulation, &location, &mut counts)?;
        }
    }
    std::fs::write(file, out).map_err(|error| {
        Failure::with_message(
            "Standard_Failure",
            &format!("wavefront: cannot write {file:?}: {error}; expected a writable path"),
        )
    })
}

/// The running face, node and polygon counts of the file.
#[derive(Default)]
struct ObjCounts {
    faces: i32,
    nodes: i32,
    polygons: i32,
}

fn write_face(
    out: &mut String,
    face: &Face,
    triangulation: &Triangulation,
    location: &Location,
    counts: &mut ObjCounts,
) -> Result<(), Failure> {
    let nb_nodes = triangulation.nb_nodes();
    for i in 1..=nb_nodes {
        let point = triangulation
            .node(i)?
            .transformed(location.transformation());
        let _ = writeln!(
            out,
            "v      {:.6}  {:.6}  {:.6}",
            point.x(),
            point.y(),
            point.z()
        );
    }
    let _ = write!(out, "\n# number of vertex    {nb_nodes}\n\n");
    if triangulation.has_uv_nodes() {
        write_normals(out, face, triangulation)?;
        let _ = write!(out, "\n# number of vertex normals    {nb_nodes}\n\n");
    }
    let _ = writeln!(out, "s    {}", counts.faces);
    let nb_triangles = triangulation.nb_triangles();
    for i in 1..=nb_triangles {
        let (n1, mut n2, mut n3) = triangulation.triangle(i)?.get();
        if face.orientation() == Orientation::Reversed {
            std::mem::swap(&mut n2, &mut n3);
        }
        let [k1, k2, k3] = [n1, n2, n3].map(|node| node + counts.nodes);
        let _ = writeln!(out, "f {k1}//{k1} {k2}//{k2} {k3}//{k3}");
    }
    counts.polygons += nb_triangles;
    counts.nodes += nb_nodes;
    let _ = write!(out, "\n# number of smooth groups    {}\n", counts.faces);
    let _ = write!(out, "\n# number of polygons    {}\n", counts.polygons);
    Ok(())
}

/// One `vn` line per node: the CSLib normal from the first derivatives, else from the
/// second ones; the previous normal is kept when neither is defined (the native `Nor`
/// variable lives across the loop), and a reversed face reverses it.
fn write_normals(
    out: &mut String,
    face: &Face,
    triangulation: &Triangulation,
) -> Result<(), Failure> {
    let surface = BRepAdaptorSurface::with_face(face, false)?;
    let mut normal = Direction3::default();
    for i in 1..=triangulation.nb_nodes() {
        let uv = triangulation.uv_node(i)?;
        let d1 = surface.d1(uv.x(), uv.y())?;
        let first = cslib::normal(d1.d1u, d1.d1v, precision::angular())?;
        if let Some(found) = first.normal {
            normal = found;
        }
        if first.status != cslib::DerivativeStatus::Done {
            let d2 = surface.d2(uv.x(), uv.y())?;
            let second = cslib::normal_from_second_derivatives(
                d2.d1u,
                d2.d1v,
                d2.d2u,
                d2.d2v,
                d2.d2uv,
                precision::angular(),
            );
            if let Some(found) = second.normal {
                normal = found;
            }
        }
        if face.orientation() == Orientation::Reversed {
            normal.reverse();
        }
        let _ = writeln!(
            out,
            "vn      {:.6}  {:.6}  {:.6}",
            normal.x(),
            normal.y(),
            normal.z()
        );
    }
    Ok(())
}
