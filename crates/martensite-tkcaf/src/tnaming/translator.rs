//! `TNamingTranslator`: copies a set of shapes with `TNamingCopyShape`, sharing the
//! copies of their common TShapes and datums.
use super::{TNamingCopyShape, TNamingTransientMap};
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::toploc::Datum3;
use std::io;

/// `NCollectionDataMap<TopoDSShape, TopoDSShape, TopToolsShapeMapHasher>`.
pub type TNamingShapeShapeMap = DataMap<Shape, Shape, ShapeMapHasher>;

/// The TShape classes `DumpMap` recognizes.
const TSHAPE_TYPES: [&str; 8] = [
    "BRep_TVertex",
    "BRep_TEdge",
    "BRep_TFace",
    "TopoDS_TWire",
    "TopoDS_TShell",
    "TopoDS_TSolid",
    "TopoDS_TCompSolid",
    "TopoDS_TCompound",
];

/// `TNamingTranslator`.
///
/// ```
/// use martensite_tkbrep::topods::Shape;
/// use martensite_tkcaf::tnaming::TNamingTranslator;
/// let mut translator = TNamingTranslator::new();
/// translator.perform().unwrap();
/// assert!(!translator.is_done());
/// assert!(translator.copied_shape(&Shape::default()).is_null());
/// ```
#[derive(Default)]
pub struct TNamingTranslator {
    is_done: bool,
    map: TNamingTransientMap,
    results: TNamingShapeShapeMap,
}

impl TNamingTranslator {
    /// `TNamingTranslator()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `Add(aShape)`: a shape to copy (its result starts null).
    pub fn add(&mut self, shape: &Shape) {
        self.results.bind(shape.clone(), Shape::default());
    }
    /// `Perform`: copies every added shape in map order; done when there is one.
    pub fn perform(&mut self) -> Result<(), Failure> {
        let shapes: Vec<Shape> = self
            .results
            .items()
            .map(|(shape, _)| shape.clone())
            .collect();
        for shape in &shapes {
            let mut result = Shape::default();
            TNamingCopyShape::copy_tool(shape, &mut self.map, &mut result)?;
            if !result.is_null() {
                *self.results.change_find(shape)? = result;
            }
        }
        if !self.results.is_empty() {
            self.is_done = true;
        }
        Ok(())
    }
    /// `IsDone`.
    pub fn is_done(&self) -> bool {
        self.is_done
    }
    /// `Copied(aShape)`: the copy of an added shape, null otherwise.
    pub fn copied_shape(&self, shape: &Shape) -> Shape {
        self.results.seek(shape).cloned().unwrap_or_default()
    }
    /// `Copied()`: every added shape with its copy.
    pub fn copied(&self) -> &TNamingShapeShapeMap {
        &self.results
    }
    /// `DumpMap(isWrite)` (on `std::cout` natively): the extent and the type of each
    /// translated object; with `isWrite`, the datums are dumped. Writing the shapes uses
    /// `BRepTools::Write`, a native file format that is not implemented (user decision 2026-09):
    /// that request is an `Unsupported` error.
    pub fn dump_map(&self, stream: &mut dyn io::Write, is_write: bool) -> io::Result<()> {
        if self.map.is_empty() {
            return Ok(());
        }
        writeln!(
            stream,
            "TNamingTranslator:: IndexedDataMap Extent = {}",
            self.map.extent()
        )?;
        for (key, item, index) in self.map.indexed_items() {
            dump_entry(stream, key, item, index, is_write)?;
        }
        Ok(())
    }
}

/// One entry of `DumpMap`.
fn dump_entry(
    stream: &mut dyn io::Write,
    key: &Handle<StandardTransient>,
    item: &Handle<StandardTransient>,
    index: usize,
    is_write: bool,
) -> io::Result<()> {
    let type_name = key.dynamic_type().map_or("", |t| t.name());
    writeln!(
        stream,
        "TNamingTranslator::DumpMap:  Index = {index} Type = {type_name}"
    )?;
    if TSHAPE_TYPES.contains(&type_name) {
        if is_write {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                format!(
                    "TNamingTranslator::DumpMap: writing the shapes of index {index} needs BRepTools::Write, a native file format that is not implemented"
                ),
            ));
        }
        return Ok(());
    }
    if type_name == "TopLoc_Datum3D" {
        if is_write {
            dump_datums(stream, key, item, index)?;
        }
        return Ok(());
    }
    writeln!(
        stream,
        "TNamingTranslator::DumpMap: Unexpected Type >> Idex = {index} Type = {type_name}"
    )
}

/// The datum branch of `DumpMap` with `isWrite`.
fn dump_datums(
    stream: &mut dyn io::Write,
    key: &Handle<StandardTransient>,
    item: &Handle<StandardTransient>,
    index: usize,
) -> io::Result<()> {
    writeln!(
        stream,
        "TNamingTranslator::DumpMap: Location_Key_name  = Map_Key{index}"
    )?;
    if let Some(datum) = key.cast::<Datum3>().get() {
        datum.shallow_dump(&mut &mut *stream)?;
    }
    writeln!(
        stream,
        "TNamingTranslator::DumpMap: Location_Item_name = Map_Item{index}"
    )?;
    if let Some(datum) = item.cast::<Datum3>().get() {
        datum.shallow_dump(&mut &mut *stream)?;
    }
    Ok(())
}
