//! Stored tolerance measurements and explicit report/refuse policy.
use super::{failure::*, history::Bindings, result::SurfaceDiagnostic};
use martensite_tkbo::profile_extrusion::{AchievedTolerance, ToleranceCap};
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkg3d::top_abs::ShapeEnum;
pub(super) fn tolerances(bindings: &Bindings) -> Checked<AchievedTolerance> {
    let mut achieved = AchievedTolerance {
        vertex: 0.,
        edge: 0.,
        face: 0.,
    };
    for (_, shape) in bindings {
        let (kind, value) =
            entity_tolerance(shape).map_err(|error| native("stored tolerance", error))?;
        match kind {
            ShapeEnum::Vertex => achieved.vertex = achieved.vertex.max(value),
            ShapeEnum::Edge => achieved.edge = achieved.edge.max(value),
            ShapeEnum::Face => achieved.face = achieved.face.max(value),
            _ => {}
        }
    }
    Ok(achieved)
}
pub(super) fn cap_diagnostics(
    achieved: AchievedTolerance,
    cap: ToleranceCap,
) -> Checked<Vec<SurfaceDiagnostic>> {
    match cap {
        ToleranceCap::Report(limit) if achieved.max() > limit => {
            Ok(vec![SurfaceDiagnostic::ToleranceCapReported {
                achieved: achieved.max(),
                limit,
            }])
        }
        ToleranceCap::Refuse(limit) if achieved.max() > limit => {
            Err(SurfaceFailureCause::ToleranceExceeded {
                achieved: achieved.max(),
                limit,
            })
        }
        _ => Ok(Vec::new()),
    }
}

fn entity_tolerance(
    shape: &topods::Shape,
) -> Result<(ShapeEnum, f64), martensite_tkernel::standard::Failure> {
    let kind = shape.shape_type()?;
    let value = match kind {
        ShapeEnum::Vertex => BRepTool::vertex_tolerance(&topods::vertex(shape)?)?,
        ShapeEnum::Edge => BRepTool::edge_tolerance(&topods::edge(shape)?)?,
        ShapeEnum::Face => BRepTool::face_tolerance(&topods::face(shape)?)?,
        _ => 0.,
    };
    Ok((kind, value))
}
