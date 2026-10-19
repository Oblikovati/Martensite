//! Exclusive operation frames are taken before calling user code and restored on unwind.
use martensite_tkernel::standard::Failure;
use std::sync::Mutex;
pub(super) struct OperationFrameState<'frame, Frame> {
    pub(super) frame: Option<&'frame mut Frame>,
    pub(super) failure: Option<Failure>,
}
pub(super) struct OperationFrameLease<'state, 'frame, Frame> {
    state: &'state Mutex<OperationFrameState<'frame, Frame>>,
    frame: Option<&'frame mut Frame>,
}
impl<'state, 'frame, Frame> OperationFrameLease<'state, 'frame, Frame> {
    pub(super) fn take(
        state: &'state Mutex<OperationFrameState<'frame, Frame>>,
        poisoned: impl FnOnce() -> Failure,
        active: impl FnOnce() -> Failure,
    ) -> Result<Self, Failure> {
        let frame = state
            .lock()
            .map_err(|_| poisoned())?
            .frame
            .take()
            .ok_or_else(active)?;
        Ok(Self {
            state,
            frame: Some(frame),
        })
    }
    pub(super) fn frame_mut(&mut self, expected: &'static str) -> &mut Frame {
        self.frame.as_deref_mut().expect(expected)
    }
}
impl<Frame> Drop for OperationFrameLease<'_, '_, Frame> {
    fn drop(&mut self) {
        // Restoration also runs while evaluator unwinding; user code holds no guard.
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.frame = self.frame.take();
    }
}
