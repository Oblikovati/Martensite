//! Vertex-bearing edge fixture; source: `tkhlr_probe_hlrbrep_records`::hb_edge_v.
use super::brep_curve_fixture::{CurveFixtureInput, geometry};
use martensite_tkbrep::{brep_adaptor::BRepAdaptorCurve, topods::Edge};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkhlr::{
    hlr_algo::{Projector, init_min_max},
    hlr_brep::EdgeData,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;
use std::sync::Arc;

/// Build the original parameter-bounded edge with vertices; `vertex_edge(reader)`.
pub fn vertex_edge(reader: &mut impl CurveFixtureInput) -> Result<Edge, Failure> {
    let curve = geometry(reader)?;
    let (first, last) = (reader.real(), reader.real());
    BRepBuilderAPIMakeEdge::from_curve_params(&curve, first, last)?.edge()
}

/// Prepare original edge bounds/projected geometry; `projected_record(edge, projector)`.
/// Source: `tkhlr_probe_hlrbrep_intersector`::hb_set_edge.
pub fn projected_record(e: &Edge, p: &Arc<Projector>) -> Result<EdgeData, Failure> {
    let bc = BRepAdaptorCurve::with_edge(e)?;
    let (a, b) = (bc.first_parameter()?, bc.last_parameter()?);
    let mut d = EdgeData::new();
    d.set(
        false, false, e, 1, 2, false, false, false, false, a, 1e-7, b, 1e-7,
    )?;
    let c = d.change_geometry();
    c.set_projector(Some(p.clone()));
    let (mut mn, mut mx) = ([0.; 16], [0.; 16]);
    init_min_max(1e100, &mut mn, &mut mx);
    c.update(&mut mn, &mut mx)?;
    Ok(d)
}
