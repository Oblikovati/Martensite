//! Geometric edge-on-restriction transition using the face normal and tangents.
use super::{TopOpeBRepFFTransitionTool, TopOpeBRepVPointInter};
use crate::top_ope_brep_ds::TopOpeBRepDSTransition as Transition;
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Edge, Face, Shape},
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, State};
use martensite_tkgeomalgo::geom_api::ProjectPointOnCurve;
use martensite_tkmath::gp::{Point2, Vector3};

fn surface_normal(face: &Face, uv: Point2) -> Result<Vector3, Failure> {
    let surface = BRepTool::surface(face)?;
    let surface = surface
        .get()
        .ok_or_else(|| Failure::new("null face surface; expected edge-on support surface"))?;
    let derivatives = surface.d1(uv.x(), uv.y())?;
    derivatives
        .d1u
        .normalized()?
        .crossed(derivatives.d1v.normalized()?)
        .normalized()
}
fn crossing_tangents(
    edge: &Edge,
    restriction: &Edge,
    parameter: f64,
) -> Result<Option<(Vector3, Vector3)>, Failure> {
    let (curve, _, _) = BRepTool::curve(edge)?;
    let (boundary, first, last) = BRepTool::curve(restriction)?;
    let (Some(curve), Some(boundary_curve)) = (curve.get(), boundary.get()) else {
        return Ok(None);
    };
    let (position, tangent) = curve.d1(parameter)?;
    let projection = ProjectPointOnCurve::with_range(position, &boundary, first, last)?;
    if projection.nb_points() == 0 {
        return Ok(None);
    }
    let (_, other_tangent) = boundary_curve.d1(projection.lower_distance_parameter()?)?;
    Ok(Some((tangent, other_tangent)))
}
fn crossing_transition(entering: bool, orientation: Orientation) -> Transition {
    let mut result = Transition::default();
    result.set_state_before(if entering { State::Out } else { State::In });
    result.set_state_after(if entering { State::In } else { State::Out });
    if orientation == Orientation::Reversed {
        result.complement()
    } else {
        result
    }
}
impl TopOpeBRepFFTransitionTool {
    /// Uses oriented edge tangents on the surface; `Self::process_edge_on_transition(&point, 1, &restriction, &edge, &face)?`.
    pub fn process_edge_on_transition(
        point: &TopOpeBRepVPointInter<'_>,
        index: i32,
        restriction: &Shape,
        edge: &Shape,
        face: &Shape,
    ) -> Result<Transition, Failure> {
        let edge = topods::edge(edge)?;
        let restriction = topods::edge(restriction)?;
        let face = topods::face(face)?;
        let parameter = point.edge_parameter(index)?;
        let Some((tangent, other)) = crossing_tangents(&edge, &restriction, parameter)? else {
            return Ok(Transition::default());
        };
        let normal = surface_normal(&face, point.surface_parameters(index)?)?;
        Ok(crossing_transition(
            normal.crossed(tangent).dot(other) > 0.,
            edge.orientation(),
        ))
    }
}
