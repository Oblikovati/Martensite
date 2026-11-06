//! `TPrsStdAISViewer`: the root attribute that holds the interactive context of a data
//! framework.
use super::TPrsStdAISPresentation;
use martensite_tkcaf::tdata_xtd::TDataXtdPresentation;
use martensite_tkcdf::json_dump::{class_begin, field_pointer};
use martensite_tkernel::{
    impl_transient,
    standard::{
        Failure, Guid, Handle, SharedState, StandardTransient, dump::DumpStream, exceptions,
    },
};
use martensite_tklcaf::guid_from_text;
use martensite_tklcaf::json::dump_attribute_json_base;
use martensite_tklcaf::tdf::{
    TdfAttribute, TdfAttributeBase, TdfChildIterator, TdfLabel, TdfRelocationTable,
};
use martensite_tkv3d::ais::AisInteractiveContext;
use martensite_tkv3d::v3d::V3dViewer;
use std::io;

/// `TPrsStdAISViewer::GetID()`.
const INTERACTIVE_VIEWER_ID: Guid = guid_from_text("04fb4d05-5690-11d1-8940-080009dc3333");

/// `TPrsStdAISViewer`.
///
/// ```
/// use martensite_tkvcaf::tprs_std::TPrsStdAISViewer;
/// use martensite_tklcaf::tdf::TdfData;
/// let data = TdfData::new();
/// assert!(!TPrsStdAISViewer::has(&data.root()).unwrap());
/// ```
pub struct TPrsStdAISViewer {
    transient: StandardTransient,
    base: TdfAttributeBase,
    interactive_context: SharedState<Handle<AisInteractiveContext>>,
}
impl_transient!(TPrsStdAISViewer, "TPrsStdAISViewer", dyn TdfAttribute, transient, interfaces: [dyn TdfAttribute]);

impl Default for TPrsStdAISViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl TPrsStdAISViewer {
    /// `TPrsStdAISViewer()`: no interactive context.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: TdfAttributeBase::default(),
            interactive_context: SharedState::new(Handle::default()),
        }
    }
    /// `GetID`.
    pub fn get_id() -> Guid {
        INTERACTIVE_VIEWER_ID
    }
    /// `Has(acces)`: true when the root of the data framework holds a viewer attribute.
    pub fn has(access: &TdfLabel) -> Result<bool, Failure> {
        access.root()?.is_attribute(&Self::get_id())
    }
    /// `New(acces, selector)`: sets a viewer attribute with the given context at the root,
    /// then a presentation at every label that holds a `TDataXtdPresentation`;
    /// `StandardDomainError` when the root has a viewer already.
    pub fn new_with_context(
        access: &TdfLabel,
        context: &Handle<AisInteractiveContext>,
    ) -> Result<Handle<TPrsStdAISViewer>, Failure> {
        let root = viewer_free_root(access)?;
        install_viewer(&root, context)
    }
    /// `New(acces, viewer)`: as [`Self::new_with_context`], with a new interactive context
    /// over the viewer.
    pub fn new_with_viewer(
        access: &TdfLabel,
        viewer: &Handle<V3dViewer>,
    ) -> Result<Handle<TPrsStdAISViewer>, Failure> {
        let root = viewer_free_root(access)?;
        install_viewer(&root, &AisInteractiveContext::new(viewer)?)
    }
    /// `Find(acces, A)`: the viewer attribute of the root.
    pub fn find(access: &TdfLabel, viewer: &mut Handle<TPrsStdAISViewer>) -> Result<bool, Failure> {
        access.root()?.find_attribute_as(&Self::get_id(), viewer)
    }
    /// `Find(acces, IC)`: the interactive context of the root's viewer attribute.
    pub fn find_context(
        access: &TdfLabel,
        context: &mut Handle<AisInteractiveContext>,
    ) -> Result<bool, Failure> {
        let mut viewer = Handle::default();
        if !Self::find(access, &mut viewer)? {
            return Ok(false);
        }
        *context = live_viewer(&viewer)?.get_interactive_context();
        Ok(true)
    }
    /// `Find(acces, V)`: the current viewer of the root's interactive context
    /// (`StandardNullObject` when the attribute has no context, a null dereference natively).
    pub fn find_viewer(access: &TdfLabel, viewer: &mut Handle<V3dViewer>) -> Result<bool, Failure> {
        let mut context = Handle::default();
        if !Self::find_context(access, &mut context)? {
            return Ok(false);
        }
        let context = martensite_tklcaf::live(&context, "TPrsStdAISViewer::Find")?;
        *viewer = context.current_viewer().clone();
        Ok(true)
    }
    /// `Update(acces)`: updates the viewer of the root's viewer attribute, if any.
    pub fn update_label(access: &TdfLabel) -> Result<(), Failure> {
        let mut viewer = Handle::default();
        if Self::find(access, &mut viewer)? {
            live_viewer(&viewer)?.update()?;
        }
        Ok(())
    }
    /// `Update()`: `UpdateCurrentViewer` of the context (`StandardNullObject` without a
    /// context, a null dereference natively).
    pub fn update(&self) -> Result<(), Failure> {
        let context = self.get_interactive_context();
        martensite_tklcaf::live(&context, "TPrsStdAISViewer::Update")?.update_current_viewer();
        Ok(())
    }
    /// `SetInteractiveContext(ctx)`.
    pub fn set_interactive_context(&self, context: &Handle<AisInteractiveContext>) {
        *self.interactive_context.write() = context.clone();
    }
    /// `GetInteractiveContext()`.
    pub fn get_interactive_context(&self) -> Handle<AisInteractiveContext> {
        self.interactive_context.read().clone()
    }
}

