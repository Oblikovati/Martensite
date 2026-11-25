// SPDX-License-Identifier: GPL-2.0-only
//! Bind directed coedges to complete outer or clear-aperture wires by incidence.
use super::*;

pub(super) struct DirectedWires(Vec<Vec<topods::Edge>>);

impl DirectedWires {
    pub(super) fn read(face: &topods::Face, topology: &ExpectedTopology) -> Checked<Self> {
        let mut wires = topods::ShapeIterator::new(face, true, true);
        let mut groups = Vec::new();
        while wires.more() {
            let wire = wires.value().map_err(|e| native("periodic face wire", e))?;
            groups.push(read_wire(wire)?);
            wires.next();
        }
        let result = Self(groups);
        result.check_counts(topology)?;
        Ok(result)
    }

    fn check_counts(&self, topology: &ExpectedTopology) -> Checked<()> {
        let counts = [self.0.len(), self.edges().count()];
        let expected = [
            1 + topology.clear,
            4 + 8 * topology.crossed + 4 * topology.clear,
        ];
        if counts == expected {
            return Ok(());
        }
        Err(incomplete(
            "complete periodic wires and directed uses",
            format!("{counts:?} vs {expected:?}"),
        ))
    }

    pub(super) fn edges(&self) -> impl Iterator<Item = &topods::Edge> {
        self.0.iter().flatten()
    }

    pub(super) fn clear_wire(&self, named: [&topods::Shape; 4]) -> Checked<[topods::Edge; 4]> {
        let mut matches = self.0.iter().filter_map(|wire| match_wire(wire, named));
        let group = matches.next();
        if matches.next().is_some() {
            return Err(incomplete(
                "one rooted clear aperture wire",
                "ambiguous occurrences",
            ));
        }
        group.ok_or_else(|| {
            incomplete(
                "one complete rooted clear aperture wire",
                "no matching wire",
            )
        })
    }
}

fn match_wire(wire: &[topods::Edge], named: [&topods::Shape; 4]) -> Option<[topods::Edge; 4]> {
    if wire.len() != named.len() {
        return None;
    }
    let find = |shape: &topods::Shape| wire.iter().find(|edge| shape.is_same(edge)).cloned();
    let [a, b, c, d] = named.map(find);
    Some([a?, b?, c?, d?])
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
