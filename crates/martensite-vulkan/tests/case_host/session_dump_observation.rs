//! Operation-local dump ordinals preserve every repeated diagnostic snapshot.
use super::*;
impl SessionRun {
    pub(super) fn observe_dump(
        &mut self,
        view: &Graphic3dRenderView,
        record: &SubmittedDraws,
        basename: &str,
    ) {
        self.observed_dump_ordinal += 1;
        let error =
            capture_witness::observe_ordinal(view, record, basename, self.observed_dump_ordinal)
                .and_then(|()| angle_role_observer::observe(self.session(), basename))
                .err();
        if self.pending_witness_error.is_none() {
            self.pending_witness_error = error;
        }
    }
}
