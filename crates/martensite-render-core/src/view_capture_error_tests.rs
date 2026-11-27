use super::{capture_failure_record, json_string};
use crate::RenderError;

#[test]
fn capture_failure_report_preserves_message_and_json_escapes() {
    let error = RenderError::rejected(17, "finite clip values");
    assert_eq!(
        capture_failure_record(&error),
        r#"{"event":"vulkan_frame_capture_failed","error":"received 17; expected finite clip values","cause":null}"#
    );
    assert_eq!(json_string("line\n\"quote\""), "\"line\\n\\\"quote\\\"\"");
}
