//! Owned field identity and actual virtual capture dispatch, independent of rendering.
#[path = "capture_buffer_contention_checks.rs"]
mod contention;
#[path = "capture_buffer_reference_fixture.rs"]
mod fixture;
use fixture::ReferenceCaptureView;
use martensite_tkservice::{graphic3d::*, image::ImagePixMap};

fn reference_view() -> ReferenceCaptureView {
    let scene = super::scene();
    use martensite_tkservice::graphic3d::Graphic3dGraphicDriverMembers;
    ReferenceCaptureView::new(&scene.manager, &scene.driver.layers())
}
#[test]
fn live_capture_dispatch_rereads_same_field_after_callback_write() {
    let source = reference_view();
    let view: &dyn Graphic3dCView = &source;
    let field = source.kind.clone();
    let copied = CaptureBufferKindRef::new(field.read());
    view.redraw();
    assert_eq!(
        view.buffer_dump_referenced(&ImagePixMap::default(), &field),
        Ok(false)
    );
    assert_eq!(
        *source.reads.read(),
        [Graphic3dBufferType::Rgba, Graphic3dBufferType::Red]
    );
    assert_eq!(copied.read(), Graphic3dBufferType::Rgb);
    assert!(!copied.same_slot(&field));
}
#[test]
fn legacy_view_refuses_live_capture_without_changing_immutable_capture() {
    let scene = super::scene();
    let view: &dyn Graphic3dCView = &*scene.view;
    let kind = CaptureBufferKindRef::new(Graphic3dBufferType::Depth);
    let image = ImagePixMap::default();
    assert!(!view.buffer_dump(&image, &kind.read()));
    let error = view.buffer_dump_referenced(&image, &kind).unwrap_err();
    assert_eq!(error.operation(), "buffer_dump_referenced");
    assert_eq!(error.requested_kind(), Graphic3dBufferType::Depth);
    assert_eq!(error.cause(), CaptureBufferKindCause::UnsupportedReference);
    assert_eq!(
        error.to_string(),
        "buffer_dump_referenced requested_kind=Depth cause=UnsupportedReference; expected a reference-capable capture backend"
    );
}
#[test]
fn callback_unwind_releases_slot_and_allows_retry() {
    let mut source = reference_view();
    source.panic_after_write = true;
    let field = source.kind.clone();
    let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        source.buffer_dump_referenced(&ImagePixMap::default(), &field)
    }));
    assert!(failed.is_err());
    assert_eq!(field.read(), Graphic3dBufferType::Red);
    source.panic_after_write = false;
    source.redraw();
    assert_eq!(
        source.buffer_dump_referenced(&ImagePixMap::default(), &field),
        Ok(false)
    );
}
#[test]
fn retained_capability_survives_owner_retirement() {
    let source = reference_view();
    let field = source.kind.clone();
    drop(source);
    field.set(Graphic3dBufferType::ShadowMap);
    assert_eq!(field.read(), Graphic3dBufferType::ShadowMap);
}
#[test]
fn every_buffer_kind_round_trips_and_slot_clones_keep_identity() {
    let field = CaptureBufferKindRef::new(Graphic3dBufferType::Rgb);
    let alias = field.clone();
    for kind in [
        Graphic3dBufferType::Rgb,
        Graphic3dBufferType::Rgba,
        Graphic3dBufferType::Depth,
        Graphic3dBufferType::RgbRayTraceHdrLeft,
        Graphic3dBufferType::Red,
        Graphic3dBufferType::ShadowMap,
    ] {
        field.set(kind);
        assert_eq!(alias.read(), kind);
        assert!(field.same_slot(&alias));
        assert!(!field.same_slot(&CaptureBufferKindRef::new(kind)));
    }
}
#[test]
fn synchronized_threads_share_only_the_selected_slot() {
    let field = CaptureBufferKindRef::new(Graphic3dBufferType::Rgb);
    let _: &(dyn Send + Sync) = &field;
    let _: &(dyn Send + Sync) = &CaptureBufferKindError::unsupported(field.read());
    let copied = CaptureBufferKindRef::new(field.read());
    let writer = field.clone();
    std::thread::spawn(move || writer.set(Graphic3dBufferType::Red))
        .join()
        .unwrap();
    assert_eq!(field.read(), Graphic3dBufferType::Red);
    assert_eq!(copied.read(), Graphic3dBufferType::Rgb);
}

#[test]
fn nested_virtual_capture_rereads_the_same_slot_before_outer_return() {
    let mut source = reference_view();
    source.reenter_once = true;
    let view: &dyn Graphic3dCView = &source;
    source.redraw();
    assert_eq!(
        view.buffer_dump_referenced(&ImagePixMap::default(), &source.kind),
        Ok(false)
    );
    assert_nested_entry_order(&source);
    assert_nested_read_order(&source);
}
fn assert_nested_entry_order(source: &ReferenceCaptureView) {
    assert_eq!(
        *source.events.read(),
        [
            ("enter", false),
            ("enter", true),
            ("exit", true),
            ("exit", false)
        ]
    );
}
fn assert_nested_read_order(source: &ReferenceCaptureView) {
    assert_eq!(
        *source.reads.read(),
        [
            Graphic3dBufferType::Red,
            Graphic3dBufferType::Red,
            Graphic3dBufferType::Rgba,
            Graphic3dBufferType::Red
        ]
    );
}
