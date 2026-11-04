//! Optional test artifacts use the established artifact root and never affect renderer policy.
use std::path::PathBuf;
pub(super) struct DashCaptureDirectory(PathBuf);
impl DashCaptureDirectory {
    pub(super) fn requested() -> Option<Self> {
        let root = std::env::var_os("MARTENSITE_GEOMETRY_ARTIFACT_DIR")?;
        Some(Self(PathBuf::from(root)))
    }
    pub(super) fn save(&self, name: &str, rgba: &[u8], depth: &[f32]) {
        std::fs::create_dir_all(&self.0).unwrap();
        std::fs::write(self.0.join(format!("{name}.rgba")), rgba).unwrap();
        let depth: Vec<u8> = depth.iter().flat_map(|v| v.to_ne_bytes()).collect();
        std::fs::write(self.0.join(format!("{name}.depth")), depth).unwrap();
    }
}
