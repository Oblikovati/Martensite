//! Opposite-rank restrictions are linked when either endpoint lies on the other curve.
use super::*;
use crate::top_ope_brep_tool::fun_tool_pin_c;
use martensite_tkbrep::{brep_adaptor::BRepAdaptorCurve, topexp};

impl TopOpeBRepFacesFiller<'_> {
    pub(super) fn section_same_domain_pairs(
        &self,
        edges: &List<Shape>,
    ) -> Result<Vec<(Edge, Edge)>, Failure> {
        let storage = self.hds.get().ok_or_else(missing_storage)?;
        let ds = storage.ds();
        let mut pairs = Vec::new();
        for first in edges {
            let first_index = ds.shape_index(first, true);
            if ds.ancestor_rank_index(first_index) != 1 {
                continue;
            }
            for second in edges {
                let second_index = ds.shape_index(second, true);
                let rank = ds.ancestor_rank_index(second_index);
                if rank == 0 || second_index == first_index || rank == 1 {
                    continue;
                }
                let a = topods::edge(first)?;
                let b = topods::edge(second)?;
                if restrictions_overlap(&a, &b)? {
                    pairs.push((a, b));
                }
            }
        }
        Ok(pairs)
    }
}

pub(super) fn restrictions_overlap(first: &Edge, second: &Edge) -> Result<bool, Failure> {
    let (a, b) = topexp::vertices(first, false);
    let (c, d) = topexp::vertices(second, false);
    if endpoint_on_curve(first, &c, &d)? {
        return Ok(true);
    }
    endpoint_on_curve(second, &a, &b)
}

fn endpoint_on_curve(
    edge: &Edge,
    first: &martensite_tkbrep::topods::Vertex,
    last: &martensite_tkbrep::topods::Vertex,
) -> Result<bool, Failure> {
    let tolerance = BRepTool::edge_tolerance(edge)?
        .max(BRepTool::vertex_tolerance(first)?.max(BRepTool::vertex_tolerance(last)?));
    let curve = BRepAdaptorCurve::with_edge(edge)?;
    if fun_tool_pin_c(BRepTool::pnt(first)?, &curve, tolerance)? {
        return Ok(true);
    }
    fun_tool_pin_c(BRepTool::pnt(last)?, &curve, tolerance)
}
