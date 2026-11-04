//! A worker-wide explicit precision/origin contract never inspects source case or vendor names.
use martensite_vulkan::{RenderLineGrid::*, RenderLineOrigin::*, RenderThinAxisCoverage};
use std::{ffi::OsStr, io};
pub(crate) const SELECTION: &str = "MARTENSITE_CASE_THIN_AXIS_COVERAGE";
pub(crate) fn selected(value: Option<&OsStr>) -> io::Result<RenderThinAxisCoverage> {
    let Some(actual) = value else {
        return Ok(Default::default());
    };
    if actual == "hardware-v1" {
        return Ok(Default::default());
    }
    fixed_choice(actual).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput,
        format!("{SELECTION}={actual:?}; expected hardware-v1 or fixed-grid{{8|12}}-{{bottom-up|top-down}}-v2")))
}
fn fixed_choice(value: &OsStr) -> Option<RenderThinAxisCoverage> {
    let (grid, origin) = match value.to_str()? {
        "fixed-grid8-bottom-up-v2" => (Fractional8, BottomUp),
        "fixed-grid12-bottom-up-v2" => (Fractional12, BottomUp),
        "fixed-grid8-top-down-v2" => (Fractional8, TopDown),
        "fixed-grid12-top-down-v2" => (Fractional12, TopDown),
        _ => return None,
    };
    Some(RenderThinAxisCoverage::FixedGridV2 { grid, origin })
}
pub(super) fn metadata(behavior: RenderThinAxisCoverage) -> String {
    format!(
        "{{\"thin_axis_coverage\":\"{behavior:?}\",\"scope\":\"single-sample-solid-unlit-axis-lines\"}}"
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn explicit_grid_origin_and_legacy_selectors_are_exact() {
        assert_eq!(selected(None).unwrap(), RenderThinAxisCoverage::HardwareV1);
        assert_eq!(
            selected(Some(OsStr::new("hardware-v1"))).unwrap(),
            Default::default()
        );
        for value in [
            "fixed-grid8-bottom-up-v2",
            "fixed-grid12-bottom-up-v2",
            "fixed-grid8-top-down-v2",
            "fixed-grid12-top-down-v2",
        ] {
            assert_eq!(
                selected(Some(OsStr::new(value))).unwrap(),
                fixed_choice(OsStr::new(value)).unwrap()
            );
        }
        assert!(selected(Some(OsStr::new("amd"))).is_err());
        assert!(metadata(RenderThinAxisCoverage::HardwareV1).contains("HardwareV1"));
    }
}
