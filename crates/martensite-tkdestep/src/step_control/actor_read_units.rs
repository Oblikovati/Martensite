//! `StepControlActorRead`: the units of a representation, and the transformations between
//! representations.
use super::StepControlActorRead;
use crate::step_construct::StepConstructUnitContext;
use crate::step_data::{StepDataFactors, StepDataSelectType, StepDataStepModel};
use crate::step_entity::step_live;
use crate::step_geom::{
    StepGeomAxis2Placement3d, StepGeomCartesianTransformationOperator3d,
    StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx,
    StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext,
};
use crate::step_repr::{
    StepReprGlobalUncertaintyAssignedContext, StepReprGlobalUnitAssignedContext,
    StepReprRepresentation, StepReprRepresentationRelationship,
    StepReprShapeRepresentationRelationshipWithTransformation,
};
use crate::step_to_geom::StepToGeom;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::{Axis3, Transform3, TransformForm};
use martensite_tkxsbase::transfer::TransferTransientProcess;

/// The global contexts of a representation context.
fn global_contexts(
    context: &Handle<StandardTransient>,
) -> (
    Handle<StepReprGlobalUnitAssignedContext>,
    Handle<StepReprGlobalUncertaintyAssignedContext>,
) {
    let mut units = Handle::default();
    let mut tolerance = Handle::default();
    let simple: Handle<StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext> =
        context.cast();
    if let Some(live) = simple.get() {
        units = live.global_unit_assigned_context();
    }
    let full: Handle<StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx> =
        context.cast();
    if let Some(live) = full.get() {
        units = live.global_unit_assigned_context();
        tolerance = live.global_uncertainty_assigned_context();
    }
    (units, tolerance)
}

/// `Axis2Placement->Ax2()` of an entity, as an `Axis3`.
fn placement_frame(
    axis: &Handle<StepGeomAxis2Placement3d>,
    factors: &StepDataFactors,
) -> Result<Axis3, Failure> {
    let made = StepToGeom::make_axis2_placement(axis, factors)?;
    Ok(Axis3::from_axis2(
        step_live(&made, "StepControlActorRead::ComputeTransformation")?.ax2()?,
    ))
}

