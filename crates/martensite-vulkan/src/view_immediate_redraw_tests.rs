use crate::immediate_redraw_fixture::{display_box, display_sphere, lit_sphere_view};
use crate::{FrameCapture, Graphic3dRenderView};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dCView as _;
use martensite_tkv3d::{
    ais::AisInteractiveContext,
    v3d::{V3dViewSource, V3dViewer},
};

/// A lit sphere drawn once, then hovered: its dynamic highlight is in an immediate layer.
struct HoveredSphere {
    driver: Handle<crate::Graphic3dRenderDriver>,
    _viewer: Handle<V3dViewer>,
    view: Handle<dyn V3dViewSource>,
    context: Handle<AisInteractiveContext>,
    rendered: Handle<Graphic3dRenderView>,
}

impl HoveredSphere {
    fn new() -> Self {
        let options = crate::VulkanOptions {
            validation: true,
            device_index: None,
        };
        let driver = crate::Graphic3dRenderDriver::new(options).unwrap();
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        let view = lit_sphere_view(&viewer);
        let context = AisInteractiveContext::new(&viewer).unwrap();
        display_sphere(&context);
        view.auto_z_fit().unwrap();
        let rendered: Handle<Graphic3dRenderView> = view.get().unwrap().view().cast();
        rendered.try_redraw().unwrap();
        context.move_to(64, 64, &view, false).unwrap();
        assert!(context.has_detected());
        // The first immediate redraw draws the main pass and keeps it.
        rendered.redraw_immediate();
        Self {
            driver,
            _viewer: viewer,
            view,
            context,
            rendered,
        }
    }

    /// An immediate redraw, whether it kept the main pass without building it, and its image.
    fn redraw_immediate(&self) -> (bool, FrameCapture) {
        let before = self.rendered.elided_redraws();
        self.rendered.redraw_immediate();
        let elided = self.rendered.elided_redraws() > before;
        (elided, self.rendered.capture().unwrap())
    }

    /// The image of a whole redraw of the same state.
    fn whole_redraw(&self) -> FrameCapture {
        self.rendered.try_redraw().unwrap();
        self.rendered.capture().unwrap()
    }

    fn assert_no_validation_errors(&self) {
        assert!(self.driver.validation_errors().unwrap().is_empty());
    }
}

fn assert_same_image(actual: &FrameCapture, expected: &FrameCapture) {
    assert_eq!(actual.rgba, expected.rgba);
    assert_eq!(actual.depth, expected.depth);
}

#[test]
fn an_unchanged_main_pass_redraws_only_the_immediate_layers() {
    let scene = HoveredSphere::new();
    let (elided, image) = scene.redraw_immediate();
    assert!(elided, "no main-layer notice since the main pass was kept");
    let work = scene.rendered.last_frame_work();
    assert_eq!(work.geometry_uploaded_bytes, 0);
    assert_eq!(work.geometry_buffers_created, 0);
    assert_same_image(&image, &scene.whole_redraw());
    scene.assert_no_validation_errors();
}

#[test]
fn hovering_on_and_off_keeps_the_main_pass() {
    let scene = HoveredSphere::new();
    for (x, y) in [(2, 2), (64, 64), (2, 2)] {
        scene.context.move_to(x, y, &scene.view, false).unwrap();
        let (elided, _) = scene.redraw_immediate();
        assert!(
            elided,
            "a dynamic highlight notices only an immediate layer at {x}, {y}"
        );
    }
    scene.assert_no_validation_errors();
}

#[test]
fn a_main_layer_notice_draws_the_whole_frame() {
    let scene = HoveredSphere::new();
    assert!(scene.redraw_immediate().0);
    display_box(&scene.context);
    let (elided, image) = scene.redraw_immediate();
    assert!(!elided, "a displayed structure is a main-layer notice");
    assert_same_image(&image, &scene.whole_redraw());
    // The whole frame proved and kept the new main pass.
    scene.redraw_immediate();
    assert!(scene.redraw_immediate().0);
    scene.assert_no_validation_errors();
}

#[test]
fn a_camera_change_without_a_notice_draws_the_whole_frame() {
    let scene = HoveredSphere::new();
    assert!(scene.redraw_immediate().0);
    scene.view.camera().set_scale(12.);
    let (elided, image) = scene.redraw_immediate();
    assert!(!elided, "the camera changes in place, without a notice");
    assert_same_image(&image, &scene.whole_redraw());
    scene.assert_no_validation_errors();
}
