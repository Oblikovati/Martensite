//! Actual fake-device submissions prove restored-frame sequencing without a GPU.
use super::*;
#[allow(dead_code)]
#[path = "../../../support/selection_frame_probe.rs"]
mod frame_probe;
use martensite_vulkan::{PrimitiveTopology, RenderFrame};
#[derive(Clone, Copy, Debug, PartialEq)]
enum OutputFault {
    None,
    Uncovered,
    Interference,
    Restored,
    Trace,
}
struct NamedRecordingFrameOutput<'a> {
    probe: &'a frame_probe::SelectionFrameProbe,
    group: Handle<dyn Graphic3dGroup>,
    old: Handle<dyn Graphic3dAspectsSource>,
    before: RenderFrame,
    events: Vec<&'static str>,
    fault: OutputFault,
}
impl<'a> NamedRecordingFrameOutput<'a> {
    fn new(
        probe: &'a frame_probe::SelectionFrameProbe,
        scale: &Handle<AisColorScale>,
        fault: OutputFault,
    ) -> Self {
        let role = role::FrameRole::resolve(&probe.session, scale).unwrap();
        Self {
            probe,
            group: role.group,
            old: role.aspect,
            before: probe.snapshot().0,
            events: Vec::new(),
            fault,
        }
    }
    fn require_case(&self) {
        assert_eq!(self.group.aspects(), self.old);
        let restored = self.probe.snapshot().0;
        assert_eq!(restored.draws(), self.before.draws());
        assert_eq!(
            format!("{:?}", restored.layers()),
            format!("{:?}", self.before.layers())
        );
        assert_eq!(restored.extent(), self.before.extent());
        assert_eq!(restored.clear_color(), self.before.clear_color());
    }
}
impl ScaleFrameCaptureOutput for NamedRecordingFrameOutput<'_> {
    fn write_uncovered(&mut self, _session: &ViewerSession) -> Outcome {
        self.events.push("uncovered");
        let frame = self.probe.snapshot().0;
        let expected: Vec<_> = self
            .before
            .draws()
            .iter()
            .filter(|d| d.topology() != PrimitiveTopology::Lines)
            .cloned()
            .collect();
        assert_eq!(frame.draws(), expected);
        if self.fault == OutputFault::Interference {
            self.group.aspects().set_line_width(9.)?;
        }
        if self.fault == OutputFault::Uncovered {
            return Err(refused("named uncovered capture failure").into());
        }
        Ok(())
    }
    fn write_restored(&mut self, _session: &ViewerSession) -> Outcome {
        self.require_case();
        self.events.push("restored");
        if self.fault == OutputFault::Restored {
            return Err(refused("named restored capture failure").into());
        }
        Ok(())
    }
    fn write_trace(&mut self, _trace: &trace::FrameTrace) -> Outcome {
        self.require_case();
        if self.fault == OutputFault::Trace {
            return Err(refused("named exclusive trace failure").into());
        }
        self.events.push("trace");
        Ok(())
    }
}
fn actual_default_probe() -> (frame_probe::SelectionFrameProbe, Handle<AisColorScale>) {
    let probe = frame_probe::SelectionFrameProbe::new();
    let scale = super::super::super::color_scale::prepare_scale(&probe.session).unwrap();
    probe
        .session
        .context()
        .display(&scale.cast(), true)
        .unwrap();
    (probe, scale)
}
#[test]
fn restored_actual_frame_precedes_positive_trace() {
    let (probe, scale) = actual_default_probe();
    let mut output = NamedRecordingFrameOutput::new(&probe, &scale, OutputFault::None);
    capture_with_output(&probe.session, &scale, Some(OsStr::new("1")), &mut output).unwrap();
    assert_eq!(output.events, ["uncovered", "restored", "trace"]);
    output.require_case();
}
#[test]
fn every_output_failure_refuses_trace_after_exact_role_restoration() {
    for fault in [
        OutputFault::Uncovered,
        OutputFault::Interference,
        OutputFault::Restored,
        OutputFault::Trace,
    ] {
        let (probe, scale) = actual_default_probe();
        let mut output = NamedRecordingFrameOutput::new(&probe, &scale, fault);
        assert!(
            capture_with_output(&probe.session, &scale, Some(OsStr::new("1")), &mut output)
                .is_err(),
            "fault {fault:?}"
        );
        assert!(!output.events.contains(&"trace"), "fault {fault:?}");
        output.require_case();
    }
}
#[test]
fn absent_request_performs_no_optional_capture_or_trace() {
    let (probe, scale) = actual_default_probe();
    let mut output = NamedRecordingFrameOutput::new(&probe, &scale, OutputFault::None);
    capture_with_output(&probe.session, &scale, None, &mut output).unwrap();
    assert!(output.events.is_empty());
    output.require_case();
}
