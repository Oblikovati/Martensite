//! Point classification retains the operand mask and face-intersection tolerances.
use super::*;

impl<'a> TopOpeBRepFacesFiller<'a> {
    /// Binds one pair's topology, intersection context, and storage; `filler.bind_context(&f1, &f2, &mut pair, &hds)?`.
    pub fn bind_context(
        &mut self,
        first: &Shape,
        second: &Shape,
        pair: &'a mut TopOpeBRepFacesIntersector,
        hds: &Handle<TopOpeBRepDSHDataStructure>,
    ) -> Result<(), Failure> {
        self.bind_intersector(
            first,
            second,
            intersector_binding::IntersectorBinding::Borrowed(pair),
            hds,
        )
    }

    pub(super) fn bind_intersector(
        &mut self,
        first: &Shape,
        second: &Shape,
        pair: intersector_binding::IntersectorBinding<'a>,
        hds: &Handle<TopOpeBRepDSHDataStructure>,
    ) -> Result<(), Failure> {
        self.faces[0] = topods::face(first)?;
        self.faces[1] = topods::face(second)?;
        let (arc, tangent) = pair.get_tolerances();
        self.face_tolerance = if arc < tangent { tangent } else { arc };
        self.intersector = Some(pair);
        self.hds = hds.clone();
        hds.get().ok_or_else(missing_storage)?;
        self.ensure_shape_classifier();
        Ok(())
    }

    /// Returns operand face one or two; `filler.face(1)?`.
    pub fn face(&self, index: i32) -> Result<&Face, Failure> {
        match index {
            1 | 2 => Ok(&self.faces[(index - 1) as usize]),
            _ => Err(Failure::with_message(
                "Standard_ProgramError",
                "FacesFiller::Face",
            )),
        }
    }

    /// Borrows the retained mutable face intersection; `filler.change_faces_intersector()?.init_line()?`.
    pub fn change_faces_intersector(&mut self) -> Result<&mut TopOpeBRepFacesIntersector, Failure> {
        self.intersector
            .as_deref_mut()
            .ok_or_else(missing_intersector)
    }

    /// Returns the retained face-intersection context, if bound; `filler.p_faces_intersector_dummy()`.
    pub fn p_faces_intersector_dummy(&self) -> Option<&TopOpeBRepFacesIntersector> {
        self.intersector.as_deref()
    }

    /// Classifies only the operand sides selected by the VPoint's mask; `filler.vp_position(&mut point, &mut classifier)?`.
    pub fn vp_position(
        &mut self,
        point: &mut TopOpeBRepVPointInter<'_>,
        classifier: &mut TopOpeBRepVPointInterClassifier,
    ) -> Result<(), Failure> {
        let (first, second) = self
            .intersector
            .as_deref()
            .ok_or_else(missing_intersector)?
            .get_tolerances();
        let tolerance = if first > second { first } else { second };
        let assume = self
            .current_kind
            .is_some_and(|kind| kind != TopOpeBRepTypeLineCurve::Restriction);
        let mask = point.shape_index();
        for (index, classify) in [
            (1, matches!(mask, 0 | 2 | 3)),
            (2, matches!(mask, 0 | 1 | 3)),
        ] {
            if classify {
                classifier.vpoint_position(
                    self.faces[index - 1].as_ref(),
                    point,
                    index as i32,
                    &mut self.point_classifier,
                    assume,
                    tolerance,
                )?;
            }
        }
        Ok(())
    }
}

pub(super) fn missing_intersector() -> Failure {
    Failure::with_message(
        "Standard_NullObject",
        "FacesFiller: intersector=None; expected a bound face intersection",
    )
}
