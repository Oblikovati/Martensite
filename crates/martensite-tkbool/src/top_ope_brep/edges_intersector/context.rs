use super::TopOpeBRepEdgesIntersector;
use crate::top_ope_brep_tool::{TopOpeBRepToolShapeTool, ftol_face_tolerances2d};
use martensite_tkbrep::{
    brep_adaptor::BRepAdaptorSurface,
    topods::{Edge, Face},
};
use martensite_tkernel::standard::Failure;
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkmath::bnd::BoundingBox3;

impl TopOpeBRepEdgesIntersector {
    /// Initializes unrestricted surface contexts; `state.set_faces(&first, &second)?`.
    pub fn set_faces(&mut self, first: &Face, second: &Face) -> Result<(), Failure> {
        self.set_faces_with_boxes(
            first,
            second,
            &BoundingBox3::default(),
            &BoundingBox3::default(),
        )
    }
    /// Computes face tolerances using the supplied boxes; `state.set_faces_with_boxes(&a, &b, &ba, &bb)?`.
    pub fn set_faces_with_boxes(
        &mut self,
        first: &Face,
        second: &Face,
        a: &BoundingBox3,
        b: &BoundingBox3,
    ) -> Result<(), Failure> {
        self.faces = [first.clone(), second.clone()];
        self.surfaces[0].initialize(first, false)?;
        self.surfaces[1].initialize(second, false)?;
        self.surfaces_same =
            TopOpeBRepToolShapeTool::surfaces_same_oriented(&self.surfaces[0], &self.surfaces[1])?;
        self.faces_same = TopOpeBRepToolShapeTool::faces_same_oriented(first, second)?;
        let reference = first.oriented(martensite_tkg3d::top_abs::Orientation::Forward);
        for index in [0, 1] {
            self.reference_orientations[index] =
                TopOpeBRepToolShapeTool::faces_same_oriented(&reference, &self.faces[index])?;
        }

        self.update_tolerances(a, b)?;
        Ok(())
    }
    fn update_tolerances(&mut self, a: &BoundingBox3, b: &BoundingBox3) -> Result<(), Failure> {
        if self.tolerance_forced {
            return Ok(());
        }
        let (x, y) = ftol_face_tolerances2d(
            a,
            b,
            &self.faces[0],
            &self.faces[1],
            &self.surfaces[0],
            &self.surfaces[1],
        )?;
        self.tolerances = (
            if x > 1e-4 { 1e-4 } else { x },
            if y > 1e-4 { 1e-4 } else { y },
        );
        Ok(())
    }
    /// Reads an input edge at native index 1 or 2; `state.edge(1)?`.
    pub fn edge(&self, index: i32) -> Result<&Edge, Failure> {
        Ok(&self.edges[slot(index, "Edge")?])
    }
    /// Reads the edge's curve adaptor; `state.curve(2)?`.
    pub fn curve(&self, index: i32) -> Result<&Geom2dAdaptorCurve, Failure> {
        Ok(&self.curves[slot(index, "Curve")?])
    }
    /// Reads an input face; `state.face(1)?`.
    pub fn face(&self, index: i32) -> Result<&Face, Failure> {
        Ok(&self.faces[slot(index, "Face")?])
    }
    /// Reads the unrestricted surface adaptor; `state.surface(2)?`.
    pub fn surface(&self, index: i32) -> Result<&BRepAdaptorSurface, Failure> {
        Ok(&self.surfaces[slot(index, "Surface")?])
    }
    /// Compares geometric surface normals; `state.surfaces_same_oriented()`.
    pub fn surfaces_same_oriented(&self) -> bool {
        self.surfaces_same
    }
    /// Compares oriented face normals; `state.faces_same_oriented()`.
    pub fn faces_same_oriented(&self) -> bool {
        self.faces_same
    }
}
fn slot(index: i32, operation: &str) -> Result<usize, Failure> {
    match index {
        1 => Ok(0),
        2 => Ok(1),
        _ => Err(Failure::new(&format!(
            "TopOpeBRepEdgesIntersector::{operation}: index={index}; expected 1 or 2"
        ))),
    }
}
