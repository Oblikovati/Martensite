// SPDX-License-Identifier: GPL-2.0-only
//! Certify the directed coedge partition of the decoded stock face.
use super::*;
use martensite_tkbrep::topods::ShapeIterator;

pub(super) struct Names<'a> {
    pub rims: [&'a topods::Shape; 2],
    pub seam: Option<&'a topods::Shape>,
    pub hole: Option<[&'a topods::Shape; 4]>,
}

pub(super) struct FaceTrims {
    pub rims: [topods::Edge; 2],
    pub seam: Option<[topods::Edge; 2]>,
    pub hole: Option<[topods::Edge; 4]>,
}

#[derive(Default)]
struct Occurrences {
    rims: [Vec<topods::Edge>; 2],
    seam: Vec<topods::Edge>,
    hole: [Vec<topods::Edge>; 4],
}

pub(super) fn classify(face: &topods::Face, names: Names<'_>) -> Checked<FaceTrims> {
    let mut wires = ShapeIterator::new(face, true, true);
    let mut found = Occurrences::default();
    let [mut outer, mut inner] = [0, 0];
    while wires.more() {
        let wire = wires
            .value()
            .map_err(|e| native("full-turn face wire", e))?;
        if found.read_wire(wire, &names)? {
            inner += 1;
        } else {
            outer += 1;
        }
        wires.next();
    }
    let periodic = names.seam.is_some();
    let hole = names.hole.is_some();
    verify_partition([outer, inner], periodic, hole)?;
    found.finish(periodic, hole)
}

impl Occurrences {
    fn read_wire(&mut self, wire: &topods::Shape, names: &Names<'_>) -> Checked<bool> {
        let kind = wire
            .shape_type()
            .map_err(|e| native("full-turn wire kind", e))?;
        if kind != ShapeEnum::Wire {
            return Err(incomplete("full-turn trim wire", format!("{kind:?}")));
        }
        let mut uses = Explorer::new(wire, ShapeEnum::Edge, ShapeEnum::Shape)
            .map_err(|e| native("full-turn wire edges", e))?;
        let [mut outer, mut inner] = [0, 0];
        while uses.more() {
            let edge = topods::edge(uses.current()).map_err(|e| native("full-turn coedge", e))?;
            if self.append(edge, names)? {
                inner += 1;
            } else {
                outer += 1;
            }
            uses.next()
                .map_err(|e| native("next full-turn wire edge", e))?;
        }
        wire_role([outer, inner], names)
    }

    fn append(&mut self, edge: topods::Edge, names: &Names<'_>) -> Checked<bool> {
        if let Some(index) = names.rims.into_iter().position(|rim| rim.is_same(&edge)) {
            self.rims[index].push(edge);
            return Ok(false);
        }
        if names.seam.is_some_and(|seam| seam.is_same(&edge)) {
            self.seam.push(edge);
            return Ok(false);
        }
        if let Some(index) = names
            .hole
            .and_then(|hole| hole.into_iter().position(|image| image.is_same(&edge)))
        {
            self.hole[index].push(edge);
            return Ok(true);
        }
        Err(incomplete(
            "bound rim, seam or aperture edge",
            format!("{:?}", edge.orientation()),
        ))
    }

    fn finish(self, periodic: bool, has_hole: bool) -> Checked<FaceTrims> {
        let [low, high] = self.rims;
        let [low] = cardinality(low, "one use of the first rim")?;
        let [high] = cardinality(high, "one use of the second rim")?;
        let seam = if periodic {
            Some(cardinality(self.seam, "two uses of the periodic seam")?)
        } else {
            let [] = cardinality(self.seam, "zero planar seam uses")?;
            None
        };
        let hole = if has_hole {
            let [a, b, c, d] = self
                .hole
                .map(|uses| cardinality(uses, "one use of an aperture edge"));
            let ([a], [b], [c], [d]) = (a?, b?, c?, d?);
            Some([a, b, c, d])
        } else {
            None
        };
        Ok(FaceTrims {
            rims: [low, high],
            seam,
            hole,
        })
    }
}

fn wire_role([outer, inner]: [usize; 2], names: &Names<'_>) -> Checked<bool> {
    if [outer, inner] == [if names.seam.is_some() { 4 } else { 1 }, 0] {
        return Ok(false);
    }
    if names.hole.is_some() && [outer, inner] == [0, 4] {
        return Ok(true);
    }
    Err(incomplete(
        "complete outer or aperture wire",
        format!("{outer} outer uses and {inner} aperture uses"),
    ))
}

fn verify_partition([outer, inner]: [usize; 2], periodic: bool, hole: bool) -> Checked<()> {
    if [outer, inner] == [if periodic { 1 } else { 2 }, usize::from(hole)] {
        return Ok(());
    }
    Err(incomplete(
        "one seam-cut loop or two planar rim loops, with a separate admitted aperture loop",
        format!("{outer} outer and {inner} inner loops"),
    ))
}

fn cardinality<const N: usize>(
    items: Vec<topods::Edge>,
    expected: &'static str,
) -> Checked<[topods::Edge; N]> {
    items
        .try_into()
        .map_err(|items: Vec<_>| incomplete(expected, format!("{} uses", items.len())))
}
