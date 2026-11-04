//! External observations happen between minimal owned snapshots and reconciliation.
use super::*;
use crate::{gui_target_stamp::GuiTargetStamp, view_gui_layer::GuiLayerSlot};

struct RedrawObservationStamp {
    target: RedrawTargetStamp,
    epoch: Arc<()>,
    capture: GuiTargetStamp,
    serial: u64,
}

struct RedrawExternalObservation {
    logical: [u32; 2],
    checked: Option<[u32; 2]>,
}

impl Graphic3dRenderView {
    /// Retain this view and current target without pixels; `view.redraw_cursor()`.
    pub fn redraw_cursor(&self) -> Result<RenderRedrawCursor, RenderRedrawFailure> {
        let before = self.redraw_observation_stamp();
        let external = self.observe_redraw_scope(&before)?;
        // This outer owner scope outlives helper leases on typed error and unwind.
        let mut current = None;
        let result = self.validate_cursor_observation(&before, &external, &mut current);
        drop(current);
        result?;
        Ok(RenderRedrawCursor {
            owner: self.this.read().upgrade(),
            publication: before.serial,
            target_epoch: before.epoch,
            target: before.target,
            extent: external.logical,
        })
    }

    fn validate_cursor_observation(
        &self,
        before: &RedrawObservationStamp,
        external: &RedrawExternalObservation,
        current: &mut Option<RedrawTargetStamp>,
    ) -> Result<(), RenderRedrawFailure> {
        let gui = self.gui_layers.read();
        let _outcome = self.outcome.read();
        let current = current.insert(self.redraw_target_stamp());
        self.reconcile_redraw_observation(before, current, &gui, external.logical)
    }

    /// Verify exactly one callback publication; `view.check_redraw_since(&cursor)?`.
    pub fn check_redraw_since(
        &self,
        cursor: &RenderRedrawCursor,
    ) -> Result<RenderRedrawCursor, RenderRedrawFailure> {
        self.require_cursor_owner(cursor)?;
        let before = self.redraw_observation_stamp();
        let external = self.observe_redraw_scope(&before)?;
        // This outer owner scope outlives helper leases on typed error and unwind.
        let mut current = None;
        let result = self.validate_completed_observation(cursor, &before, &external, &mut current);
        drop(current);
        result?;
        Ok(RenderRedrawCursor {
            publication: before.serial,
            ..cursor.clone()
        })
    }

    fn validate_completed_observation(
        &self,
        cursor: &RenderRedrawCursor,
        before: &RedrawObservationStamp,
        external: &RedrawExternalObservation,
        current: &mut Option<RedrawTargetStamp>,
    ) -> Result<(), RenderRedrawFailure> {
        let gui = self.gui_layers.read();
        let outcome = self.outcome.read();
        let current = current.insert(self.redraw_target_stamp());
        self.reconcile_redraw_observation(before, current, &gui, external.logical)?;
        require_same_target(cursor, &before.target, &gui.epoch, external.logical)?;
        self.validate_observed_gui(&gui, &before.capture, external)
            .map_err(invalid_target)?;
        self.require_recorded_completion(cursor, outcome.as_ref(), before.serial)
    }

    fn redraw_observation_stamp(&self) -> RedrawObservationStamp {
        let gui = self.gui_layers.read();
        let _outcome = self.outcome.read();
        RedrawObservationStamp {
            target: RedrawTargetStamp::observe(self),
            epoch: gui.epoch.clone(),
            capture: gui.capture_stamp(),
            serial: self.redraw_serial.load(Ordering::Relaxed),
        }
    }

    fn observe_redraw_scope(
        &self,
        before: &RedrawObservationStamp,
    ) -> Result<RedrawExternalObservation, RenderRedrawFailure> {
        self.require_capture_target().map_err(invalid_target)?;
        let logical = capture_extent(self, self.limit).map_err(invalid_target)?;
        let checked = if self.checked_gui_policy() && before.capture.extent.is_some() {
            self.checked_observed_extent(before.capture.extent)
                .map_err(invalid_target)?
        } else {
            Some(logical)
        };
        self.require_capture_target().map_err(invalid_target)?;
        Ok(RedrawExternalObservation { logical, checked })
    }

    fn reconcile_redraw_observation(
        &self,
        before: &RedrawObservationStamp,
        current: &RedrawTargetStamp,
        gui: &GuiLayerSlot,
        extent: [u32; 2],
    ) -> Result<(), RenderRedrawFailure> {
        if !before.target.matches(current) || !Arc::ptr_eq(&before.epoch, &gui.epoch) {
            return Err(RenderRedrawFailure::TargetChanged {
                accepted: extent,
                observed: extent,
                binding_changed: true,
            });
        }
        let observed = self.redraw_serial.load(Ordering::Relaxed);
        require_observation_serial(before.serial, observed)
    }

    fn validate_observed_gui(
        &self,
        gui: &GuiLayerSlot,
        stamp: &GuiTargetStamp,
        observed: &RedrawExternalObservation,
    ) -> Result<(), RenderError> {
        if self.checked_gui_policy() && stamp.extent.is_some() {
            if let Some(change) =
                stamp.observe(gui, observed.checked, crate::RenderGuiSurfacePhase::Capture)
            {
                return Err(RenderError::surface_changed(change));
            }
            return Ok(());
        }
        gui.validate_extent(observed.logical)
    }

    fn require_published_target(
        &self,
        cursor: &RenderRedrawCursor,
        publication: u64,
    ) -> Result<(), RenderRedrawFailure> {
        let recorded = self.redraw_target.read();
        let target = recorded
            .as_ref()
            .ok_or(RenderRedrawFailure::MissingTargetStamp { publication })?;
        if !cursor.target.matches(&target.started) || !cursor.target.matches(&target.published) {
            return Err(RenderRedrawFailure::TargetChanged {
                accepted: cursor.extent,
                observed: cursor.extent,
                binding_changed: true,
            });
        }
        Ok(())
    }

    fn require_recorded_completion(
        &self,
        cursor: &RenderRedrawCursor,
        outcome: Option<&Result<FrameCapture, RenderError>>,
        publication: u64,
    ) -> Result<(), RenderRedrawFailure> {
        require_fresh_serial(cursor.publication, publication)?;
        require_completed_outcome(outcome, publication)?;
        self.require_published_target(cursor, publication)?;
        if let Some(Ok(capture)) = outcome {
            require_capture_dimensions(cursor, [capture.width, capture.height])?;
        }
        Ok(())
    }
}

fn require_observation_serial(before: u64, observed: u64) -> Result<(), RenderRedrawFailure> {
    if before != observed {
        return Err(RenderRedrawFailure::UnexpectedPublications { before, observed });
    }
    Ok(())
}

fn require_capture_dimensions(
    cursor: &RenderRedrawCursor,
    observed: [u32; 2],
) -> Result<(), RenderRedrawFailure> {
    if cursor.extent != observed {
        return Err(RenderRedrawFailure::TargetChanged {
            accepted: cursor.extent,
            observed,
            binding_changed: false,
        });
    }
    Ok(())
}
