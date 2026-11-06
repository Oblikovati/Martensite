//! Real existing Graphic3d/V3d path on a virtual window; no null driver or substitute shape.
use martensite_process_policy::set_process_limit;
use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingNamedShape};
use martensite_tkernel::{
    quantity::{Color, ColorSpace},
    standard::Handle,
    tcollection::ExtendedString,
};
use martensite_tklcaf::{tdf::TdfLabel, tdoc_std::TDocStdDocument};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tkservice::aspect::{
    AspectNeutralWindow, AspectWindow as _, AspectWindowMembers as _,
};
use martensite_tkv3d::{
    ais::AisInteractiveContext,
    v3d::{V3dViewSource, V3dViewer},
};
use martensite_tkvcaf::tprs_std::{TPrsStdAISPresentation, TPrsStdAISViewer};
use martensite_vulkan::{Graphic3dRenderDriver, Graphic3dRenderView, VulkanOptions};

pub struct WidthDocumentScene {
    pub document: Handle<TDocStdDocument>,
    pub label: TdfLabel,
    pub presentation: Handle<TPrsStdAISPresentation>,
    host: DocumentWidthHost,
}
impl Default for WidthDocumentScene {
    fn default() -> Self {
        Self::with_before_view_setup(100, |_| {})
    }
}
impl WidthDocumentScene {
    /// Prepare additional document attributes before viewer initialization.
    /// Example: `WidthDocumentScene::with_before_view_setup(100, |doc| { let _ = doc.get_data(); })`.
    pub fn with_before_view_setup(
        undo_limit: i32,
        setup: impl FnOnce(&Handle<TDocStdDocument>),
    ) -> Self {
        // tests/bugs/begin installs the process CPU cap and elapsed watchdog first.
        set_process_limit(Some(1000)).unwrap();
        let (document, label) = document_box(undo_limit);
        setup(&document);
        Self::from_prepared_document(document, label)
    }
    fn from_prepared_document(document: Handle<TDocStdDocument>, label: TdfLabel) -> Self {
        let host = DocumentWidthHost::new();
        TPrsStdAISViewer::new_with_context(&label, &host.context).unwrap();
        let presentation =
            TPrsStdAISPresentation::set(&label, &TNamingNamedShape::get_id()).unwrap();
        Self {
            document,
            label,
            presentation,
            host,
        }
    }
    pub fn close(&self) {
        self.host.assert_renderable();
        self.host.view.remove();
        assert!(
            self.host.driver.validation_errors().unwrap().is_empty(),
            "{:?}",
            self.host.driver.validation_errors().unwrap()
        );
    }
}
fn document_box(undo_limit: i32) -> (Handle<TDocStdDocument>, TdfLabel) {
    let document = TDocStdDocument::new(&ExtendedString::from_utf8("BinOcaf")).unwrap();
    document.set_undo_limit(undo_limit).unwrap();
    document.new_command().unwrap();
    let label = document.get_data().root().find_child(2, true).unwrap();
    let shape = BRepPrimAPIMakeBox::from_sizes(100., 200., 300.)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape();
    TNamingBuilder::new(&label)
        .unwrap()
        .generated(&shape)
        .unwrap();
    (document, label)
}
struct DocumentWidthHost {
    driver: Handle<Graphic3dRenderDriver>,
    view: Handle<dyn V3dViewSource>,
    context: Handle<AisInteractiveContext>,
    window: Handle<AspectNeutralWindow>,
}
impl DocumentWidthHost {
    fn new() -> Self {
        let driver = Graphic3dRenderDriver::new(VulkanOptions {
            validation: true,
            device_index: selected_device(),
        })
        .unwrap();
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        viewer.set_default_background_color(Color::new(0., 0., 0., ColorSpace::Rgb).unwrap());
        let context = AisInteractiveContext::new(&viewer).unwrap();
        let window = virtual_document_window();
        let view = initialize_document_view(&viewer, &window);
        let host = Self {
            driver,
            view,
            context,
            window,
        };
        host.assert_renderable();
        host
    }
    fn assert_renderable(&self) {
        assert!(self.window.is_mapped());
        let (width, height) = self.window.size();
        assert!(
            width > 0 && height > 0,
            "live extent {width}x{height} must be positive"
        );
        println!("{{\"event\":\"document-view-health\",\"extent\":[{width},{height}]}}");
        let render = self
            .view
            .get()
            .unwrap()
            .view()
            .cast::<Graphic3dRenderView>();
        // The void redraw adapter discards errors. This separate host-health guard
        // proves a real render succeeds without adding an original image predicate.
        render
            .try_redraw()
            .expect("initialized document view must render");
    }
}
/// A device named by `MARTENSITE_VULKAN_DEVICE`, or the driver's own choice.
fn selected_device() -> Option<usize> {
    let text = std::env::var("MARTENSITE_VULKAN_DEVICE").ok()?;
    Some(
        text.parse()
            .expect("MARTENSITE_VULKAN_DEVICE: expected an index"),
    )
}
fn virtual_document_window() -> Handle<AspectNeutralWindow> {
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_size(409, 409);
    window.set_virtual(true);
    window
}
fn initialize_document_view(
    viewer: &V3dViewer,
    window: &Handle<AspectNeutralWindow>,
) -> Handle<dyn V3dViewSource> {
    let view = viewer.create_view().unwrap();
    view.set_window(&window.cast(), 0).unwrap();
    view.set_computed_mode(false);
    viewer.set_default_lights().unwrap();
    viewer.set_light_on().unwrap();
    window.map();
    view.redraw();
    view
}
