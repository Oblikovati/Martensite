//! Borrow the invocation's surviving owner while applying one temporary dump option.
use martensite_tkv3d::v3d::V3dStereoDumpOptions;
use std::error::Error;

pub(super) trait StereoDumpTarget {
    type Output;
    fn projection(&mut self) -> Option<V3dStereoDumpOptions>;
    fn set_projection(&mut self, option: V3dStereoDumpOptions);
    fn capture_dump(&mut self, basename: &'static str) -> Result<Self::Output, Box<dyn Error>>;
}

struct StereoDumpScope<'a, T: StereoDumpTarget> {
    target: &'a mut T,
    previous: V3dStereoDumpOptions,
}

impl<'a, T: StereoDumpTarget> StereoDumpScope<'a, T> {
    fn enter(
        target: &'a mut T,
        basename: &'static str,
        option: V3dStereoDumpOptions,
    ) -> Result<Self, Box<dyn Error>> {
        let previous = target.projection().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                format!("stereo dump {basename:?}: Stage=None; expected a live session"),
            )
        })?;
        target.set_projection(option);
        Ok(Self { target, previous })
    }
}

impl<T: StereoDumpTarget> Drop for StereoDumpScope<'_, T> {
    fn drop(&mut self) {
        if self.target.projection().is_some() {
            self.target.set_projection(self.previous);
        }
    }
}

/// Apply one temporary projection to a surviving owner; `with_stereo_dump(run, "a.png", Blended)?`.
pub(super) fn with_stereo_dump<T: StereoDumpTarget>(
    target: &mut T,
    basename: &'static str,
    option: V3dStereoDumpOptions,
) -> Result<T::Output, Box<dyn Error>> {
    let scope = StereoDumpScope::enter(target, basename, option)?;
    scope.target.capture_dump(basename)
}

#[cfg(test)]
#[path = "stereo_dump_scope_contract.rs"]
mod stereo_scope_contract;
