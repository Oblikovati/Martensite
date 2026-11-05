//! `XCAFDocShapeMapTool`: the map of the sub-shapes of a shape label.
use super::support::find_or_add;
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkcdf::json_dump::field_pointer;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::{Failure, Guid, Handle, SharedState, StandardTransient};
use martensite_tklcaf::guid_from_text;
use martensite_tklcaf::json::dump_attribute_json_base;
use martensite_tklcaf::tdf::{TdfAttribute, TdfAttributeBase, TdfLabel, TdfRelocationTable};
use martensite_tkservice::json_dump::class_begin;
use std::io;

/// `XCAFDocShapeMapTool::GetID()`.
const SHAPE_MAP_TOOL_ID: Guid = guid_from_text("3B913F4D-4A82-44ef-A0BF-9E01E9FF317A");

/// `XCAFDocShapeMapTool`.
///
/// ```
/// use martensite_tkbrep::topods::{Builder, Compound};
/// use martensite_tklcaf::tdf::TdfData;
/// use martensite_tkxcaf::xcaf_doc::XCAFDocShapeMapTool;
/// let data = TdfData::new();
/// let tool = XCAFDocShapeMapTool::set(&data.root())?;
/// let mut compound = Compound::default();
/// Builder.make_compound(&mut compound);
/// tool.set_shape(&compound);
/// assert_eq!(tool.get_map().extent(), 0);
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
#[derive(Default)]
pub struct XCAFDocShapeMapTool {
    transient: StandardTransient,
    base: TdfAttributeBase,
    map: SharedState<IndexedMapOfShape>,
}
martensite_tkernel::impl_transient!(XCAFDocShapeMapTool, "XCAFDocShapeMapTool", dyn TdfAttribute, transient, interfaces: [dyn TdfAttribute]);

/// `AddSubShape`: the shape and, depth first, every sub-shape.
fn add_sub_shape(shape: &Shape, map: &mut IndexedMapOfShape) {
    map.add(shape.clone());
    let mut iterator = ShapeIterator::new(shape, true, true);
    while let Ok(child) = iterator.value() {
        add_sub_shape(child, map);
        iterator.next();
    }
}

impl XCAFDocShapeMapTool {
    /// `XCAFDocShapeMapTool()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `GetID`.
    pub fn get_id() -> Guid {
        SHAPE_MAP_TOOL_ID
    }
    /// `Set(L)`: the tool of the label, added when absent.
    pub fn set(label: &TdfLabel) -> Result<Handle<XCAFDocShapeMapTool>, Failure> {
        find_or_add(label, &SHAPE_MAP_TOOL_ID, Self::new)
    }
    /// `IsSubShape(sub)`: the shape is in the map.
    pub fn is_sub_shape(&self, sub: &Shape) -> bool {
        self.map.read().contains(sub)
    }
    /// `SetShape(S)`: the map of the sub-shapes of the shape (not the shape itself); not
    /// backed up, as natively.
    pub fn set_shape(&self, shape: &Shape) {
        let mut map = IndexedMapOfShape::default();
        let mut iterator = ShapeIterator::new(shape, true, true);
        while let Ok(child) = iterator.value() {
            add_sub_shape(child, &mut map);
            iterator.next();
        }
        *self.map.write() = map;
    }
    /// `GetMap`: a copy of the map.
    pub fn get_map(&self) -> IndexedMapOfShape {
        self.map.read().clone()
    }
}

impl TdfAttribute for XCAFDocShapeMapTool {
    fn attribute_base(&self) -> &TdfAttributeBase {
        &self.base
    }
    fn id(&self) -> Guid {
        SHAPE_MAP_TOOL_ID
    }
    /// `Restore`: does nothing.
    fn restore(&self, _with: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
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
    /// `DumpJson`: the address of the TShape of each mapped shape.
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        class_begin(stream, "XCAFDocShapeMapTool")?;
        dump_attribute_json_base(self, stream, depth)?;
        let map = self.get_map();
        for shape in map.iter() {
            field_pointer(stream, "aShape", shape.tshape().object_address())?;
        }
        Ok(())
    }
}
