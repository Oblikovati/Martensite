//! Repeated document presentation uses the live selection event pipeline.
#![cfg(target_os = "linux")]
mod support;
use martensite_tkcaf::tnaming::TNamingBuilder;
use martensite_tkernel::{ncollection::Vec2, standard::Handle, tcollection::AsciiString};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial;
use martensite_tkvcaf::tprs_std::TPrsStdAISPresentation;
use martensite_tkxcaf::xcaf_prs::XCAFPrsAISObject;
use martensite_vulkan::Graphic3dRenderView;
use support::document_viewer::{ViewerLifecycle, register_vulkan_factory};
use support::shape_document::XdeShapeRegressionContext;

#[path = "support/document_presentations.rs"]
mod document_presentations;
use document_presentations::{register_document_presentation_driver, show_explicit_label};

const DOCUMENT_VIEW_NAME: &str = "Driver1/Document_D/View1";

#[test]
fn showing_main_shape_twice_then_moving_pointer_keeps_live_selection() {
    support::case_policy::run_with_case_policy(
        "showing_main_shape_twice_then_moving_pointer_keeps_live_selection",
        case_repeated_document_show,
    );
}

fn case_repeated_document_show() {
    register_vulkan_factory();
    register_document_presentation_driver();
    let context = XdeShapeRegressionContext::new();
    let label = context.document.main().unwrap();
    TNamingBuilder::new(&label)
        .unwrap()
        .generated(&support::make_box(100., 100., 100.))
        .unwrap();
    let lifecycle = ViewerLifecycle::new(true);
    show_twice_with_replacement_checks(&lifecycle, &label);
    let point = lifecycle.move_cursor(parsed_case_pointer()).unwrap();
    assert_eq!([point.x(), point.y(), point.z()], [f64::MAX; 3]);
    assert_cursor_dispatched(&lifecycle);
    assert_real_display(&lifecycle);
    context.close();
    lifecycle.remove(DOCUMENT_VIEW_NAME, true).unwrap();
}

fn parsed_case_pointer() -> Vec2<i32> {
    let original = ["10", "10"].map(AsciiString::from);
    assert!(original.iter().all(AsciiString::is_integer_value));
    Vec2::new(
        original[0].integer_value().unwrap(),
        original[1].integer_value().unwrap(),
    )
}

fn assert_replaced_presentation(
    lifecycle: &ViewerLifecycle,
    first: &Handle<TPrsStdAISPresentation>,
    second: &Handle<TPrsStdAISPresentation>,
    previous: &Handle<dyn martensite_tkv3d::ais::AisInteractiveObject>,
) {
    assert_eq!(first, second);
    assert_ne!(previous, &second.get_ais());
    assert!(!second.get_ais().cast::<XCAFPrsAISObject>().is_null());
    assert!(!lifecycle.interactive_context().is_displayed(previous));
    assert!(
        lifecycle
            .interactive_context()
            .is_displayed(&second.get_ais())
    );
    assert_eq!(
        second.material().unwrap(),
        Graphic3dNameOfMaterial::Plastified
    );
    assert_eq!(lifecycle.state().views.borrow().extent(), 1);
}

fn assert_real_display(lifecycle: &ViewerLifecycle) {
    let view = lifecycle.current_view();
    let rendered = view.get().unwrap().view().cast::<Graphic3dRenderView>();
    assert!(!rendered.is_null());
    rendered.try_redraw().unwrap();
    let frame = rendered.capture().unwrap();
    assert_eq!((frame.width, frame.height), (409, 409));
    let (pixels, remainder) = frame.rgba.as_chunks::<4>();
    assert!(remainder.is_empty());
    assert!(pixels.iter().any(|pixel| pixel[..3] != [0, 0, 0]));
}

fn show_twice_with_replacement_checks(lifecycle: &ViewerLifecycle, label: &TdfLabel) {
    let first = show_explicit_label(lifecycle, DOCUMENT_VIEW_NAME, label);
    let previous = first.get_ais();
    let second = show_explicit_label(lifecycle, DOCUMENT_VIEW_NAME, label);
    assert_replaced_presentation(lifecycle, &first, &second, &previous);
}

fn assert_cursor_dispatched(lifecycle: &ViewerLifecycle) {
    let manager = lifecycle
        .state()
        .event_managers
        .borrow()
        .front()
        .unwrap()
        .clone();
    assert_eq!(*manager.controller().previous_move_to(), Vec2::new(10, 10));
    assert!(lifecycle.interactive_context().detected_owner().is_null());
}
