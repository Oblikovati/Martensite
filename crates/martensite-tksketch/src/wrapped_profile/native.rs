//! Construction and authored history for a rational cylinder sweep with holes.
use super::{boundaries::*, *};
use crate::surface_sketch::HistoryRole;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topods::{Builder, Vertex},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::GeomBezierCurve;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;

mod cylinder;
use operation::topology::DevelopedMapHandedness;
pub(super) trait CarrierBuilder {
    fn handedness(&self) -> DevelopedMapHandedness;
    fn caps(&self) -> Result<[Face; 2], Failure>;
    fn vertex_pair(&self, point: [f64; 2]) -> Result<[Vertex; 2], Failure>;
    fn curves(
        &self,
        source: &AuthoredPiece,
        ends: [&[Vertex; 2]; 2],
    ) -> Result<[Handle<GeomBezierCurve>; 2], Failure>;
    fn attach(
        &self,
        source: &AuthoredPiece,
        edges: &[Edge; 2],
        caps: &[Face; 2],
    ) -> Result<(), Failure>;
    fn check_caps(&self, pieces: &[NativePiece], caps: &[Face; 2]) -> Result<(), Failure>;
}
pub(super) fn construct(request: &Request) -> Result<Constructed, Failure> {
    construct_mapped(&request.source, &cylinder::Cylinder(request))
}
pub(super) fn construct_mapped(
    request: &DevelopedRegionRequest,
    mapping: &impl CarrierBuilder,
) -> Result<Constructed, Failure> {
    let caps = mapping.caps()?;
    let mut bindings = cap_history(request, &caps);
    let mut pieces = Vec::new();
    for contour in &request.loops {
        contour_pieces(request, mapping, contour, &caps, &mut pieces, &mut bindings)?;
    }
    finish(request, mapping, &caps, &pieces, bindings)
}
fn finish(
    request: &DevelopedRegionRequest,
    mapping: &impl CarrierBuilder,
    caps: &[Face; 2],
    pieces: &[NativePiece],
    mut bindings: Vec<(BindingKey, Shape)>,
) -> Result<Constructed, Failure> {
    let solid = assemble(caps, pieces, mapping.handedness())?;
    martensite_tktopalgo::brep_lib::same_parameter_shape(&solid, request.tolerance, false)?;
    mapping.check_caps(pieces, caps)?;
    let source = Source::Region(request.region);
    bindings.insert(0, (key(source, HistoryRole::Solid), solid.clone()));
    Ok(Constructed { solid, bindings })
}

fn cap_history(request: &DevelopedRegionRequest, caps: &[Face; 2]) -> Vec<(BindingKey, Shape)> {
    let source = Source::Region(request.region);
    vec![
        (
            key(source, HistoryRole::FirstDepthFace),
            caps[0].clone().into_shape(),
        ),
        (
            key(source, HistoryRole::LastDepthFace),
            caps[1].clone().into_shape(),
        ),
    ]
}

fn contour_pieces(
    request: &DevelopedRegionRequest,
    mapping: &impl CarrierBuilder,
    contour: &[AuthoredPiece],
    caps: &[Face; 2],
    pieces: &mut Vec<NativePiece>,
    history: &mut Vec<(BindingKey, Shape)>,
) -> Result<(), Failure> {
    let incidences = ContourIncidences::new(contour, mapping)?;
    let first = pieces.len();
    for (i, source) in contour.iter().enumerate() {
        let piece = incidences.piece_at(request, mapping, source, i, caps)?;
        record_history(source, &piece, &incidences.vertices[i], history);
        pieces.push(piece);
    }
    cap_wires(caps, &pieces[first..], mapping.handedness())
}
struct ContourIncidences {
    vertices: Vec<[Vertex; 2]>,
    connectors: Vec<Edge>,
}
impl ContourIncidences {
    fn new(contour: &[AuthoredPiece], mapping: &impl CarrierBuilder) -> Result<Self, Failure> {
        let vertices = contour
            .iter()
            .map(|p| mapping.vertex_pair(p.poles[0]))
            .collect::<Result<Vec<_>, _>>()?;
        let connectors = vertices
            .iter()
            .map(connector)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            vertices,
            connectors,
        })
    }
    fn piece_at(
        &self,
        request: &DevelopedRegionRequest,
        mapping: &impl CarrierBuilder,
        source: &AuthoredPiece,
        i: usize,
        caps: &[Face; 2],
    ) -> Result<NativePiece, Failure> {
        let next = (i + 1) % self.vertices.len();
        piece(
            request,
            mapping,
            source,
            [&self.vertices[i], &self.vertices[next]],
            [self.connectors[i].clone(), self.connectors[next].clone()],
            caps,
        )
    }
}

