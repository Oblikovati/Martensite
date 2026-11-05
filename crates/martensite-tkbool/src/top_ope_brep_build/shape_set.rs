//! `TopOpeBRepBuildShapeSet`: the elements (edges of a face, vertices of an edge) of
//! the shapes being built, with the start elements and the neighbours of an element
//! through their sub-shapes. The virtual members overridden by the derived sets
//! (`TopOpeBRepBuildWireEdgeSet`, ...) are trait methods.
use super::dump_name_stream::{DefaultShapeSetDumpStream, ShapeSetDumpStream};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::{IndexedDataMap, IndexedMap, List};
use martensite_tkernel::standard::Failure;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use std::io::Write;

mod extension;
mod extension_ref;
pub(crate) mod traversal;
pub use extension::ShapeSetExtensionState;
pub use extension_ref::ShapeSetExtensionStateRef;
use traversal::{IncidentSource, ShapeSetTraversal, active_incident_source, cursor_value};

/// The data of a shape set (the protected members of `TopOpeBRepBuildShapeSet`).
pub struct TopOpeBRepBuildShapeSetBase {
    /// Shape kind selected by an extending set; `base.my_shape_type = ShapeEnum::Face`.
    pub my_shape_type: ShapeEnum,
    /// Subshape kind traversed by extending sets; `base.my_sub_shape_type = ShapeEnum::Vertex`.
    pub my_sub_shape_type: ShapeEnum,
    pub(crate) traversal: ShapeSetTraversal,
    pub(crate) my_start_shapes: List<Shape>,
    /// Indexed candidates for a derived neighbour selector; `base.my_sub_shape_map.find_from_key(&vertex)?`.
    pub my_sub_shape_map: IndexedDataMap<Shape, List<Shape>, ShapeMapHasher>,
    pub(crate) my_shapes: List<Shape>,
    /// Shape selected by the base traversal; `base.my_current_shape.is_same(&edge)`.
    pub my_current_shape: Shape,
    /// Result buffer retained by a derived selector; `base.my_current_shape_neighbours.clear()`.
    pub my_current_shape_neighbours: List<Shape>,
    pub(crate) my_deb_number: i32,
    pub(crate) my_deb_name: AsciiString,
    /// Oriented start-entry index; replay keeps prior starts; `base.my_omss.clear(true)`.
    pub my_omss: IndexedMap<Shape>,
    /// Oriented element index; replay keeps prior incidence; `base.my_omes.clear(true)`.
    pub my_omes: IndexedMap<Shape>,
    /// Oriented shape index; replay keeps prior shapes; `base.my_omsh.clear(true)`.
    pub my_omsh: IndexedMap<Shape>,
    pub(crate) my_check_shape: bool,
}

impl TopOpeBRepBuildShapeSetBase {
    /// `TopOpeBRepBuildShapeSet(SubShapeType, checkshape)`: faces of edges or edges of
    /// vertices; StandardProgramError for another type; the shape check is always off
    /// ("temporary NYI" natively).
    pub fn new(sub_shape_type: ShapeEnum, _checkshape: bool) -> Result<Self, Failure> {
        let my_shape_type = match sub_shape_type {
            ShapeEnum::Edge => ShapeEnum::Face,
            ShapeEnum::Vertex => ShapeEnum::Edge,
            _ => {
                return Err(Failure::with_type(
                    "Standard_ProgramError",
                    "ShapeSet : bad ShapeType",
                    "TopOpeBRepBuildShapeSet",
                ));
            }
        };
        Ok(Self {
            my_shape_type,
            my_sub_shape_type: sub_shape_type,
            traversal: ShapeSetTraversal::default(),
            my_start_shapes: List::default(),
            my_sub_shape_map: IndexedDataMap::default(),
            my_shapes: List::default(),
            my_current_shape: Shape::default(),
            my_current_shape_neighbours: List::default(),
            my_deb_number: 0,
            my_deb_name: AsciiString::default(),
            my_omss: IndexedMap::default(),
            my_omes: IndexedMap::default(),
            my_omsh: IndexedMap::default(),
            my_check_shape: false,
        })
    }

