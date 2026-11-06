//! `TNamingUsedShapes`: the root attribute that maps every named shape of a framework to
//! its `TNamingRefShape`.
use super::TNamingRefShape;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkcdf::json_dump::class_begin;
use martensite_tkernel::{
    impl_transient,
    ncollection::DataMap,
    standard::{
        Failure, Guid, Handle, RuntimeTyped, SharedState, StandardTransient, dump::DumpStream,
    },
};
use martensite_tklcaf::guid_from_text;
use martensite_tklcaf::json::dump_attribute_json_base;
use martensite_tklcaf::tdf::{
    TdfAttribute, TdfAttributeBase, TdfAttributeDelta, TdfDataSet, TdfDeltaOnAddition,
    TdfDeltaOnRemoval, TdfRelocationTable,
};
use std::io;
use std::sync::{Arc, RwLockWriteGuard};

/// `TNamingUsedShapes::GetID()`.
const USED_SHAPES_ID: Guid = guid_from_text("c4ef4201-568f-11d1-8940-080009dc3333");

/// `NCollectionDataMap<TopoDSShape, TNamingRefShape*, TopToolsShapeMapHasher>`.
pub type TNamingShapeRefMap = DataMap<Shape, Arc<TNamingRefShape>, ShapeMapHasher>;

/// `TNamingUsedShapes`: created on the root label by the first `TNamingBuilder`.
///
/// ```
/// use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingUsedShapes};
/// use martensite_tkernel::standard::Handle;
/// use martensite_tklcaf::tdf::TdfData;
/// let data = TdfData::new();
/// TNamingBuilder::new(&data.root()).unwrap();
/// let mut used: Handle<TNamingUsedShapes> = Handle::default();
/// assert!(data.root().find_attribute_as(&TNamingUsedShapes::get_id(), &mut used).unwrap());
/// assert!(used.map().is_empty());
/// ```
pub struct TNamingUsedShapes {
    transient: StandardTransient,
    base: TdfAttributeBase,
    map: SharedState<TNamingShapeRefMap>,
}
impl_transient!(TNamingUsedShapes, "TNamingUsedShapes", dyn TdfAttribute, transient, interfaces: [dyn TdfAttribute]);

impl TNamingUsedShapes {
    /// `GetID`.
    pub fn get_id() -> Guid {
        USED_SHAPES_ID
    }
    /// The private constructor (reached by `TNamingBuilder` and `NewEmpty`).
    pub(crate) fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: TdfAttributeBase::default(),
            map: SharedState::new(TNamingShapeRefMap::default()),
        }
    }
    /// `Map()`: the map itself; drop the guard before calling back into the framework.
    pub fn map(&self) -> RwLockWriteGuard<'_, TNamingShapeRefMap> {
        self.map.write()
    }
    /// `Destroy`: clears the map (the ref shapes are not deleted natively; the nodes
    /// still reach them).
    pub fn destroy(&self) {
        let released = std::mem::take(&mut *self.map.write());
        drop(released);
    }
}

impl Drop for TNamingUsedShapes {
    /// `~TNamingUsedShapes`: `Destroy()`.
    fn drop(&mut self) {
        self.destroy();
    }
}

impl TdfAttribute for TNamingUsedShapes {
    fn attribute_base(&self) -> &TdfAttributeBase {
        &self.base
    }
    fn id(&self) -> Guid {
        USED_SHAPES_ID
    }
    /// `BackupCopy`: a null handle (the map is never backed up).
    fn backup_copy(&self) -> Result<Handle<dyn TdfAttribute>, Failure> {
        Ok(Handle::default())
    }
    /// `Restore`: does nothing.
    fn restore(&self, _attribute: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        Ok(())
    }
    /// `BeforeRemoval`: `Destroy()`.
    fn before_removal(&self) -> Result<(), Failure> {
        self.destroy();
        Ok(())
    }
    /// `AfterUndo`: an undone addition calls the attribute's `BeforeRemoval`.
    fn after_undo(
        &self,
        delta: &Handle<dyn TdfAttributeDelta>,
        _force_it: bool,
    ) -> Result<bool, Failure> {
        if delta.is_kind(TdfDeltaOnAddition::static_type()) {
            let delta = martensite_tklcaf::live(delta, "TNamingUsedShapes::AfterUndo")?;
            let attribute = delta.attribute();
            martensite_tklcaf::live(&attribute, "TNamingUsedShapes::AfterUndo")?
                .before_removal()?;
        }
        Ok(true)
    }
    /// `DeltaOnAddition`: a null delta (nothing to undo).
    fn delta_on_addition(&self) -> Result<Handle<TdfDeltaOnAddition>, Failure> {
        Ok(Handle::default())
    }
    /// `DeltaOnRemoval`: a null delta.
    fn delta_on_removal(&self) -> Result<Handle<dyn TdfDeltaOnRemoval>, Failure> {
        Ok(Handle::default())
    }
    fn new_empty(&self) -> Handle<dyn TdfAttribute> {
        Handle::new(Self::new()).cast()
    }
    /// `Paste`: does nothing.
    fn paste(
        &self,
        _into: &Handle<dyn TdfAttribute>,
        _relocation_table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// `References`: adds nothing.
    fn references(&self, _data_set: &Handle<TdfDataSet>) {}
    /// `Dump`: one line per shape of the map, in map order.
    fn dump(&self, stream: &mut dyn io::Write) -> io::Result<()> {
        writeln!(stream, "The content of UsedShapes attribute:")?;
        let entries: Vec<(Shape, Arc<TNamingRefShape>)> = self
            .map
            .read()
            .items()
            .map(|(shape, reference)| (shape.clone(), reference.clone()))
            .collect();
        for (shape, reference) in &entries {
            dump_used_shape(stream, shape, reference)?;
        }
        Ok(())
    }
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        class_begin(stream, "TNamingUsedShapes")?;
        dump_attribute_json_base(self, stream, depth)
    }
}

/// One line of `TNamingUsedShapes::Dump`.
fn dump_used_shape(
    stream: &mut dyn io::Write,
    shape: &Shape,
    reference: &TNamingRefShape,
) -> io::Result<()> {
    let label = reference.label().map_err(io::Error::other)?;
    if shape.is_null() {
        write!(stream, "Empty Shape at label =")?;
        label.entry_dump(stream)?;
        return writeln!(stream);
    }
    write!(stream, "  ")?;
    let shape_type = shape.shape_type().map_err(io::Error::other)?;
    martensite_tkg3d::top_abs::print_shape_type(shape_type, &mut &mut *stream)?;
    write!(stream, "  ")?;
    label.entry_dump(stream)?;
    write!(
        stream,
        " Key_TShape   = {:p}",
        shape.tshape().object_address()
    )?;
    let value = reference.shape();
    writeln!(
        stream,
        " Value_TShape = {:p}",
        value.tshape().object_address()
    )
}
