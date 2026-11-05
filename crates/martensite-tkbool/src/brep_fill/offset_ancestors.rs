//! `BRepFillOffsetAncestors`: the element of the spine each parallel of an offset wire
//! comes from.
use super::BRepFillOffsetWire;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Edge, Shape};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

/// The ancestors of the parallels of an offset wire (`BRepFillOffsetAncestors`);
/// `let a = BRepFillOffsetAncestors::with_offset(&mut offset_wire)?;`.
#[derive(Default)]
pub struct BRepFillOffsetAncestors {
    my_is_perform: bool,
    my_map: DataMap<Shape, Shape, ShapeMapHasher>,
}

impl BRepFillOffsetAncestors {
    /// `BRepFillOffsetAncestors()`: not performed; `BRepFillOffsetAncestors::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// `BRepFillOffsetAncestors(Paral)`: `Perform` of the offset wire.
    pub fn with_offset(paral: &mut BRepFillOffsetWire) -> Result<Self, Failure> {
        let mut ancestors = Self::new();
        ancestors.perform(paral)?;
        Ok(ancestors)
    }

    /// `Perform(Paral)`: each parallel of an edge, then of a vertex, of the spine
    /// mapped to it (a later binding replaces an earlier one); `a.perform(&mut ow)?`.
    pub fn perform(&mut self, paral: &mut BRepFillOffsetWire) -> Result<(), Failure> {
        let spine = paral.spine().clone();
        for kind in [ShapeEnum::Edge, ShapeEnum::Vertex] {
            let mut exp = Explorer::new(&spine, kind, ShapeEnum::Shape)?;
            while exp.more() {
                let current: Shape = exp.current().clone();
                for parallel in paral.generated_shapes(&current)?.iter() {
                    self.my_map.bind(parallel.clone(), current.clone());
                }
                exp.next()?;
            }
        }
        self.my_is_perform = true;
        Ok(())
    }

    /// `IsDone()`; `a.is_done()`.
    pub fn is_done(&self) -> bool {
        self.my_is_perform
    }

    /// `HasAncestor(S1)`: whether the edge is a parallel; `a.has_ancestor(&edge)`.
    pub fn has_ancestor(&self, s1: &Edge) -> bool {
        self.my_map.is_bound(s1.as_ref())
    }

    /// `Ancestor(S1)`: the element of the spine of a parallel; StdFailNotDone before
    /// Perform, StandardNoSuchObject for another edge; `a.ancestor(&edge)?`.
    pub fn ancestor(&self, s1: &Edge) -> Result<&Shape, Failure> {
        if !self.my_is_perform {
            return Err(Failure::with_type(
                "StdFail_NotDone",
                "BRepFillOffsetAncestors::Ancestor() - Perform() should be called before accessing results",
                "BRepFillOffsetAncestors",
            ));
        }
        self.my_map.find(s1.as_ref())
    }
}