impl StepControlActorRead {
    /// `prepare_units(rep, TP, local_factors)`.
    pub fn prepare_units(
        &self,
        rep: &Handle<StepReprRepresentation>,
        tp: &Handle<TransferTransientProcess>,
        factors: &mut StepDataFactors,
    ) -> Result<(), Failure> {
        self.state.write().sr_context = rep.clone();
        let model = Self::tp_model(tp)?;
        let Some(live) = rep.get() else {
            return self.reset_units(&model, factors);
        };
        let live_tp = step_live(tp, "StepControlActorRead::PrepareUnits")?;
        let context = live.context_of_items();
        if context.is_null() {
            let text = "Bad RepresentationContext, default unit taken";
            live_tp.add_warning_text(&rep.cast(), text, "");
            return self.reset_units(&model, factors);
        }
        let start: Handle<StandardTransient> = context.cast();
        let (units, tolerance) = global_contexts(&start);
        let mut unit = StepConstructUnitContext::new();
        if units.is_some() {
            let status = unit.compute_factors(&units, factors)?;
            let mode = model.internal_parameters().angle_unit.as_raw();
            let angle_factor = match mode {
                0 => unit.plane_angle_factor(),
                1 => 1.0,
                _ => std::f64::consts::PI / 180.0,
            };
            factors.initialize_factors(
                unit.length_factor(),
                angle_factor,
                unit.solid_angle_factor(),
            );
            if status != 0 {
                live_tp.add_warning_text(&start, unit.status_message(status), "");
            }
        }
        if tolerance.is_some() {
            let status = unit.compute_tolerance(&tolerance)?;
            if status != 0 {
                live_tp.add_warning_text(&start, unit.status_message(status), "");
            }
        }
        let (mode, read_value, read_max) = {
            let parameters = model.internal_parameters();
            (
                parameters.read_precision_mode.as_raw(),
                parameters.read_precision_val,
                parameters.read_max_precision_val,
            )
        };
        let precision = if mode == 1 {
            // :i1 gka S4136 05.04.99
            read_value
        } else if unit.has_uncertainty() {
            unit.uncertainty() * unit.length_factor()
        } else {
            let text = "No Length Uncertainty, value of read.precision.val is taken";
            live_tp.add_warning_text(&start, text, "");
            read_value
        };
        let mut state = self.state.write();
        state.precision = precision;
        state.max_tol = precision.max(read_max);
        Ok(())
    }
    /// `reset_units(model, local_factors)`.
    pub fn reset_units(
        &self,
        model: &Handle<StepDataStepModel>,
        factors: &mut StepDataFactors,
    ) -> Result<(), Failure> {
        factors.initialize_factors(1.0, 1.0, 1.0);
        let live = step_live(model, "StepControlActorRead::ResetUnits")?;
        let parameters = live.internal_parameters();
        let mut state = self.state.write();
        state.precision = parameters.read_precision_val;
        state.max_tol = state.precision.max(parameters.read_max_precision_val);
        Ok(())
    }
    /// `compute_transformation(Origin, Target, OrigContext, TargContext, TP, Trsf, local_factors)`.
    #[allow(clippy::too_many_arguments, reason = "the member takes each of these")]
    pub fn compute_transformation(
        &self,
        origin: &Handle<StepGeomAxis2Placement3d>,
        target: &Handle<StepGeomAxis2Placement3d>,
        orig_context: &Handle<StepReprRepresentation>,
        targ_context: &Handle<StepReprRepresentation>,
        tp: &Handle<TransferTransientProcess>,
        trsf: &mut Transform3,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        *trsf = Transform3::default(); // reinit
        if origin.is_null() || target.is_null() {
            return Ok(false);
        }
        let (mut org, mut trg) = (origin.clone(), target.clone());
        let org_entity: Handle<StandardTransient> = org.cast();
        let trg_entity: Handle<StandardTransient> = trg.cast();
        let (mut ok_origin, mut swap_origin, mut ok_target, mut swap_target) =
            (false, false, false, false);
        let orig_live = step_live(orig_context, "StepControlActorRead::ComputeTransformation")?;
        for rank in 1..=orig_live.nb_items() {
            let item: Handle<StandardTransient> = orig_live.items_value(rank)?.cast();
            if item == org_entity {
                ok_origin = true;
            } else if item == trg_entity {
                swap_target = true;
            }
        }
        let targ_live = step_live(targ_context, "StepControlActorRead::ComputeTransformation")?;
        for rank in 1..=targ_live.nb_items() {
            let item: Handle<StandardTransient> = targ_live.items_value(rank)?.cast();
            if item == trg_entity {
                ok_target = true;
            } else if item == org_entity {
                swap_origin = true;
            }
        }
        let live_tp = step_live(tp, "StepControlActorRead::ComputeTransformation")?;
        if !ok_origin || !ok_target {
            if swap_origin && swap_target {
                std::mem::swap(&mut org, &mut trg);
                let text = "Axis placements are swapped in SRRWT; corrected";
                live_tp.add_warning_text(&org.cast(), text, "");
            } else {
                let text =
                    "Axis placement used by SRRWT does not belong to corresponding representation";
                let culprit = if ok_origin { &trg_entity } else { &org_entity };
                live_tp.add_warning_text(culprit, text, "");
            }
        }
        let old_context = self.state.read().sr_context.clone();
        let mut local = *factors;
        if *orig_context != old_context {
            self.prepare_units(orig_context, tp, &mut local)?;
        }
        let orig_frame = placement_frame(&org, &local)?;
        if *targ_context != *orig_context {
            self.prepare_units(targ_context, tp, &mut local)?;
        }
        let targ_frame = placement_frame(&trg, &local)?;
        if old_context != *targ_context {
            self.prepare_units(&old_context, tp, &mut local)?;
        }
        trsf.set_transformation_between(targ_frame, orig_frame);
        Ok(trsf.form() != TransformForm::Identity)
    }
    /// `compute_srrwt(SRR, TP, Trsf, local_factors)`.
    pub fn compute_srrwt(
        &self,
        srr: &Handle<StepReprRepresentationRelationship>,
        tp: &Handle<TransferTransientProcess>,
        trsf: &mut Transform3,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        *trsf = Transform3::default(); // init
        let with_transformation: Handle<StepReprShapeRepresentationRelationshipWithTransformation> =
            srr.cast();
        let Some(live) = with_transformation.get() else {
            return Ok(false);
        };
        let select = live.transformation_operator();
        let mut local = *factors;
        let operator: Handle<StepGeomCartesianTransformationOperator3d> = select.value().cast();
        let live_srr = step_live(srr, "StepControlActorRead::ComputeSRRWT")?;
        if operator.is_some() {
            let old_context = self.state.read().sr_context.clone();
            let rep2 = live_srr.rep2();
            if rep2 != old_context {
                self.prepare_units(&rep2, tp, &mut local)?;
            }
            if let Some(made) = StepToGeom::make_transformation_3d(&operator, &local)? {
                *trsf = made;
            }
            if rep2 != old_context {
                self.prepare_units(&old_context, tp, &mut local)?;
            }
            return Ok(trsf.form() != TransformForm::Identity);
        }
        let item_defined = select.item_defined_transformation();
        let Some(live_item) = item_defined.get() else {
            return Ok(false);
        };
        let axis1: Handle<StepGeomAxis2Placement3d> = live_item.transform_item1().cast();
        let axis2: Handle<StepGeomAxis2Placement3d> = live_item.transform_item2().cast();
        if axis1.is_null() || axis2.is_null() {
            return Ok(false);
        }
        self.compute_transformation(
            &axis1,
            &axis2,
            &live_srr.rep1(),
            &live_srr.rep2(),
            tp,
            trsf,
            &local,
        )
    }
}
