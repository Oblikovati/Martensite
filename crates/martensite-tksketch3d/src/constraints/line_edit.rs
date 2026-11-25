// SPDX-License-Identifier: GPL-2.0-only
//! Prepare unaffected relations on an already checked line replacement.
use super::{ConstrainedSketch, poll};
use crate::line_edit::{
    ExtendLineRequest, LineEdit, LineEditFailure, LineEditOutcome, TrimLineRequest,
};

impl ConstrainedSketch {
    /// Trim with the existing authored relations and exact Fix anchors retained.
    /// Example `let edited = sketch.trim_line(request, || false)?;`.
    pub fn trim_line(
        &self,
        request: TrimLineRequest,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<LineEditOutcome<Self>, LineEditFailure> {
        let outcome = self.geometry.trim_line(request, &mut cancelled)?;
        self.prepare_line_edit(outcome, &mut cancelled)
    }

    /// Extend with the existing authored relations and exact Fix anchors retained.
    /// Example `let edited = sketch.extend_line(request, || false)?;`.
    pub fn extend_line(
        &self,
        request: ExtendLineRequest<'_>,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<LineEditOutcome<Self>, LineEditFailure> {
        let outcome = self.geometry.extend_line(request, &mut cancelled)?;
        self.prepare_line_edit(outcome, &mut cancelled)
    }

    fn prepare_line_edit(
        &self,
        outcome: LineEditOutcome,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<LineEditOutcome<Self>, LineEditFailure> {
        let edit = match outcome {
            LineEditOutcome::Unchanged(reason) => return Ok(LineEditOutcome::Unchanged(reason)),
            LineEditOutcome::Edited(edit) => *edit,
        };
        let mut relations = Vec::with_capacity(self.relations.len());
        for relation in self.relations() {
            poll(cancelled)?;
            relations.push(relation);
        }
        // Checked replay rejects retired operands before binding and never solves.
        let sketch = Self::restore(edit.sketch, &relations, self.policy, &mut *cancelled)?;
        self.verify_line_edit_rank(&sketch, cancelled)?;
        Ok(LineEditOutcome::Edited(Box::new(LineEdit {
            sketch,
            boundary: edit.boundary,
            boundary_location: edit.boundary_location,
            contact: edit.contact,
            requested_error: edit.requested_error,
            replacement: edit.replacement,
        })))
    }

    fn verify_line_edit_rank(
        &self,
        sketch: &Self,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), LineEditFailure> {
        poll(cancelled)?;
        let before = self.inspect_rank()?.rank;
        poll(cancelled)?;
        let after = sketch.inspect_rank()?.rank;
        poll(cancelled)?;
        if before != after {
            return Err(LineEditFailure::RankChanged { before, after });
        }
        Ok(())
    }
}
