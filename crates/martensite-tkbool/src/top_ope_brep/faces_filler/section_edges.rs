//! Restriction section edges retain input order, operand rank, and same-domain links.
use super::*;
use crate::top_ope_brep_ds::TopOpeBRepDSConfig;
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Edge, Shape},
    toptools::MapOfShape,
};
use martensite_tkernel::ncollection::List;

mod keep;
mod midpoint;
mod same_domain;

impl TopOpeBRepFacesFiller<'_> {
    /// Appends unique kept restriction edges in current-line order; `filler.get_esl(&mut edges)?`.
    pub fn get_esl(&mut self, edges: &mut List<Shape>) -> Result<(), Failure> {
        let mut pair = self.intersector.take().ok_or_else(missing_pair)?;
        let result = self.collect_restriction_edges(&mut pair, edges);
        self.intersector = Some(pair);
        result
    }

    fn collect_restriction_edges(
        &mut self,
        pair: &mut TopOpeBRepFacesIntersector,
        edges: &mut List<Shape>,
    ) -> Result<(), Failure> {
        let mut seen = MapOfShape::default();
        pair.init_line()?;
        while pair.more_line() {
            {
                let mut line = pair.current_line()?;
                if line.type_line_curve() == TopOpeBRepTypeLineCurve::Restriction {
                    let edge = line.arc()?;
                    if !seen.contains(&edge) && self.keep_rline(&mut line, false)? {
                        seen.add(edge.clone());
                        edges.append(edge);
                    }
                }
            }
            pair.next_line()?;
        }
        Ok(())
    }

    /// Registers kept restriction edges and opposite-rank same-domain relations; `filler.process_section_edges()?`.
    pub fn process_section_edges(&mut self) -> Result<(), Failure> {
        let mut edges = List::default();
        self.get_esl(&mut edges)?;
        self.register_section_edges(&edges)?;
        let ranks = self.section_edge_ranks(&edges)?;
        self.register_section_ranks(&edges, &ranks)?;
        self.link_section_same_domains(&edges)
    }

    fn register_section_edges(&self, edges: &List<Shape>) -> Result<(), Failure> {
        let mut ds = self.change_data_structure()?;
        for shape in edges {
            let edge = topods::edge(shape)?;
            if !BRepTool::degenerated(&edge)? {
                ds.add_section_edge(&edge);
            }
        }
        Ok(())
    }

    fn section_edge_ranks(&mut self, edges: &List<Shape>) -> Result<Vec<i32>, Failure> {
        let pair = self.intersector.as_deref_mut().ok_or_else(missing_pair)?;
        let mut ranks = Vec::new();
        for edge in edges {
            pair.init_line()?;
            while pair.more_line() {
                let rank = restriction_rank(pair, edge)?;
                if let Some(rank) = rank {
                    if rank != 0 {
                        ranks.push(rank);
                    }
                    break;
                }
                pair.next_line()?;
            }
        }
        Ok(ranks)
    }

    fn register_section_ranks(&self, edges: &List<Shape>, ranks: &[i32]) -> Result<(), Failure> {
        let mut ds = self.change_data_structure()?;
        for (edge, rank) in edges.iter().zip(ranks) {
            ds.add_shape_with_rank(edge, *rank);
        }
        Ok(())
    }

    fn link_section_same_domains(&mut self, edges: &List<Shape>) -> Result<(), Failure> {
        let pairs = self.section_same_domain_pairs(edges)?;
        for (first, second) in pairs {
            let first_face = self.rank_face(&first)?;
            let second_face = self.rank_face(&second)?;
            let ref_first = BRepTool::is_closed_on_face(&first, &first_face)?
                || !BRepTool::is_closed_on_face(&second, &second_face)?;
            self.change_data_structure()?
                .fill_shapes_same_domain_with_configs(
                    first.as_ref(),
                    second.as_ref(),
                    TopOpeBRepDSConfig::UnshGeometry,
                    TopOpeBRepDSConfig::UnshGeometry,
                    ref_first,
                )?;
        }
        Ok(())
    }

    fn rank_face(&self, edge: &Edge) -> Result<martensite_tkbrep::topods::Face, Failure> {
        let rank = self
            .hds
            .get()
            .ok_or_else(missing_storage)?
            .ds()
            .ancestor_rank(edge.as_ref());
        let pair = self.intersector.as_deref().ok_or_else(missing_pair)?;
        topods::face(pair.face(rank)?)
    }
}

fn restriction_rank(
    pair: &TopOpeBRepFacesIntersector,
    edge: &Shape,
) -> Result<Option<i32>, Failure> {
    let line = pair.current_line()?;
    if line.type_line_curve() != TopOpeBRepTypeLineCurve::Restriction || !line.arc()?.is_equal(edge)
    {
        return Ok(None);
    }
    Ok(Some(if line.arc_is_edge(1)? {
        1
    } else if line.arc_is_edge(2)? {
        2
    } else {
        0
    }))
}

fn missing_pair() -> Failure {
    Failure::with_message(
        "Standard_NullObject",
        "FacesFiller: intersector=None; expected a bound face intersection",
    )
}

#[cfg(test)]
mod tests;
