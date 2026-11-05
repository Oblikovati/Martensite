//! Pole transitions use one restriction or the next untreated incident edge.
use super::*;
use crate::top_ope_brep_ds::export::fun_ds_getoov_hds;
use crate::top_ope_brep_tool::fun_tool_in_s;

impl TopOpeBRepFacesFiller<'_> {
    pub(super) fn degenerate_crossing(
        &mut self,
        point: &TopOpeBRepVPointInter<'_>,
    ) -> Result<Option<DegenerateCrossing>, Failure> {
        let Some((vertex, vertex_rank)) = self.degenerate_vertex(point)? else {
            return Ok(None);
        };
        let pair = self.degenerate_context.entries.find(&vertex)?;
        let seam = topods::edge(pair.first().ok_or_else(invalid_degenerate_entry)?)?;
        let edge = topods::edge(pair.last().ok_or_else(invalid_degenerate_entry)?)?;
        let rank = self.degenerate_edge_rank(&edge, &vertex, vertex_rank)?;
        let opposite = if rank == 1 { 2 } else { 1 };
        let mut maker = TopOpeBRepToolMkTondgE::new();
        if !maker.initialize(
            &edge,
            &self.faces[(rank - 1) as usize],
            point.surface_parameters(opposite)?,
            &self.faces[(opposite - 1) as usize],
        )? || !maker.setcl_e(&seam)
        {
            return Ok(None);
        }
        let both_vertices = point.is_vertex_on_s1()? && point.is_vertex_on_s2()?;
        if !both_vertices
            && self
                .hds
                .get()
                .ok_or_else(missing_storage)?
                .has_same_domain(&vertex, true)
            && fun_ds_getoov_hds(&vertex, &self.hds)?.is_none()
        {
            return Ok(None);
        }
        let mut transitions = (0, 0., 0.);
        let ok =
            self.compute_degenerate_transition(point, opposite, &mut maker, &mut transitions)?;
        if !ok || transitions.0 == 0 {
            return Ok(None);
        }
        Ok(Some(DegenerateCrossing {
            rank,
            edge,
            transitions,
            planar: maker.is_t2d(),
        }))
    }

    fn degenerate_vertex(
        &self,
        point: &TopOpeBRepVPointInter<'_>,
    ) -> Result<Option<(Shape, i32)>, Failure> {
        for side in 1..=2 {
            if point.is_vertex(side)? {
                let vertex = point.vertex(side)?;
                if self.degenerate_context.entries.is_bound(&vertex) {
                    return Ok(Some((vertex, side)));
                }
            }
        }
        Ok(None)
    }

    fn degenerate_edge_rank(
        &self,
        edge: &Edge,
        vertex: &Shape,
        vertex_rank: i32,
    ) -> Result<i32, Failure> {
        let storage = self.hds.get().ok_or_else(missing_storage)?;
        let ds = storage.ds();
        if ds.has_shape(edge.as_ref(), true) {
            return Ok(ds.ancestor_rank(edge.as_ref()));
        }
        Ok(if fun_tool_in_s(vertex, edge.as_ref())? {
            vertex_rank
        } else if vertex_rank == 1 {
            2
        } else {
            1
        })
    }

    fn compute_degenerate_transition(
        &mut self,
        point: &TopOpeBRepVPointInter<'_>,
        opposite: i32,
        maker: &mut TopOpeBRepToolMkTondgE,
        transitions: &mut (i32, f64, f64),
    ) -> Result<bool, Failure> {
        let on_domain = if opposite == 1 {
            point.is_on_dom_s1()?
        } else {
            point.is_on_dom_s2()?
        };
        if point.shape_index() == 3 || on_domain {
            let (edge, parameter) = restriction_on_side(point, opposite)?;
            maker.set_rest(parameter, &edge)?;
            return maker.mk_ton_e_on(&edge, transitions);
        }
        let face = &self.faces[(opposite - 1) as usize];
        let mut ancestors = IndexedDataMapOfShapeListOfShape::default();
        topexp::map_shapes_and_ancestors(
            face.as_ref(),
            ShapeEnum::Vertex,
            ShapeEnum::Edge,
            &mut ancestors,
        )?;
        let Some(vertex) = find_point_vertex(point, &ancestors)? else {
            return maker.mk_ton_e(transitions);
        };
        self.degenerate_context
            .compute_untreated_edge(&vertex, &ancestors, maker, transitions)
    }
}

fn restriction_on_side(
    point: &TopOpeBRepVPointInter<'_>,
    side: i32,
) -> Result<(Edge, f64), Failure> {
    if side == 1 {
        Ok((
            topods::edge(&point.arc_on_s1()?)?,
            point.parameter_on_arc1()?,
        ))
    } else {
        Ok((
            topods::edge(&point.arc_on_s2()?)?,
            point.parameter_on_arc2()?,
        ))
    }
}

fn find_point_vertex(
    point: &TopOpeBRepVPointInter<'_>,
    ancestors: &IndexedDataMapOfShapeListOfShape,
) -> Result<Option<Vertex>, Failure> {
    let (position, tolerance) = (point.value()?, point.tolerance()?);
    for index in 1..=ancestors.extent() {
        let vertex = topods::vertex(ancestors.find_key(index)?)?;
        if position.is_equal(&BRepTool::pnt(&vertex)?, tolerance) {
            return Ok(Some(vertex));
        }
    }
    Ok(None)
}

fn invalid_degenerate_entry() -> Failure {
    Failure::with_message(
        "Standard_NoSuchObject",
        "ProcessVPondgE: entry=[]; expected [closing edge, degenerate edge]",
    )
}