    /// The sub-shapes of the type of the set in a shape.
    pub(crate) fn sub_shapes(&self, s: &Shape) -> Result<Vec<Shape>, Failure> {
        Ok(Explorer::new(s, self.my_sub_shape_type, ShapeEnum::Shape)?.collect())
    }
}

/// A set of shapes and their elements (`TopOpeBRepBuildShapeSet`); the derived sets
/// override the virtual members; `ss.init_start_elements(); while ss.more_start_elements() { ss.next_start_element(); }`.
pub trait TopOpeBRepBuildShapeSet {
    /// The data of the set.
    fn shape_set_base(&self) -> &TopOpeBRepBuildShapeSetBase;

    /// The data of the set, changed.
    fn shape_set_base_mut(&mut self) -> &mut TopOpeBRepBuildShapeSetBase;

    /// `AddShape(S)` (virtual): a shape of the set (once), when it passes the check;
    /// `ss.add_shape(&wire)?`.
    fn add_shape(&mut self, s: &Shape) -> Result<(), Failure> {
        if !self.check_shape_of(s, false)? {
            return Ok(());
        }
        self.process_add_shape(s);
        Ok(())
    }

    /// `AddStartElement(S)` (virtual): a start element, also an element;
    /// `ss.add_start_element(&edge)?`.
    fn add_start_element(&mut self, s: &Shape) -> Result<(), Failure> {
        if !self.check_shape_of(s, false)? {
            return Ok(());
        }
        self.process_add_start_element(s)
    }

    /// `AddElement(S)` (virtual): an element, indexed by its sub-shapes;
    /// `ss.add_element(&edge)?`.
    fn add_element(&mut self, s: &Shape) -> Result<(), Failure> {
        if !self.check_shape_of(s, false)? {
            return Ok(());
        }
        self.process_add_element(s)
    }

    /// `ProcessAddShape(S)` (protected): the shape appended unless it is there (with its
    /// orientation).
    fn process_add_shape(&mut self, s: &Shape) {
        let base = self.shape_set_base_mut();
        if !base.my_omsh.contains(s) {
            base.my_omsh.add(s.clone());
            base.my_shapes.append(s.clone());
        }
    }

    /// `ProcessAddStartElement(S)` (protected).
    fn process_add_start_element(&mut self, s: &Shape) -> Result<(), Failure> {
        let base = self.shape_set_base_mut();
        if base.my_omss.contains(s) {
            return Ok(());
        }
        base.my_omss.add(s.clone());
        base.my_start_shapes.append(s.clone());
        self.process_add_element(s)
    }

    /// `ProcessAddElement(S)` (protected): the element appended to the list of each of
    /// its sub-shapes.
    fn process_add_element(&mut self, s: &Shape) -> Result<(), Failure> {
        let subs = self.shape_set_base().sub_shapes(s)?;
        let base = self.shape_set_base_mut();
        if base.my_omes.contains(s) {
            return Ok(());
        }
        base.my_omes.add(s.clone());
        for subshape in subs {
            if !base.my_sub_shape_map.contains(&subshape) {
                base.my_sub_shape_map.add(subshape.clone(), List::default());
            }
            base.my_sub_shape_map
                .change_from_key(&subshape)?
                .append(s.clone());
        }
        Ok(())
    }

    /// `StartElements()`; `ss.start_elements()`.
    fn start_elements(&self) -> &List<Shape> {
        &self.shape_set_base().my_start_shapes
    }

    /// `InitShapes()`; `ss.init_shapes()`.
    fn init_shapes(&mut self) {
        let base = self.shape_set_base_mut();
        base.traversal.shapes_owner = None;
        base.traversal.shapes = base.my_shapes.cursor();
    }

    /// `MoreShapes()`; `ss.more_shapes()`.
    fn more_shapes(&self) -> bool {
        self.shape_set_base().traversal.shapes.more()
    }

    /// `NextShape()`; `ss.next_shape()`.
    fn next_shape(&mut self) {
        let base = self.shape_set_base_mut();
        let mut cursor = base.traversal.shapes;
        cursor.next(base.shape_iteration_source());
        base.traversal.shapes = cursor;
    }

    /// `Shape()`; `ss.shape()?`.
    fn shape(&self) -> Result<&Shape, Failure> {
        let base = self.shape_set_base();
        cursor_value(
            base.traversal.shapes,
            base.shape_iteration_source(),
            "TopOpeBRepBuildShapeSet::Shape",
        )
    }

