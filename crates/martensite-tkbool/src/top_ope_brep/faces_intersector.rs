//! Face intersection state and the tolerances retained between operations.
use super::TopOpeBRepLineInter;
mod array;
mod perform;
mod prepare;
mod restrictions;
mod traversal;
pub use array::TopOpeBRepFacesLineMut;
use martensite_tkbrep::{
    brep_adaptor::BRepAdaptorSurface, topods::Shape, toptools::IndexedMapOfShape,
};
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkgeomalgo::int_patch::IntPatchIntersection;
use martensite_tktopalgo::brep_top_adaptor::BRepTopAdaptorTopolTool;

/// Holds one face-pair intersection; `let intersection = TopOpeBRepFacesIntersector::new();`.
pub struct TopOpeBRepFacesIntersector {
    intersection: IntPatchIntersection,
    done: bool,
    tolerances: (f64, f64),
    forced_tolerances: bool,
    faces: [Shape; 2],
    restrictions: IndexedMapOfShape,
    same_orientation: bool,
    lines: Handle<martensite_tkernel::ncollection::HArray1<TopOpeBRepLineInter>>,
    line_index: i32,
    line_count: i32,
    line_found: bool,
    surfaces: [Handle<BRepAdaptorSurface>; 2],
    domains: [BRepTopAdaptorTopolTool; 2],
}

impl Default for TopOpeBRepFacesIntersector {
    fn default() -> Self {
        Self {
            intersection: IntPatchIntersection::new(),
            done: false,
            tolerances: (precision::confusion(), precision::confusion()),
            forced_tolerances: false,
            faces: [Shape::default(), Shape::default()],
            restrictions: IndexedMapOfShape::default(),
            same_orientation: false,
            lines: Handle::default(),
            line_index: 1,
            line_count: 0,
            line_found: false,
            surfaces: [
                Handle::new(BRepAdaptorSurface::default()),
                Handle::new(BRepAdaptorSurface::default()),
            ],
            domains: [
                BRepTopAdaptorTopolTool::new(),
                BRepTopAdaptorTopolTool::new(),
            ],
        }
    }
}

impl TopOpeBRepFacesIntersector {
    /// Creates an unperformed pair with confusion tolerances; `Self::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reports whether intersection completed; `assert!(!Self::new().is_done());`.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Forces the next operation's arc/tangent tolerances; `pair.force_tolerances(1e-6, 2e-6)`.
    pub fn force_tolerances(&mut self, arc: f64, tangent: f64) {
        self.tolerances = (arc, tangent);
        self.forced_tolerances = true;
    }

    /// Returns the retained arc/tangent tolerances; `let (arc, tangent) = pair.get_tolerances();`.
    pub fn get_tolerances(&self) -> (f64, f64) {
        self.tolerances
    }

    /// Returns face one or two, initially null; `pair.face(1)?`.
    pub fn face(&self, index: i32) -> Result<&Shape, Failure> {
        match index {
            1 | 2 => Ok(&self.faces[(index - 1) as usize]),
            _ => Err(Failure::with_message(
                "Standard_ProgramError",
                &format!("TopOpeBRepFacesIntersector::Face: index={index}; expected 1 or 2"),
            )),
        }
    }

    /// Tests whether an edge belongs to a restriction line; `pair.is_restriction(&edge)`.
    pub fn is_restriction(&self, edge: &Shape) -> bool {
        self.restrictions.contains(edge)
    }

    /// Returns restriction edges in insertion order; `pair.restrictions().extent()`.
    pub fn restrictions(&self) -> &IndexedMapOfShape {
        &self.restrictions
    }

    /// Queries tangency after completion; `pair.same_domain()?`.
    pub fn same_domain(&self) -> Result<bool, Failure> {
        if !self.done {
            return Err(Failure::with_message(
                "Standard_ProgramError",
                "FacesIntersector : bad SameDomain",
            ));
        }
        self.intersection.tangent_faces()
    }

    /// Queries relative normal orientation for same-domain faces; `pair.surfaces_same_oriented()?`.
    pub fn surfaces_same_oriented(&self) -> Result<bool, Failure> {
        if self.same_domain()? {
            return Ok(self.same_orientation);
        }
        Err(Failure::with_message(
            "Standard_ProgramError",
            "FacesIntersector : bad SurfacesSameOriented",
        ))
    }
}
