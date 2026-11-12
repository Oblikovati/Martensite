use super::types::*;
pub(super) type CaptureResult<T> = Result<T, FontTextCaptureFailure>;
pub(super) struct CaptureBudget<'a> {
    pub limits: FontTextCaptureLimits,
    pub work: FontTextCaptureWork,
    pub cancelled: &'a mut dyn FnMut() -> bool,
}
impl<'a> CaptureBudget<'a> {
    pub fn new(limits: FontTextCaptureLimits, cancelled: &'a mut dyn FnMut() -> bool) -> Self {
        Self {
            limits,
            work: FontTextCaptureWork::default(),
            cancelled,
        }
    }
    pub fn after_call<T>(&mut self, outcome: CaptureResult<T>) -> CaptureResult<T> {
        self.checkpoint()?;
        outcome
    }
    pub fn checkpoint(&mut self) -> CaptureResult<()> {
        if (self.cancelled)() {
            return Err(FontTextCaptureFailure::Cancelled);
        }
        let count = self.work.checkpoints.saturating_add(1);
        self.check("checkpoints", count, self.limits.checkpoints)?;
        self.work.checkpoints = count;
        Ok(())
    }
    pub fn check(
        &self,
        resource: &'static str,
        observed: usize,
        maximum: usize,
    ) -> CaptureResult<()> {
        if observed > maximum {
            return Err(FontTextCaptureFailure::Limit {
                resource,
                observed,
                maximum,
            });
        }
        Ok(())
    }
    pub fn check_glyph_count(&self) -> CaptureResult<()> {
        self.check(
            "glyphs",
            self.work.glyphs.saturating_add(1),
            self.limits.glyphs,
        )
    }
    pub fn charge_raw(
        &mut self,
        segments: usize,
        contours: usize,
        points: usize,
    ) -> CaptureResult<()> {
        let w = &self.work;
        let counts = [
            w.glyphs.saturating_add(1),
            w.segments.saturating_add(segments),
            w.contours.saturating_add(contours),
            w.points.saturating_add(points),
        ];
        self.check_raw_counts(counts)?;
        self.work.glyphs = counts[0];
        self.work.segments = counts[1];
        self.work.contours = counts[2];
        self.work.points = counts[3];
        Ok(())
    }
    fn check_raw_counts(&self, counts: [usize; 4]) -> CaptureResult<()> {
        let names = ["glyphs", "segments", "contours", "points"];
        let limits = [
            self.limits.glyphs,
            self.limits.segments,
            self.limits.contours,
            self.limits.points,
        ];
        for ((name, count), limit) in names.into_iter().zip(counts).zip(limits) {
            self.check(name, count, limit)?;
        }
        Ok(())
    }

    pub fn charge_processed(&mut self, count: usize) -> CaptureResult<()> {
        let count = self.work.processed_entities.saturating_add(count);
        self.check("processed_entities", count, self.limits.processed_entities)?;
        self.work.processed_entities = count;
        Ok(())
    }
}
pub(super) fn native_failure(
    operation: &'static str,
    cause: martensite_tkernel::standard::Failure,
) -> FontTextCaptureFailure {
    FontTextCaptureFailure::Native {
        operation,
        cause: Box::new(cause),
    }
}