    /// `InitStartElements()`; `ss.init_start_elements()`.
    fn init_start_elements(&mut self) {
        let base = self.shape_set_base_mut();
        base.traversal.starts_owner = None;
        base.traversal.starts = base.my_start_shapes.cursor();
    }

    /// `MoreStartElements()`; `ss.more_start_elements()`.
    fn more_start_elements(&self) -> bool {
        self.shape_set_base().traversal.starts.more()
    }

    /// `NextStartElement()`; `ss.next_start_element()`.
    fn next_start_element(&mut self) {
        let base = self.shape_set_base_mut();
        let mut cursor = base.traversal.starts;
        cursor.next(base.start_iteration_source());
        base.traversal.starts = cursor;
    }

    /// `StartElement()`; `ss.start_element()?`.
    fn start_element(&self) -> Result<&Shape, Failure> {
        let base = self.shape_set_base();
        cursor_value(
            base.traversal.starts,
            base.start_iteration_source(),
            "TopOpeBRepBuildShapeSet::StartElement",
        )
    }

    /// `InitNeighbours(S)` (virtual): the neighbours of `s` through its first sub-shape
    /// having some; `ss.init_neighbours(&edge)?`.
    fn init_neighbours(&mut self, s: &Shape) -> Result<(), Failure> {
        let kind = self.shape_set_base().my_sub_shape_type;
        let explorer = Explorer::new(s, kind, ShapeEnum::Shape)?;
        let base = self.shape_set_base_mut();
        base.traversal.explorer = explorer;
        base.my_current_shape = s.clone();
        self.find_neighbours()
    }

    /// `MoreNeighbours()`; `ss.more_neighbours()`.
    fn more_neighbours(&self) -> bool {
        self.shape_set_base().traversal.incident.more()
    }

    /// `NextNeighbour()`: the next neighbour, through the next sub-shapes when those of
    /// this one are exhausted; `ss.next_neighbour()?`.
    fn next_neighbour(&mut self) -> Result<(), Failure> {
        let mut cursor = self.shape_set_base().traversal.incident;
        cursor.next(active_incident_source(self)?);
        let base = self.shape_set_base_mut();
        base.traversal.incident = cursor;
        if !base.traversal.incident.more() && base.traversal.explorer.more() {
            base.traversal.explorer.next()?;
            return self.find_neighbours();
        }
        Ok(())
    }

    /// `Neighbour()`; `ss.neighbour()?`.
    fn neighbour(&self) -> Result<&Shape, Failure> {
        cursor_value(
            self.shape_set_base().traversal.incident,
            active_incident_source(self)?,
            "TopOpeBRepBuildShapeSet::Neighbour",
        )
    }

    /// `ChangeStartShapes()`; `ss.change_start_shapes().clear()`.
    fn change_start_shapes(&mut self) -> &mut List<Shape> {
        &mut self.shape_set_base_mut().my_start_shapes
    }

    /// `FindNeighbours()` (virtual): from the current sub-shape, the first with
    /// neighbours (MakeNeighboursList); `ss.find_neighbours()?`.
    fn find_neighbours(&mut self) -> Result<(), Failure> {
        while self.shape_set_base().traversal.explorer.more() {
            let v = self.shape_set_base().traversal.explorer.current().clone();
            let current = self.shape_set_base().my_current_shape.clone();
            self.shape_set_base_mut().traversal.source = IncidentSource::External;
            let cursor = self.borrowed_neighbours(&current, &v)?.cursor();
            let base = self.shape_set_base_mut();
            base.traversal.incident = cursor;
            if base.traversal.incident.more() {
                break;
            }
            base.traversal.explorer.next()?;
        }
        Ok(())
    }

    /// `MakeNeighboursList(E, V)` (virtual): the elements of the sub-shape `v`
    /// (StandardNoSuchObject for an unknown one); `ss.make_neighbours_list(&e, &v)?`.
    fn make_neighbours_list(&mut self, _e: &Shape, v: &Shape) -> Result<List<Shape>, Failure> {
        self.shape_set_base_mut().traversal.source = IncidentSource::Indexed(v.clone());
        Ok(self
            .shape_set_base()
            .my_sub_shape_map
            .find_from_key(v)?
            .clone())
    }

