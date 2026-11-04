//! Highlight transparency on the literal box/cone and camera of bug27818_1.
use super::*;
use crate::primitive_shapes::{box_sized, cone};
use martensite_tkv3d::prs_mgr::{PrsMgrDisplayStatus, PrsMgrPresentableObjectMembers as _};
use martensite_tkv3d::select_mgr::SelectMgrSelectableObjectMembers as _;

/// Preserve the exact named-color predicate; `colour_error("RED", "RED", "wrong")`.
pub(super) fn colour_error(
    actual: &str,
    expected: &str,
    message: &'static str,
) -> Option<&'static str> {
    (actual != expected).then_some(message)
}

fn context_geometry(session: &mut ViewerSession) -> Result<(), Failure> {
    session.clear()?;
    let box_shape = box_sized([10., 20., 30.])?;
    let cone_shape = cone(15., 0., 40.)?;
    let shaded = DisplayRequest {
        display_mode: Some(1),
        ..Default::default()
    };
    session.update_session(|session| {
        session.display("b", &box_shape, &shaded)?;
        repeat_bound_box(session)?;
        session.display("p", &cone_shape, &shaded)
    })?;
    session.set_color("b", Color::named(NamedColor::RED))?;
    session.set_color("p", Color::named(NamedColor::GRAY))?;
    Ok(())
}

fn repeat_bound_box(session: &ViewerSession) -> Result<(), Failure> {
    let object = session
        .object("b")
        .expect("b; expected the just-displayed box owner");
    object.set_display_mode(1);
    println!("Display b");
    session.context().display_with_modes(
        object,
        1,
        object.global_selection_mode(),
        false,
        PrsMgrDisplayStatus::None,
    )
}

fn context_camera() -> ViewParameters {
    ViewParameters {
        scale: Some(17.8),
        proj: Some([0.892687, -0.331602, 0.305206]),
        up: Some([-0.162521, 0.394789, 0.904284]),
        at: Some([1.99, 2.4, 20.9]),
        ..Default::default()
    }
}

fn context_style(session: &ViewerSession) -> Result<(), Failure> {
    let style = HighlightStyle {
        transparency: Some(0.3),
        color: Some(Color::named(NamedColor::PALEGREEN2)),
        display_mode: Some(1),
        ..Default::default()
    };
    session.set_view_parameters(&context_camera())?;
    session.set_highlight_style(Prs3dTypeOfHighlight::Dynamic, &style)
}

fn check_colour(
    session: &ViewerSession,
    x: i32,
    y: i32,
    expected: &str,
    message: &'static str,
) -> Outcome {
    let actual = session.pixel_color_name(x, y)?;
    if let Some(error) = colour_error(&actual, expected, message) {
        println!("{error}");
    }
    Ok(())
}

fn reset_detection(session: &ViewerSession) {
    // The authored fresh view has no grid echo; preserve ClearDetected(false) then immediate redraw.
    if session.context().clear_detected(false) {
        session.context().current_viewer().redraw_immediate();
    }
}

fn check_box_highlight(session: &mut ViewerSession) -> Outcome {
    session.move_cursor(285, 212)?;
    check_colour(session, 285, 212, "KHAKI3", BOX_TRANSPARENCY)?;
    reset_detection(session);
    check_colour(session, 285, 212, "RED", BOX_RESET)
}

fn check_cone_highlight(session: &mut ViewerSession) -> Outcome {
    session.move_cursor(167, 263)?;
    check_colour(session, 167, 263, "DARKSEAGREEN2", CONE_TRANSPARENCY)
}

const BOX_TRANSPARENCY: &str = "Error: highlighting of a box is not transparent!";
const BOX_RESET: &str = "Error: highlighting of a box is not reset!";
const CONE_TRANSPARENCY: &str = "Error: highlighting of a cone is not transparent!";
const BANNER: &[&str] = &[
    "============",
    "0027818: Visualization - provide an interface to define highlight presentation properties",
    "Test change of highlight properties for whole interactive context",
    "============",
    "",
];

/// Run all three exact RGB-name assertions and the mandatory final capture; `context_highlight()?`.
pub(crate) fn context_highlight() -> Outcome {
    banner(BANNER);
    let mut run = SessionRun::new()?;
    context_geometry(run.session())?;
    context_style(run.session())?;
    check_box_highlight(run.session())?;
    check_cone_highlight(run.session())?;
    run.dump("bug27818_1.png")?;
    run.finish()
}

#[test]
fn context_colour_predicates_preserve_case_and_exact_names() {
    assert_eq!(colour_error("RED", "RED", "wrong"), None);
    assert_eq!(colour_error("red", "RED", "wrong"), Some("wrong"));
    assert_eq!(colour_error("KHAKI2", "KHAKI3", "wrong"), Some("wrong"));
}

#[test]
fn context_source_geometry_keeps_two_named_objects_and_exact_shaded_modes() {
    let mut session = null_highlight_session();
    context_geometry(&mut session).unwrap();
    assert_eq!(session.object("b").unwrap().display_mode(), 1);
    assert_eq!(session.object("p").unwrap().display_mode(), 1);
    assert_eq!(
        session.object("b").unwrap().color(),
        Color::named(NamedColor::RED)
    );
    assert_eq!(
        session.object("p").unwrap().color(),
        Color::named(NamedColor::GRAY)
    );
}

#[test]
fn context_literal_camera_and_style_keep_source_float_values() {
    use martensite_tkservice::graphic3d::Graphic3dPresentationAttributesMembers as _;
    let session = null_highlight_session();
    context_style(&session).unwrap();
    let style = (*session
        .context()
        .highlight_style(Prs3dTypeOfHighlight::Dynamic)
        .unwrap())
    .clone();
    let style = style.get().unwrap();
    assert_eq!(style.display_mode(), 1);
    assert_eq!(style.color(), Color::named(NamedColor::PALEGREEN2));
    assert_eq!(style.transparency().to_bits(), 0.3f32.to_bits());
    assert_eq!(context_camera().scale, Some(17.8));
    reset_detection(&session);
    assert!(session.context().detected_interactive().is_null());
}
