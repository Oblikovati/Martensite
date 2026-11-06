//! Byte-level neutrality checks for reversible same-session image controls.
use super::*;

impl SessionRun {
    /// Require a restored control capture to match the original image byte for byte.
    /// Example: `run.verify_restored_control("case.png", "restored")?`.
    pub(crate) fn verify_restored_control(
        &self,
        original: &str,
        restored_suffix: &str,
    ) -> Result<(), Box<dyn Error>> {
        let case_path = source_output::capture_path(original)?;
        let restored_name = format!("{original}.{restored_suffix}.png");
        let restored_path = source_output::capture_path(&restored_name)?;
        let case_bytes = std::fs::read(&case_path)?;
        let restored_bytes = std::fs::read(&restored_path)?;
        require_restored_png(&case_path, &restored_path, &case_bytes, &restored_bytes)
    }
}

fn require_restored_png(
    case_path: &std::path::Path,
    restored_path: &std::path::Path,
    original: &[u8],
    restored: &[u8],
) -> Result<(), Box<dyn Error>> {
    if original == restored {
        return Ok(());
    }
    Err(std::io::Error::other(format!(
        "restored control image {restored_path:?}; expected byte-identical original {case_path:?}"
    ))
    .into())
}

#[cfg(test)]
mod identic_restore_tests {
    use super::require_restored_png;
    use std::path::Path;

    #[test]
    fn restored_control_accepts_only_byte_identical_image() {
        let original = Path::new("original.png");
        let restored = Path::new("restored.png");
        assert!(require_restored_png(original, restored, b"png-bytes", b"png-bytes").is_ok());
        let error = require_restored_png(original, restored, b"png-bytes", b"changed").unwrap_err();
        let message = error.to_string();
        assert!(message.contains("restored.png"));
        assert!(message.contains("original.png"));
    }
}
