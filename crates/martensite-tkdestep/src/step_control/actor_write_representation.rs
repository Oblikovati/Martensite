//! `StepControlActorWrite`: the shape_representation of a transfer, and its items.
use super::StepControlActorWrite;
use super::actor_write_items::{
    as_is_mode, bind_item_result, has_geometry, representation_of_item, translate_shape_item,
    used_tolerance,
};
use crate::de_step::StepControlStepModelType;
use crate::geom_to_step::shared_array1;
use crate::step_construct::StepConstructUnitContext;
use crate::step_data::{StepDataFactors, StepDataStepModel};
use crate::step_entity::{step_array1_item, step_array1_len, step_live};
use crate::step_geom::StepGeomAxis2Placement3d;
use crate::step_repr::{StepReprRepresentationItem, StepReprShapeRepresentationRelationship};
use crate::step_shape::{
    StepShapeAdvancedBrepShapeRepresentation, StepShapeFacetedBrepShapeRepresentation,
    StepShapeGeometricallyBoundedWireframeShapeRepresentation,
    StepShapeManifoldSurfaceShapeRepresentation, StepShapeShapeDefinitionRepresentation,
    StepShapeShapeRepresentation,
};
use crate::step_visual::{StepVisualTessellatedItem, StepVisualTessellatedShapeRepresentation};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressScope;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkxsbase::transfer::{
    TransferActorOfFinderProcessOps, TransferActorOps, TransferBinder, TransferFinder,
    TransferFinderProcess,
};
use martensite_tkxsbase::transfer_brep::TransferBRepShapeMapper;
use martensite_tkxsbase::xs_algo::XSAlgoShapeProcessor;

const WHERE: &str = "StepControlActorWrite";

/// The shapes a transfer goes on with, or the result it stops at.
pub(super) enum ShapesToWrite {
    /// The shapes to translate, in order.
    Items(Vec<Shape>),
    /// The transfer ends with this binder.
    Done(Handle<dyn TransferBinder>),
}

