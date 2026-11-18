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
    Budget,
    Completion,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in super::super) struct Checkpoint {
    pub stage: FamilyStage,
    pub region: Option<SurfaceRegionId>,
}
pub(in crate::surface_sketch::wrapped_profile) struct Control<'a> {
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
    #[cfg(test)]
    pub fn run<T>(
        &mut self,
        stage: FamilyStage,
        region: Option<SurfaceRegionId>,
        operation: impl FnOnce(&mut CancelPoll<'_>) -> Checked<T>,
    ) -> Checked<T> {
        self.run_for(stage, region, operation)
    }
    pub fn run_for<T, E: From<Checkpoint>>(
        &mut self,
        stage: FamilyStage,
        region: Option<SurfaceRegionId>,
        operation: impl FnOnce(&mut CancelPoll<'_>) -> Result<T, E>,
    ) -> Result<T, E> {
        self.ensure_active()?;
        let point = Checkpoint { stage, region };
        let mut poll = || self.observe(point);
        check_cancellation(point, &mut poll)?;
        let result = operation(&mut poll)?;
        check_cancellation(point, &mut poll)?;
        Ok(result)
    }
    fn ensure_active<E: From<Checkpoint>>(&self) -> Result<(), E> {
        match self.cancelled_at {
            Some(point) => Err(E::from(point)),
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

fn check_cancellation<E: From<Checkpoint>>(
    point: Checkpoint,
    poll: &mut CancelPoll<'_>,
) -> Result<(), E> {
    if poll() {
        return Err(E::from(point));
    }
    Ok(())
}

impl From<Checkpoint> for FamilyFailure {
    fn from(point: Checkpoint) -> Self {
        Self::Cancelled(point)
    }
}
