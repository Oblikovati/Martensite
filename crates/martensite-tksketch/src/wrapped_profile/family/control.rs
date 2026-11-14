//! A latched cancellation refuses handoff and every later stage of the same operation.
use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in super::super) enum FamilyStage {
    Selection,
    Construction,
    Encoding,
    Coverage,
    History,
    Storage,
    Binding,
    MemberChecks,
    Completion,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in super::super) struct Checkpoint {
    pub stage: FamilyStage,
    pub region: Option<SurfaceRegionId>,
}
pub(super) struct Control<'a> {
    cancelled: &'a mut (dyn FnMut(Checkpoint) -> bool + Send),
    cancelled_at: Option<Checkpoint>,
}
impl<'a> Control<'a> {
    pub fn new(cancelled: &'a mut (dyn FnMut(Checkpoint) -> bool + Send)) -> Self {
        Self {
            cancelled,
            cancelled_at: None,
        }
    }
    pub fn run<T>(
        &mut self,
        stage: FamilyStage,
        region: Option<SurfaceRegionId>,
        operation: impl FnOnce(&mut CancelPoll<'_>) -> Checked<T>,
    ) -> Checked<T> {
        self.ensure_active()?;
        let point = Checkpoint { stage, region };
        let mut poll = || self.observe(point);
        check_cancellation(point, &mut poll)?;
        let result = operation(&mut poll);
        check_cancellation(point, &mut poll)?;
        result
    }
    fn ensure_active(&self) -> Checked<()> {
        match self.cancelled_at {
            Some(point) => Err(FamilyFailure::Cancelled(point)),
            None => Ok(()),
        }
    }
    fn observe(&mut self, point: Checkpoint) -> bool {
        if self.cancelled_at.is_some() {
            return true;
        }
        if (self.cancelled)(point) {
            self.cancelled_at = Some(point);
            return true;
        }
        false
    }
}

fn check_cancellation(point: Checkpoint, poll: &mut CancelPoll<'_>) -> Checked<()> {
    if poll() {
        return Err(FamilyFailure::Cancelled(point));
    }
    Ok(())
}
