//! The command and native client share exact width-aware dictionary formatting.
/// Render actual diagnostic entries with the source title; `format_information(&dictionary, 80)`.
pub(crate) fn format_information(
    dictionary: &martensite_tkservice::graphic3d::Graphic3dFrameStatsDict,
    width: i32,
) -> String {
    let entries: Vec<String> = dictionary
        .items()
        .map(|(key, value)| {
            format_entry(
                &String::from_utf8_lossy(key.as_bytes()),
                &String::from_utf8_lossy(value.as_bytes()),
                width,
            )
        })
        .collect();
    format!("OpenGL info:\n{}", entries.join("\n"))
}

/// Format one actual diagnostic value; `format_entry("device", "actual device", 80)`.
pub(crate) fn format_entry(key: &str, value: &str, width: i32) -> String {
    if (key.len() + value.len() + 4) as i64 <= i64::from(width) {
        return format!("  {key}: {value}");
    }
    let mut lines = vec![String::new()];
    for word in value.split(' ').filter(|word| !word.is_empty()) {
        append_word(&mut lines, word, width);
    }
    let suffix: String = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| format!("\n    {line}"))
        .collect();
    format!("  {key}:{suffix}")
}
fn append_word(lines: &mut Vec<String>, word: &str, width: i32) {
    let line = lines.last_mut().expect("one initial diagnostic line");
    if !line.is_empty() && (line.len() + word.len() + 5) as i64 > i64::from(width) {
        lines.push(word.to_owned());
        return;
    }
    if !line.is_empty() {
        line.push(' ');
    }
    line.push_str(word);
}
#[cfg(test)]
mod controls {
    use super::*;
    #[test]
    fn diagnostic_entry_preserves_short_and_long_real_values() {
        assert_eq!(
            format_entry("GPU", "actual device", 80),
            "  GPU: actual device"
        );
        assert_eq!(
            format_entry("GPU", "actual  device family", 15),
            "  GPU:\n    actual\n    device\n    family"
        );
        assert_eq!(format_entry("LongKey", "", 1), "  LongKey:");
    }
}
