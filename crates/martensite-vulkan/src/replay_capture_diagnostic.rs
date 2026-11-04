//! Invocation-local owned-buffer transfers bind readback, publication and actual image copies.
use crate::{FrameCapture, RenderCoordinateConvention, RenderError};
use serde_json::{Value, json};

pub(crate) struct ReadbackArguments {
    pub color: [u64; 3],
    pub depth: [u64; 3],
    pub formats: [i32; 2],
    pub samples: [u32; 2],
    pub attachment_extent: [u32; 2],
    pub leading_rows: u32,
    pub coordinates: RenderCoordinateConvention,
}
pub(crate) fn readback(capture: &FrameCapture, arguments: ReadbackArguments) {
    eprintln!("{}", readback_json(capture, arguments));
}
fn readback_json(capture: &FrameCapture, arguments: ReadbackArguments) -> Value {
    json!({"kind":"frame-readback-owned", "capture":capture_json(capture),
        "color_image_view_buffer":arguments.color, "depth_image_view_buffer":arguments.depth,
        "formats":arguments.formats, "samples":arguments.samples,
        "attachment_extent":arguments.attachment_extent, "leading_rows":arguments.leading_rows,
        "coordinates":format!("{:?}",arguments.coordinates), "rgba":capture.rgba,
        "depth_bits":capture.depth.iter().map(|value|value.to_bits()).collect::<Vec<_>>(),
        "submission_completion_join_required":true})
}
pub(crate) fn rendered(view: usize, capture: &FrameCapture, transforms: [usize; 4]) {
    eprintln!("{}", rendered_json(view, capture, transforms));
}
fn rendered_json(view: usize, capture: &FrameCapture, transforms: [usize; 4]) -> Value {
    json!({"kind":"view-rendered-owned", "view":view,
        "capture":capture_json(capture), "scaled_subviews_flip_gui":transforms})
}
pub(crate) fn published(view: usize, serial: u64, outcome: &Result<FrameCapture, RenderError>) {
    eprintln!("{}", publication_json(view, serial, outcome));
}
fn publication_json(
    view: usize,
    serial: u64,
    outcome: &Result<FrameCapture, RenderError>,
) -> Value {
    json!({"kind":"view-published-owned", "view":view, "publication":serial,
        "capture":outcome.as_ref().ok().map(capture_json),
        "publication_refusal":outcome.as_ref().err().map(ToString::to_string)})
}
pub(crate) fn cloned(
    view: usize,
    serial: u64,
    source: &Option<Result<FrameCapture, RenderError>>,
    output: &Option<Result<FrameCapture, RenderError>>,
) {
    eprintln!("{}", clone_json(view, serial, source, output));
}
fn clone_json(
    view: usize,
    serial: u64,
    source: &Option<Result<FrameCapture, RenderError>>,
    output: &Option<Result<FrameCapture, RenderError>>,
) -> Value {
    json!({"kind":"view-capture-owned-clone", "view":view, "publication":serial,
        "source":source.as_ref().and_then(|value|value.as_ref().ok()).map(capture_json),
        "output":output.as_ref().and_then(|value|value.as_ref().ok()).map(capture_json)})
}
pub(crate) fn dumped(
    view: usize,
    serials: [u64; 2],
    kind: &str,
    copied: bool,
    capture: &FrameCapture,
) {
    eprintln!("{}", dump_json(view, serials, kind, copied, capture));
}
fn dump_json(
    view: usize,
    serials: [u64; 2],
    kind: &str,
    copied: bool,
    capture: &FrameCapture,
) -> Value {
    json!({"kind":"view-buffer-dump-owned", "view":view, "publications_before_after":serials,
        "buffer_kind":kind, "copied":copied, "capture":capture_json(capture)})
}
fn capture_json(capture: &FrameCapture) -> Value {
    json!({"extent":[capture.width,capture.height], "rgba_owner":capture.rgba.as_ptr() as usize,
        "rgba_bytes":capture.rgba.len(), "depth_owner":capture.depth.as_ptr() as usize,
        "depth_values":capture.depth.len()})
}

#[cfg(test)]
#[path = "replay_capture_diagnostic_tests.rs"]
mod tests;
