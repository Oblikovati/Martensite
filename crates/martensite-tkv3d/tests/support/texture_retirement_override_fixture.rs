//! An application subclass whose setter overrides record the field around their base call,
//! and a pixmap whose destructor calls those setters through a retirement scope.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{standard::*, tcollection::AsciiString};
use martensite_tkservice::{graphic3d::*, image::ImagePixMap};
use martensite_tkv3d::{ais::*, prs_mgr::*, select_mgr::*};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub type RoutingTrace = Arc<SharedState<Vec<String>>>;
pub struct RoutingShape {
    transient: StandardTransient,
    pub base: AisTexturedShapeBase,
    pub file_calls: AtomicUsize,
    pub pixel_calls: AtomicUsize,
    pub trace: RoutingTrace,
}
martensite_tkernel::impl_transient!(RoutingShape,"RoutingShape",dyn AisTexturedShapeSource,transient,interfaces:[dyn AisTexturedShapeSource,dyn AisShapeSource,dyn AisInteractiveObject,dyn SelectMgrSelectableObject,dyn PrsMgrPresentableObject]);
impl RoutingShape {
    pub fn new(trace: &RoutingTrace) -> Handle<Self> {
        Handle::new(Self {
            transient: StandardTransient::default(),
            base: AisTexturedShapeBase::new(&Shape::default()),
            file_calls: AtomicUsize::new(0),
            pixel_calls: AtomicUsize::new(0),
            trace: trace.clone(),
        })
    }
    fn record(&self, kind: &str, phase: &str) {
        let slot = self.texture_pix_map();
        let count = if slot.is_null() { -1 } else { slot.ref_count() };
        let row = format!(
            "OVERRIDE {kind} {phase} {} {} {} {count} {}",
            self.file_calls.load(Ordering::SeqCst),
            self.pixel_calls.load(Ordering::SeqCst),
            i32::from(slot.is_null()),
            routing_file(self)
        );
        self.trace.write().push(row);
    }
}
/// The native `TextureFile()` text; `routing_file(&*owner)`.
pub fn routing_file(owner: &RoutingShape) -> String {
    let file = owner.texture_file().read().unwrap();
    file.to_string_lossy().to_string()
}
impl AisTexturedShapeSource for RoutingShape {
    fn textured_shape_base(&self) -> &AisTexturedShapeBase {
        &self.base
    }
    fn set_texture_file_name(&self, file: &AsciiString) -> Result<(), Failure> {
        self.file_calls.fetch_add(1, Ordering::SeqCst);
        self.record("file", "enter");
        ais_textured_shape_set_texture_file_name(self, file)?;
        self.record("file", "leave");
        Ok(())
    }
    fn set_texture_pix_map(&self, pixmap: &dyn HandleReference<ImagePixMap>) {
        self.pixel_calls.fetch_add(1, Ordering::SeqCst);
        self.record("pixmap", "enter");
        ais_textured_shape_set_texture_pix_map(self, pixmap);
        self.record("pixmap", "leave");
    }
}
impl AisShapeSource for RoutingShape {
    fn shape_base(&self) -> &AisShapeBase {
        &self.base.shape
    }
}
impl AisInteractiveObject for RoutingShape {
    fn interactive_base(&self) -> &AisInteractiveObjectBase {
        &self.base.shape.interactive
    }
}
impl PrsMgrPresentableObject for RoutingShape {
    fn presentable_base(&self) -> &PrsMgrPresentableObjectBase {
        &self.base.shape.interactive.presentable
    }
    fn compute(
        &self,
        _: &dyn HandleReference<PrsMgrPresentationManager>,
        _: &dyn HandleReference<dyn Graphic3dStructureSource>,
        mode: i32,
    ) -> Result<(), Failure> {
        Err(Failure::new(&format!(
            "routing fixture Compute mode {mode}; expected source-state operations"
        )))
    }
}
impl SelectMgrSelectableObject for RoutingShape {
    fn selectable_base(&self) -> &SelectMgrSelectableObjectBase {
        &self.base.shape.interactive.selectable
    }
    fn compute_selection(
        &self,
        _: &dyn HandleReference<SelectMgrSelection>,
        mode: i32,
    ) -> Result<(), Failure> {
        Err(Failure::new(&format!(
            "routing fixture selection mode {mode}; expected source-state operations"
        )))
    }
}

pub struct RoutingPixels {
    pub base: ImagePixMap,
    owner: WeakHandle<RoutingShape>,
    nested: WeakHandle<ImagePixMap>,
    mode: &'static str,
}
martensite_tkernel::impl_transient!(RoutingPixels, "RoutingPixels", ImagePixMap, base);
impl Drop for RoutingPixels {
    fn drop(&mut self) {
        let owner = self.owner.observe();
        let count = self.base.transient_base().ref_count();
        owner
            .trace
            .write()
            .push(format!("DROP {} enter {count}", self.mode));
        if self.mode == "unscoped" {
            // Accepted deviation: without a scope the dropping target is refused, typed.
            let refused = owner.set_texture_file_name(&AsciiString::from("inner.raw"));
            let row = format!(
                "UNSCOPED {}",
                refused.err().map(|f| f.to_string()).unwrap_or_default()
            );
            owner.trace.write().push(row);
            return;
        }
        ais_textured_shape_with_retirement(&**owner, self, |scope| {
            assert!(
                owner.base.retirement.is_active(),
                "scope open; expected an active stash"
            );
            routing_stage(&scope, self)
        })
        .unwrap()
        .unwrap();
        assert!(
            !owner.base.retirement.is_active(),
            "scope closed; expected no stash entry"
        );
        let nested = self.nested.observe();
        let row = format!(
            "DROP {} leave {} {} {}",
            self.mode,
            self.base.transient_base().ref_count(),
            i32::from(owner.texture_pix_map().is_null()),
            if nested.is_null() {
                -1
            } else {
                nested.ref_count()
            }
        );
        owner.trace.write().push(row);
    }
}
fn routing_stage(
    scope: &AisTexturedShapeRetirement<'_, RoutingShape>,
    pixels: &RoutingPixels,
) -> Result<(), Failure> {
    match pixels.mode {
        "file" => scope.set_texture_file_name(&AsciiString::from("inner.raw")),
        "pixmap" => scope.set_texture_pix_map(&*pixels.nested.observe()),
        _ => Ok(()),
    }
}
/// A retiring pixmap that calls `mode` ("none", "file", "pixmap") from its destructor.
/// Example: `routing_pixels(&owner, &nested, "file")`.
pub fn routing_pixels(
    owner: &Handle<RoutingShape>,
    nested: &Handle<ImagePixMap>,
    mode: &'static str,
) -> Handle<ImagePixMap> {
    Handle::new(RoutingPixels {
        base: ImagePixMap::default(),
        owner: owner.downgrade(),
        nested: nested.downgrade(),
        mode,
    })
    .into_cast()
}
