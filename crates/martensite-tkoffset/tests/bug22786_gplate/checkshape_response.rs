//! The check of the plate returns the analyzer's own result, with no invented invalid dump.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, exceptions::not_implemented};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

/// The valid text, or a refusal of an unsupported contextual dump; `returned_checkshape(&shape, "a")`.
pub(super) fn returned_checkshape(shape: &Shape, binding: &str) -> Result<&'static str, Failure> {
    let analyzer = BRepCheckAnalyzer::new(shape, true, false, false)?;
    if !analyzer.is_valid()? {
        return Err(not_implemented(&format!(
            "checkshape {binding}: invalid shape; expected implemented full contextual diagnostics"
        )));
    }
    Ok("This shape seems to be valid")
}
