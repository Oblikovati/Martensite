//! No case of this target draws in the 3D viewer: those run in `martensite-vulkan`, whose
//! `fillet_view_cases` target has the commands. This keeps the runner the same in both.
use crate::script::FilletScript;
use crate::steps::V3d;
use martensite_tkernel::standard::Failure;

/// Refuses a viewer command; `apply(&mut script, &V3d::Init)`.
pub fn apply(_script: &mut FilletScript<'_>, view: &V3d) -> Result<(), Failure> {
    panic!("{view:?}: the viewer cases are hosted in martensite-vulkan")
}

/// Nothing to close here; `finish()`.
pub fn finish() {}
