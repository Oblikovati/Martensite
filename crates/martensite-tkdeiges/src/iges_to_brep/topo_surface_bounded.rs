//! The trimmed and bounded surfaces of IGES as faces.
use super::IgesToBRep;
use crate::iges_geom::{IgesGeomBoundedSurface, IgesGeomTrimmedSurface};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topods::{self, Face, Shape};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{GeneralTransform3, Transform3};
use martensite_tkmath::toploc::Location;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIGTransform, BRepBuilderAPIMakeShape};

use super::IgesToBRepTopoSurface;
use super::topo_surface::face_of_shape;

impl IgesToBRepTopoSurface {
    /// A face from the parameter surface; the shape to return at once when it is not one face.
    fn param_face(
        &self,
        start_label: impl Fn(&Self) -> Result<(), Failure>,
        shape: &Shape,
    ) -> Result<Result<Face, Shape>, Failure> {
        let kind = shape.shape_type()?;
        if kind != ShapeEnum::Face && kind != ShapeEnum::Shell {
            start_label(self)?;
            return Ok(Err(Shape::default()));
        }
        match face_of_shape(shape)? {
            Some(face) => Ok(Ok(face)),
            None => {
                start_label(self)?;
                Ok(Err(shape.clone()))
            }
        }
    }
    /// The face of a trimmed surface, with its outer contour and its holes;
    /// `tool.transfer_trimmed_surface(&surface)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the face.
    pub fn transfer_trimmed_surface(
        &mut self,
        start: &Handle<IgesGeomTrimmedSurface>,
    ) -> Result<Shape, Failure> {
        let Some(live) = start.get() else {
            self.fail_key(start, "IGES_1005");
            return Ok(Shape::default());
        };
        let basis = live.surface();
        if basis.is_null() || !IgesToBRep::is_topo_surface(&basis) {
            self.fail_key(start, "XSTEP_169");
            return Ok(Shape::default());
        }
        let mut tool = self.curve_tool();
        let (shape, trans, u_fact) = self.param_surface(&basis)?;
        if shape.is_null() {
            return Ok(Shape::default());
        }
        let label = |this: &Self| this.label_fail(start, &basis, b"basis surface");
        let original = match self.param_face(label, &shape)? {
            Ok(face) => face,
            Err(other) => return Ok(other),
        };
        let mut face = original.clone();
        tool.set_surface(&BRepTool::surface(&face)?);
        if live.has_outer_contour() {
            face.empty_copy()?;
            let outer = live.outer_contour();
            let wire = tool.transfer_curve_on_face(&mut face, &outer, trans, u_fact, false)?;
            if wire.is_null() {
                face = original;
                BRepBuilder::default().natural_restriction(&face, false)?;
            }
        }
        for rank in 1..=live.nb_inner_contours() {
            let inner = live.inner_contour(rank)?;
            let _ = tool.transfer_curve_on_face(&mut face, &inner, trans, u_fact, false)?;
        }
        let matrix = live.transf();
        if let Some(entity) = matrix.get() {
            face = Self::face_in_matrix(face, entity.value()?)?;
        }
        brep_tools::update_face(&face)?;
        Ok(face.into_shape())
    }
    /// A face under the matrix of its entity: a move for a similarity, else a general transform.
    fn face_in_matrix(mut face: Face, general: GeneralTransform3) -> Result<Face, Failure> {
        let matrix = general.vectorial_part();
        let translation = general.translation_part();
        let value = |row: i32, column: i32| matrix.value(row, column);
        let column_norm = |column: i32| -> Result<f64, Failure> {
            Ok(value(1, column)? * value(1, column)?
                + value(2, column)? * value(2, column)?
                + value(3, column)? * value(3, column)?)
        };
        let (s1, s2, s3) = (column_norm(1)?, column_norm(2)?, column_norm(3)?);
        let tolerance = precision::confusion();
        if (s1 - s2).abs() > tolerance || (s1 - s3).abs() > tolerance {
            let mut transform = BRepBuilderAPIGTransform::new(general);
            transform.perform(&face, true)?;
            if transform.is_done() {
                let made = transform.shape()?.clone();
                if made.shape_type()? == ShapeEnum::Face {
                    face = topods::face(&made)?;
                }
            }
            return Ok(face);
        }
        let mut away = translation.modulus();
        for row in 1..=3 {
            for column in 1..=3 {
                let unit = if row == column { 1.0 } else { 0.0 };
                away += (value(row, column)? - unit).abs();
            }
        }
        if away > tolerance {
            let mut similarity = Transform3::default();
            similarity.set_values([
                [value(1, 1)?, value(1, 2)?, value(1, 3)?, translation.x()],
                [value(2, 1)?, value(2, 2)?, value(2, 3)?, translation.y()],
                [value(3, 1)?, value(3, 2)?, value(3, 3)?, translation.z()],
            ])?;
            face.move_by(&Location::new(similarity), false)?;
        }
        Ok(face)
    }
    /// The face of a bounded surface with its boundaries;
    /// `tool.transfer_bounded_surface(&surface)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the face.
    pub fn transfer_bounded_surface(
        &mut self,
        start: &Handle<IgesGeomBoundedSurface>,
    ) -> Result<Shape, Failure> {
        let Some(live) = start.get() else {
            self.fail_key(start, "IGES_1005");
            return Ok(Shape::default());
        };
        if live.representation_type() == 0 {
            self.warn_key(start, "IGES_1275");
        }
        let basis = live.surface();
        if basis.is_null() || !IgesToBRep::is_topo_surface(&basis) {
            self.fail_key(start, "XSTEP_166");
            return Ok(Shape::default());
        }
        let mut tool = self.curve_tool();
        let (shape, trans, u_fact) = self.param_surface(&basis)?;
        if shape.is_null() {
            return Ok(Shape::default());
        }
        let label = |this: &Self| this.label_fail(start, &basis, b"basis surface");
        let mut face = match self.param_face(label, &shape)? {
            Ok(face) => face,
            Err(other) => return Ok(other),
        };
        face.empty_copy()?;
        for rank in 1..=live.nb_boundaries() {
            let boundary = live.boundary(rank)?;
            let _ = tool.transfer_boundary_on_face(&mut face, &boundary, trans, u_fact)?;
        }
        brep_tools::update_face(&face)?;
        Ok(face.into_shape())
    }
}
