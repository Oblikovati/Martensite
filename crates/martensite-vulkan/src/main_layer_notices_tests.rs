use crate::Graphic3dRenderView;
use crate::immediate_redraw_fixture::{
    SceneExtractionOnly, display_box, display_sphere, lit_sphere_view,
};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ZLAYER_ID_DEFAULT, GRAPHIC3D_ZLAYER_ID_TOP, GRAPHIC3D_ZLAYER_ID_UNKNOWN,
};
use martensite_tkv3d::{
    ais::AisInteractiveContext,
    v3d::{V3dViewSource, V3dViewer},
};

/// A viewer without a device, one of its views (as the V3d view and as the render view) and
/// an interactive context.
struct NoticeScene {
    _viewer: Handle<V3dViewer>,
    v3d: Handle<dyn V3dViewSource>,
    view: Handle<Graphic3dRenderView>,
    context: Handle<AisInteractiveContext>,
}

fn scene_view() -> NoticeScene {
    let driver = crate::Graphic3dRenderDriver::with_backend(SceneExtractionOnly, 4096).unwrap();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let v3d = lit_sphere_view(&viewer);
    let context = AisInteractiveContext::new(&viewer).unwrap();
    let view = v3d.get().unwrap().view().cast();
    NoticeScene {
        _viewer: viewer,
        v3d,
        view,
        context,
    }
}

#[test]
fn notices_count_main_layers_and_skip_immediate_ones() {
    let NoticeScene { view, .. } = scene_view();
    let count = || view.main_notice_count_for_test();
    let start = count();
    view.notice_layer_for_test(GRAPHIC3D_ZLAYER_ID_TOP);
    assert_eq!(count(), start, "the top layer is immediate");
    view.notice_layer_for_test(GRAPHIC3D_ZLAYER_ID_DEFAULT);
    assert_eq!(count(), start + 1);
    view.notice_layer_for_test(GRAPHIC3D_ZLAYER_ID_UNKNOWN);
    assert_eq!(count(), start + 2, "a notice for every layer counts");
    view.notice_layer_for_test(12_345);
    assert_eq!(count(), start + 3, "a layer the view lacks counts");
}

#[test]
fn displays_count_and_dynamic_highlights_do_not() {
    let NoticeScene {
        view, v3d, context, ..
    } = scene_view();
    let count = || view.main_notice_count_for_test();
    let start = count();
    display_sphere(&context);
    assert!(count() > start, "a displayed main structure is a notice");
    // The first hover computes the highlight presentation; later hovers only show it.
    context.move_to(64, 64, &v3d, false).unwrap();
    context.move_to(2, 2, &v3d, false).unwrap();
    let hovered = count();
    for (x, y) in [(64, 64), (2, 2), (64, 64)] {
        context.move_to(x, y, &v3d, false).unwrap();
    }
    assert_eq!(count(), hovered);
    display_box(&context);
    assert!(count() > hovered);
}
