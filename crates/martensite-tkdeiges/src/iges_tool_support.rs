//! What the tools of the annotation, drawing and definition packages test on their texts.
use martensite_tkernel::tcollection::AsciiString;

/// The first character of a message buffer, 0 for an empty one: a buffer is tested on it to know
/// whether a message was composed.
///
/// ```
/// use martensite_tkdeiges::iges_tool_support::iges_buffer_first;
/// assert_eq!(iges_buffer_first("bad"), b'b');
/// assert_eq!(iges_buffer_first(""), 0);
/// ```
pub fn iges_buffer_first(buffer: &str) -> u8 {
    buffer.as_bytes().first().copied().unwrap_or(0)
}

/// 0 when a text equals another, as a C string comparison reports equality; non-zero otherwise.
///
/// ```
/// use martensite_tkdeiges::iges_tool_support::iges_text_compare;
/// use martensite_tkernel::tcollection::AsciiString;
/// assert_eq!(iges_text_compare(&AsciiString::new("MM"), "MM"), 0);
/// assert_ne!(iges_text_compare(&AsciiString::new("MM"), "IN"), 0);
/// ```
pub fn iges_text_compare(first: &AsciiString, second: &str) -> i32 {
    i32::from(first.as_bytes() != second.as_bytes())
}
