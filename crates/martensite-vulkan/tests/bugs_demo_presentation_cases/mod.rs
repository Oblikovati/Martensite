//! Unicode and bounding-box programs retain actual presentation ownership and source checks.
mod bounding;
use crate::case_view::viewer_session::ViewerSession;
use crate::session_run::{SessionRun, banner};
pub(crate) use bounding::{printed_sphere_bounds, shown_sphere_bounds};
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::{Failure, Handle},
    tcollection::ExtendedString,
};
use martensite_tkservice::aspect::AspectTypeOfColorScaleData;
use martensite_tkv3d::ais::AisColorScale;
type Outcome = Result<(), Box<dyn std::error::Error>>;
const TITLE: &str = "Языки Восточной Азии";
const LABELS: [&str; 3] = ["Japanese 日本語", "Korean 한국어", "Chinese 中文"];

fn configure_scale(scale: &AisColorScale) {
    scale.set_title(&ExtendedString::from_utf8(TITLE));
    scale.set_range(1., 3.);
    scale.set_number_of_intervals(3);
    scale.set_colors(&[NamedColor::RED, NamedColor::GREEN, NamedColor::BLUE].map(Color::named));
    scale.set_color_type(AspectTypeOfColorScaleData::User);
    scale.set_label_at_border(false);
    scale.set_labels(&LABELS.map(ExtendedString::from_utf8));
    scale.set_label_type(AspectTypeOfColorScaleData::User);
}

fn show_languages(session: &mut ViewerSession) -> Result<Handle<AisColorScale>, Failure> {
    let scale = session.color_scale("languages")?;
    configure_scale(
        scale
            .get()
            .expect("new languages scale; expected live AIS object"),
    );
    session.show_color_scale("languages", &scale)?;
    Ok(scale)
}

/// Retain the literal Cyrillic title and CJK labels and mandatory image; `unicode_scale()?`.
pub(crate) fn unicode_scale() -> Outcome {
    banner(&[
        "# ============================================================",
        "# 0014673: Provide true support for Unicode symbols",
        "# ============================================================",
        "",
        "# Use non-Ascii names for color scale",
    ]);
    let mut run = SessionRun::new()?;
    show_languages(run.session())?;
    run.dump("bug14673_1.png")?;
    run.finish()
}

#[test]
fn unicode_scale_keeps_exact_title_labels_range_and_user_colours() {
    let scale = AisColorScale::new().unwrap();
    let scale = scale.get().unwrap();
    configure_scale(scale);
    assert_eq!(*scale.get_title(), ExtendedString::from_utf8(TITLE));
    assert_eq!(&*scale.labels(), &LABELS.map(ExtendedString::from_utf8));
    assert_eq!(scale.get_range(), (1., 3.));
    assert_eq!(scale.get_number_of_intervals(), 3);
    assert!(!scale.is_label_at_border());
    assert_eq!(scale.get_color_type(), AspectTypeOfColorScaleData::User);
    assert_eq!(scale.get_label_type(), AspectTypeOfColorScaleData::User);
    assert_eq!(
        &*scale.colors(),
        &[NamedColor::RED, NamedColor::GREEN, NamedColor::BLUE].map(Color::named)
    );
}

#[test]
fn unicode_scale_is_bound_as_the_actual_displayed_session_object() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let scale = show_languages(&mut session).unwrap();
    assert_eq!(session.object("languages"), Some(&scale.cast()));
    assert!(session.context().is_displayed(&scale.cast()));
    assert_eq!(scale.get().unwrap().get_breadth(), 409);
    assert_eq!(scale.get().unwrap().get_height(), 409);
}
