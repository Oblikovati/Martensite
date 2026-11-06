//! Same-driver control captures beside an original image: the view dumped again with some
//! objects erased, for the uncovered-surface and undefined-locus graders
//! (the recording tool, the recording tool).
use super::*;

/// Dump `scene` to `<image>.<suffix>.png` without registering it as an original image; a
/// failed dump or save is an error, since a missing control would leave a grade unproven.
/// Example: `control_dump(&scene, "case.png", "locus-1")?`.
pub(crate) fn control_dump<V: SourceViewOutput>(
    scene: &V,
    image: &str,
    suffix: &str,
) -> Result<(), Box<dyn Error>> {
    let basename = format!("{image}.{suffix}.png");
    match source_dump(scene, &basename)? {
        Some(_) => Ok(()),
        None => Err(std::io::Error::other(format!(
            "control capture {basename} not written; expected a dumped and saved control image"
        ))
        .into()),
    }
}