impl StepControlActorWrite {
    /// Translates the shapes and writes the representation of the items.
    #[allow(clippy::too_many_arguments, reason = "the member reads each of these")]
    pub(super) fn write_representation(
        &self,
        start: &Handle<dyn TransferFinder>,
        sdr0: &Handle<StepShapeShapeDefinitionRepresentation>,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        shape_group: &Handle<martensite_tkernel::ncollection::HSequence<Shape>>,
        is_manifold: bool,
        the_shape: &Shape,
        rep_items: Vec<Shape>,
        nm_binder: Handle<dyn TransferBinder>,
        root: &ProgressScope<'_>,
    ) -> Result<Handle<dyn TransferBinder>, Failure> {
        let process = step_live(fp, WHERE)?;
        let mapper: Handle<TransferBRepShapeMapper> = start.cast();
        let original = step_live(&mapper, WHERE)?.value().clone();
        let model: Handle<StepDataStepModel> = process.model().cast();
        let live_model = step_live(&model, WHERE)?;
        let mode = self.mode();
        let tolerance = used_tolerance(live_model, self.state.read().mytoler, the_shape)?;
        let nbs = rep_items.len();
        let mut item_seq: Vec<Handle<StandardTransient>> = vec![self.default_axis()?.cast()];
        let mut trmode = mode;
        let range = root.next(1.0);
        let scope = ProgressScope::unnamed(&range, nbs as f64, false);
        let max_read = live_model.internal_parameters().read_max_precision_val;
        for x_shape in &rep_items {
            if !scope.more() {
                break;
            }
            if mode == StepControlStepModelType::AS_IS {
                trmode = as_is_mode(x_shape)?;
            }
            let shape_range = scope.next(1.0);
            let sub_scope = ProgressScope::unnamed(&shape_range, 2.0, false);
            let mut parameters = self.shape_processing().shape_fix_parameters();
            XSAlgoShapeProcessor::set_parameter_real(
                "FixShape.Tolerance3d",
                tolerance,
                true,
                &mut parameters,
            );
            XSAlgoShapeProcessor::set_parameter_real(
                "FixShape.MaxTolerance3d",
                max_read,
                true,
                &mut parameters,
            );
            let mut processor = XSAlgoShapeProcessor::new(&parameters, &Default::default());
            let mut shape = x_shape.clone();
            if has_geometry(&shape)? {
                let flags = self.shape_processing().flags().0;
                shape = processor.process_shape(x_shape, flags, &sub_scope.next(1.0))?;
                if sub_scope.user_break() {
                    return Ok(Handle::default());
                }
            }
            if !is_manifold {
                self.merge_info_for_nm(fp, &processor.get_context())?;
            }
            let range = sub_scope.next(1.0);
            let Some(made) =
                translate_shape_item(trmode, &shape, start, fp, factors, tolerance, &range)?
            else {
                return Ok(process.find(start));
            };
            if made.item.is_null() && made.tessellated.is_null() {
                continue;
            }
            if made.item.is_some() {
                item_seq.push(made.item.cast());
                bind_item_result(self, &made.item, x_shape, start, &original, fp)?;
            }
            if made.tessellated.is_some() {
                item_seq.push(made.tessellated.cast());
                bind_item_result(self, &made.tessellated, x_shape, start, &original, fp)?;
            }
            processor.merge_transfer_info_finder(fp)?;
        }
        if item_seq.len() < 2 && item_seq[0].is_null() {
            process.add_fail_text(start, "The Shape has not the appropriate type", "");
            return Ok(process.find(start));
        }
        let (shape_rep, use_existing) =
            self.representation_for(the_shape, mode, trmode, nbs, is_manifold, shape_group, fp)?;
        if shape_rep.is_null() {
            return Ok(Handle::default());
        }
        let live_rep = step_live(&shape_rep, WHERE)?;
        let items: Vec<Handle<StepReprRepresentationItem>> =
            item_seq.iter().map(|item| item.cast()).collect();
        let tessellated_binders = self.distribute_items(
            &shape_rep,
            &items,
            nbs,
            use_existing,
            tolerance,
            &model,
            factors,
        )?;
        let mut context = StepConstructUnitContext::new();
        context.init(tolerance, &model, factors)?;
        live_rep.set_context_of_items(&context.value().cast()); // la tolerance, voir au debut
        live_rep.set_name(&Handle::new(HAsciiString::new("")));
        if use_existing {
            return Ok(process.find(start));
        }
        step_live(sdr0, WHERE)?.set_used_representation(&shape_rep.cast());
        let result: Handle<dyn TransferBinder> = self.transient_result(&shape_rep.cast()).cast();
        let known = process.find(start);
        if known.is_some() {
            result.add_result(&known);
            process.rebind(start, &result)?;
        }
        for binder in &tessellated_binders {
            result.add_result(binder);
        }
        if nm_binder.is_some() {
            result.add_result(&nm_binder);
        }
        Ok(result)
    }
    /// The shape_representation of this write, and whether an existing one is added to.
    #[allow(clippy::too_many_arguments, reason = "the switch reads each of these")]
    pub(super) fn representation_for(
        &self,
        the_shape: &Shape,
        mode: StepControlStepModelType,
        trmode: StepControlStepModelType,
        nbs: usize,
        is_manifold: bool,
        shape_group: &Handle<martensite_tkernel::ncollection::HSequence<Shape>>,
        fp: &Handle<TransferFinderProcess>,
    ) -> Result<(Handle<StepShapeShapeRepresentation>, bool), Failure> {
        if the_shape.shape_type()? == ShapeEnum::Shape {
            // for external references
            return Ok((
                Handle::new(StepShapeShapeRepresentation::new()).cast(),
                false,
            ));
        }
        let of_mode = match mode {
            StepControlStepModelType::AS_IS if nbs != 1 => {
                return Ok((
                    Handle::new(StepShapeShapeRepresentation::new()).cast(),
                    false,
                ));
            }
            StepControlStepModelType::AS_IS => trmode,
            other => other,
        };
        let new_rep: Handle<StepShapeShapeRepresentation> = match of_mode {
            StepControlStepModelType::MANIFOLD_SOLID_BREP => {
                Handle::new(StepShapeAdvancedBrepShapeRepresentation::new()).cast()
            }
            StepControlStepModelType::FACETED_BREP => {
                Handle::new(StepShapeFacetedBrepShapeRepresentation::new()).cast()
            }
            StepControlStepModelType::GEOMETRIC_CURVE_SET => {
                Handle::new(StepShapeGeometricallyBoundedWireframeShapeRepresentation::new()).cast()
            }
            StepControlStepModelType::SHELL_BASED_SURFACE_MODEL if is_manifold => {
                Handle::new(StepShapeManifoldSurfaceShapeRepresentation::new()).cast()
            }
            StepControlStepModelType::SHELL_BASED_SURFACE_MODEL => {
                let (found, created) = self.nmssr_for_group(shape_group, fp)?;
                return Ok((found, !created));
            }
            _ => Handle::default(),
        };
        Ok((new_rep, false))
    }
    /// Sets the items on the representation, split per the AP203 rule; the binders to chain.
    #[allow(clippy::too_many_arguments, reason = "the block reads each of these")]
    pub(super) fn distribute_items(
        &self,
        shape_rep: &Handle<StepShapeShapeRepresentation>,
        items: &[Handle<StepReprRepresentationItem>],
        nbs: usize,
        use_existing: bool,
        tolerance: f64,
        model: &Handle<StepDataStepModel>,
        factors: &StepDataFactors,
    ) -> Result<Vec<Handle<dyn TransferBinder>>, Failure> {
        let live_rep = step_live(shape_rep, WHERE)?;
        let schema = step_live(model, WHERE)?
            .internal_parameters()
            .write_schema
            .as_raw();
        let mut binders: Vec<Handle<dyn TransferBinder>> = Vec::new();
        if schema == 3 && nbs > 1 {
            let mut rank = 0;
            if items[0].cast::<StepGeomAxis2Placement3d>().is_some() {
                live_rep.set_items(&shared_array1(vec![items[0].clone()]));
                rank = 1;
            }
            for item in &items[rank..] {
                let split = representation_of_item(item);
                let live_split = step_live(&split, WHERE)?;
                let axis: Handle<StepReprRepresentationItem> = self.default_axis()?.cast();
                live_split.set_items(&shared_array1(vec![axis, item.clone()]));
                let mut context = StepConstructUnitContext::new();
                context.init(tolerance, model, factors)?;
                live_split.set_context_of_items(&context.value().cast()); // la tolerance, voir au debut
                live_split.set_name(&Handle::new(HAsciiString::new("")));
                let relation = Handle::new(StepReprShapeRepresentationRelationship::new());
                relation.set_name(&Handle::new(HAsciiString::new("")));
                relation.set_description(&Handle::new(HAsciiString::new("")));
                relation.set_rep2(&shape_rep.cast());
                relation.set_rep1(&split.cast());
                binders.push(self.transient_result(&relation.cast()).cast());
            }
            return Ok(binders);
        }
        let tessellated: Vec<_> = items
            .iter()
            .filter(|item| item.cast::<StepVisualTessellatedItem>().is_some())
            .cloned()
            .collect();
        if !tessellated.is_empty() {
            let tess_rep: Handle<StepShapeShapeRepresentation> =
                Handle::new(StepVisualTessellatedShapeRepresentation::new()).cast();
            let live_tess = step_live(&tess_rep, WHERE)?;
            live_tess.set_items(&shared_array1(tessellated));
            let mut context = StepConstructUnitContext::new();
            context.init(tolerance, model, factors)?;
            live_tess.set_context_of_items(&context.value().cast());
            live_tess.set_name(&Handle::new(HAsciiString::new("")));
            binders.push(self.transient_result(&tess_rep.cast()).cast());
        }
        if !use_existing {
            live_rep.set_items(&shared_array1(items.to_vec()));
        } else {
            let old = live_rep.items();
            let mut merged = Vec::new();
            for rank in 1..=step_array1_len(&old) {
                merged.push(step_array1_item(&old, rank, WHERE)?);
            }
            merged.push(items[items.len() - 1].clone());
            live_rep.set_items(&shared_array1(merged));
        }
        Ok(binders)
    }
}
