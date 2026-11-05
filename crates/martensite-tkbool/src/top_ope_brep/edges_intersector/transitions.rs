use super::TopOpeBRepEdgesIntersector;
use crate::{top_ope_brep::TopOpeBRepP2DStatus, top_ope_brep_ds::TopOpeBRepDSTransition};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};
use martensite_tkgeomalgo::int_res2d::{IntersectionPoint, Position, Situation, TypeTrans};
impl TopOpeBRepEdgesIntersector {
    /// Reports the raw intersection/segment endpoint cursor category; `state.status1()`.
    pub fn status1(&self) -> TopOpeBRepP2DStatus {
        if self.raw_cursor <= self.point_count {
            return TopOpeBRepP2DStatus::Intersection;
        }
        if (self.raw_cursor - self.point_count - 1) % 2 == 0 {
            TopOpeBRepP2DStatus::SegmentFirst
        } else {
            TopOpeBRepP2DStatus::SegmentLast
        }
    }
    pub(super) fn point_transition(
        &self,
        point: IntersectionPoint,
        index: usize,
        opposite: Option<bool>,
    ) -> Result<TopOpeBRepDSTransition, Failure> {
        let orientation = self.edges[1 - index].orientation();
        let pure = opposite.is_some() && self.same_domain;
        let shape = if self.dimension == 1 || pure {
            ShapeEnum::Edge
        } else {
            ShapeEnum::Face
        };
        if matches!(orientation, Orientation::Internal | Orientation::External) {
            return Ok(oriented_transition(shape, orientation));
        }
        let (before, after, extremity) = transition_states(point, index, opposite)?;
        let shape = if extremity { ShapeEnum::Edge } else { shape };
        let result = TopOpeBRepDSTransition::new(before, after, shape, shape);
        if pure || extremity {
            return Ok(result);
        }
        Ok(compose_transition(
            result,
            orientation,
            self.reference_orientations[1 - index],
        ))
    }
}
fn oriented_transition(shape: ShapeEnum, orientation: Orientation) -> TopOpeBRepDSTransition {
    let mut result = TopOpeBRepDSTransition::new(State::In, State::In, shape, shape);
    result.set_orientation(orientation);
    result
}
fn compose_transition(
    result: TopOpeBRepDSTransition,
    orientation: Orientation,
    reference: bool,
) -> TopOpeBRepDSTransition {
    let reversed = (orientation == Orientation::Reversed) != !reference;
    if reversed {
        result.complement()
    } else {
        result
    }
}
fn transition_states(
    point: IntersectionPoint,
    index: usize,
    opposite: Option<bool>,
) -> Result<(State, State, bool), Failure> {
    let transition = if index == 0 {
        point.transition_of_first()
    } else {
        point.transition_of_second()
    };
    match transition.transition_type() {
        TypeTrans::In => Ok((State::Out, State::In, false)),
        TypeTrans::Out => Ok((State::In, State::Out, false)),
        TypeTrans::Undecided => Err(Failure::new(
            "TopOpeBRepEdgesIntersector : TransitionType Undecided",
        )),
        TypeTrans::Touch => match transition.situation()? {
            Situation::Inside => Ok((State::In, State::In, false)),
            Situation::Outside => Ok((State::Out, State::Out, false)),
            Situation::Unknown => unknown_touch(point, index, opposite),
        },
    }
}
fn unknown_touch(
    point: IntersectionPoint,
    index: usize,
    opposite: Option<bool>,
) -> Result<(State, State, bool), Failure> {
    let transitions = [point.transition_of_first(), point.transition_of_second()];
    let position = transitions[index].position_on_curve();
    let Some(opposite) = opposite else {
        return extremity_states(position);
    };
    let other = transitions[1 - index].position_on_curve();
    if other == Position::Middle {
        if position == Position::Middle {
            return Err(Failure::new(
                "TopOpeBRepEdgesIntersector : Situation Unknown MM",
            ));
        }
        return Ok((State::In, State::In, false));
    }
    let outgoing = (other == Position::Head) == opposite;
    Ok(if outgoing {
        (State::In, State::Out, false)
    } else {
        (State::Out, State::In, false)
    })
}
fn extremity_states(position: Position) -> Result<(State, State, bool), Failure> {
    match position {
        Position::Head => Ok((State::In, State::Out, true)),
        Position::End => Ok((State::Out, State::In, true)),
        Position::Middle => Err(Failure::new(
            "TopOpeBRepEdgesIntersector : Situation Unknown M",
        )),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkgeomalgo::int_res2d::Transition;
    use martensite_tkmath::gp::Point2;
    fn point(first: Transition, second: Transition) -> IntersectionPoint {
        IntersectionPoint::new(Point2::default(), 0., 0., first, second, false)
    }
    #[test]
    fn transition_table_covers_crossings_touching_extremities_and_failures() -> Result<(), Failure>
    {
        let middle = Position::Middle;
        for (kind, before, after) in [
            (TypeTrans::In, State::Out, State::In),
            (TypeTrans::Out, State::In, State::Out),
        ] {
            let t = Transition::crossing(false, middle, kind);
            assert_eq!(
                transition_states(point(t, t), 0, None)?,
                (before, after, false)
            );
        }
        for (situation, state) in [
            (Situation::Inside, State::In),
            (Situation::Outside, State::Out),
        ] {
            let t = Transition::touch(true, middle, situation, false);
            assert_eq!(
                transition_states(point(t, t), 0, None)?,
                (state, state, false)
            );
        }
        let undecided = Transition::default();
        assert!(transition_states(point(undecided, undecided), 0, None).is_err());
        let unknown = Transition::touch(true, middle, Situation::Unknown, false);
        assert!(transition_states(point(unknown, unknown), 0, None).is_err());
        assert!(transition_states(point(unknown, unknown), 0, Some(false)).is_err());
        Ok(())
    }
    #[test]
    fn unknown_touch_resolves_segment_orientation_and_endpoint_position() -> Result<(), Failure> {
        let t = |p| Transition::touch(true, p, Situation::Unknown, false);
        for opposite in [false, true] {
            let first = transition_states(
                point(t(Position::Head), t(Position::Head)),
                0,
                Some(opposite),
            )?;
            assert_eq!(
                first,
                if opposite {
                    (State::In, State::Out, false)
                } else {
                    (State::Out, State::In, false)
                }
            );
            let end =
                transition_states(point(t(Position::End), t(Position::End)), 0, Some(opposite))?;
            assert_eq!(
                end,
                if opposite {
                    (State::Out, State::In, false)
                } else {
                    (State::In, State::Out, false)
                }
            );
        }
        assert_eq!(
            transition_states(
                point(t(Position::Head), t(Position::Middle)),
                0,
                Some(false)
            )?,
            (State::In, State::In, false)
        );
        assert_eq!(
            transition_states(point(t(Position::Head), t(Position::Head)), 0, None)?,
            (State::In, State::Out, true)
        );
        assert_eq!(
            transition_states(point(t(Position::End), t(Position::End)), 0, None)?,
            (State::Out, State::In, true)
        );
        Ok(())
    }
}
