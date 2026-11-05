use super::*;
use crate::top_ope_brep_ds::TopOpeBRepDSTransition;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeomalgo::int_curve_surface::TransitionOnCurve;
use martensite_tkmath::gp::{Point2, Point3};

impl TopOpeBRepFaceEdgeIntersector {
    /// Reports no stored intersections; `assert!(inter.is_empty());`.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
    /// Returns the forward face (1) or edge (2); `let face = inter.shape(1)?;`.
    pub fn shape(&self, index: i32) -> Result<&Shape, Failure> {
        match index {
            1 => Ok(self.face.as_ref()),
            2 => Ok(self.edge.as_ref()),
            _ => Err(Failure::with_message(
                "Standard_ProgramError",
                &format!("TopOpeBRepFaceEdgeIntersector::Shape: index {index}; expected 1 or 2"),
            )),
        }
    }
    /// Returns the retained event count; `let count = inter.nb_points();`.
    pub fn nb_points(&self) -> i32 {
        self.points.len() as i32
    }
    /// Selects the first retained event; `inter.init_point();`.
    pub fn init_point(&mut self) {
        self.point_index = 1;
    }
    /// Tests the source traversal upper bound; `while inter.more_point() { inter.next_point(); }`.
    pub fn more_point(&self) -> bool {
        self.point_index <= self.nb_points()
    }
    /// Advances the source one-based cursor; `inter.next_point();`.
    pub fn next_point(&mut self) {
        self.point_index += 1;
    }
    /// Returns the current spatial point; `let point = inter.value()?;`.
    pub fn value(&self) -> Result<Point3, Failure> {
        Ok(self.current_point()?.0.pnt())
    }
    /// Returns its parameter on the edge; `let parameter = inter.parameter()?;`.
    pub fn parameter(&self) -> Result<f64, Failure> {
        Ok(self.current_point()?.0.w())
    }
    /// Returns its face parameters; `let uv = inter.uv_point()?;`.
    pub fn uv_point(&self) -> Result<Point2, Failure> {
        let point = &self.current_point()?.0;
        Ok(Point2::new(point.u(), point.v()))
    }
    /// Returns its IN/ON face classification; `let state = inter.state()?;`.
    pub fn state(&self) -> Result<State, Failure> {
        Ok(self.current_point()?.1)
    }
    /// Retains the release-build trace result; `assert_eq!(inter.index(), 0);`.
    pub fn index(&self) -> i32 {
        0
    }
    /// Returns the event's oriented transition; `let transition = inter.transition(2, Orientation::Forward)?;`.
    pub fn transition(
        &self,
        index: i32,
        orientation: Orientation,
    ) -> Result<TopOpeBRepDSTransition, Failure> {
        let point = &self.current_point()?.0;
        if index == 1 {
            return Ok(face_transition(State::In, State::In));
        }
        if index != 2 {
            return Err(Failure::with_message(
                "Standard_ProgramError",
                &format!("FEINT Transition Index: index {index}; expected 1 or 2"),
            ));
        }
        let states = crossing_states(point.transition());
        Ok(oriented_transition(states, orientation))
    }
}

fn face_transition(before: State, after: State) -> TopOpeBRepDSTransition {
    TopOpeBRepDSTransition::new(before, after, ShapeEnum::Face, ShapeEnum::Face)
}
fn oriented_transition(states: (State, State), orientation: Orientation) -> TopOpeBRepDSTransition {
    match orientation {
        Orientation::Forward => face_transition(states.0, states.1),
        Orientation::Reversed => face_transition(states.1, states.0),
        Orientation::Internal => face_transition(State::In, State::In),
        Orientation::External => face_transition(State::Out, State::Out),
    }
}

fn crossing_states(transition: TransitionOnCurve) -> (State, State) {
    match transition {
        TransitionOnCurve::In => (State::Out, State::In),
        TransitionOnCurve::Out => (State::In, State::Out),
        TransitionOnCurve::Tangent => (State::In, State::In),
    }
}
