//! Positive restoration traces are exclusive; an old artifact is a refusal.
use super::*;
use std::{io::Write, path::Path};
#[derive(Debug)]
pub(super) struct FrameTrace {
    object: usize,
    presentation: usize,
    group: usize,
    old: usize,
    replacement: usize,
}
impl FrameTrace {
    pub(super) fn actual(
        role: &role::FrameRole,
        replacement: &Handle<dyn Graphic3dAspectsSource>,
    ) -> Self {
        Self {
            object: role.scale.object_address() as usize,
            presentation: role.presentation.object_address() as usize,
            group: role.group.object_address() as usize,
            old: role.aspect.object_address() as usize,
            replacement: replacement.object_address() as usize,
        }
    }
    fn write_to(&self, writer: &mut dyn Write) -> io::Result<()> {
        writeln!(
            writer,
            "{{\"role\":\"actual-operation-scale-frame\",\"object\":{},\"presentation\":{},\"group\":{},\"old_aspect\":{},\"replacement_aspect\":{},\"restored\":true,\"invariants_verified\":true}}",
            self.object, self.presentation, self.group, self.old, self.replacement
        )
    }
    pub(super) fn write_exclusive(&self, path: &Path) -> io::Result<()> {
        let mut output = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
        self.write_to(&mut output)
    }
}
#[cfg(test)]
mod controls {
    use super::*;
    #[derive(Default)]
    struct NamedTraceWriter {
        bytes: Vec<u8>,
    }
    impl Write for NamedTraceWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    fn fixture_trace() -> FrameTrace {
        FrameTrace {
            object: 1,
            presentation: 2,
            group: 3,
            old: 4,
            replacement: 5,
        }
    }
    #[test]
    fn owned_trace_emits_actual_fields() {
        let mut writer = NamedTraceWriter::default();
        fixture_trace().write_to(&mut writer).unwrap();
        assert!(
            String::from_utf8(writer.bytes)
                .unwrap()
                .contains("\"restored\":true,\"invariants_verified\":true")
        );
    }
    #[test]
    fn owned_trace_refuses_existing_file() {
        let trace = fixture_trace();
        let path =
            std::env::temp_dir().join(format!("frame-trace-exclusive-{}.json", std::process::id()));
        trace.write_exclusive(&path).unwrap();
        assert_eq!(
            trace.write_exclusive(&path).unwrap_err().kind(),
            io::ErrorKind::AlreadyExists
        );
        std::fs::remove_file(path).unwrap();
    }
}
