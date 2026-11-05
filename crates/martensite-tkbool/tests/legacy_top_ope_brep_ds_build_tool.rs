//! Regressions of TopOpeBRepDSBuildTool tests.
use martensite_tkbool::top_ope_brep_ds::TopOpeBRepDSBuildTool;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCircle, GeomCurve};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Axis2;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;

/// TEST(TopOpeBRepDSBuildToolTest, CopyReversedPeriodicEdgePreservesRange).
#[test]
fn copy_reversed_periodic_edge_preserves_range() -> Result<(), Failure> {
    let circle = Handle::new(GeomCircle::new(Axis2::default(), 1.0)?);
    let a_circle: Handle<dyn GeomCurve> = circle.clone().cast();
    let mut a_source = BRepBuilderAPIMakeEdge::from_curve(&a_circle)?
        .edge()?
        .into_shape();
    a_source.reverse();
    let a_build_tool = TopOpeBRepDSBuildTool::new();
    let mut a_copy = Shape::default();
    a_build_tool.copy_edge(&a_source, &mut a_copy)?;
    let vertices: Vec<Shape> =
        Explorer::new(&a_source, ShapeEnum::Vertex, ShapeEnum::Shape)?.collect();
    for a_vertex in &vertices {
        a_build_tool.add_edge_vertex_from(&a_source, &mut a_copy, a_vertex)?;
    }
    let (a_first, a_last) = BRepTool::range(&topods::edge(&a_copy)?)?;
    let c = circle.get().ok_or_else(|| Failure::new("null circle"))?;
    assert!((a_first - c.first_parameter()).abs() <= precision::p_confusion());
    assert!((a_last - c.last_parameter()).abs() <= precision::p_confusion());
    Ok(())
}
