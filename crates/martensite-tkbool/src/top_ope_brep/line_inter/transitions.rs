use super::*;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkgeomalgo::int_surf::{IntSurfSituation, IntSurfTypeTrans};

impl TopOpeBRepLineInter {
    /// Reads first-surface crossing type; `line.transition_on_s1()?`.
    pub fn transition_on_s1(&self) -> Result<IntSurfTypeTrans, Failure> {
        Ok(self.source_line()?.transition_on_s1())
    }
    /// Reads second-surface crossing type; `line.transition_on_s2()?`.
    pub fn transition_on_s2(&self) -> Result<IntSurfTypeTrans, Failure> {
        Ok(self.source_line()?.transition_on_s2())
    }
    /// Reads first-surface touching situation; `line.situation_s1()?`.
    pub fn situation_s1(&self) -> Result<IntSurfSituation, Failure> {
        self.source_line()?.situation_s1()
    }
    /// Reads second-surface touching situation; `line.situation_s2()?`.
    pub fn situation_s2(&self) -> Result<IntSurfSituation, Failure> {
        self.source_line()?.situation_s2()
    }
    /// Applies the opposite face orientation to each crossing; `line.compute_face_face_transition()?`.
    pub fn compute_face_face_transition(&mut self) -> Result<(), Failure> {
        self.transitions[0] = self.process_face_transition(1, self.faces[1].orientation())?;
        self.transitions[1] = self.process_face_transition(2, self.faces[0].orientation())?;
        Ok(())
    }
    /// Reads the transition for a one-based face index; `line.face_face_transition(1)?`.
    pub fn face_face_transition(&self, index: i32) -> Result<&TopOpeBRepDSTransition, Failure> {
        self.transitions
            .get((index - 1) as usize)
            .ok_or_else(|| failure("FaceFaceTransition", index))
    }
    fn source_line(&self) -> Result<&dyn IntPatchLine, Failure> {
        self.source
            .get()
            .ok_or_else(|| failure("Transition", "null source"))
    }
    fn process_face_transition(
        &self,
        index: i32,
        face: Orientation,
    ) -> Result<TopOpeBRepDSTransition, Failure> {
        if matches!(face, Orientation::Internal | Orientation::External) {
            return Ok(TopOpeBRepDSTransition::from_orientation(face));
        }
        let transition = if index == 1 {
            self.transition_on_s1()?
        } else {
            self.transition_on_s2()?
        };
        let orientation = match transition {
            IntSurfTypeTrans::In => Some(Orientation::Forward),
            IntSurfTypeTrans::Out => Some(Orientation::Reversed),
            IntSurfTypeTrans::Touch => self.touch_orientation(index)?,
            IntSurfTypeTrans::Undecided => None,
        };
        Ok(oriented_transition(orientation, face))
    }
    fn touch_orientation(&self, index: i32) -> Result<Option<Orientation>, Failure> {
        let situation = if index == 1 {
            self.situation_s1()?
        } else {
            self.situation_s2()?
        };
        Ok(match situation {
            IntSurfSituation::Inside => Some(Orientation::Internal),
            IntSurfSituation::Outside => Some(Orientation::External),
            IntSurfSituation::Unknown => None,
        })
    }
}

fn oriented_transition(value: Option<Orientation>, face: Orientation) -> TopOpeBRepDSTransition {
    let Some(orientation) = value else {
        return TopOpeBRepDSTransition::default();
    };
    let oriented = if face == Orientation::Reversed {
        martensite_tkg3d::top_abs::complement(orientation)
    } else {
        orientation
    };
    TopOpeBRepDSTransition::from_orientation(oriented)
}
