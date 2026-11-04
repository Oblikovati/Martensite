//! Immutable diagnostic-only selection; completed frame records are not acceptance evidence.
use crate::{RenderError, error::rejected};
use std::path::{Path, PathBuf};
#[derive(Clone, Debug)]
pub(super) struct RayHitControl {
    pub mode: u32,
    pub directory: PathBuf,
}
impl RayHitControl {
    pub fn from_environment() -> Result<Option<Self>, RenderError> {
        let mode = hit_selector("MARTENSITE_RAYTRACE_HIT_ORIGIN")?;
        let directory = hit_selector("MARTENSITE_RAYTRACE_HIT_DIR")?;
        let selected = Self::parse(mode.as_deref(), directory.as_deref())?;
        if selected.is_some() && !crate::raytrace_input_observation::requested()? {
            return Err(rejected(
                "hit-origin enabled without input observer",
                "actual input observation",
            ));
        }
        Ok(selected)
    }
    pub fn parse(mode: Option<&str>, directory: Option<&str>) -> Result<Option<Self>, RenderError> {
        if mode.is_none() && directory.is_none() {
            return Ok(None);
        }
        let mode = hit_mode(mode)?;
        let directory = PathBuf::from(
            directory.ok_or_else(|| rejected("missing directory", "both selectors or neither"))?,
        );
        require_hit_directory(&directory)?;
        Ok(Some(Self { mode, directory }))
    }
    pub fn shader_bytes(&self) -> &'static [u8] {
        include_bytes!("../../shaders/raytrace_controls/hit_origin_ray.frag.spv")
    }
    pub fn require_directory(&self) -> Result<(), RenderError> {
        require_hit_directory(&self.directory)
    }
}
fn hit_mode(mode: Option<&str>) -> Result<u32, RenderError> {
    match mode {
        Some("baseline") => Ok(0),
        Some("y-center-origin") => Ok(1),
        other => Err(rejected(
            format!("mode={other:?}"),
            "baseline or y-center-origin",
        )),
    }
}
fn hit_selector(name: &str) -> Result<Option<String>, RenderError> {
    std::env::var_os(name)
        .map(|value| {
            value
                .into_string()
                .map_err(|value| rejected(format!("{name}={value:?}"), "UTF-8 diagnostic selector"))
        })
        .transpose()
}
fn require_hit_directory(path: &Path) -> Result<(), RenderError> {
    if !path.is_absolute() {
        return Err(rejected(
            path.display(),
            "absolute existing diagnostic directory",
        ));
    }
    for ancestor in path.ancestors() {
        let metadata = std::fs::symlink_metadata(ancestor)
            .map_err(|e| rejected(e, "existing diagnostic ancestors"))?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(rejected(
                ancestor.display(),
                "real directory without symbolic ancestors",
            ));
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selectors_are_exact_paired_and_default_off() {
        assert!(RayHitControl::parse(None, None).unwrap().is_none());
        assert_eq!(hit_mode(Some("baseline")).unwrap(), 0);
        assert_eq!(hit_mode(Some("y-center-origin")).unwrap(), 1);
        for value in [None, Some(""), Some("Baseline"), Some("1")] {
            assert!(hit_mode(value).is_err());
        }
        assert!(RayHitControl::parse(Some("baseline"), None).is_err());
        assert!(RayHitControl::parse(None, Some("/")).is_err());
        assert!(RayHitControl::parse(Some("baseline"), Some("relative")).is_err());
        assert_eq!(
            RayHitControl::parse(Some("baseline"), Some("/"))
                .unwrap()
                .unwrap()
                .mode,
            0
        );
    }
    #[test]
    fn diagnostic_modes_share_one_bounded_spirv_module() {
        let control = |mode| RayHitControl {
            mode,
            directory: PathBuf::from("/"),
        };
        let baseline = control(0).shader_bytes();
        let centered = control(1).shader_bytes();
        assert_eq!(baseline, centered);
        for module in [baseline, centered] {
            assert_eq!(&module[..4], &0x07230203u32.to_le_bytes());
            assert_eq!(module.len() % 4, 0);
            assert!(module.len() < 1024 * 1024);
        }
    }
}
