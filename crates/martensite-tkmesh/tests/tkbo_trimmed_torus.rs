//! A seam-crossing trimmed toroidal face must receive a dense triangulation.
#![allow(non_snake_case)]

use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::SurfaceType;
use martensite_tkmath::gp::{Axis3, Direction3, Point3, Torus3};
use martensite_tkmath::poly::mesh_purpose;
use martensite_tkmesh::brep_mesh::{BRepMeshDiscretRoot, BRepMeshIncrementalMesh};
use martensite_tkmesh::imesh_tools::MeshParameters;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use std::f64::consts::PI;

fn seam_crossing_torus() -> Shape {
    let axis = Axis3::from_normal(
        Point3::new(0.0, -20.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    )
    .unwrap();
    let torus = Torus3::new(axis, 200.0, 17.5).unwrap();
    let maker =
        BRepBuilderAPIMakeFace::from_torus_bounds(torus, (PI, 3.0 * PI, 0.0, 2.0 * PI)).unwrap();
    assert!(maker.is_done());
    maker.face().unwrap().into_shape()
}

fn torus_mesh_counts(shape: &Shape) -> (usize, i32, i32) {
    let (mut faces, mut nodes, mut triangles) = (0, 0, 0);
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let face = topods::face(explorer.current()).unwrap();
        explorer.next().unwrap();
        if BRepAdaptorSurface::with_face(&face, true)
            .unwrap()
            .get_type()
            .unwrap()
            != SurfaceType::Torus
        {
            continue;
        }
        faces += 1;
        let (triangulation, _) = BRepTool::triangulation(&face, mesh_purpose::NONE).unwrap();
        let triangulation = triangulation.get().expect("toroidal face must be meshed");
        nodes += triangulation.nb_nodes();
        triangles += triangulation.nb_triangles();
    }
    (faces, nodes, triangles)
}

#[test]
fn BRepMesh_IncrementalMeshTest__BooleanTrimmedTorus_CrossingSeamProducesDenseMesh() {
    let shape = seam_crossing_torus();
    assert!(!shape.is_null());
    assert!(
        BRepCheckAnalyzer::new(&shape, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    let parameters = MeshParameters {
        deflection: 0.1,
        angle: 0.4,
        ..Default::default()
    };
    let mesher =
        BRepMeshIncrementalMesh::with_parameters(&shape, &parameters, &ProgressRange::default())
            .unwrap();
    assert!(mesher.is_done());
    let (faces, nodes, triangles) = torus_mesh_counts(&shape);
    assert!(faces > 0);
    assert!(nodes > 1000, "{nodes} torus nodes");
    assert!(triangles > 1000, "{triangles} torus triangles");
}
