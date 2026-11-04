//! Required GPU witnesses use real submissions and synthetic observation errors, never real driver OOM.
use super::{
    VulkanRenderer, record,
    resource_lifecycle_test_support::{FaultingCloseCalls, inject_completion_failure},
    retained_submission::RetainedOwnerSnapshot,
    submission::CommandSubmission,
};
use crate::{
    PrimitiveDraw, RenderBackend, RenderDeviceCloseOutcome, RenderDeviceState, RenderFrame,
    RenderVertex, identity_clip,
};
use ash::vk::Handle;
fn resource_witness_renderer() -> VulkanRenderer {
    // The canonical runner selects a profile; missing hardware/layer is a failure.
    super::resource_lifecycle_test_support::create_resource_renderer()
}
fn solid_resource_frame() -> RenderFrame {
    let points = [[-1., -1., 0.], [1., -1., 0.], [1., 1., 0.], [-1., 1., 0.]];
    let vertices = points
        .map(|point| RenderVertex::new(point, [1., 0., 0., 1.]))
        .to_vec();
    let draw = PrimitiveDraw::new(vertices, vec![0, 1, 2, 0, 2, 3], identity_clip()).unwrap();
    RenderFrame::new(2, 2, [0., 0., 1., 1.], vec![draw]).unwrap()
}
fn inject_actual_frame(
    renderer: &mut VulkanRenderer,
    frame: &RenderFrame,
) -> RetainedOwnerSnapshot {
    let resources = renderer.frame_resources(frame).unwrap();
    let commands = CommandSubmission::new(&renderer.runtime).unwrap();
    let expected = frame_owner_identity(&commands, &resources);
    record::record_frame(&renderer.runtime, commands.command, &resources, frame);
    let outcome = inject_completion_failure(&commands, &[], &[]);
    assert!(
        renderer
            .settle_owned_frame(commands, resources, outcome)
            .is_err()
    );
    expected
}
fn frame_owner_identity(
    commands: &CommandSubmission,
    resources: &super::frame::FrameResources,
) -> RetainedOwnerSnapshot {
    RetainedOwnerSnapshot::Frame {
        command: commands.command.as_raw(),
        color: resources.color.handle.as_raw(),
        readback: resources.color_read.handle.as_raw(),
    }
}
struct PendingFrameWitness {
    renderer: VulkanRenderer,
    monitor: std::sync::Arc<super::validation::ValidationMessages>,
    expected: RetainedOwnerSnapshot,
    capacity: usize,
}
impl PendingFrameWitness {
    fn new() -> Self {
        let mut renderer = resource_witness_renderer();
        let monitor = renderer.runtime.validation.clone();
        let capacity = renderer.retirements.reserved_slots();
        let expected = inject_actual_frame(&mut renderer, &solid_resource_frame());
        Self {
            renderer,
            monitor,
            expected,
            capacity,
        }
    }
    fn assert_incomplete(&mut self) {
        self.assert_retained_identity();
        let mut failure = FaultingCloseCalls { calls: 0 };
        assert!(matches!(
            self.renderer.close_dispatched(&mut failure),
            RenderDeviceCloseOutcome::Incomplete { .. }
        ));
        assert_eq!(failure.calls, 1);
        self.assert_retained_identity();
    }
    fn assert_retained_identity(&self) {
        assert_eq!(
            self.renderer.retirements.snapshot().as_slice(),
            std::slice::from_ref(&self.expected)
        );
        assert_eq!(
            self.renderer.device_state(),
            RenderDeviceState::RetirementIncomplete
        );
        assert_eq!(self.renderer.retirements.reserved_slots(), self.capacity);
    }
    fn close_actual(mut self) {
        assert_eq!(
            self.renderer.close(),
            RenderDeviceCloseOutcome::Closed { failure: None }
        );
        assert!(self.renderer.retirements.snapshot().is_empty());
        drop(self.renderer);
        assert!(self.monitor.is_observable());
        assert!(
            self.monitor.errors().unwrap().is_empty(),
            "{:?}",
            self.monitor.errors().unwrap()
        );
        assert_eq!(std::sync::Arc::strong_count(&self.monitor), 1);
    }
}
#[test]
fn actual_submitted_frame_survives_incomplete_close_until_real_idle() {
    let mut witness = PendingFrameWitness::new();
    witness.assert_incomplete();
    witness.close_actual();
}
#[test]
fn closed_renderer_rejects_work_and_repeated_close_preserves_owned_capture() {
    let mut renderer = resource_witness_renderer();
    let monitor = renderer.runtime.validation.clone();
    let frame = solid_resource_frame();
    let capture = renderer.render(&frame).unwrap();
    assert_eq!(capture.rgba, [255, 0, 0, 255].repeat(4));
    assert_eq!(capture.depth, vec![renderer.stored_depth(0.5); 4]);
    println!(
        "{{\"event\":\"owned-resource-capture\",\"rgba\":{:?},\"depth\":{:?}}}",
        capture.rgba, capture.depth
    );
    assert_closed_capture(&mut renderer, &frame, &capture);
    drop(renderer);
    assert!(monitor.is_observable());
    assert!(
        monitor.errors().unwrap().is_empty(),
        "{:?}",
        monitor.errors().unwrap()
    );
}
fn assert_closed_capture(
    renderer: &mut VulkanRenderer,
    frame: &RenderFrame,
    capture: &crate::FrameCapture,
) {
    assert_eq!(
        renderer.close(),
        RenderDeviceCloseOutcome::Closed { failure: None }
    );
    assert_eq!(
        renderer.close(),
        RenderDeviceCloseOutcome::Closed { failure: None }
    );
    assert!(renderer.render(frame).is_err());
    assert_eq!(capture.rgba, [255, 0, 0, 255].repeat(4));
}
