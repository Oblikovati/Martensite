//! Typed lookups require one actual slot, never a guessed first matching entity.
use super::super::budget::*;
use martensite_tkbrep::brep::face_input_capture::{
    CapturedFaceCell as Cell, CapturedOccurrence, FaceInputSnapshot,
};
use martensite_tkg3d::{geom::StoredSplineInput, top_abs::ShapeEnum};
pub(super) struct Cells<'a> {
    pub input: &'a FaceInputSnapshot,
}
impl<'a> Cells<'a> {
    pub fn one(
        &self,
        owner: usize,
        accept: fn(&Cell) -> bool,
        expected: &'static str,
    ) -> CheckResult<&'a Cell> {
        let mut found = self
            .input
            .cells()
            .iter()
            .filter(|(id, cell)| *id == owner && accept(cell));
        let first = found
            .next()
            .ok_or_else(|| unsupported(owner, "input slot", "absent", expected))?;
        if found.next().is_some() {
            return Err(invalid(owner, "unique input slot", expected));
        }
        Ok(&first.1)
    }
    pub fn children(&self, owner: usize, kind: ShapeEnum) -> CheckResult<&'a [CapturedOccurrence]> {
        let Cell::Topology { state, children } =
            self.one(owner, |c| matches!(c, Cell::Topology { .. }), "topology")?
        else {
            return Err(invalid(owner, "topology slot", "wrong type"));
        };
        if state & 0xF != kind as u16 {
            return Err(invalid(
                owner,
                "topology kind",
                format!("state={state}, expected={kind:?}"),
            ));
        }
        Ok(children)
    }
    pub fn spline(&self, owner: usize) -> CheckResult<&'a StoredSplineInput> {
        let Cell::Spline(input) =
            self.one(owner, |c| matches!(c, Cell::Spline(_)), "stored spline")?
        else {
            return Err(invalid(owner, "spline slot", "wrong type"));
        };
        Ok(input)
    }
    pub fn range(&self, owner: usize) -> CheckResult<[f64; 2]> {
        let Cell::Range(range) = self.one(owner, |c| matches!(c, Cell::Range(_)), "range")? else {
            return Err(invalid(owner, "range slot", "wrong type"));
        };
        Ok(*range)
    }
    pub fn location(
        &self,
        owner: usize,
    ) -> CheckResult<&'a martensite_tkbrep::brep::face_input_capture::CapturedLocation> {
        let Cell::Location(location) =
            self.one(owner, |c| matches!(c, Cell::Location(_)), "location")?
        else {
            return Err(invalid(owner, "location slot", "wrong type"));
        };
        Ok(location)
    }
}
