//! `GeomFill_AppSweep`: the instance of the template `AppBlend_AppSurf.gxx` on
//! `GeomFill_Line` and `GeomFill_SweepSectionGenerator`.
use crate::app_blend::AppBlendAppSurf;

/// The approximation of the sections of `GeomFill_SweepSectionGenerator`
/// (`GeomFill_AppSweep`). As for `GeomFill_AppSurf`, the template stores neither
/// its line nor its generator, so the instance is the AppBlend type itself; `GeomFillSweepSectionGenerator`
/// implements `TheSectionGenerator`.
///
/// `let mut app = GeomFillAppSweep::new(3, 8, 1e-7, 1e-9, 0, false);
/// app.perform(line.get(), &mut generator, false)?;`
pub type GeomFillAppSweep = AppBlendAppSurf;
