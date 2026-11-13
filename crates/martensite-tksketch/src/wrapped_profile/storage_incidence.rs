//! Shared endpoint identity and retained pole agreement, checked after decoding.
use super::*;
use martensite_tkbrep::topods::Vertex;
use martensite_tkbrep::{brep::BRepTool, topexp, topods};

pub(super) fn span(
    piece: &AuthoredPiece,
    curves: [&OwnedCurve; 2],
    bindings: &Bindings<'_>,
) -> Result<(), StorageFailure> {
    let roles = [HistoryRole::FirstDepthEdge, HistoryRole::LastDepthEdge];
    for (depth, role) in roles.into_iter().enumerate() {
        let key = key(piece.span, role);
        let ends = depth_vertices(piece, depth, bindings)?;
        check_edge(key, curves[depth], ends, bindings)?;
    }
    connector(piece.start, bindings)
}

fn depth_vertices(
    piece: &AuthoredPiece,
    depth: usize,
    bindings: &Bindings<'_>,
) -> Result<[Vertex; 2], StorageFailure> {
    let role = [HistoryRole::FirstDepthVertex, HistoryRole::LastDepthVertex][depth];
    Ok([
        vertex(piece.start, role, bindings)?,
        vertex(piece.end, role, bindings)?,
    ])
}

fn vertex(
    source: CornerSource,
    role: HistoryRole,
    bindings: &Bindings<'_>,
) -> Result<Vertex, StorageFailure> {
    let key = BindingKey {
        source: Source::Corner(source),
        role,
    };
    native(key, topods::vertex(lookup(key, bindings)?))
}

fn connector(source: CornerSource, bindings: &Bindings<'_>) -> Result<(), StorageFailure> {
    let key = BindingKey {
        source: Source::Corner(source),
        role: HistoryRole::NormalEdge,
    };
    let curve = capture::curve(key, bindings)?;
    linear_connector(key, &curve)?;
    let ends = [
        vertex(source, HistoryRole::FirstDepthVertex, bindings)?,
        vertex(source, HistoryRole::LastDepthVertex, bindings)?,
    ];
    check_edge(key, &curve, ends, bindings)
}

fn check_edge(
    key: BindingKey,
    curve: &OwnedCurve,
    expected: [Vertex; 2],
    bindings: &Bindings<'_>,
) -> Result<(), StorageFailure> {
    let edge = native(key, topods::edge(lookup(key, bindings)?))?;
    let (first, last) = topexp::vertices(&edge, false);
    let last_pole = curve.poles.len() - 1;
    for ((actual, expected), pole) in [first, last].iter().zip(&expected).zip([0, last_pole]) {
        check_identity(key, actual, expected)?;
        check_point(key, expected, curve.poles[pole])?;
    }
    Ok(())
}

fn check_identity(
    key: BindingKey,
    actual: &Vertex,
    expected: &Vertex,
) -> Result<(), StorageFailure> {
    if !actual.is_same(expected) {
        return Err(mismatch(key, "shared endpoint identity", vec![0], vec![1]));
    }
    Ok(())
}

fn check_point(key: BindingKey, vertex: &Vertex, pole: [f64; 3]) -> Result<(), StorageFailure> {
    let point = native(key, BRepTool::pnt(vertex))?;
    let actual = [point.x(), point.y(), point.z()].map(f64::to_bits);
    let expected = pole.map(f64::to_bits);
    if actual != expected {
        return Err(mismatch(
            key,
            "shared endpoint coordinates",
            actual.to_vec(),
            expected.to_vec(),
        ));
    }
    Ok(())
}

fn linear_connector(key: BindingKey, curve: &OwnedCurve) -> Result<(), StorageFailure> {
    if curve.poles.len() != 2 || curve.weights != [1., 1.] {
        return Err(mismatch(
            key,
            "linear normal connector",
            std::iter::once(curve.poles.len() as u64)
                .chain(curve.weights.iter().map(|w| w.to_bits()))
                .collect(),
            vec![2, 1f64.to_bits(), 1f64.to_bits()],
        ));
    }
    Ok(())
}
