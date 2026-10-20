//! `GeomFill_AppSurf`: the instance of the template `AppBlend_AppSurf.gxx` on
//! `GeomFill_Line` and `GeomFill_SectionGenerator`.
use crate::app_blend::AppBlendAppSurf;

/// The approximation of a skinned surface (`GeomFill_AppSurf`). The template class
/// stores neither its line nor its generator, so the instance is the AppBlend type
/// itself: `GeomFillLine` implements
/// `TheLine` and `GeomFillSectionGenerator` implements `TheSectionGenerator`, and
/// the `perform` methods take them.
///
/// `let mut app = GeomFillAppSurf::new(3, 8, 1e-7, 1e-9, 0, false);
/// app.perform(line.get(), &mut generator, false)?;`
pub type GeomFillAppSurf = AppBlendAppSurf;