/// The live viewer attribute behind a found handle.
fn live_viewer(viewer: &Handle<TPrsStdAISViewer>) -> Result<&TPrsStdAISViewer, Failure> {
    martensite_tklcaf::live(viewer, "TPrsStdAISViewer")
}

/// The root of the data framework; `StandardDomainError` when it has a viewer already.
fn viewer_free_root(access: &TdfLabel) -> Result<TdfLabel, Failure> {
    let root = access.root()?;
    let mut viewer: Handle<TPrsStdAISViewer> = Handle::default();
    if root.find_attribute_as(&TPrsStdAISViewer::get_id(), &mut viewer)? {
        return Err(exceptions::domain_error("already done"));
    }
    Ok(root)
}

/// The rest of both `New`: the new attribute with its context at the root, then the
/// presentations below.
fn install_viewer(
    root: &TdfLabel,
    context: &Handle<AisInteractiveContext>,
) -> Result<Handle<TPrsStdAISViewer>, Failure> {
    let viewer = Handle::new(TPrsStdAISViewer::new());
    viewer.set_interactive_context(context);
    root.add_attribute(&viewer.clone().cast(), false)?;
    set_presentations_below(root)?;
    Ok(viewer)
}

/// The loop of both `New`: a presentation (driver of the data) at every label below the
/// root that holds a `TDataXtdPresentation`.
fn set_presentations_below(root: &TdfLabel) -> Result<(), Failure> {
    let mut iterator = TdfChildIterator::new(root, true)?;
    while iterator.more() {
        let label = iterator.value();
        let mut data: Handle<TDataXtdPresentation> = Handle::default();
        if label.find_attribute_as(&TDataXtdPresentation::get_id(), &mut data)? {
            let driver = martensite_tklcaf::live(&data, "TPrsStdAISViewer::New")?.get_driver_guid();
            TPrsStdAISPresentation::set(&label, &driver)?;
        }
        iterator.next();
    }
    Ok(())
}

impl TdfAttribute for TPrsStdAISViewer {
    fn attribute_base(&self) -> &TdfAttributeBase {
        &self.base
    }
    fn id(&self) -> Guid {
        INTERACTIVE_VIEWER_ID
    }
    /// `Restore`: does nothing.
    fn restore(&self, _attribute: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        Ok(())
    }
    fn new_empty(&self) -> Handle<dyn TdfAttribute> {
        Handle::new(Self::new()).cast()
    }
    /// `Paste`: does nothing.
    fn paste(
        &self,
        _into: &Handle<dyn TdfAttribute>,
        _table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        Ok(())
    }
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        class_begin(stream, "TPrsStdAISViewer")?;
        dump_attribute_json_base(self, stream, depth)?;
        let context = self.get_interactive_context();
        field_pointer(
            stream,
            "myInteractiveContext.get()",
            context.object_address(),
        )
    }
}
