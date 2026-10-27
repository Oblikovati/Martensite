//! The explicit read/rewind boundary stresses lookahead without another decoder.
use martensite_tkbrep::{brep::BRepBuilder, brep_tools, topods::Shape};
use martensite_tkernel::{message::ProgressRange, standard::Failure, storage::IoStream};
pub(super) struct Decoded {
    pub(super) shape: Shape,
    pub(super) result: Option<Result<(), Failure>>,
}
/// Invoke the existing archive reader on the borrowed stream; `decode(stream)`.
pub(super) fn decode(stream: &mut IoStream) -> Decoded {
    let mut shape = Shape::default();
    let reader_called = prepare(stream);
    let result = if reader_called {
        Some(brep_tools::read(
            &mut shape,
            stream,
            &BRepBuilder::default(),
            &ProgressRange::default(),
        ))
    } else {
        None
    };
    Decoded { shape, result }
}
fn prepare(stream: &mut IoStream) -> bool {
    if stream.fail() {
        return false;
    }
    stream.clear(0);
    stream.seekg(0);
    if stream.fail() {
        return false;
    }
    stream.read(&mut [0; 19]);
    stream.clear(0);
    stream.seekg(0);
    !stream.fail()
}
/// Record every actual state and decoder outcome before asserting; `report(name, before, actual, &decoded)`.
pub(super) fn report(
    name: &str,
    before: u8,
    actual: (u8, usize, i64, bool, bool),
    decoded: &Decoded,
) {
    let (after, count, position, has_shape, rejected) = actual;
    println!(
        "{{\"archive_read\":\"{name}\",\"before\":{before},\"after\":{after},\"count\":{count},\"position\":{position},\"has_shape\":{has_shape},\"rejected\":{rejected},\"reader_called\":{}}}",
        decoded.result.is_some()
    );
    if let Some(Err(error)) = &decoded.result {
        report_error(error);
    }
}
fn report_error(error: &Failure) {
    let class = hex(error.exception_type().as_bytes());
    let message = hex(error.message().as_bytes());
    let stack = hex(error.stack_string().as_bytes());
    println!(
        "{{\"archive_error\":true,\"class_hex\":\"{class}\",\"message_hex\":\"{message}\",\"stack_hex\":\"{stack}\"}}"
    );
}
fn hex(bytes: &[u8]) -> String {
    let encoded: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
    encoded
}
