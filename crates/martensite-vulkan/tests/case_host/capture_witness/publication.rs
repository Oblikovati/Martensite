//! Closed dump calls and accepted publication cursors join copied buffers without pixel identity.
use martensite_vulkan::{FrameCapture, Graphic3dRenderView};
use serde_json::{Value, json};
use std::sync::Arc;

pub(crate) struct DumpScope {
    token: Arc<()>,
    basename: &'static str,
}
impl DumpScope {
    pub(crate) fn begin(view: &Graphic3dRenderView, basename: &'static str) -> Self {
        let scope = Self {
            token: Arc::new(()),
            basename,
        };
        scope.emit("begin", view, None);
        scope
    }
    pub(crate) fn finish(self, view: &Graphic3dRenderView, succeeded: bool) {
        self.emit("end", view, Some(succeeded));
    }
    fn emit(&self, phase: &str, view: &Graphic3dRenderView, succeeded: Option<bool>) {
        eprintln!(
            "{}",
            scope_json(
                phase,
                Arc::as_ptr(&self.token) as usize,
                view as *const _ as usize,
                self.basename,
                cursor(view),
                succeeded
            )
        );
    }
}
fn scope_json(
    phase: &str,
    token: usize,
    view: usize,
    basename: &str,
    cursor: Value,
    succeeded: Option<bool>,
) -> Value {
    json!({"kind":"original-dump-call", "phase":phase, "token":token, "view":view,
        "basename":basename, "cursor":cursor, "succeeded":succeeded})
}
pub(crate) fn cursor(view: &Graphic3dRenderView) -> Value {
    match view.redraw_cursor() {
        Ok(cursor) => json!({"valid":true, "publication":cursor.publication()}),
        Err(error) => json!({"valid":false, "failure":format!("{error:?}")}),
    }
}
pub(crate) fn snapshot(
    view: &Graphic3dRenderView,
    basename: &str,
    capture: &FrameCapture,
    before: Value,
) {
    eprintln!(
        "{}",
        snapshot_json(
            view as *const _ as usize,
            basename,
            capture,
            before,
            cursor(view)
        )
    );
}
fn snapshot_json(
    view: usize,
    basename: &str,
    capture: &FrameCapture,
    before: Value,
    after: Value,
) -> Value {
    json!({"kind":"original-witness-owned-snapshot", "view":view, "basename":basename,
        "cursor_before":before, "cursor_after":after, "extent":[capture.width,capture.height],
        "rgba_owner":capture.rgba.as_ptr() as usize, "depth_owner":capture.depth.as_ptr() as usize,
        "rgba_bytes":capture.rgba.len(), "depth_values":capture.depth.len()})
}
#[cfg(test)]
support_unit_tests! {
    #[path = "publication_tests.rs"]
    mod tests;
}