fn connector(vertices: &[Vertex; 2]) -> Result<Edge, Failure> {
    let points = vertices
        .iter()
        .map(|v| {
            let p = BRepTool::pnt(v)?;
            Ok([p.x(), p.y(), p.z()])
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    let poles = angular::RationalPoles {
        points,
        weights: vec![1.; 2],
    };
    edge(&curve3(&poles)?, [&vertices[0], &vertices[1]])
}

fn piece(
    request: &DevelopedRegionRequest,
    mapping: &impl CarrierBuilder,
    source: &AuthoredPiece,
    ends: [&[Vertex; 2]; 2],
    connectors: [Edge; 2],
    caps: &[Face; 2],
) -> Result<NativePiece, Failure> {
    let curves = mapping.curves(source, ends)?;
    let edges = [
        edge(&curves[0], [&ends[0][0], &ends[1][0]])?,
        edge(&curves[1], [&ends[0][1], &ends[1][1]])?,
    ];
    mapping.attach(source, &edges, caps)?;
    let side = side(&curves, &edges, &connectors, request.tolerance / 2.)?;
    Ok(NativePiece {
        edges,
        connectors,
        side,
    })
}

fn cap_wires(
    caps: &[Face; 2],
    pieces: &[NativePiece],
    handedness: DevelopedMapHandedness,
) -> Result<(), Failure> {
    for (depth, cap) in caps.iter().enumerate() {
        let edges = cap_edges(pieces, depth, handedness);
        let boundary = wire(&edges)?;
        Builder.add(&mut cap.clone(), &boundary)?;
    }
    Ok(())
}

fn cap_edges(
    pieces: &[NativePiece],
    depth: usize,
    handedness: DevelopedMapHandedness,
) -> Vec<Shape> {
    // #540: reverse oriented incidence, never the authored curve's parameterization or identity.
    if handedness.reverses() {
        return pieces
            .iter()
            .rev()
            .map(|piece| piece.edges[depth].reversed())
            .collect();
    }
    pieces
        .iter()
        .map(|piece| piece.edges[depth].clone().into_shape())
        .collect()
}

fn assemble(
    caps: &[Face; 2],
    pieces: &[NativePiece],
    handedness: DevelopedMapHandedness,
) -> Result<Shape, Failure> {
    let mut assembly = operation::topology::SolidAssembly::new();
    assembly.add(&caps[0], true)?;
    assembly.add(&caps[1], false)?;
    for piece in pieces {
        assembly.add(&piece.side, handedness.reverses())?;
    }
    Ok(assembly.finish()?.into_shape())
}

fn key(source: Source, role: HistoryRole) -> BindingKey {
    BindingKey { source, role }
}

fn record_history(
    source: &AuthoredPiece,
    piece: &NativePiece,
    start: &[Vertex; 2],
    history: &mut Vec<(BindingKey, Shape)>,
) {
    let curve = Source::Span(source.span);
    let vertex = Source::Corner(source.start);
    let [first_edge, last_edge, side] = span_history(curve, piece);
    let [normal, first_vertex, last_vertex] = corner_history(vertex, piece, start);
    history.extend([
        first_edge,
        last_edge,
        side,
        normal,
        first_vertex,
        last_vertex,
    ]);
}
fn span_history(source: Source, piece: &NativePiece) -> [(BindingKey, Shape); 3] {
    let images = [
        (
            HistoryRole::FirstDepthEdge,
            piece.edges[0].clone().into_shape(),
        ),
        (
            HistoryRole::LastDepthEdge,
            piece.edges[1].clone().into_shape(),
        ),
        (HistoryRole::SideFace, piece.side.clone().into_shape()),
    ];
    images.map(|(role, shape)| (key(source, role), shape))
}
fn corner_history(
    source: Source,
    piece: &NativePiece,
    start: &[Vertex; 2],
) -> [(BindingKey, Shape); 3] {
    let images = [
        (
            HistoryRole::NormalEdge,
            piece.connectors[0].clone().into_shape(),
        ),
        (HistoryRole::FirstDepthVertex, start[0].clone().into_shape()),
        (HistoryRole::LastDepthVertex, start[1].clone().into_shape()),
    ];
    images.map(|(role, shape)| (key(source, role), shape))
}
