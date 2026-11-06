//! The steps that restore, explode and edit the shapes in a script's variables.
use crate::input_shapes::{
    bubble_order, distinct_parts, explode_kind, numbered_key, restored_shape,
};
use crate::script::{FilletScript, command_stopped};
use crate::steps::Topo;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topexp,
    topods::{ShapeIterator, Solid},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkshhealing::shape_fix::ShapeFixShapeTolerance;
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape, BRepBuilderAPINurbsConvert},
    brep_lib,
};
use std::f64::consts::PI;

impl FilletScript<'_> {
    /// Run one step that restores, explodes or edits shapes.
    pub fn topology(&mut self, topo: &Topo) -> Result<(), Failure> {
        match topo {
            Topo::Restore(path, name) => self.set_shape(name, restored_shape(path)?),
            Topo::Explode(name, kind) => self.explode(name, kind)?,
            Topo::ExplodeChildren(name) => self.explode_children(name)?,
            Topo::NumberedExplode(name, kind) => self.numbered_explode(name, kind)?,
            Topo::GeometryCopy(from, to) => self.geometry_copy(from, to)?,
            Topo::SharedCopy(from, to) | Topo::Rename(from, to) => {
                self.set_shape(to, self.required(from)?)
            }
            Topo::NurbsConvert(to, from) => self.nurbs_convert(to, from)?,
            _ => self.edit_topology(topo)?,
        }
        Ok(())
    }

    fn explode(&mut self, name: &str, kind: &str) -> Result<(), Failure> {
        let parts = distinct_parts(&self.required(name)?, explode_kind(kind)?)?;
        self.set_parts(name, parts);
        Ok(())
    }

    fn explode_children(&mut self, name: &str) -> Result<(), Failure> {
        let shape = self.required(name)?;
        let mut iterator = ShapeIterator::new(&shape, true, true);
        let mut parts = Vec::new();
        while iterator.more() {
            parts.push(iterator.value()?.clone());
            iterator.next();
        }
        self.set_parts(name, parts);
        Ok(())
    }

    fn numbered_explode(&mut self, name: &str, kind: &str) -> Result<(), Failure> {
        let shape = self.required(name)?;
        let kind = explode_kind(kind)?;
        let mut indexed = IndexedMapOfShape::default();
        indexed.add(shape.clone());
        topexp::map_shapes_of_type(&shape, kind, &mut indexed)?;
        let mut parts = distinct_parts(&shape, kind)?;
        parts.truncate(indexed.extent().saturating_sub(1));
        let keys = parts
            .iter()
            .map(numbered_key)
            .collect::<Result<Vec<f64>, Failure>>()?;
        let ordered = bubble_order(&keys)
            .into_iter()
            .map(|i| parts[i].clone())
            .collect();
        self.set_parts(name, ordered);
        Ok(())
    }

    fn geometry_copy(&mut self, from: &str, to: &'static str) -> Result<(), Failure> {
        let mut copier = BRepBuilderAPICopy::new();
        copier.perform(&self.required(from)?, true, false)?;
        self.set_shape(to, copier.shape()?.clone());
        Ok(())
    }

    fn invert(&mut self, name: &'static str) -> Result<(), Failure> {
        let shape = self.required(name)?;
        let mut inverted = shape.empty_copied()?;
        let closed = shape.tshape().get().is_some_and(|t| t.closed());
        if let Some(tshape) = inverted.tshape().get() {
            tshape.set_closed(closed);
        }
        let builder = BRepBuilder::default();
        let mut children = ShapeIterator::new(&shape, true, true);
        while children.more() {
            builder.add(&mut inverted, &children.value()?.reversed())?;
            children.next();
        }
        self.set_shape(name, inverted);
        Ok(())
    }

    fn nurbs_convert(&mut self, to: &'static str, from: &str) -> Result<(), Failure> {
        let mut converter = BRepBuilderAPINurbsConvert::new();
        converter.perform(&self.required(from)?, false)?;
        if !converter.is_done() {
            return Err(command_stopped(&format!("nurbsconvert {from} not done")));
        }
        self.set_shape(to, converter.shape()?.clone());
        Ok(())
    }

    /// The commands that change a shape in its variable.
    fn edit_topology(&mut self, topo: &Topo) -> Result<(), Failure> {
        match topo {
            Topo::OrientReversed(name) => {
                let mut shape = self.required(name)?;
                shape.set_orientation(Orientation::Reversed);
                self.set_shape(name, shape);
            }
            Topo::Reverse(name) => self.set_shape(name, self.required(name)?.reversed()),
            Topo::EmptySolid(name) => {
                let mut solid = Solid::default();
                BRepBuilder::default().make_solid(&mut solid);
                self.set_shape(name, solid.into_shape());
            }
            Topo::Add(what, into) => self.add_into(what, into)?,
            Topo::Invert(name) => self.invert(name)?,
            _ => self.fix_topology(topo)?,
        }
        Ok(())
    }

    fn add_into(&mut self, what: &str, into: &str) -> Result<(), Failure> {
        let mut target = self.required(into)?;
        BRepBuilder::default().add(&mut target, &self.required(what)?)?;
        self.set_shape(into, target);
        Ok(())
    }

    /// The healing commands, which change the shape's tolerances and flags in place.
    fn fix_topology(&mut self, topo: &Topo) -> Result<(), Failure> {
        match topo {
            Topo::ForceTolerance(name, value) => {
                let (min, max, any) = (*value, *value, ShapeEnum::Shape);
                let shape = self.required(name)?;
                println!("Setting Tolerance to {min}");
                ShapeFixShapeTolerance::new().limit_tolerance(&shape, min, max, any)?;
                Ok(())
            }
            Topo::SameParameter(name, tolerance) => self.same_parameter(name, *tolerance),
            Topo::EncodeRegularity(name, degrees) => self.encode_regularity(name, *degrees),
            _ => unreachable!("topology and edit_topology dispatch the other commands"),
        }
    }

    /// `fsameparameter name [tol]`, forced, with the default of the command.
    fn same_parameter(&mut self, name: &str, tolerance: Option<f64>) -> Result<(), Failure> {
        let tolerance = tolerance.filter(|t| *t > 0.).unwrap_or(1.0e-7);
        brep_lib::same_parameter_shape(&self.required(name)?, tolerance, true)
    }

    fn encode_regularity(&mut self, name: &str, degrees: Option<f64>) -> Result<(), Failure> {
        let tolerance = degrees.map_or(1.0e-10, |d| d * PI / 180.0);
        brep_lib::encode_regularity(&self.required(name)?, tolerance)
    }
}
