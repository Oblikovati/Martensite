//! State-only elements retain their native position even when geometry is deferred.
use crate::PrimitiveDraw;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StencilCommand {
    Options(bool),
    CappingRestore,
    TextReset,
    /// Dimension text: clear the stencil buffer and write the label rectangle as 1.
    DimensionMask,
    /// Dimension text: the reference returns to 0 after the rectangle.
    DimensionMaskEnd,
}
impl StencilCommand {
    fn opaque_pass_only(self) -> bool {
        // Native ShouldRender excludes both non-fill elements; the capping sentry runs independently.
        matches!(
            self,
            Self::Options(_) | Self::TextReset | Self::DimensionMask | Self::DimensionMaskEnd
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct StencilBoundary {
    pub before_draw: usize,
    pub command: StencilCommand,
}
#[derive(Clone, Debug, Default)]
pub(crate) struct StencilProgram {
    pub draws: Vec<PrimitiveDraw>,
    pub boundaries: Vec<StencilBoundary>,
}
impl StencilProgram {
    pub fn append(&mut self, mut other: Self) {
        let offset = self.draws.len();
        for boundary in &mut other.boundaries {
            boundary.before_draw += offset;
        }
        self.geometry(other.draws);
        self.boundaries.extend(other.boundaries);
    }
    /// Append copies of `other`'s draws, each changed by `patch`, and its boundaries: a
    /// replayed record goes straight into the layer's program instead of through a program of
    /// its own (#594); `program.append_patched(&record.program, |draw| camera.patch(draw))`.
    pub fn append_patched(&mut self, other: &Self, mut patch: impl FnMut(&mut PrimitiveDraw)) {
        let offset = self.draws.len();
        self.draws.reserve(other.draws.len());
        for draw in &other.draws {
            self.draws.push(draw.clone());
            if let Some(added) = self.draws.last_mut() {
                patch(added);
            }
        }
        self.boundaries
            .extend(other.boundaries.iter().map(|boundary| StencilBoundary {
                before_draw: boundary.before_draw + offset,
                command: boundary.command,
            }));
    }
    /// The program's draw and boundary counts, to restore with `truncate`.
    pub fn mark(&self) -> (usize, usize) {
        (self.draws.len(), self.boundaries.len())
    }
    /// Drop what was appended after `mark`; `program.truncate(mark)`.
    pub fn truncate(&mut self, (draws, boundaries): (usize, usize)) {
        self.draws.truncate(draws);
        self.boundaries.truncate(boundaries);
    }
    pub fn command(&mut self, command: StencilCommand) {
        self.boundaries.push(StencilBoundary {
            before_draw: self.draws.len(),
            command,
        });
    }
    pub fn ordered_geometry(&mut self, draws: Vec<PrimitiveDraw>, text: bool) {
        // A text element disables testing once it has drawn initialized nonempty text.
        let reset = text && !draws.is_empty();
        self.text_geometry(draws);
        if reset {
            self.command(StencilCommand::TextReset);
        }
    }
    /// Text draws; a dimension label's stencil rectangle is bracketed by its stencil commands.
    fn text_geometry(&mut self, mut draws: Vec<PrimitiveDraw>) {
        let Some(rectangle) = draws.pop_if(|draw| draw.stencil_rect) else {
            return self.geometry(draws);
        };
        self.geometry(draws);
        self.command(StencilCommand::DimensionMask);
        self.geometry(vec![rectangle]);
        self.command(StencilCommand::DimensionMaskEnd);
    }
    pub fn geometry(&mut self, draws: Vec<PrimitiveDraw>) {
        // A group often contributes one already lowered draw buffer. Move it into the
        // empty program so its records are only copied when they have to be concatenated.
        if self.draws.capacity() == 0 {
            self.draws = draws;
        } else {
            self.draws.extend(draws);
        }
    }
}
pub(crate) fn filtered_boundaries(
    program: &[StencilBoundary],
    blended: &[bool],
    transparent: bool,
) -> Vec<StencilBoundary> {
    program
        .iter()
        .filter_map(|event| filtered_stencil_boundary(event, blended, transparent))
        .collect()
}
fn filtered_stencil_boundary(
    event: &StencilBoundary,
    blended: &[bool],
    transparent: bool,
) -> Option<StencilBoundary> {
    if transparent && event.command.opaque_pass_only() {
        return None;
    }
    Some(StencilBoundary {
        before_draw: blended[..event.before_draw]
            .iter()
            .filter(|&&deferred| deferred == transparent)
            .count(),
        command: event.command,
    })
}
pub(crate) fn implicit_cap_restores(draws: &[PrimitiveDraw]) -> Vec<StencilBoundary> {
    let mut boundaries = Vec::new();
    let mut active = false;
    for (index, draw) in draws.iter().enumerate() {
        let cap = draw.capping_pass() != crate::RenderCappingPass::None;
        if active && !cap {
            boundaries.push(cap_restore_boundary(index));
        }
        active = cap;
    }
    if active {
        boundaries.push(cap_restore_boundary(draws.len()));
    }
    boundaries
}
fn cap_restore_boundary(before_draw: usize) -> StencilBoundary {
    StencilBoundary {
        before_draw,
        command: StencilCommand::CappingRestore,
    }
}
#[cfg(test)]
#[path = "stencil_program_tests.rs"]
mod tests;