    /// Prepares and borrows the retained list; `let list = set.borrowed_neighbours(&edge, &vertex)?;`.
    /// External implementations overriding this method also override `neighbours_source`
    /// to return that same live list until the next preparation.
    fn borrowed_neighbours(
        &mut self,
        edge: &Shape,
        vertex: &Shape,
    ) -> Result<&List<Shape>, Failure> {
        self.shape_set_base_mut().traversal.source = IncidentSource::Owned;
        let legacy = self.make_neighbours_list(edge, vertex)?;
        self.shape_set_base_mut().traversal.owned = legacy;
        self.neighbours_source()
    }

    /// Borrows the list retained by the current cursor; `let list = set.neighbours_source()?;`.
    /// An external provider returns its own retained list here, preserving node identity.
    fn neighbours_source(&self) -> Result<&List<Shape>, Failure> {
        self.shape_set_base().incident_source()
    }

    /// `MaxNumberSubShape(Shape)`: the greatest element count of the known sub-shapes of
    /// the shape; `ss.max_number_sub_shape(&edge)?`.
    fn max_number_sub_shape(&self, shape: &Shape) -> Result<i32, Failure> {
        let base = self.shape_set_base();
        let mut m: i32 = 0;
        for sub in base.sub_shapes(shape)? {
            if let Some(l) = base.my_sub_shape_map.seek(&sub) {
                m = m.max(l.len() as i32);
            }
        }
        Ok(m)
    }

    /// `CheckShape(checkshape)`; `ss.set_check_shape(true)`.
    fn set_check_shape(&mut self, checkshape: bool) {
        self.shape_set_base_mut().my_check_shape = checkshape;
    }

    /// `CheckShape()`; `ss.check_shape()`.
    fn check_shape(&self) -> bool {
        self.shape_set_base().my_check_shape
    }

    /// `CheckShape(S, checkgeom)` (checkgeom natively defaulted to false): the validity
    /// of the shape when the check is on, else true; `ss.check_shape_of(&s, false)?`.
    fn check_shape_of(&self, s: &Shape, checkgeom: bool) -> Result<bool, Failure> {
        if !self.shape_set_base().my_check_shape {
            return Ok(true);
        }
        BRepCheckAnalyzer::new(s, checkgeom, false, false)?.is_valid()
    }

    /// Full-span dump convenience; `ss.dump_name(&mut out, "set")?`.
    /// Use `dump_name_ascii` for first-NUL stream semantics.
    fn dump_name(&self, os: &mut dyn Write, s: &str) -> std::io::Result<()> {
        let base = self.shape_set_base();
        write!(os, "{s}(")?;
        os.write_all(base.my_deb_name.as_bytes())?;
        write!(os, ",{})", base.my_deb_number)
    }

    /// `DumpCheck(OS, str, S, chk)`: nothing in a release build; `ss.dump_check(&mut out, "", &s, true)`.
    fn dump_check(&self, _os: &mut dyn Write, _s: &str, _shape: &Shape, _chk: bool) {}

    /// `DumpSS()` (virtual): nothing in a release build; `ss.dump_ss()`.
    fn dump_ss(&mut self) {}

    /// `DumpBB()` (virtual): nothing in a release build; `ss.dump_bb()`.
    fn dump_bb(&mut self) {}

    /// `DEBName(N)`; `ss.set_deb_name("set")`.
    fn set_deb_name(&mut self, n: &str) {
        self.shape_set_base_mut().my_deb_name = AsciiString::from_raw(n.as_bytes());
    }

    /// The UTF-8 name convenience; `ss.deb_name()`.
    ///
    /// Panics for non-UTF-8 bytes; use `try_deb_name` or `deb_name_ascii` for those states.
    fn deb_name(&self) -> &str {
        self.try_deb_name().unwrap_or_else(|error| {
            panic!(
                "ShapeSet::deb_name: bytes {:?}; expected valid UTF-8 ({error})",
                self.shape_set_base().my_deb_name.as_bytes()
            )
        })
    }

