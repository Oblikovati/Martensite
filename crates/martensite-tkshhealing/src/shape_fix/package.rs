//! Shape repair utilities for tolerances, vertices and small edges.
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Edge, Shape},
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::bnd::BoundingBox3;

/// Shape healing package utilities; `ShapeFix::least_edge_size(&shape)?`.
pub struct ShapeFix;
impl ShapeFix {
    /// Measure the least three-sample bounding diagonal; `ShapeFix::least_edge_size(&shape)?`.
    /// Empty shapes return sqrt(RealLast), following behavior.
    pub fn least_edge_size(shape: &Shape) -> Result<f64, Failure> {
        let mut minimum = f64::MAX;
        let mut edges = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
        while edges.more() {
            let size = sampled_edge_extent(&topods::edge(edges.current())?)?;
            if size < minimum {
                minimum = size;
            }
            edges.next()?;
        }
        Ok(minimum.sqrt())
    }
    /// Store edge regularity using BRepLib; `ShapeFix::encode_regularity(&shape, 1e-10)?`.
    pub fn encode_regularity(shape: &Shape, angular_tolerance: f64) -> Result<(), Failure> {
        martensite_tktopalgo::brep_lib::encode_regularity(shape, angular_tolerance)
    }
}
fn sampled_edge_extent(edge: &Edge) -> Result<f64, Failure> {
    let (curve, first, last) = BRepTool::curve(edge)?;
    if curve.is_null() {
        return Ok(f64::MAX);
    }
    let mut bounds = BoundingBox3::default();
    for parameter in [first, last, (last + first) / 2.] {
        bounds.add_point(curve.value(parameter)?);
    }
    let [x1, y1, z1, x2, y2, z2] = bounds.get()?;
    Ok((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1) + (z2 - z1) * (z2 - z1))
}

mod plane_tolerances;
mod remove_small_edges;
pub use remove_small_edges::ShapeFixRemovedSmallEdges;
mod same_parameter;

#[cfg(test)]
mod plane_tolerances_tests;

mod vertex_position;
