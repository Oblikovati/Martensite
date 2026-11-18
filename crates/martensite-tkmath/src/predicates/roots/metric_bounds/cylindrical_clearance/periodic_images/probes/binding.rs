// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn image(
    periodic: &CertifiedCylindricalPeriodicClearance,
    turn: u32,
) -> Checked<&CylindricalPeriodicTranslation> {
    periodic
        .images()
        .get(turn.wrapping_sub(1) as usize)
        .map(CertifiedCylindricalImage::translation)
        .filter(|t| t.turn() == turn)
        .ok_or(CylindricalProbeFailure::Image {
            turn,
            available: periodic.images().len(),
        })
}
pub(super) fn same_probe(
    old: &CertifiedCylindricalProbe,
    new: &CylindricalPeriodicProbe,
    periodic: &CertifiedCylindricalPeriodicClearance,
) -> bool {
    let Ok(shift) = image(periodic, new.turn) else {
        return false;
    };
    old.request.turn == new.turn
        && old.request.sense == new.sense
        && old.request.point.map(f64::to_bits) == new.point.map(f64::to_bits)
        && old.translation.offset().to_bits() == shift.offset().to_bits()
        && old.translation.translation_error_bound() == shift.translation_error_bound()
        && old.translation.comparison_clearance().to_bits()
            == shift.comparison_clearance().to_bits()
}
