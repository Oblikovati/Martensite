//! A whole footprint rectangle stays in one material component when no trim crosses it.
use super::*;
use band::{BandChecked as Checked, BandFailure};
use holes::{TrimBox, curves};
use martensite_tkbo::bop_tools::BoxSelector;
use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::{
    bvh::{BvhBoxSet, BvhSetTraverse, BvhVec2d},
    predicates::{OrientationSign, orient2d},
};

/// An enclosure could not be proved within the selected outer trim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum OuterTrimRefusal {
    /// The enclosure could not be separated from one outer pcurve.
    Boundary {
        /// One-based edge occurrence ordinal local to this support.
        edge: usize,
    },
    /// The enclosed region lies in a component without the required material winding.
    Winding {
        /// Exact winding number of the directed boundary about the enclosure's first corner.
        actual: i64,
    },
}

pub(super) struct OuterTrim {
    edges: Vec<DirectedEdge>,
    bounds: TrimBox,
    tree: BvhBoxSet<BvhVec2d>,
}

struct DirectedEdge {
    chord: [[f64; 2]; 2],
    bounds: TrimBox,
    straight: bool,
}

impl OuterTrim {
    pub fn capture(face: &Face, wire: &Shape) -> Checked<Self> {
        Self::from_edges(capture_edges(face, wire)?)
    }

    fn from_edges(edges: Vec<DirectedEdge>) -> Checked<Self> {
        validate_cycle(&edges)?;
        let mut bounds = TrimBox::empty();
        let mut tree = BvhBoxSet::default();
        for (index, edge) in edges.iter().enumerate() {
            let ordinal = i32::try_from(index)
                .map_err(|_| unsupported("representable outer edge ordinal"))?;
            tree.add(ordinal, edge.bounds.bvh());
            bounds.include_box(edge.bounds);
        }
        admit_lift(bounds)?;
        native(tree.build())?;
        Ok(Self {
            edges,
            bounds,
            tree,
        })
    }

    pub fn bounds(&self) -> TrimBox {
        self.bounds
    }
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn check(&mut self, query: TrimBox) -> Checked<Option<OuterTrimRefusal>> {
        if !query.finite() {
            return Err(unsupported(
                "finite whole-profile enclosure for the outer trim",
            ));
        }
        for index in self.candidates(query)? {
            let edge = &self.edges[index];
            if !edge.straight || !line_separates(edge.chord, query)? {
                return Ok(Some(OuterTrimRefusal::Boundary { edge: index + 1 }));
            }
        }
        let actual = self.winding(query.corners()[0])?;
        Ok((actual != 1).then_some(OuterTrimRefusal::Winding { actual }))
    }

    fn candidates(&mut self, bounds: TrimBox) -> Checked<Vec<usize>> {
        let mut selector = BoxSelector::default();
        selector.set_bvh_set(Some(&mut self.tree));
        selector.set_box(bounds.bvh());
        native(selector.select())?;
        let mut candidates: Vec<_> = selector
            .indices()
            .iter()
            .map(|&index| index as usize)
            .collect();
        candidates.sort_unstable();
        Ok(candidates)
    }

    fn winding(&mut self, at: [f64; 2]) -> Checked<i64> {
        let [[_, right], _] = self.bounds.0;
        if at[0] > right {
            return Ok(0);
        }
        let ray = TrimBox([[at[0], right], [at[1]; 2]]);
        self.candidates(ray)?
            .into_iter()
            .try_fold(0, |total, index| {
                // Once the query is separated, each arc can move to its chord inside its own hull.
                Ok(total + crossing(self.edges[index].chord, at)?)
            })
    }
}

#[cfg(test)]
mod tests;

fn capture_edges(face: &Face, wire: &Shape) -> Checked<Vec<DirectedEdge>> {
    let mut edges = native(Explorer::new(wire, ShapeEnum::Edge, ShapeEnum::Shape))?;
    let mut directed = Vec::new();
    while edges.more() {
        directed.push(capture_edge(face, edges.current())?);
        native(edges.next())?;
    }
    Ok(directed)
}

fn capture_edge(face: &Face, shape: &Shape) -> Checked<DirectedEdge> {
    let edge = native(topods::edge(shape))?;
    if band::stored_seam(face, &edge)? {
        return Err(unsupported("one nonseam pcurve per shaped outer trim edge"));
    }
    let stored = native(BRepTool::curve_on_face(&edge, face))?;
    if !stored.is_stored
        || !stored.first.is_finite()
        || !stored.last.is_finite()
        || stored.first >= stored.last
    {
        return Err(unsupported(
            "stored outer pcurve with positive finite parameter range",
        ));
    }
    let captured = curves::envelope(&stored.pcurve, [stored.first, stored.last])?;
    let mut chord = captured
        .ends
        .ok_or_else(|| unsupported("exact representable outer pcurve endpoints"))?;
    match edge.orientation() {
        Orientation::Forward => (),
        Orientation::Reversed => chord.reverse(),
        _ => return Err(unsupported("forward or reversed shaped outer edge")),
    }
    Ok(DirectedEdge {
        chord,
        bounds: captured.bounds,
        straight: captured.straight,
    })
}

fn validate_cycle(edges: &[DirectedEdge]) -> Checked<()> {
    if edges.len() < 3
        || edges
            .iter()
            .zip(edges.iter().cycle().skip(1))
            .any(|(here, next)| here.chord[1] != next.chord[0] || here.chord[0] == here.chord[1])
    {
        return Err(unsupported(
            "one directed outer pcurve cycle with exact endpoint joins",
        ));
    }
    Ok(())
}

fn admit_lift(bounds: TrimBox) -> Checked<()> {
    let [[u0, u1], [v0, v1]] = bounds.0;
    if bounds.finite() && u0 < u1 && v0 < v1 && (u1 - u0).next_up() < std::f64::consts::TAU {
        return Ok(());
    }
    Err(unsupported(
        "nondegenerate outer trim in one lift strictly narrower than a period",
    ))
}

fn line_separates([first, last]: [[f64; 2]; 2], bounds: TrimBox) -> Checked<bool> {
    let [a, b, c, d] = bounds.corners();
    let sign = orientation(first, last, a)?;
    Ok(sign != OrientationSign::Zero
        && sign == orientation(first, last, b)?
        && sign == orientation(first, last, c)?
        && sign == orientation(first, last, d)?)
}

fn crossing([a, b]: [[f64; 2]; 2], at: [f64; 2]) -> Checked<i64> {
    if a[1] <= at[1] && at[1] < b[1] && orientation(a, b, at)? == OrientationSign::Positive {
        return Ok(1);
    }
    if b[1] <= at[1] && at[1] < a[1] && orientation(a, b, at)? == OrientationSign::Negative {
        return Ok(-1);
    }
    Ok(0)
}

fn orientation(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> Checked<OrientationSign> {
    orient2d(a, b, c).map_err(|_| unsupported("finite exact orientation inputs for the outer trim"))
}

fn native<T>(result: Result<T, Failure>) -> Checked<T> {
    result.map_err(|cause| BandFailure::Native(Box::new(cause)))
}

fn unsupported(expected: &'static str) -> BandFailure {
    BandFailure::Unsupported(expected)
}
