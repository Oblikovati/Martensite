//! `BRepFeatGluer`: the gluing of a new shape on a basis shape through bound faces and
//! edges (`Gluer`), as a shape builder with the history of the faces.
use crate::loc_ope::{Gluer, Operation};
use martensite_tkbrep::topods::{self, Edge, Face, Shape};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeShape, BRepBuilderAPIMakeShapeBase,
};

/// `BRepFeatGluer`: `snew` glued on `sbase`.
pub struct BRepFeatGluer {
    base: BRepBuilderAPIMakeShapeBase,
    gluer: Gluer,
}

impl Default for BRepFeatGluer {
    fn default() -> Self {
        Self::new()
    }
}

impl BRepFeatGluer {
    /// An empty one; `BRepFeatGluer::new()`.
    pub fn new() -> Self {
        Self {
            base: BRepBuilderAPIMakeShapeBase::new(),
            gluer: Gluer::new(),
        }
    }

    /// The shape `snew` to glue on `sbase` (the reference
    /// gluer takes them the other way round); `BRepFeatGluer::with_shapes(&snew, &sbase)`.
    pub fn with_shapes(snew: &Shape, sbase: &Shape) -> Self {
        Self {
            base: BRepBuilderAPIMakeShapeBase::new(),
            gluer: Gluer::with_shapes(sbase, snew),
        }
    }

    /// `init(Snew, Sbase)`; `gluer.init(&snew, &sbase)`.
    pub fn init(&mut self, snew: &Shape, sbase: &Shape) {
        self.gluer.init(sbase, snew);
    }

    /// The face `fnew` of the new shape lies on `fbase`;
    /// `gluer.bind_faces(&fnew, &fbase)?`.
    pub fn bind_faces(&mut self, fnew: &Face, fbase: &Face) -> Result<(), Failure> {
        self.gluer.bind_faces(fnew, fbase)
    }

    /// The edge `enew` of the new shape is `ebase`;
    /// `gluer.bind_edges(&enew, &ebase)?`.
    pub fn bind_edges(&mut self, enew: &Edge, ebase: &Edge) -> Result<(), Failure> {
        self.gluer.bind_edges(enew, ebase)
    }

    /// The Boolean operation the gluing amounts to; `gluer.ope_type()`.
    pub fn ope_type(&self) -> Operation {
        self.gluer.ope_type()
    }

    /// An empty one; `gluer.basis_shape()`.
    pub fn basis_shape(&self) -> &Shape {
        self.gluer.basis_shape()
    }

    /// An empty one; `gluer.glued_shape()`.
    pub fn glued_shape(&self) -> &Shape {
        self.gluer.glued_shape()
    }
}

impl BRepBuilderAPIMakeShape for BRepFeatGluer {
    fn make_shape_base(&self) -> &BRepBuilderAPIMakeShapeBase {
        &self.base
    }

    fn make_shape_base_mut(&mut self) -> &mut BRepBuilderAPIMakeShapeBase {
        &mut self.base
    }

    /// The gluing performed, its resulting shape the result.
    fn build(&mut self, _range: &ProgressRange<'_>) -> Result<(), Failure> {
        self.gluer.perform()?;
        if self.gluer.is_done() {
            self.base.set_done();
            self.base.my_shape = self.gluer.resulting_shape()?.clone();
        } else {
            self.base.set_not_done();
        }
        Ok(())
    }

    /// Whether the face has no descendant.
    fn is_deleted(&mut self, f: &Shape) -> Result<bool, Failure> {
        Ok(self.gluer.descendant_faces(&topods::face(f)?)?.is_empty())
    }

    /// The descendants of a face unless it is kept itself first.
    fn modified(
        &mut self,
        f: &Shape,
    ) -> Result<&martensite_tkernel::ncollection::List<Shape>, Failure> {
        self.base.my_generated.clear();
        if f.shape_type()? == ShapeEnum::Face {
            let ls = self.gluer.descendant_faces(&topods::face(f)?)?;
            if ls.first().is_some_and(|first| !first.is_same(f)) {
                self.base.my_generated = ls;
            }
        }
        Ok(&self.base.my_generated)
    }
}
