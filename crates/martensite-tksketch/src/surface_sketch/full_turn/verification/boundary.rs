// SPDX-License-Identifier: GPL-2.0-only
//! Bind directed coedges to complete outer or clear-aperture wires by incidence.
use super::*;

pub(in crate::surface_sketch::full_turn) struct DirectedWires(Vec<Vec<topods::Edge>>);

impl DirectedWires {
    pub(in crate::surface_sketch::full_turn) fn read(
        face: &topods::Face,
        expected: [usize; 2],
    ) -> Checked<Self> {
        let mut wires = topods::ShapeIterator::new(face, true, true);
        let mut groups = Vec::new();
        while wires.more() {
            let wire = wires.value().map_err(|e| native("periodic face wire", e))?;
            groups.push(read_wire(wire)?);
            wires.next();
        }
        let result = Self(groups);
        result.check_counts(expected)?;
        Ok(result)
    }

    fn check_counts(&self, expected: [usize; 2]) -> Checked<()> {
        let counts = [self.0.len(), self.edges().count()];
        if counts == expected {
            return Ok(());
        }
        Err(incomplete(
            "complete periodic wires and directed uses",
            format!("{counts:?} vs {expected:?}"),
        ))
    }

    pub(in crate::surface_sketch::full_turn) fn edges(
        &self,
    ) -> impl Iterator<Item = &topods::Edge> {
        self.0.iter().flatten()
    }

    pub(super) fn clear_wire(&self, named: [&topods::Shape; 4]) -> Checked<[topods::Edge; 4]> {
        let wire = self.identify_wire(&named)?;
        let [a, b, c, d] = named.map(|shape| {
            wire.iter()
                .find(|edge| shape.is_same(edge))
                .cloned()
                .ok_or_else(|| incomplete("directed clear aperture edge", "no named coedge"))
        });
        Ok([a?, b?, c?, d?])
    }

    pub(in crate::surface_sketch::full_turn) fn identify_wire(
        &self,
        names: &[&topods::Shape],
    ) -> Checked<&[topods::Edge]> {
        let mut matching = self.0.iter().filter(|wire| matches_wire(wire, names));
        let wire = matching.next().ok_or_else(|| {
            incomplete(
                "one complete rooted clear aperture wire",
                "no matching wire",
            )
        })?;
        if matching.next().is_some() {
            return Err(incomplete(
                "one rooted clear aperture wire",
                "ambiguous occurrences",
            ));
        }
        Ok(wire)
    }
}

fn matches_wire(wire: &[topods::Edge], names: &[&topods::Shape]) -> bool {
    !names.is_empty()
        && wire.len() == names.len()
        && names
            .iter()
            .all(|name| wire.iter().any(|edge| name.is_same(edge)))
}

fn read_wire(wire: &topods::Shape) -> Checked<Vec<topods::Edge>> {
    topods::wire(wire).map_err(|e| native("periodic face wire kind", e))?;
    let mut explorer = Explorer::new(wire, ShapeEnum::Edge, ShapeEnum::Shape)
        .map_err(|e| native("periodic face coedges", e))?;
    let mut edges = Vec::new();
    while explorer.more() {
        edges
            .push(topods::edge(explorer.current()).map_err(|e| native("periodic face coedge", e))?);
        explorer
            .next()
            .map_err(|e| native("next periodic face coedge", e))?;
    }
    Ok(edges)
}
