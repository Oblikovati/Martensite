//! `StepToTopoDSMakeTransformed`: the placement of a mapped item.
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::{StepGeomAxis2Placement3d, StepGeomCartesianTransformationOperator3d};
use crate::step_repr::StepReprMappedItem;
use crate::step_to_geom::StepToGeom;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::{Axis3, Transform3, TransformForm};
use martensite_tkmath::toploc::Location;
use martensite_tkxsbase::transfer::TransferTransientProcess;
use martensite_tkxsbase::transfer_brep::TransferBRepShapeBinder;

/// ```
/// use martensite_tkdestep::step_to_topo_ds::StepToTopoDSMakeTransformed;
/// use martensite_tkbrep::topods::Shape;
/// let mut shape = Shape::default();
/// assert!(!StepToTopoDSMakeTransformed::new().transform(&mut shape).unwrap());
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct StepToTopoDSMakeTransformed {
    the_trsf: Transform3,
}

impl StepToTopoDSMakeTransformed {
    /// `StepToTopoDSMakeTransformed()`: the identity.
    pub fn new() -> Self {
        Self::default()
    }
    /// `Compute(Origin, Target, factors)`: sln 23.10.2001: false when a direction cannot be made.
    pub fn compute_between(
        &mut self,
        origin: &Handle<StepGeomAxis2Placement3d>,
        target: &Handle<StepGeomAxis2Placement3d>,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        self.the_trsf = Transform3::default();
        if origin.is_null() || target.is_null() {
            return Ok(false);
        }
        let made_origin = StepToGeom::make_axis2_placement(origin, factors)?;
        let Some(made_origin) = made_origin.get() else {
            return Ok(false);
        };
        let made_target = StepToGeom::make_axis2_placement(target, factors)?;
        let Some(made_target) = made_target.get() else {
            return Ok(false);
        };
        let from = Axis3::from_axis2(made_origin.ax2()?);
        let to = Axis3::from_axis2(made_target.ax2()?);
        // ne pas se tromper de sens !
        self.the_trsf.set_transformation_between(to, from);
        Ok(true)
    }
    /// `Compute(Operator, factors)`.
    pub fn compute_operator(
        &mut self,
        operator: &Handle<StepGeomCartesianTransformationOperator3d>,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        let Some(transform) = StepToGeom::make_transformation_3d(operator, factors)? else {
            return Ok(false);
        };
        self.the_trsf = transform;
        Ok(true)
    }
    /// `transformation`.
    pub fn transformation(&self) -> &Transform3 {
        &self.the_trsf
    }
    /// `Transform(shape)`: false when the transformation is the identity.
    pub fn transform(&self, shape: &mut Shape) -> Result<bool, Failure> {
        if self.the_trsf.form() == TransformForm::Identity {
            return Ok(false);
        }
        shape.move_by(&Location::new(self.the_trsf), false)?;
        Ok(true)
    }
    /// The placement of a mapped item: an Ax2 pair goes from source to target, an operator as
    /// target is applied. False when the case is not recognized.
    fn compute_item_placement(
        &mut self,
        item: &StepReprMappedItem,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        let source = item.mapping_source();
        let source = step_live(&source, "StepToTopoDSMakeTransformed::TranslateMappedItem")?;
        let origin: Handle<StepGeomAxis2Placement3d> = source.mapping_origin().cast();
        let target = item.mapping_target();
        let target_axis: Handle<StepGeomAxis2Placement3d> = target.cast();
        if origin.is_some() && target_axis.is_some() {
            return self.compute_between(&origin, &target_axis, factors);
        }
        let operator: Handle<StepGeomCartesianTransformationOperator3d> = target.cast();
        if operator.is_some() {
            return self.compute_operator(&operator, factors);
        }
        Ok(false)
    }
    /// `translate_mapped_item(mapit, TP, factors, progress)`: the shape of the mapped
    /// representation in position; a null shape with a warning when none is produced.
    pub fn translate_mapped_item(
        &mut self,
        mapit: &Handle<StepReprMappedItem>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Shape, Failure> {
        let item = step_live(mapit, "StepToTopoDSMakeTransformed::TranslateMappedItem")?;
        let process = step_live(tp, "StepToTopoDSMakeTransformed::TranslateMappedItem")?;
        let entity: Handle<StandardTransient> = mapit.cast();
        if !self.compute_item_placement(item, factors)? {
            process.add_warning_text(
                &entity,
                "Mapped Item, case not recognized, location ignored",
                "",
            );
        }
        let source = item.mapping_source();
        let representation =
            step_live(&source, "StepToTopoDSMakeTransformed::TranslateMappedItem")?
                .mapped_representation();
        let start: Handle<StandardTransient> = representation.cast();
        let mut binder = process.find(&start);
        if binder.is_null() {
            binder = process.transferring(&start, progress)?;
        }
        let shape_binder = binder.cast::<TransferBRepShapeBinder>();
        let Some(live) = shape_binder.get() else {
            process.add_warning_text(&entity, "No Shape Produced", "");
            return Ok(Shape::default());
        };
        let mut result = live.result();
        self.transform(&mut result)?;
        Ok(result)
    }
}
