// SPDX-License-Identifier: GPL-2.0-only
//! #562: numeric rectangle admission cannot replace strict raw-source revision binding.
use super::*;
use martensite_tkg3d::geom::GeomBezierCurve;
use martensite_tkmath::gp::Transform3;
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::body_check::certified_conical) struct SideSource {
    wire_orientation: u64,
    boundaries: Vec<BoundarySource>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct BoundarySource {
    use_orientation: u64,
    pcurve: CurveSource<2>,
    spatial: CurveSource<3>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct CurveSource<const D: usize> {
    poles: Vec<[u64; D]>,
    weights: Vec<u64>,
    range: [u64; 2],
    degree: i32,
    rational: bool,
    location: Vec<u64>,
}
pub(in crate::body_check::certified_conical) fn capture(
    face: &Face,
    edges: &[Shape; 4],
) -> Result<SideSource, Failure> {
    let wires = children(face, ShapeEnum::Wire)?;
    let wire = wires
        .first()
        .ok_or_else(|| rejected("raw side source requires admitted wire"))?;
    let boundaries = edges
        .iter()
        .map(|shape| boundary(face, shape))
        .collect::<Result<_, _>>()?;
    Ok(SideSource {
        wire_orientation: wire.orientation() as u64,
        boundaries,
    })
}
fn boundary(face: &Face, shape: &Shape) -> Result<BoundarySource, Failure> {
    let edge = topods::edge(shape)?;
    let pcurve = planar(face, &edge)?;
    let spatial = spatial(&edge)?;
    Ok(BoundarySource {
        use_orientation: shape.orientation() as u64,
        pcurve,
        spatial,
    })
}
fn planar(face: &Face, edge: &topods::Edge) -> Result<CurveSource<2>, Failure> {
    let captured = BRepTool::curve_on_face(edge, face)?;
    let handle: Handle<Geom2dBezierCurve> = captured.pcurve.cast();
    let curve = handle
        .get()
        .ok_or_else(|| rejected("raw side source requires admitted Bezier pcurve"))?;
    let poles = curve
        .poles()
        .iter()
        .map(|p| [p.x().to_bits(), p.y().to_bits()])
        .collect();
    let weights = curve.weights_array(|w| w.iter().map(|x| x.to_bits()).collect());
    let (_, location) = BRepTool::surface_with_location(face)?;
    Ok(CurveSource {
        poles,
        weights,
        range: [captured.first, captured.last].map(f64::to_bits),
        degree: curve.degree(),
        rational: curve.is_rational(),
        location: location_bits(&location.transformation())?,
    })
}
fn spatial(edge: &topods::Edge) -> Result<CurveSource<3>, Failure> {
    let captured = BRepTool::curve_with_location(edge)?;
    let handle: Handle<GeomBezierCurve> = captured.curve.cast();
    let curve = handle
        .get()
        .ok_or_else(|| rejected("raw side source requires admitted Bezier spatial curve"))?;
    let poles = curve
        .poles()
        .iter()
        .map(|p| [p.x(), p.y(), p.z()].map(f64::to_bits))
        .collect();
    let weights = curve.weights_array(|w| w.iter().map(|x| x.to_bits()).collect());
    Ok(CurveSource {
        poles,
        weights,
        range: [captured.first, captured.last].map(f64::to_bits),
        degree: curve.degree(),
        rational: curve.is_rational(),
        location: location_bits(&captured.location.transformation())?,
    })
}
fn location_bits(transform: &Transform3) -> Result<Vec<u64>, Failure> {
    (0..12)
        .map(|index| {
            transform
                .value(index / 4 + 1, index % 4 + 1)
                .map(f64::to_bits)
        })
        .collect()
}
impl SideSource {
    pub(in crate::body_check::certified_conical) fn append(&self, bits: &mut impl Extend<u64>) {
        bits.extend([self.wire_orientation, self.boundaries.len() as u64]);
        for boundary in &self.boundaries {
            bits.extend([boundary.use_orientation]);
            append_curve(&boundary.pcurve, bits);
            append_curve(&boundary.spatial, bits);
        }
    }
}
fn append_curve<const D: usize>(curve: &CurveSource<D>, bits: &mut impl Extend<u64>) {
    bits.extend([
        D as u64,
        curve.degree as u64,
        u64::from(curve.rational),
        curve.poles.len() as u64,
        curve.weights.len() as u64,
    ]);
    bits.extend(curve.range);
    bits.extend(curve.poles.iter().flatten().copied());
    bits.extend(curve.weights.iter().copied());
    bits.extend([curve.location.len() as u64]);
    bits.extend(curve.location.iter().copied());
}
