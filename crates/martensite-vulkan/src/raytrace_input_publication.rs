//! Bind one actual completed input observation to the view's accepted image publication.
use super::*;
#[derive(Debug)]
struct Candidate {
    token: Arc<RayInputObservation>,
    capture: FrameCapture,
}
#[derive(Debug)]
struct Accepted {
    candidate: Candidate,
    publication: u64,
}
#[derive(Debug, Default)]
pub(crate) struct ViewRayInputs {
    generation: u64,
    active: usize,
    ambiguous: bool,
    pending: Option<Candidate>,
    accepted: Option<Accepted>,
    dump: Option<(u64, u64)>,
    dump_ordinal: u64,
}
impl ViewRayInputs {
    pub fn begin(
        &mut self,
        owner: usize,
        view_id: i64,
    ) -> Result<Arc<RayInputObservation>, RenderError> {
        self.accepted = None;
        self.dump = None;
        self.pending = None;
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or_else(|| rejected(self.generation, "unexhausted diagnostic frame sequence"))?;
        self.active = self.active.saturating_add(1);
        self.ambiguous |= self.active != 1;
        Ok(RayInputObservation::new(owner, view_id, self.generation))
    }
    pub fn finished(&mut self, token: &Arc<RayInputObservation>, capture: &FrameCapture) {
        if token.generation != self.generation
            || self.pending.is_some()
            || !bounded_capture(capture)
        {
            self.ambiguous = true;
            return;
        }
        self.pending = Some(Candidate {
            token: token.clone(),
            capture: capture.clone(),
        });
    }
    pub fn publish(&mut self, outcome: &Result<FrameCapture, RenderError>, serial: u64) {
        self.accepted = None;
        self.dump = None;
        if self.active == 1 && !self.ambiguous && serial != u64::MAX {
            self.accept_pending(outcome, serial);
        }
        self.active = self.active.saturating_sub(1);
        self.pending = None;
    }
    fn accept_pending(&mut self, outcome: &Result<FrameCapture, RenderError>, publication: u64) {
        let Some(candidate) = self.pending.take() else {
            return;
        };
        if outcome
            .as_ref()
            .is_ok_and(|capture| same_capture(capture, &candidate.capture))
        {
            self.accepted = Some(Accepted {
                candidate,
                publication,
            });
        }
    }
    pub fn witness(
        &self,
        owner: usize,
        serial: u64,
        capture: &FrameCapture,
    ) -> Result<String, RenderError> {
        let accepted = self.accepted_for(owner, serial, capture)?;
        let ordinal = self.observed_dump_ordinal(serial)?;
        let mut json = accepted.candidate.token.encode(serial)?;
        json.pop();
        Ok(format!(
            "{json},\"accepted_buffer_dump_ordinal\":{ordinal},\"buffer_dump_succeeded\":true}}"
        ))
    }
    fn accepted_for(
        &self,
        owner: usize,
        serial: u64,
        capture: &FrameCapture,
    ) -> Result<&Accepted, RenderError> {
        self.accepted
            .as_ref()
            .filter(|accepted| {
                accepted.publication == serial
                    && accepted.candidate.token.owner == owner
                    && same_capture(capture, &accepted.candidate.capture)
            })
            .ok_or_else(|| {
                rejected(
                    (owner, serial),
                    "the same uniquely accepted actual view, publication and image bytes",
                )
            })
    }
    fn observed_dump_ordinal(&self, serial: u64) -> Result<u64, RenderError> {
        self.dump
            .filter(|(dump_serial, _)| *dump_serial == serial)
            .map(|(_, ordinal)| ordinal)
            .ok_or_else(|| {
                rejected(
                    serial,
                    "successful same-publication color buffer_dump before witness",
                )
            })
    }
    pub fn dumped(&mut self, serial: u64, capture: &FrameCapture, copied_color: bool) {
        self.dump = None;
        let Some(accepted) = &self.accepted else {
            return;
        };
        if !copied_color
            || accepted.publication != serial
            || !same_capture(capture, &accepted.candidate.capture)
        {
            return;
        }
        let Some(ordinal) = self.dump_ordinal.checked_add(1) else {
            return;
        };
        self.dump_ordinal = ordinal;
        self.dump = Some((serial, ordinal));
    }
}
fn bounded_capture(capture: &FrameCapture) -> bool {
    let pixels = u64::from(capture.width) * u64::from(capture.height);
    pixels > 0
        && pixels <= 16 * 1024 * 1024 / 8
        && capture.rgba.len() as u64 == pixels * 4
        && capture.depth.len() as u64 == pixels
}
