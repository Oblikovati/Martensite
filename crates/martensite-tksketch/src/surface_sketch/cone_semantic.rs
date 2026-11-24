//! Match decoded cone entities to authored corners, analytic boundaries and face roles.
use super::{cone_admission::AdmittedCone, cone_math, failure::*, history::*, semantic::*};
use martensite_tkbrep::{brep::BRepTool, brep_adaptor::BRepAdaptorCurve, topods};
use martensite_tkg3d::top_abs::ShapeEnum;

pub(super) fn validate(admitted: &AdmittedCone, bindings: &Bindings) -> Checked<()> {
    for (key, shape) in bindings {
        match role_kind(key.role) {
            ShapeEnum::Vertex => check_vertex(admitted, *key, shape)?,
            ShapeEnum::Edge => check_edge(admitted, *key, shape)?,
            ShapeEnum::Face => super::cone_faces::check(admitted, *key, shape, bindings)?,
            _ => {}
        }
    }
    Ok(())
}
pub(super) fn tolerance(admitted: &AdmittedCone) -> f64 {
    admitted.sweep.options.resolution.weld + admitted.sweep.coordinate_error
}
pub(super) fn curve_index(admitted: &AdmittedCone, source: HistorySource) -> Checked<usize> {
    admitted
        .sector
        .boundaries
        .iter()
        .position(|id| source == HistorySource::Curve(*id))
        .ok_or_else(|| {
            incomplete(
                "cone authored boundary",
                format!("unexpected source {source:?}"),
            )
        })
}
fn corner_index(admitted: &AdmittedCone, source: HistorySource) -> Checked<usize> {
    admitted
        .sector
        .corners
        .iter()
        .position(|id| source == HistorySource::Vertex(*id))
        .ok_or_else(|| {
            incomplete(
                "cone authored corner",
                format!("unexpected source {source:?}"),
            )
        })
}
fn check_vertex(admitted: &AdmittedCone, key: HistoryKey, shape: &topods::Shape) -> Checked<()> {
    let index = corner_index(admitted, key.source)?;
    let depth = usize::from(key.role == HistoryRole::LastDepthVertex);
    let vertex = topods::vertex(shape).map_err(|e| native("cone semantic vertex", e))?;
    let actual = BRepTool::pnt(&vertex).map_err(|e| native("cone semantic point", e))?;
    close(
        distance(actual, cone_math::corner(admitted, index, depth)),
        0.,
        tolerance(admitted),
        "cone corner image",
    )
}
fn edge_samples(admitted: &AdmittedCone, key: HistoryKey) -> Checked<([[f64; 3]; 3], bool)> {
    if key.role == HistoryRole::NormalEdge {
        let index = corner_index(admitted, key.source)?;
        let first = cone_math::corner(admitted, index, 0);
        let last = cone_math::corner(admitted, index, 1);
        return Ok((
            [
                first,
                last,
                std::array::from_fn(|i| (first[i] + last[i]) / 2.),
            ],
            false,
        ));
    }
    boundary_samples(admitted, key)
}
fn boundary_samples(admitted: &AdmittedCone, key: HistoryKey) -> Checked<([[f64; 3]; 3], bool)> {
    let index = curve_index(admitted, key.source)?;
    let depth = usize::from(key.role == HistoryRole::LastDepthEdge);
    let first = cone_math::corner(admitted, index, depth);
    let last = cone_math::corner(admitted, (index + 1) % 4, depth);
    let circular = index.is_multiple_of(2);
    let middle = if circular {
        cone_math::point(
            admitted,
            admitted.start + admitted.sweep.angle / 2.,
            admitted.sector.slant[index / 2],
            admitted.depths[depth],
        )
    } else {
        std::array::from_fn(|i| (first[i] + last[i]) / 2.)
    };
    Ok(([first, last, middle], circular))
}
fn check_edge(admitted: &AdmittedCone, key: HistoryKey, shape: &topods::Shape) -> Checked<()> {
    let edge = topods::edge(shape).map_err(|e| native("cone semantic edge", e))?;
    let curve = BRepAdaptorCurve::with_edge(&edge).map_err(|e| native("cone semantic curve", e))?;
    let (samples, circular) = edge_samples(admitted, key)?;
    check_curve_kind(&curve, circular)?;
    check_edge_samples(&curve, samples, tolerance(admitted))?;
    if circular {
        close(
            curve_span(&curve)?,
            admitted.sweep.angle,
            admitted.sweep.options.resolution.angular,
            "cone circular trim span",
        )?;
    }
    Ok(())
}
