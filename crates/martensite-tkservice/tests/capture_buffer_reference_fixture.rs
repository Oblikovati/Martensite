//! Named reference-aware callback adapter for the capture protocol (#323).
use martensite_tkernel::{
    standard::{Failure, Handle, SharedState, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkservice::{aspect::*, graphic3d::*, image::ImagePixMap};

/// Test adapter whose real retained owner supplies every delegated view operation.
pub(super) struct ReferenceCaptureView {
    transient: StandardTransient,
    retained: Graphic3dRetainedView,
    pub(super) kind: CaptureBufferKindRef,
    pub(super) reads: SharedState<Vec<Graphic3dBufferType>>,
    pub(super) panic_after_write: bool,
}
martensite_tkernel::impl_transient!(ReferenceCaptureView, "ReferenceCaptureView", StandardTransient, transient, interfaces:[dyn Graphic3dCView]);
impl ReferenceCaptureView {
    /// Own a callback adapter sharing one kind field; `ReferenceCaptureView::new(manager, layers)`.
    pub(super) fn new(
        manager: &Handle<Graphic3dStructureManager>,
        layers: &[Handle<Graphic3dLayer>],
    ) -> Self {
        Self {
            transient: StandardTransient::default(),
            retained: Graphic3dRetainedView::new(manager, layers).unwrap(),
            kind: CaptureBufferKindRef::new(Graphic3dBufferType::Rgb),
            reads: SharedState::new(Vec::new()),
            panic_after_write: false,
        }
    }
}
impl Graphic3dCView for ReferenceCaptureView {
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
        // The same caller-owned slot changes before capture dispatch.
        self.kind.set(Graphic3dBufferType::Rgba);
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
    fn buffer_dump_referenced(
        &self,
        _image: &ImagePixMap,
        kind: &CaptureBufferKindRef,
    ) -> Result<bool, CaptureBufferKindError> {
        assert!(
            self.kind.same_slot(kind),
            "actual slot differs; expected caller-owned kind field"
        );
        let before = kind.read();
        self.kind.set(Graphic3dBufferType::Red);
        assert!(
            !self.panic_after_write,
            "NamedCaptureWritePanic after field write"
        );
        let after = kind.read();
        self.reads.write().extend([before, after]);
        Ok(false)
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
