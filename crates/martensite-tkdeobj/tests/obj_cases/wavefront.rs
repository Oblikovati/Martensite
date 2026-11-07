//! `wavefront name file`: a meshed shape as an OBJ file, which is how the persistence scripts get
//! their triangulation-only input.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::precision;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface as _;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::cslib;
use martensite_tkmath::gp::Direction3;
use martensite_tkmath::poly::mesh_purpose;
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
use std::fmt::Write as _;

fn fixed(value: f64) -> String {
    format!("{value:.6}")
}

/// Meshes the shape when it has no mesh, at 0.4% of its largest extent, and writes the file.
pub fn wavefront(shape: &Shape, file: &std::path::Path) {
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add(shape, &mut bounds, true).unwrap();
    let [xmin, ymin, zmin, xmax, ymax, zmax] = bounds.get().unwrap();
    let largest = (xmax - xmin)
        .abs()
        .max((ymax - ymin).abs())
        .max((zmax - zmin).abs());
    BRepMeshIncrementalMesh::with_shape(shape, largest * 0.004, false, 0.5, false).unwrap();
    let mut out = String::new();
    let (mut nb_faces, mut total_nodes) = (0, 0);
    let mut normal = Direction3::default();
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    while faces.more() {
        nb_faces += 1;
        let face = topods::face(faces.current()).unwrap();
        let (mesh, location) = BRepTool::triangulation(&face, mesh_purpose::NONE).unwrap();
        if let Some(mesh) = mesh.get() {
            let nb_nodes = mesh.nb_nodes();
            for index in 1..=nb_nodes {
                let point = mesh
                    .node(index)
                    .unwrap()
                    .transformed(location.transformation());
                let _ = writeln!(
                    out,
                    "v      {}  {}  {}",
                    fixed(point.x()),
                    fixed(point.y()),
                    fixed(point.z())
                );
            }
            if mesh.has_uv_nodes() {
                let surface = BRepAdaptorSurface::with_face(&face, false).unwrap();
                for index in 1..=nb_nodes {
                    let uv = mesh.uv_node(index).unwrap();
                    let d1 = surface.d1(uv.x(), uv.y()).unwrap();
                    let first = cslib::normal(d1.d1u, d1.d1v, precision::angular()).unwrap();
                    if let Some(found) = first.normal {
                        normal = found;
                    }
                    if first.status != cslib::DerivativeStatus::Done {
                        let d2 = surface.d2(uv.x(), uv.y()).unwrap();
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
                        "vn      {}  {}  {}",
                        fixed(normal.x()),
                        fixed(normal.y()),
                        fixed(normal.z())
                    );
                }
            }
            let _ = writeln!(out, "s    {nb_faces}");
            for index in 1..=mesh.nb_triangles() {
                let (n1, mut n2, mut n3) = mesh.triangle(index).unwrap().get();
                if face.orientation() == Orientation::Reversed {
                    std::mem::swap(&mut n2, &mut n3);
                }
                let (k1, k2, k3) = (n1 + total_nodes, n2 + total_nodes, n3 + total_nodes);
                let _ = writeln!(out, "f {k1}//{k1} {k2}//{k2} {k3}//{k3}");
            }
            total_nodes += nb_nodes;
        }
        faces.next().unwrap();
    }
    std::fs::write(file, out).unwrap();
}
