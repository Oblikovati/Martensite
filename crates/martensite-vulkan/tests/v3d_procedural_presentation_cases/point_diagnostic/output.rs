//! Claims precede all counterfactual mutations and refuse stale capture destinations.
use super::*;
use std::{fs::OpenOptions, io::Write, path::Path};
trait OutputInventory {
    fn occupied(&self, path: &Path) -> io::Result<bool>;
    fn claim(&self, path: &Path) -> io::Result<()>;
}
struct DiskOutputInventory;
impl OutputInventory for DiskOutputInventory {
    fn occupied(&self, path: &Path) -> io::Result<bool> {
        match path.symlink_metadata() {
            Ok(_) => Ok(true),
            Err(cause) if cause.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(cause) => Err(cause),
        }
    }
    fn claim(&self, path: &Path) -> io::Result<()> {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?
            .write_all(b"point-frame-unlit-diagnostic-v1\n")
    }
}
/// Reserve one fresh witness-bearing attempt before swapping; `reserve()?`.
pub(super) fn reserve() -> Outcome {
    if std::env::var_os("MARTENSITE_CASE_CAPTURE_WITNESS").as_deref() != Some(OsStr::new("1")) {
        return Err(io::Error::other("point diagnostic capture witness; expected literal1").into());
    }
    let claim = crate::source_output::capture_path("bug27700_unlit_diagnostic.attempt")?;
    let images = [capture::Phase::Unlit, capture::Phase::Restored]
        .map(|phase| crate::source_output::capture_path(phase.basename()))
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    reserve_paths(&claim, &images, &DiskOutputInventory)?;
    Ok(())
}
fn reserve_paths(
    claim: &Path,
    images: &[std::path::PathBuf],
    inventory: &dyn OutputInventory,
) -> io::Result<()> {
    inventory.claim(claim)?;
    for image in images {
        for path in diagnostic_paths(image) {
            if inventory.occupied(&path)? {
                return Err(io::Error::other(format!(
                    "point diagnostic output={path:?}; expected fresh image/witness/tie-sidecar"
                )));
            }
        }
    }
    Ok(())
}
fn diagnostic_paths(image: &Path) -> [std::path::PathBuf; 3] {
    [
        image.to_path_buf(),
        std::path::PathBuf::from(format!("{}.witness", image.display())),
        std::path::PathBuf::from(format!("{}.ties.json", image.display())),
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, path::PathBuf};
    struct NamedOutputInventory {
        claimed: Cell<bool>,
        stale: Option<PathBuf>,
    }
    impl OutputInventory for NamedOutputInventory {
        fn occupied(&self, path: &Path) -> io::Result<bool> {
            Ok(self.stale.as_deref() == Some(path))
        }
        fn claim(&self, _: &Path) -> io::Result<()> {
            if self.claimed.replace(true) {
                return Err(io::Error::from(io::ErrorKind::AlreadyExists));
            }
            Ok(())
        }
    }
    #[test]
    fn fresh_claim_is_exclusive_and_stale_images_refuse() {
        let inventory = NamedOutputInventory {
            claimed: Cell::new(false),
            stale: None,
        };
        let images = [PathBuf::from("point.png")];
        reserve_paths(Path::new("claim"), &images, &inventory).unwrap();
        assert_eq!(
            reserve_paths(Path::new("claim"), &images, &inventory)
                .unwrap_err()
                .kind(),
            io::ErrorKind::AlreadyExists
        );
    }
    #[test]
    fn stale_images_refuse_after_the_exclusive_claim() {
        let images = [PathBuf::from("point.png")];
        let stale = NamedOutputInventory {
            claimed: Cell::new(false),
            stale: Some(PathBuf::from("point.png")),
        };
        assert!(
            reserve_paths(Path::new("claim"), &images, &stale)
                .unwrap_err()
                .to_string()
                .contains("fresh image/witness")
        );
    }
    #[test]
    fn stale_tie_sidecar_refuses_without_existing_image() {
        let images = [PathBuf::from("point.png")];
        let stale = NamedOutputInventory {
            claimed: Cell::new(false),
            stale: Some(PathBuf::from("point.png.ties.json")),
        };
        let error = reserve_paths(Path::new("claim"), &images, &stale).unwrap_err();
        assert!(error.to_string().contains("point.png.ties.json"));
        assert!(stale.claimed.get());
    }
}
