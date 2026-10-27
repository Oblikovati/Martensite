//! Formatting must release the field lease before a writer can reenter (#327).
use martensite_tkservice::graphic3d::{CaptureBufferKindRef, Graphic3dBufferType};
use std::fmt::{self, Write};
use std::sync::mpsc;
use std::time::Duration;

struct RebindingKindFormatter {
    field: CaptureBufferKindRef,
    text: String,
}
impl fmt::Write for RebindingKindFormatter {
    fn write_str(&mut self, chunk: &str) -> fmt::Result {
        self.field.set(Graphic3dBufferType::Red);
        self.text.push_str(chunk);
        Ok(())
    }
}
fn render_reentrant_debug() -> (String, Graphic3dBufferType) {
    let field = CaptureBufferKindRef::new(Graphic3dBufferType::Rgb);
    let mut output = RebindingKindFormatter {
        field: field.clone(),
        text: String::new(),
    };
    write!(&mut output, "{field:?}").unwrap();
    (output.text, field.read())
}
#[test]
fn debug_writer_rebinds_same_slot_without_a_held_read_guard() {
    let (sender, receiver) = mpsc::channel();
    // A regression fails with a bounded timeout rather than wedging the test runner.
    std::thread::spawn(move || sender.send(render_reentrant_debug()).unwrap());
    let completed = receiver.recv_timeout(Duration::from_secs(2));
    let (text, final_kind) = completed.expect("formatter actual=no completion within 2s; expected reentrant field write and Debug completion");
    assert_eq!(text, "CaptureBufferKindRef { kind: Rgb }");
    assert_eq!(final_kind, Graphic3dBufferType::Red);
}
