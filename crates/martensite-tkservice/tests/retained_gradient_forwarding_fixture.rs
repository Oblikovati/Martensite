//! A real shared-forwarding consumer, distinct from the retained storage type.
use martensite_tkernel::{
    standard::{Failure, Handle, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkservice::{aspect::*, graphic3d::*, image::ImagePixMap};

/// Test adapter whose real retained owner supplies every delegated view operation.
pub(super) struct ForwardedGradientView {
    transient: StandardTransient,
    retained: Graphic3dRetainedView,
}
martensite_tkernel::impl_transient!(ForwardedGradientView, "ForwardedGradientView", StandardTransient, transient, interfaces:[dyn Graphic3dCView]);
impl ForwardedGradientView {
    /// Own a shared-forwarding consumer; `ForwardedGradientView::new(manager, layers)`.
    pub(super) fn new(
        manager: &Handle<Graphic3dStructureManager>,
        layers: &[Handle<Graphic3dLayer>],
    ) -> Self {
        Self {
            transient: StandardTransient::default(),
            retained: Graphic3dRetainedView::new(manager, layers).unwrap(),
        }
    }
}
impl Graphic3dCView for ForwardedGradientView {
    martensite_tkservice::martensite_retained_cview_methods!();
    fn cview_base(&self) -> &Graphic3dCViewBase {
        let retained: &dyn Graphic3dCView = &self.retained;
        retained.cview_base()
    }
    fn diagnostic_information(
        &self,
        dictionary: &mut Graphic3dFrameStatsDict,
        flags: Graphic3dDiagnosticInfo,
    ) {
        let retained: &dyn Graphic3dCView = &self.retained;
        retained.diagnostic_information(dictionary, flags);
    }
    fn redraw(&self) {
        let retained: &dyn Graphic3dCView = &self.retained;
        retained.redraw();
    }
    fn redraw_immediate(&self) {
        let retained: &dyn Graphic3dCView = &self.retained;
        retained.redraw_immediate();
    }
    fn resized(&self) {
        let retained: &dyn Graphic3dCView = &self.retained;
        retained.resized();
    }
    fn set_window(
        &self,
        parent: &Handle<dyn Graphic3dCView>,
        window: &Handle<dyn AspectWindow>,
        context: AspectRenderingContext,
    ) {
        let retained: &dyn Graphic3dCView = &self.retained;
        retained.set_window(parent, window, context);
    }
    fn buffer_dump(&self, image: &ImagePixMap, kind: &Graphic3dBufferType) -> bool {
        let retained: &dyn Graphic3dCView = &self.retained;
        retained.buffer_dump(image, kind)
    }
    fn fbo_create(&self, width: i32, height: i32) -> Handle<StandardTransient> {
        let retained: &dyn Graphic3dCView = &self.retained;
        retained.fbo_create(width, height)
    }
    fn fbo_release(&self, fbo: &mut Handle<StandardTransient>) {
        let retained: &dyn Graphic3dCView = &self.retained;
        retained.fbo_release(fbo);
    }
    fn fbo_get_dimensions(&self, fbo: &Handle<StandardTransient>) -> [i32; 4] {
        let retained: &dyn Graphic3dCView = &self.retained;
        retained.fbo_get_dimensions(fbo)
    }
    fn fbo_change_viewport(&self, fbo: &Handle<StandardTransient>, width: i32, height: i32) {
        let retained: &dyn Graphic3dCView = &self.retained;
        retained.fbo_change_viewport(fbo, width, height);
    }
}