    /// Checks the UTF-8 view without replacing bytes; `ss.try_deb_name()?`.
    fn try_deb_name(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.shape_set_base().my_deb_name.as_bytes())
    }

    /// Copies every name byte, including NULs; `ss.set_deb_name_ascii(&name)`.
    fn set_deb_name_ascii(&mut self, name: &AsciiString) {
        self.shape_set_base_mut().my_deb_name.clone_from(name);
    }

    /// Borrows the complete byte name; `ss.deb_name_ascii().as_bytes()`.
    fn deb_name_ascii(&self) -> &AsciiString {
        &self.shape_set_base().my_deb_name
    }

    /// Edits the stored byte name directly; `ss.change_deb_name_ascii().append(&tail)`.
    fn change_deb_name_ascii(&mut self) -> &mut AsciiString {
        &mut self.shape_set_base_mut().my_deb_name
    }

    /// `DumpName(OS, str)`: inserts the prefix, stored name, punctuation, and debug
    /// number into a formatting-aware stream; `ss.dump_name_stream(&mut stream, &prefix)?`.
    fn dump_name_stream(
        &self,
        stream: &mut dyn ShapeSetDumpStream,
        prefix: &AsciiString,
    ) -> std::io::Result<()> {
        let base = self.shape_set_base();
        stream.insert_ascii(prefix)?;
        stream.insert_literal(b"(")?;
        stream.insert_ascii(&base.my_deb_name)?;
        stream.insert_literal(b",")?;
        stream.insert_i32(base.my_deb_number)?;
        stream.insert_literal(b")")
    }

    /// Dumps first-NUL prefix and name bytes with the default decimal stream state;
    /// `ss.dump_name_ascii(&mut out, &prefix)?`.
    fn dump_name_ascii(&self, os: &mut dyn Write, prefix: &AsciiString) -> std::io::Result<()> {
        let mut stream = DefaultShapeSetDumpStream::new(os);
        self.dump_name_stream(&mut stream, prefix)
    }

    /// `DEBNumber(I)`; `ss.set_deb_number(2)`.
    fn set_deb_number(&mut self, i: i32) {
        self.shape_set_base_mut().my_deb_number = i;
    }

    /// `DEBNumber()`; `ss.deb_number()`.
    fn deb_number(&self) -> i32 {
        self.shape_set_base().my_deb_number
    }

    /// `SName(S, sb, sa)` (virtual): empty in a release build; `ss.sname(&s, "", "")`.
    fn sname(&self, _s: &Shape, _sb: &str, _sa: &str) -> String {
        String::new()
    }

    /// `SNameori(S, sb, sa)` (virtual): empty in a release build; `ss.snameori(&s, "", "")`.
    fn snameori(&self, _s: &Shape, _sb: &str, _sa: &str) -> String {
        String::new()
    }

    /// `SName(L, sb, sa)` (virtual): empty in a release build; `ss.sname_list(&l, "", "")`.
    fn sname_list(&self, _l: &List<Shape>, _sb: &str, _sa: &str) -> String {
        String::new()
    }

    /// `SNameori(L, sb, sa)` (virtual): empty in a release build; `ss.snameori_list(&l, "", "")`.
    fn snameori_list(&self, _l: &List<Shape>, _sb: &str, _sa: &str) -> String {
        String::new()
    }
}

/// The plain shape set (the concrete `TopOpeBRepBuildShapeSet`);
/// `let ss = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Edge, true)?;`.
#[derive(Clone)]
pub struct TopOpeBRepBuildPlainShapeSet {
    base: TopOpeBRepBuildShapeSetBase,
}

impl TopOpeBRepBuildPlainShapeSet {
    /// `TopOpeBRepBuildShapeSet(SubShapeType, checkshape)` (checkshape natively
    /// defaulted to true); `TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Edge, true)?`.
    pub fn new(sub_shape_type: ShapeEnum, checkshape: bool) -> Result<Self, Failure> {
        Ok(Self {
            base: TopOpeBRepBuildShapeSetBase::new(sub_shape_type, checkshape)?,
        })
    }
}

impl TopOpeBRepBuildShapeSet for TopOpeBRepBuildPlainShapeSet {
    fn shape_set_base(&self) -> &TopOpeBRepBuildShapeSetBase {
        &self.base
    }

    fn shape_set_base_mut(&mut self) -> &mut TopOpeBRepBuildShapeSetBase {
        &mut self.base
    }
}
