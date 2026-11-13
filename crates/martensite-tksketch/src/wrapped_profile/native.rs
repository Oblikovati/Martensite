//! Construction and authored history for a rational cylinder sweep with holes.
use super::{boundaries::*, *};
use crate::surface_sketch::HistoryRole;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topods::{Builder, Shell, Solid, Vertex},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::GeomCylindricalSurface;
use martensite_tkmath::gp::{Axis3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;

pub(super) fn construct(request: &Request) -> Result<Constructed, Failure> {
    let caps = request
        .depths
        .map(|d| cap(request.radius + d, request.tolerance / 4.));
    let caps = [caps[0].clone()?, caps[1].clone()?];
    let mut bindings = cap_history(request, &caps);
    let mut pieces = Vec::new();
    for contour in &request.loops {
        contour_pieces(request, contour, &caps, &mut pieces, &mut bindings)?;
    }
    finish(request, &caps, &pieces, bindings)
}

fn finish(
    request: &Request,
    caps: &[Face; 2],
    pieces: &[NativePiece],
    mut bindings: Vec<(BindingKey, Shape)>,
) -> Result<Constructed, Failure> {
    let solid = assemble(caps, pieces)?;
    martensite_tktopalgo::brep_lib::same_parameter_shape(&solid, request.tolerance, false)?;
    retained_cap_parameters(request, pieces, caps)?;
    let source = Source::Region(request.region);
    bindings.insert(0, (key(source, HistoryRole::Solid), solid.clone()));
    Ok(Constructed { solid, bindings })
}

fn retained_cap_parameters(
    request: &Request,
    pieces: &[NativePiece],
    caps: &[Face; 2],
) -> Result<(), Failure> {
    for (source, piece) in request.loops.iter().flatten().zip(pieces) {
        for (depth, cap) in caps.iter().enumerate() {
            let captured = BRepTool::curve_on_face(&piece.edges[depth], cap)?;
            let curve: Handle<martensite_tkg2d::geom2d::Geom2dBezierCurve> = captured.pcurve.cast();
            let expected: Vec<_> = source
                .poles
                .iter()
                .map(|[s, z]| [s / request.radius, *z])
                .collect();
            retained_poles(&curve, &expected)?;
        }
    }
    Ok(())
}

fn retained_poles(
    curve: &Handle<martensite_tkg2d::geom2d::Geom2dBezierCurve>,
    expected: &[[f64; 2]],
) -> Result<(), Failure> {
    let Some(curve) = curve.get() else {
        return Err(Failure::new(
            "native conformance replaced an authored Bezier pcurve",
        ));
    };
    let poles = curve.poles();
    let actual: Vec<_> = poles.iter().map(|p| [p.x(), p.y()]).collect();
    if actual != expected {
        return Err(Failure::new(&format!(
            "native pcurve poles={actual:?}; expected unchanged authored poles={expected:?}"
        )));
    }
    Ok(())
}

fn cap(radius: f64, tolerance: f64) -> Result<Face, Failure> {
    let surface = Handle::new(GeomCylindricalSurface::new(Axis3::default(), radius)?).into_cast();
    let mut face = Face::default();
    BRepBuilder::default().make_face_on_surface(&mut face, &surface, tolerance)?;
    Ok(face)
}

fn cap_history(request: &Request, caps: &[Face; 2]) -> Vec<(BindingKey, Shape)> {
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
    request: &Request,
    contour: &[AuthoredPiece],
    caps: &[Face; 2],
    pieces: &mut Vec<NativePiece>,
    history: &mut Vec<(BindingKey, Shape)>,
) -> Result<(), Failure> {
    let vertices = contour
        .iter()
        .map(|p| vertex_pair(p.poles[0], request))
        .collect::<Result<Vec<_>, _>>()?;
    let connectors = vertices
        .iter()
        .map(connector)
        .collect::<Result<Vec<_>, _>>()?;
    let first = pieces.len();
    for (i, source) in contour.iter().enumerate() {
        let next = (i + 1) % contour.len();
        let piece = piece(
            request,
            source,
            [&vertices[i], &vertices[next]],
            [connectors[i].clone(), connectors[next].clone()],
            caps,
        )?;
        record_history(source, &piece, &vertices[i], history);
        pieces.push(piece);
    }
    cap_wires(caps, &pieces[first..])
}

fn vertex_pair(point: [f64; 2], request: &Request) -> Result<[Vertex; 2], Failure> {
    let make = |depth| -> Result<Vertex, Failure> {
        let [x, y, z] = angular::endpoint(point, request.radius, request.radius + depth);
        let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?.vertex()?;
        BRepBuilder::default().update_vertex_tolerance(&vertex, request.tolerance / 2.)?;
        Ok(vertex)
    };
    Ok([make(request.depths[0])?, make(request.depths[1])?])
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

fn depth_curve(
    coefficients: &[[f64; 4]],
    radius: f64,
    ends: [&Vertex; 2],
) -> Result<Handle<martensite_tkg3d::geom::GeomBezierCurve>, Failure> {
    let mut poles = angular::at_radius(coefficients, radius);
    let last = poles.points.len() - 1;
    for (index, vertex) in [(0, ends[0]), (last, ends[1])] {
        let point = BRepTool::pnt(vertex)?;
        poles.points[index] = [point.x(), point.y(), point.z()];
    }
    curve3(&poles)
}

fn piece(
    request: &Request,
    source: &AuthoredPiece,
    ends: [&[Vertex; 2]; 2],
    connectors: [Edge; 2],
    caps: &[Face; 2],
) -> Result<NativePiece, Failure> {
    let coefficients = angular::coefficients(&source.poles, request.radius);
    let make = |i| {
        depth_curve(
            &coefficients,
            request.radius + request.depths[i],
            [&ends[0][i], &ends[1][i]],
        )
    };
    let curves = [make(0)?, make(1)?];
    let edges = [
        edge(&curves[0], [&ends[0][0], &ends[1][0]])?,
        edge(&curves[1], [&ends[0][1], &ends[1][1]])?,
    ];
    let pcurve = curve2(
        &source
            .poles
            .iter()
            .map(|[s, z]| [s / request.radius, *z])
            .collect::<Vec<_>>(),
    )?;
    attach(&edges[0], &pcurve, &caps[0], request.tolerance / 2.)?;
    attach(&edges[1], &pcurve, &caps[1], request.tolerance / 2.)?;
    let side = side(&curves, &edges, &connectors, request.tolerance / 2.)?;
    Ok(NativePiece {
        edges,
        connectors,
        side,
    })
}

fn cap_wires(caps: &[Face; 2], pieces: &[NativePiece]) -> Result<(), Failure> {
    for (depth, cap) in caps.iter().enumerate() {
        let edges: Vec<_> = pieces
            .iter()
            .map(|p| p.edges[depth].clone().into_shape())
            .collect();
        let boundary = wire(&edges)?;
        Builder.add(&mut cap.clone(), &boundary)?;
    }
    Ok(())
}

fn assemble(caps: &[Face; 2], pieces: &[NativePiece]) -> Result<Shape, Failure> {
    let mut shell = Shell::default();
    Builder.make_shell(&mut shell);
    Builder.add(&mut shell, &caps[0].reversed())?;
    Builder.add(&mut shell, &caps[1])?;
    for piece in pieces {
        Builder.add(&mut shell, &piece.side)?;
    }
    shell.set_closed(true)?;
    let mut solid = Solid::default();
    Builder.make_solid(&mut solid);
    Builder.add(&mut solid, &shell)?;
    Ok(solid.into_shape())
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
    history.extend([
        (
            key(curve, HistoryRole::FirstDepthEdge),
            piece.edges[0].clone().into_shape(),
        ),
        (
            key(curve, HistoryRole::LastDepthEdge),
            piece.edges[1].clone().into_shape(),
        ),
        (
            key(curve, HistoryRole::SideFace),
            piece.side.clone().into_shape(),
        ),
        (
            key(vertex, HistoryRole::NormalEdge),
            piece.connectors[0].clone().into_shape(),
        ),
        (
            key(vertex, HistoryRole::FirstDepthVertex),
            start[0].clone().into_shape(),
        ),
        (
            key(vertex, HistoryRole::LastDepthVertex),
            start[1].clone().into_shape(),
        ),
    ]);
}
