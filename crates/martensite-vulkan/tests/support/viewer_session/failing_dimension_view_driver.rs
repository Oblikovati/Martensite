//! A named driver failure seam delegates all normal behavior to the existing null backend.
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::{graphic3d::*, null_driver::NullGraphicDriver};
use std::sync::atomic::{AtomicUsize, Ordering};

pub(super) struct FailingDimensionViewDriver {
    transient: StandardTransient,
    inner: Handle<dyn Graphic3dGraphicDriver>,
    requested: AtomicUsize,
    fail_at: usize,
}
martensite_tkernel::impl_transient!(FailingDimensionViewDriver, "Martensite_FailingDimensionViewDriver", StandardTransient, transient,
    interfaces: [dyn Graphic3dGraphicDriver]);

impl FailingDimensionViewDriver {
    pub(super) fn new_handle(fail_at: usize) -> Handle<dyn Graphic3dGraphicDriver> {
        Handle::new(Self {
            transient: StandardTransient::default(),
            inner: NullGraphicDriver::new_handle(),
            requested: AtomicUsize::new(0),
            fail_at,
        })
        .cast()
    }
}

impl Graphic3dGraphicDriver for FailingDimensionViewDriver {
    fn graphic_driver_base(&self) -> &Graphic3dGraphicDriverBase {
        self.inner.graphic_driver_base()
    }
    fn inquire_limit(&self, limit: Graphic3dTypeOfLimit) -> i32 {
        self.inner.inquire_limit(limit)
    }
    fn create_structure(
        &self,
        manager: &Handle<Graphic3dStructureManager>,
    ) -> Result<Handle<dyn Graphic3dCStructure>, Failure> {
        self.inner.create_structure(manager)
    }
    fn remove_structure(&self, structure: &mut Handle<dyn Graphic3dCStructure>) {
        self.inner.remove_structure(structure)
    }
    fn create_view(
        &self,
        manager: &Handle<Graphic3dStructureManager>,
    ) -> Result<Handle<dyn Graphic3dCView>, Failure> {
        let requested = self.requested.fetch_add(1, Ordering::Relaxed) + 1;
        if requested == self.fail_at {
            return Err(Failure::new(&format!(
                "owned view request {requested}; expected a non-failing request"
            )));
        }
        self.inner.create_view(manager)
    }
    fn remove_view(&self, view: &Handle<dyn Graphic3dCView>) {
        self.inner.remove_view(view)
    }
    fn enable_vbo(&self, enabled: bool) {
        self.inner.enable_vbo(enabled)
    }
    fn is_vertical_sync(&self) -> bool {
        self.inner.is_vertical_sync()
    }
    fn set_vertical_sync(&self, enabled: bool) {
        self.inner.set_vertical_sync(enabled)
    }
    fn memory_info(&self, free_bytes: &mut usize, info: &mut AsciiString) -> bool {
        self.inner.memory_info(free_bytes, info)
    }
    fn default_text_height(&self) -> f32 {
        self.inner.default_text_height()
    }
    fn text_size(&self, view: &Handle<dyn Graphic3dCView>, text: &str, height: f32) -> [f32; 3] {
        self.inner.text_size(view, text, height)
    }
    fn insert_layer_before(
        &self,
        id: Graphic3dZLayerId,
        settings: &Graphic3dZLayerSettings,
        after: Graphic3dZLayerId,
    ) -> Result<(), Failure> {
        self.inner.insert_layer_before(id, settings, after)
    }
    fn insert_layer_after(
        &self,
        id: Graphic3dZLayerId,
        settings: &Graphic3dZLayerSettings,
        before: Graphic3dZLayerId,
    ) -> Result<(), Failure> {
        self.inner.insert_layer_after(id, settings, before)
    }
    fn remove_z_layer(&self, id: Graphic3dZLayerId) -> Result<(), Failure> {
        self.inner.remove_z_layer(id)
    }
    fn set_z_layer_settings(
        &self,
        id: Graphic3dZLayerId,
        settings: &Graphic3dZLayerSettings,
    ) -> Result<(), Failure> {
        self.inner.set_z_layer_settings(id, settings)
    }
    fn view_exists(
        &self,
        window: &Handle<dyn martensite_tkservice::aspect::AspectWindow>,
        view: &mut Handle<dyn Graphic3dCView>,
    ) -> bool {
        self.inner.view_exists(window, view)
    }
}
