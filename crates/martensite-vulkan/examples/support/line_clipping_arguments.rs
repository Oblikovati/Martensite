//! Explicit replay selections are shared by boundary and unchanged wide-line captures.
use martensite_vulkan::{RenderLineClippingBehavior, RenderLineWidthPolicy};
#[derive(Debug)]
pub struct LineCaptureSelection {
    pub behavior: RenderLineClippingBehavior,
    pub widths: RenderLineWidthPolicy,
}
impl LineCaptureSelection {
    /// Parse only explicit version/core flags; `LineCaptureSelection::parse(&flags)?`.
    pub fn parse(flags: &[String]) -> Result<Self, String> {
        validate_flags(flags)?;
        Ok(Self {
            behavior: if flags.iter().any(|flag| flag == "--clip-v2") {
                RenderLineClippingBehavior::PlatformNativeV2
            } else {
                RenderLineClippingBehavior::VulkanDriverV1
            },
            widths: if flags.iter().any(|flag| flag == "--core") {
                RenderLineWidthPolicy::SinglePixelV1
            } else {
                RenderLineWidthPolicy::AspectWidthsV1
            },
        })
    }
}
fn validate_flags(flags: &[String]) -> Result<(), String> {
    let mut seen = [false; 2];
    for flag in flags {
        let group = flag_group(flag)?;
        if std::mem::replace(&mut seen[group], true) {
            return Err(format!(
                "repeated line capture flag {flag:?} in {flags:?}; expected one width and one clipping selection"
            ));
        }
    }
    Ok(())
}
fn flag_group(flag: &str) -> Result<usize, String> {
    match flag {
        "--core" => Ok(0),
        "--clip-v1" | "--clip-v2" => Ok(1),
        _ => Err(format!(
            "line capture flag {flag:?}; expected --core or --clip-v1/--clip-v2"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn absent_flags_preserve_both_legacy_selections() {
        let selected = LineCaptureSelection::parse(&[]).unwrap();
        assert_eq!(
            selected.behavior,
            RenderLineClippingBehavior::VulkanDriverV1
        );
        assert_eq!(selected.widths, RenderLineWidthPolicy::AspectWidthsV1);
    }
    #[test]
    fn candidate_and_core_selections_compose_without_case_names() {
        for flags in [["--core", "--clip-v2"], ["--clip-v2", "--core"]] {
            let selected = LineCaptureSelection::parse(&flags.map(str::to_owned)).unwrap();
            assert_eq!(
                selected.behavior,
                RenderLineClippingBehavior::PlatformNativeV2
            );
            assert_eq!(selected.widths, RenderLineWidthPolicy::SinglePixelV1);
        }
    }
    #[test]
    fn duplicate_and_unknown_flags_are_refused() {
        for flags in [
            vec!["--core", "--core"],
            vec!["--clip-v1", "--clip-v2"],
            vec!["--unknown"],
        ] {
            assert!(
                LineCaptureSelection::parse(
                    &flags.into_iter().map(str::to_owned).collect::<Vec<_>>()
                )
                .is_err()
            );
        }
    }
}
