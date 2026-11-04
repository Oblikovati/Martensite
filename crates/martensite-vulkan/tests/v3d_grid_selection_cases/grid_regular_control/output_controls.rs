//! Named fake output preserves real RecordingDevice submissions and injects each I/O failure.
use super::*;
#[derive(Clone, Copy, Debug, PartialEq)]
enum Failure {
    None,
    Before,
    Uncovered,
    Interference,
    Restored,
    Trace,
}
struct NamedGridOutput<'a> {
    probe: &'a probe::SelectionFrameProbe,
    role: role::GridRole,
    before: martensite_vulkan::RenderFrame,
    failure: Failure,
    events: Vec<String>,
}
impl<'a> NamedGridOutput<'a> {
    fn new(probe: &'a probe::SelectionFrameProbe, failure: Failure) -> Self {
        Self {
            probe,
            role: role::GridRole::resolve(&probe.session).unwrap(),
            before: probe.snapshot().0,
            failure,
            events: Vec::new(),
        }
    }
    fn require_case(&self) {
        assert_eq!(self.role.group.aspects(), self.role.aspect);
        assert_eq!(self.probe.snapshot().0.draws(), self.before.draws());
    }
    fn phase_failure(&self, suffix: &str) -> bool {
        matches!(
            (self.failure, suffix),
            (Failure::Before, "grid-regular-before")
                | (Failure::Uncovered, "uncovered")
                | (Failure::Restored, "grid-regular-restored")
        )
    }
}
impl GridCaptureOutput for NamedGridOutput<'_> {
    fn capture(&mut self, _session: &ViewerSession, suffix: &str) -> super::super::super::Outcome {
        self.events.push(suffix.to_owned());
        let current = self.probe.snapshot().0;
        if suffix != "uncovered" {
            assert_eq!(current.draws(), self.before.draws());
        } else {
            assert!(current.draws().len() < self.before.draws().len());
        }
        if self.failure == Failure::Interference && suffix == "uncovered" {
            self.role
                .group
                .set_group_primitives_aspect(&Handle::new(Graphic3dAspects::default()).cast());
        }
        if self.phase_failure(suffix) {
            return Err(refused(format!("named {suffix} output failure")).into());
        }
        Ok(())
    }
    fn trace(&mut self, value: &str) -> super::super::super::Outcome {
        if self.failure == Failure::Trace {
            return Err(refused("named stale trace output").into());
        }
        assert!(value.contains("\"role\":\"regular_only\""));
        assert!(value.contains("\"whole_underlayer\":false"));
        self.events.push("positive-trace".to_owned());
        Ok(())
    }
}
#[test]
fn actual_restored_capture_precedes_every_positive_trace() {
    let probe = fixture();
    let mut output = NamedGridOutput::new(&probe, Failure::None);
    capture_with_output(&probe.session, Some(std::ffi::OsStr::new("1")), &mut output).unwrap();
    assert_eq!(
        output.events,
        [
            "grid-regular-before",
            "uncovered",
            "grid-regular-restored",
            "positive-trace"
        ]
    );
    output.require_case();
}
#[test]
fn all_output_and_interference_failures_restore_then_refuse_positive_trace() {
    for fault in [
        Failure::Before,
        Failure::Uncovered,
        Failure::Interference,
        Failure::Restored,
        Failure::Trace,
    ] {
        let probe = fixture();
        let mut output = NamedGridOutput::new(&probe, fault);
        assert!(
            capture_with_output(&probe.session, Some(std::ffi::OsStr::new("1")), &mut output)
                .is_err(),
            "{fault:?}"
        );
        assert!(!output.events.iter().any(|event| event == "positive-trace"));
        output.require_case();
    }
}
#[test]
fn unset_diagnostic_flag_leaves_the_ordinary_capture_path_untouched() {
    let probe = fixture();
    let mut output = NamedGridOutput::new(&probe, Failure::None);
    capture_with_output(&probe.session, None, &mut output).unwrap();
    assert!(output.events.is_empty());
    output.require_case();
}

struct NamedTraceDirectory {
    path: std::path::PathBuf,
}
impl NamedTraceDirectory {
    fn new() -> Self {
        let path =
            std::env::temp_dir().join(format!("grid-regular-exclusive-{}", std::process::id()));
        std::fs::create_dir(&path).unwrap();
        Self { path }
    }
}
impl Drop for NamedTraceDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(self.path.join("trace.json"));
        let _ = std::fs::remove_dir(&self.path);
    }
}
#[test]
fn existing_trace_artifact_is_refused_without_overwriting_its_bytes() {
    let directory = NamedTraceDirectory::new();
    let path = directory.path.join("trace.json");
    write_trace_artifact(&path, "original trace").unwrap();
    let error = write_trace_artifact(&path, "replacement trace").unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
    assert_eq!(std::fs::read_to_string(path).unwrap(), "original trace");
}
