//! Original topology bounds regressions hosted above their primitive and mesher dependencies.
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Edge};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCircle, GeomCurve};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkmesh::brep_mesh::{BRepMeshDiscretRoot, BRepMeshIncrementalMesh};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape};

const MESH_BOUNDS: [f64; 6] = [
    -1.1000001,
    -1.0927089740980542,
    -0.1000001,
    1.1000001,
    1.092708974098054,
    0.1000001,
];
const CURVE_BOUNDS: [f64; 6] = [-1.0000001, -1.0000001, -1e-7, 1.0000001, 1.0000001, 1e-7];

fn assert_original_bounds(edge: &Edge) -> Result<(), Failure> {
    let mut with = BoundingBox3::default();
    let mut without = BoundingBox3::default();
    BRepBndLib::add(edge, &mut with, true)?;
    BRepBndLib::add(edge, &mut without, false)?;
    expect_bounds(&with, MESH_BOUNDS)?;
    expect_bounds(&without, CURVE_BOUNDS)
}

fn expect_bounds(bounds: &BoundingBox3, expected: [f64; 6]) -> Result<(), Failure> {
    for (value, want) in bounds.get()?.into_iter().zip(expected) {
        assert!((value - want).abs() <= 1e-12, "{value} != {want}");
    }
    Ok(())
}

#[test]
fn occ30292_meshed_geometric_edge_uses_requested_representation() -> Result<(), Failure> {
    let frame = Axis2::new(Point3::new(0., 0., 0.), Direction3::new(0., 0., 1.)?)?;
    let circle: Handle<dyn GeomCurve> = Handle::new(GeomCircle::new(frame, 1.)?).cast();
    let edge = BRepBuilderAPIMakeEdge::from_curve(&circle)?.edge()?;
    let mesher = BRepMeshIncrementalMesh::with_shape(&edge, 0.1, false, 0.5, false)?;
    assert!(mesher.is_done());
    assert_original_bounds(&edge)
}

#[test]
fn occ30292_meshed_cylinder_edge_uses_polygon3d() -> Result<(), Failure> {
    let cylinder = BRepPrimAPIMakeCylinder::new(1., 1.).shape()?.clone();
    let mut explorer = Explorer::new(&cylinder, ShapeEnum::Edge, ShapeEnum::Shape)?;
    explorer.next()?;
    explorer.next()?;
    let edge = topods::edge(explorer.current())?;
    let mesher = BRepMeshIncrementalMesh::with_shape(&cylinder, 0.1, false, 0.5, false)?;
    assert!(mesher.is_done());
    assert_original_bounds(&edge)
}
