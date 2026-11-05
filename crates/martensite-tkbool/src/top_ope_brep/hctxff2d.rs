//! Shared face-surface adaptors and their native orientation relationships.
use crate::top_ope_brep_tool::TopOpeBRepToolShapeTool;
use martensite_tkbrep::{brep_adaptor::BRepAdaptorSurface, topods::Face};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tkg3d::top_abs::Orientation;

/// A pair of face/surface contexts; `TopOpeBRepHctxff2d::new()`.
pub struct TopOpeBRepHctxff2d {
    parent: StandardTransient,
    faces: [Face; 2],
    surfaces: [Handle<BRepAdaptorSurface>; 2],
    reference_orientations: [bool; 2],
    surfaces_same: bool,
    faces_same: bool,
    tolerances: (f64, f64),
}
impl_transient!(
    TopOpeBRepHctxff2d,
    "TopOpeBRepHctxff2d",
    StandardTransient,
    parent
);

impl Default for TopOpeBRepHctxff2d {
    fn default() -> Self {
        Self {
            parent: StandardTransient::default(),
            faces: [Face::default(), Face::default()],
            surfaces: [Handle::default(), Handle::default()],
            reference_orientations: [true; 2],
            surfaces_same: false,
            faces_same: false,
            tolerances: (0., 0.),
        }
    }
}
impl TopOpeBRepHctxff2d {
    /// Starts with null faces/surfaces and zero tolerances; `Self::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Initializes only changed faces, without restricting surface bounds; `context.set_faces(&first, &second)?`.
    pub fn set_faces(&mut self, first: &Face, second: &Face) -> Result<(), Failure> {
        let changed = [
            !first.is_equal(&self.faces[0]),
            !second.is_equal(&self.faces[1]),
        ];
        if !changed[0] && !changed[1] {
            return Ok(());
        }
        for (index, face) in [first, second].into_iter().enumerate() {
            if !changed[index] {
                continue;
            }
            if self.surfaces[index].is_null() {
                self.surfaces[index] = Handle::new(BRepAdaptorSurface::default());
            }
            self.surfaces[index].initialize(face, false)?;
        }
        self.set_hsurfaces_private()
    }
    /// Reuses supplied handles when either non-null face changes; `context.set_hsurfaces(&first, &second)?`.
    pub fn set_hsurfaces(
        &mut self,
        first: &Handle<BRepAdaptorSurface>,
        second: &Handle<BRepAdaptorSurface>,
    ) -> Result<(), Failure> {
        let changed = [first, second]
            .into_iter()
            .enumerate()
            .any(|(index, surface)| {
                surface
                    .get()
                    .is_some_and(|surface| !surface.face().is_equal(&self.faces[index]))
            });
        if !changed {
            return Ok(());
        }
        self.surfaces = [first.clone(), second.clone()];
        self.set_hsurfaces_private()
    }
    fn set_hsurfaces_private(&mut self) -> Result<(), Failure> {
        for index in [0, 1] {
            let surface = self.surfaces[index].get().ok_or_else(null_surface)?;
            self.faces[index] = surface.face();
            surface.get_type()?;
        }
        self.surfaces_same = true;
        self.faces_same = true;
        self.reference_orientations = [true; 2];
        self.update_orientations()
    }
    fn update_orientations(&mut self) -> Result<(), Failure> {
        let mut reference = self.faces[0].clone();
        reference.set_orientation(Orientation::Forward);
        for index in [0, 1] {
            self.reference_orientations[index] =
                TopOpeBRepToolShapeTool::faces_same_oriented(&reference, &self.faces[index])?;
        }
        self.surfaces_same =
            TopOpeBRepToolShapeTool::surfaces_same_oriented(&self.surfaces[0], &self.surfaces[1])?;
        self.faces_same =
            TopOpeBRepToolShapeTool::faces_same_oriented(&self.faces[0], &self.faces[1])?;
        Ok(())
    }
    /// Stores tolerances without validation; `context.set_tolerances(1e-7, 2e-7)`.
    pub fn set_tolerances(&mut self, first: f64, second: f64) {
        self.tolerances = (first, second);
    }
    /// Reads both stored tolerances; `context.tolerances()`.
    pub fn tolerances(&self) -> (f64, f64) {
        self.tolerances
    }
    /// Uses native std::max ordering, including NaN; `context.max_tolerance()`.
    pub fn max_tolerance(&self) -> f64 {
        let (first, second) = self.tolerances;
        if first < second { second } else { first }
    }
    /// Reads a face or the original invalid-index error; `context.face(1)?`.
    pub fn face(&self, index: i32) -> Result<&Face, Failure> {
        Ok(&self.faces[context_slot(index, "TopOpeBRepHctxff2d::Face")?])
    }
    /// Shares a surface handle; `context.hsurface(2)?`.
    pub fn hsurface(&self, index: i32) -> Result<Handle<BRepAdaptorSurface>, Failure> {
        Ok(self.surfaces[context_slot(index, "TopOpeBRepHctxff2d::HSurface")?].clone())
    }
    /// Reads the computed surface orientation relationship; `context.surfaces_same_oriented()`.
    pub fn surfaces_same_oriented(&self) -> bool {
        self.surfaces_same
    }
    /// Reads the computed oriented-face relationship; `context.faces_same_oriented()`.
    pub fn faces_same_oriented(&self) -> bool {
        self.faces_same
    }
    /// Compares one face with the first face's forward reference; `context.face_same_oriented_with_ref(1)?`.
    pub fn face_same_oriented_with_ref(&self, index: i32) -> Result<bool, Failure> {
        Ok(self.reference_orientations[context_slot(index, "TopOpeBRepHctxff2d::FSO")?])
    }
}
pub(super) fn context_slot(index: i32, message: &str) -> Result<usize, Failure> {
    match index {
        1 => Ok(0),
        2 => Ok(1),
        _ => Err(Failure::new(message)),
    }
}
fn null_surface() -> Failure {
    Failure::with_message(
        "Standard_NullObject",
        "TopOpeBRepHctxff2d: surface=None; expected two initialized surface handles",
    )
}

use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
