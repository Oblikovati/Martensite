//! `BRepOffsetAPIFindContigousEdges`: the contiguous edges of shapes, found by a
//! `BRepBuilderAPISewing` that analyses without sewing.
use martensite_tkbrep::topods::{Edge, Shape};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPISewing;
use std::io::Write;

/// `BRepOffsetAPIFindContigousEdges`: adds shapes, then `perform` finds the pairs of
/// edges within the tolerance.
///
/// ```
/// use martensite_tkoffset::brep_offset_api::BRepOffsetAPIFindContigousEdges;
/// let finder = BRepOffsetAPIFindContigousEdges::new(1.0e-6, true);
/// assert_eq!(finder.nb_contigous_edges(), 0);
/// ```
pub struct BRepOffsetAPIFindContigousEdges {
    my_sewing: BRepBuilderAPISewing,
}

impl Default for BRepOffsetAPIFindContigousEdges {
    fn default() -> Self {
        Self::new(1.0e-06, true)
    }
}

impl BRepOffsetAPIFindContigousEdges {
    /// `BRepOffsetAPIFindContigousEdges(tolerance, option)` (usually
    /// 1e-6 and the analysis of degenerated shapes);
    /// `BRepOffsetAPIFindContigousEdges::new(1e-6, true)`.
    pub fn new(tolerance: f64, option: bool) -> Self {
        let mut finder = Self {
            my_sewing: BRepBuilderAPISewing::default(),
        };
        finder.init(tolerance, option);
        finder
    }

    /// A sewing that analyses (with `option`) and cuts but
    /// does not sew; `finder.init(1e-6, true)`.
    pub fn init(&mut self, tolerance: f64, option: bool) {
        self.my_sewing.init(tolerance, option, false, true, false);
    }

    /// A shape to analyse; `finder.add(&shape)?`.
    pub fn add(&mut self, shape: &Shape) -> Result<(), Failure> {
        self.my_sewing.add(shape)
    }

    /// The analysis; `finder.perform()?`.
    pub fn perform(&mut self) -> Result<(), Failure> {
        self.my_sewing.perform(&ProgressRange::default())
    }

    /// An empty one; `finder.nb_contigous_edges()`.
    pub fn nb_contigous_edges(&self) -> i32 {
        self.my_sewing.nb_contigous_edges()
    }

    /// Out_of_range outside `0..=NbContigousEdges()`;
    /// `finder.contigous_edge(1)?`.
    pub fn contigous_edge(&self, index: i32) -> Result<Edge, Failure> {
        self.check_index(index, self.nb_contigous_edges(), "ContigousEdge")?;
        self.my_sewing.contigous_edge(index)
    }

    /// The edges merged into a contiguous edge;
    /// `finder.contigous_edge_couple(1)?`.
    pub fn contigous_edge_couple(&self, index: i32) -> Result<&List<Shape>, Failure> {
        self.check_index(index, self.nb_contigous_edges(), "ContigousEdgeCouple")?;
        self.my_sewing.contigous_edge_couple(index)
    }

    /// The boundary edge of a section; no_such_object
    /// for an unbound section; `finder.section_to_boundary(&edge)?`.
    pub fn section_to_boundary(&self, section: &Edge) -> Result<Edge, Failure> {
        if !self.my_sewing.is_section_bound(section) {
            return Err(Failure::with_message(
                "Standard_NoSuchObject",
                "BRepOffsetAPI_FindContigousEdges::SectionToBoundary",
            ));
        }
        self.my_sewing.section_to_boundary(section)
    }

    /// An empty one; `finder.nb_degenerated_shapes()`.
    pub fn nb_degenerated_shapes(&self) -> i32 {
        self.my_sewing.nb_degenerated_shapes()
    }

    /// Out_of_range outside
    /// `0..=NbDegeneratedShapes()`; `finder.degenerated_shape(1)?`.
    pub fn degenerated_shape(&self, index: i32) -> Result<&Shape, Failure> {
        self.check_index(index, self.nb_degenerated_shapes(), "DegereratedShape")?;
        self.my_sewing.degenerated_shape(index)
    }

    /// `is_degenerated(shape)`; `finder.is_degenerated(&shape)?`.
    pub fn is_degenerated(&self, shape: &Shape) -> Result<bool, Failure> {
        self.my_sewing.is_degenerated(shape)
    }

    /// `is_modified(shape)`; `finder.is_modified(&shape)`.
    pub fn is_modified(&self, shape: &Shape) -> bool {
        self.my_sewing.is_modified(shape)
    }

    /// No_such_object for an unmodified shape;
    /// `finder.modified(&shape)?`.
    pub fn modified(&self, shape: &Shape) -> Result<Shape, Failure> {
        if !self.is_modified(shape) {
            return Err(Failure::with_message(
                "Standard_NoSuchObject",
                "BRepOffsetAPI_FindContigousEdges::Modified",
            ));
        }
        Ok(self.my_sewing.modified(shape))
    }

    /// The counts of the analysis, written to `out` (std::cout in the reference);
    /// `finder.dump(&mut std::io::stdout())?`.
    pub fn dump(&self, out: &mut dyn Write) -> Result<(), Failure> {
        self.my_sewing.dump(out)
    }

    /// The reference `out_of_range check(index < 0 || index > count, name)`.
    fn check_index(&self, index: i32, count: i32, name: &str) -> Result<(), Failure> {
        if index < 0 || index > count {
            return Err(Failure::with_message(
                "Standard_OutOfRange",
                &format!("BRepOffsetAPI_FindContigousEdges::{name}"),
            ));
        }
        Ok(())
    }
}
