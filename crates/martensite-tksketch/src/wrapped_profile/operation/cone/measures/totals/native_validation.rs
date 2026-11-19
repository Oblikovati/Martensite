//! #560: reported native failure precedes further family reference and budget work.
use super::*;
use martensite_tkbo::body_check::ValidationReport;

pub(super) fn require(family: &OwnedFamily<Selection, Geometry>) -> ConeChecked<()> {
    for (region, member) in &family.members {
        require_report(*region, &member.native.report)?;
    }
    Ok(())
}

fn require_report(region: SurfaceRegionId, report: &ValidationReport) -> ConeChecked<()> {
    if !report.is_valid() {
        return Err(storage::failure(
            region,
            None,
            ConeCause::ActualValidation(Box::new(report.clone())),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
