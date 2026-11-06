//! Face/face interference transitions with the source orientation conventions.
use super::{TopOpeBRepLineInter, TopOpeBRepVPointInter};
use crate::top_ope_brep_ds::TopOpeBRepDSTransition as Transition;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, complement};
use martensite_tkgeomalgo::int_surf::{IntSurfSituation, IntSurfTransition, IntSurfTypeTrans};

/// Converts point and line transitions to topological before/after states.
/// Example: `TopOpeBRepFFTransitionTool::process_edge_transition(&point, 1, orientation)?`.
pub struct TopOpeBRepFFTransitionTool;

fn orientation(kind: IntSurfTypeTrans, situation: IntSurfSituation) -> Option<Orientation> {
    match kind {
        IntSurfTypeTrans::In => Some(Orientation::Forward),
        IntSurfTypeTrans::Out => Some(Orientation::Reversed),
        IntSurfTypeTrans::Touch => match situation {
            IntSurfSituation::Inside => Some(Orientation::Internal),
            IntSurfSituation::Outside => Some(Orientation::External),
            IntSurfSituation::Unknown => None,
        },
        IntSurfTypeTrans::Undecided => None,
    }
}
fn oriented(value: Option<Orientation>, boundary: Orientation) -> Transition {
    value
        .map(|value| {
            Transition::from_orientation(if boundary == Orientation::Reversed {
                complement(value)
            } else {
                value
            })
        })
        .unwrap_or_default()
}
fn point_transition(
    value: IntSurfTransition,
    boundary: Orientation,
) -> Result<Transition, Failure> {
    let situation = if value.transition_type() == IntSurfTypeTrans::Touch {
        value.situation()?
    } else {
        IntSurfSituation::Unknown
    };
    Ok(oriented(
        orientation(value.transition_type(), situation),
        boundary,
    ))
}
fn fixed(boundary: Orientation) -> Option<Transition> {
    matches!(boundary, Orientation::Internal | Orientation::External)
        .then(|| Transition::from_orientation(boundary))
}
impl TopOpeBRepFFTransitionTool {
    /// Uses the point's line/arc transition and the edge orientation; `Self::process_line_transition(&point, 1, edge.orientation())?`.
    pub fn process_line_transition(
        point: &TopOpeBRepVPointInter<'_>,
        index: i32,
        edge: Orientation,
    ) -> Result<Transition, Failure> {
        if let Some(value) = fixed(edge) {
            return Ok(value);
        }
        let value = match index {
            1 => point.transition_line_arc1()?,
            2 => point.transition_line_arc2()?,
            _ => IntSurfTransition::new(),
        };
        point_transition(value, edge)
    }
    /// Classifies exact first/last line parameters; `Self::process_line_transition_on_line(&point, &line)?`.
    pub fn process_line_transition_on_line(
        point: &TopOpeBRepVPointInter<'_>,
        line: &TopOpeBRepLineInter,
    ) -> Result<Transition, Failure> {
        let first = line.vpoint(1)?.parameter_on_line()?;
        let last = line.vpoint(line.nb_vpoint())?.parameter_on_line()?;
        let parameter = point.parameter_on_line()?;
        let value = if parameter == first {
            Orientation::Forward
        } else if parameter == last {
            Orientation::Reversed
        } else {
            Orientation::Internal
        };
        Ok(Transition::from_orientation(value))
    }
    /// Uses the point's surface transition and face orientation; `Self::process_edge_transition(&point, 2, face.orientation())?`.
    pub fn process_edge_transition(
        point: &TopOpeBRepVPointInter<'_>,
        index: i32,
        face: Orientation,
    ) -> Result<Transition, Failure> {
        if let Some(value) = fixed(face) {
            return Ok(value);
        }
        let value = match index {
            1 => point.transition_on_s1()?,
            2 => point.transition_on_s2()?,
            _ => IntSurfTransition::new(),
        };
        point_transition(value, face)
    }
    /// Converts line crossing/touching relative to a face; `Self::process_face_transition(&line, 1, face.orientation())?`.
    pub fn process_face_transition(
        line: &TopOpeBRepLineInter,
        index: i32,
        face: Orientation,
    ) -> Result<Transition, Failure> {
        if let Some(value) = fixed(face) {
            return Ok(value);
        }
        let kind = if index == 1 {
            line.transition_on_s1()?
        } else {
            line.transition_on_s2()?
        };
        let situation = if kind != IntSurfTypeTrans::Touch {
            IntSurfSituation::Unknown
        } else if index == 1 {
            line.situation_s1()?
        } else {
            line.situation_s2()?
        };
        Ok(oriented(orientation(kind, situation), face))
    }
}
