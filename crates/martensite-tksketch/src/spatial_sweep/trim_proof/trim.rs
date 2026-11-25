// SPDX-License-Identifier: GPL-2.0-only
//! Actual directed, source-bound trim capture for the physical flux proof.
use super::{FaceProof, Failure, Point, Span, Stage, native, poll};
use crate::{
    spatial_prism::volume::{affine_curve, interval::l1_difference},
    spatial_sweep::geometry::trim_basis,
};
use martensite_tkbrep::{
    brep::{BRepTool, EdgeCurve3D, EdgePCurve},
    topexp::{self, Explorer},
    topods::{self, Edge, Face, Vertex},
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::{
    geom::{GeomCircle, GeomCurve, GeomTrimmedCurve},
    top_abs::{Orientation, ShapeEnum},
};

pub(super) struct Trim {
    pub pcurve: EdgePCurve,
    pub spatial: EdgeCurve3D,
    pub ends: [Vertex; 2],
    pub reverse: bool,
}

pub(super) fn ordered(
    face: &Face,
    proof: FaceProof,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<Trim>, Failure> {
    let expected = || proof.refuse("one topologically closed directed outer contour");
    let mut outer = native(
        Explorer::new(face, ShapeEnum::Wire, ShapeEnum::Shape),
        Stage::Integration,
    )?;
    if !outer.more() {
        return Err(expected());
    }
    let wire = outer.current().clone();
    native(Explorer::next(&mut outer), Stage::Integration)?;
    if outer.more() || !native(wire.closed(), Stage::Integration)? {
        return Err(expected());
    }
    let edges = native(
        Explorer::new(&wire, ShapeEnum::Edge, ShapeEnum::Shape),
        Stage::Integration,
    )?;
    let trims = edges
        .map(|edge| {
            poll(cancelled, Stage::Integration)?;
            decode(
                native(topods::edge(&edge), Stage::Integration)?,
                face,
                proof,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    sort_cycle(trims, proof)
}

fn decode(edge: Edge, face: &Face, proof: FaceProof) -> Result<Trim, Failure> {
    let reverse = match edge.orientation() {
        Orientation::Forward => false,
        Orientation::Reversed => true,
        _ => return Err(proof.refuse("an explicitly oriented face edge")),
    };
    if native(BRepTool::degenerated(&edge), Stage::Integration)? {
        return Err(proof.refuse("a nondegenerate edge on every active trim"));
    }
    let pcurve = native(BRepTool::curve_on_face(&edge, face), Stage::Integration)?;
    let pcurve =
        trim_basis(pcurve).ok_or_else(|| proof.refuse("an exact finite active pcurve range"))?;
    let spatial = native(BRepTool::curve_with_location(&edge), Stage::Integration)?;
    let (a, b) = topexp::vertices(&edge, true);
    if a.is_null() || b.is_null() || a.is_same(&b) {
        return Err(proof.refuse("two distinct incident native vertices"));
    }
    Ok(Trim {
        reverse,
        pcurve,
        spatial,
        ends: [a, b],
    })
}

fn sort_cycle(trims: Vec<Trim>, proof: FaceProof) -> Result<Vec<Trim>, Failure> {
    if trims.len() < 3 {
        return Err(proof.refuse("at least three full trim occurrences on each face"));
    }
    let mut outgoing = IndexedMapOfShape::default();
    for (i, trim) in trims.iter().enumerate() {
        if outgoing.add(trim.ends[0].to_shape()) != i + 1 {
            return Err(proof.refuse("a unique outgoing edge at every directed trim vertex"));
        }
    }
    let mut current = 0;
    let mut cycle = Vec::with_capacity(trims.len());
    let count = trims.len();
    let mut pending = trims.into_iter().map(Some).collect::<Vec<_>>();
    for visit in 1..=count {
        let trim = pending
            .get_mut(current)
            .and_then(Option::take)
            .ok_or_else(|| proof.refuse("one complete directed trim cycle"))?;
        current = outgoing
            .find_index(&trim.ends[1])
            .checked_sub(1)
            .ok_or_else(|| proof.refuse("closed actual trim vertex incidence"))?;
        if (current == 0) != (visit == count) {
            return Err(proof.refuse("a single directed contour without disconnected trims"));
        }
        cycle.push(trim);
    }
    Ok(cycle)
}

impl Trim {
    pub fn linear_uv(&self) -> Option<[[Span; 2]; 2]> {
        affine_curve::planar(&self.pcurve, self.reverse, true)
    }

    pub fn linear_xyz(&self, reference: [f64; 3]) -> Option<[Point; 2]> {
        affine_curve::spatial(&self.spatial, self.reverse, true, reference)
    }

    pub fn directed_range(&self) -> [f64; 2] {
        let mut domain = [self.pcurve.first, self.pcurve.last];
        if self.reverse {
            domain.reverse();
        }
        domain
    }
}

pub(super) fn vertex_trace(
    trim: &Trim,
    expected: [Point; 2],
    proof: FaceProof,
) -> Result<(), Failure> {
    for (vertex, expected) in trim.ends.iter().zip(expected) {
        if !vertex.location().is_identity() {
            return Err(proof.refuse("unlocated native corner vertices on every measured trim"));
        }
        let actual = native(BRepTool::pnt(vertex), Stage::Integration)?;
        let local = std::array::from_fn(|i| {
            Span::at(actual.coordinates()[i]) - Span::at(proof.reference[i])
        });
        require(
            l1_difference(local, expected),
            proof,
            "decoded trim endpoint agrees with its vertex",
        )?;
    }
    Ok(())
}

pub(super) fn require(
    bound: f64,
    proof: FaceProof,
    description: &'static str,
) -> Result<(), Failure> {
    if !bound.is_finite() || bound > proof.limit {
        return Err(Failure::Trace {
            description,
            bound,
            limit: proof.limit,
        });
    }
    Ok(())
}

pub(super) fn spatial_circle(trim: &Trim, proof: FaceProof) -> Result<[Point; 3], Failure> {
    let expected = || {
        proof.refuse(
            "a literal spatial circle with the same active parameter range as its retained pcurve",
        )
    };
    let native = &trim.spatial;
    if !native.location.is_identity()
        || native.first != trim.pcurve.first
        || native.last != trim.pcurve.last
    {
        return Err(expected());
    }
    let source = if let Some(inside) = native.curve.downcast_ref::<GeomTrimmedCurve>() {
        if native.first < inside.first_parameter() || native.last > inside.last_parameter() {
            return Err(expected());
        }
        inside.basis_curve()
    } else {
        native.curve.clone()
    };
    let circle = source.downcast_ref::<GeomCircle>().ok_or_else(expected)?;
    let carrier = super::native(circle.circ(), Stage::Integration)?;
    let origin = std::array::from_fn(|i| {
        Span::at(carrier.location().coordinates()[i]) - Span::at(proof.reference[i])
    });
    let radius = Span::at(carrier.radius());
    let x = carrier
        .x_axis()
        .direction()
        .coordinates()
        .map(|x| Span::at(x) * radius);
    let y = carrier
        .y_axis()
        .direction()
        .coordinates()
        .map(|x| Span::at(x) * radius);
    Ok([origin, x, y])
}
