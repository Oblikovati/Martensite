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
mod tests {
    use super::*;
    #[test]
    fn closed_call_tokens_do_not_depend_on_image_values_or_basename_escaping() {
        let value = scope_json(
            "end",
            19,
            23,
            "a\"b.png",
            json!({"valid":false}),
            Some(false),
        );
        let encoded = value.to_string();
        assert_eq!(serde_json::from_str::<Value>(&encoded).unwrap(), value);
        assert_eq!(value["succeeded"], false);
        assert_eq!(value["cursor"]["valid"], false);
        assert_eq!(value["token"], 19);
    }
    #[test]
    fn changed_publication_and_capture_owner_are_not_replaced_with_a_guess() {
        let capture = FrameCapture {
            width: 1,
            height: 1,
            rgba: vec![0; 4],
            depth: vec![0.0],
        };
        let value = snapshot_json(
            29,
            "a.png",
            &capture,
            json!({"valid":true,"publication":31}),
            json!({"valid":true,"publication":32}),
        );
        assert_eq!(value["cursor_before"]["publication"], 31);
        assert_eq!(value["cursor_after"]["publication"], 32);
        assert_eq!(value["rgba_owner"], capture.rgba.as_ptr() as usize);
    }
}
