//! `StepControlActorRead`: the geometric item, mapped item and face surface overloads of
//! `transfer_entity`, and the way of reading without products.
use super::StepControlActorRead;
use super::actor_read_support::{is_exactly, shared_entities, sharing_entities};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::StepGeomGeometricRepresentationItem;
use crate::step_repr::{
    StepReprProductDefinitionShape, StepReprRepresentation, StepReprShapeAspect,
    StepReprShapeRepresentationRelationship,
};
use crate::step_shape::{
    StepShapeBrepWithVoids, StepShapeContextDependentShapeRepresentation,
    StepShapeEdgeBasedWireframeModel, StepShapeFaceBasedSurfaceModel, StepShapeFacetedBrep,
    StepShapeFacetedBrepAndBrepWithVoids, StepShapeGeometricSet, StepShapeManifoldSolidBrep,
    StepShapeShapeDefinitionRepresentation, StepShapeShellBasedSurfaceModel,
};
use crate::step_to_topo_ds::StepToTopoDSBuilder;
use crate::step_visual::{
    StepVisualTessellatedFace, StepVisualTessellatedShell, StepVisualTessellatedSolid,
    StepVisualTessellatedSurfaceSet,
};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Shape};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkxsbase::transfer::{
    TransferActorOfTransientProcessOps, TransferBinder, TransferTransientProcess,
};
use martensite_tkxsbase::transfer_brep::{TransferBRep, TransferBRepShapeBinder};

/// `is_need_representation`: false when the items of `rep_a` are all in another representation of
/// the shape aspect's product (`IsSDRaspect`).
fn is_need_representation(
    aspect: &Handle<StepReprShapeAspect>,
    rep_a: &Handle<StepReprRepresentation>,
    tp: &Handle<TransferTransientProcess>,
) -> Result<bool, Failure> {
    let pds: Handle<StepReprProductDefinitionShape> =
        step_live(aspect, "IsNeedRepresentation")?.of_shape();
    if pds.is_null() {
        return Ok(true);
    }
    let mut is_sdr_aspect = true;
    for sharing in sharing_entities(tp, &pds.cast())? {
        let sdr: Handle<StepShapeShapeDefinitionRepresentation> = sharing.cast();
        let Some(live_sdr) = sdr.get() else {
            continue;
        };
        for shared in shared_entities(tp, &sdr.cast())? {
            let mut rep2: Handle<StepReprRepresentation> = shared.cast();
            if rep2.is_null() {
                let srr: Handle<StepReprShapeRepresentationRelationship> = shared.cast();
                let Some(live_srr) = srr.get() else {
                    continue;
                };
                rep2 = if live_sdr.used_representation() == live_srr.rep1() {
                    live_srr.rep2()
                } else {
                    live_srr.rep1()
                };
            }
            if every_item_is_in(rep_a, &rep2)? {
                is_sdr_aspect = false;
            }
        }
    }
    Ok(is_sdr_aspect)
}

/// Every item of `inner` is an item of `outer`.
fn every_item_is_in(
    inner: &Handle<StepReprRepresentation>,
    outer: &Handle<StepReprRepresentation>,
) -> Result<bool, Failure> {
    let (live_inner, live_outer) = (
        step_live(inner, "IsNeedRepresentation")?,
        step_live(outer, "IsNeedRepresentation")?,
    );
    for rank in 1..=live_inner.nb_items() {
        let item = live_inner.items_value(rank)?;
        let mut found = false;
        for other in 1..=live_outer.nb_items() {
            if item == live_outer.items_value(other)? {
                found = true;
                break;
            }
        }
        if !found {
            return Ok(false);
        }
    }
    Ok(true)
}

impl StepControlActorRead {
    /// `old_way(start, TP, progress)`: a shape_definition_representation read without its
    /// product, when `read.step.product.mode` is off.
    pub(super) fn old_way(
        &self,
        start: &Handle<StandardTransient>,
        tp: &Handle<TransferTransientProcess>,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<TransferBRepShapeBinder>, Failure> {
        let sdr: Handle<StepShapeShapeDefinitionRepresentation> = start.cast();
        let live = step_live(&sdr, "StepControlActorRead::OldWay")?;
        let rep = live.used_representation();
        let definition = live.definition().property_definition();
        if let Some(live_definition) = definition.get() {
            let aspect = live_definition.definition().shape_aspect();
            if aspect.is_some() && !is_need_representation(&aspect, &rep, tp)? {
                return Ok(Handle::default());
            }
        }
        let root = ProgressScope::unnamed(progress, 2.0, false);
        let live_tp = step_live(tp, "StepControlActorRead::OldWay")?;
        let mut binder: Handle<dyn TransferBinder> = live_tp.find(&rep.cast());
        {
            let range = root.next(1.0);
            if binder.is_null() {
                binder = live_tp.transferring(&rep.cast(), &range)?;
            }
        }
        if root.user_break() {
            return Ok(Handle::default());
        }
        let builder = BRepBuilder::default();
        let mut compound = Compound::default();
        builder.make_compound(&mut compound);
        let mut one_result = Shape::default();
        let mut nsh = 0;
        // :j3 abv 23 Oct 98: rp1sd.stp: bind something to protect against loops
        let guard = Handle::new(TransferBRepShapeBinder::new());
        live_tp.bind(start, &guard.cast())?;
        if binder.is_some() {
            let result = TransferBRep::shape_result(&binder);
            if !result.is_null() {
                one_result = result.clone();
                builder.add(&mut compound, &result)?;
                nsh += 1;
                guard.set_result(&result)?;
            }
        }
        let shared: Vec<_> = shared_entities(tp, start)?;
        let sub_range = root.next(1.0);
        let scope = ProgressScope::with_literal(&sub_range, "Sub", shared.len() as f64, false);
        for item in &shared {
            if !scope.more() {
                break;
            }
            let range = scope.next(1.0);
            if !is_exactly::<StepShapeContextDependentShapeRepresentation>(item)
                && !is_exactly::<StepReprShapeRepresentationRelationship>(item)
            {
                continue;
            }
            let found = if live_tp.is_bound(item) {
                live_tp.find(item)
            } else {
                live_tp.transferring(item, &range)?
            };
            let result = TransferBRep::shape_result(&found);
            if !result.is_null() {
                one_result = result.clone();
                builder.add(&mut compound, &result)?;
                nsh += 2; // abv 11.10.00: 2 instead of 1 in order to keep assembly structure
            }
        }
        let made = if nsh == 1 {
            Handle::new(TransferBRepShapeBinder::with_result(&one_result))
        } else {
            Handle::new(TransferBRepShapeBinder::with_result(&compound))
        };
        live_tp.unbind(start); // :j3: unbind start (let it be bound by TransferProcess)
        Ok(made)
    }
    /// Runs the builder on `start`: true when it was a kind the builder knows.
    #[allow(clippy::too_many_arguments, reason = "the branch reads each of these")]
    fn run_shape_builder(
        &self,
        builder: &mut StepToTopoDSBuilder,
        start: &Handle<StandardTransient>,
        tp: &Handle<TransferTransientProcess>,
        local: &StepDataFactors,
        is_manifold: bool,
        only_no_brep: bool,
        has_geom: &mut bool,
        range: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        if start.cast::<StepShapeFacetedBrep>().is_some() {
            builder.init_faceted_brep(&start.cast(), tp, local, range)?;
        } else if start.cast::<StepShapeBrepWithVoids>().is_some() {
            builder.init_brep_with_voids(&start.cast(), tp, local, range)?;
        } else if start.cast::<StepShapeManifoldSolidBrep>().is_some() {
            builder.init_manifold_solid(&start.cast(), tp, local, range)?;
        } else if start.cast::<StepShapeShellBasedSurfaceModel>().is_some() {
            // The tool leaves the state for the call; the builder does not come back to the actor
            // on this path, so nothing can miss it.
            let mut nm_tool = std::mem::take(&mut self.state.write().nm_tool);
            let outcome = builder.init_shell_based_surface_model(
                &start.cast(),
                tp,
                &mut nm_tool,
                local,
                range,
            );
            self.state.write().nm_tool = nm_tool;
            outcome?;
        } else if start
            .cast::<StepShapeFacetedBrepAndBrepWithVoids>()
            .is_some()
        {
            builder.init_faceted_brep_with_voids(&start.cast(), tp, local, range)?;
        } else if start.cast::<StepShapeGeometricSet>().is_some() {
            builder.init_geometric_set(&start.cast(), tp, local, Some(self), is_manifold, range)?;
        } else if start.cast::<StepShapeEdgeBasedWireframeModel>().is_some() {
            builder.init_edge_based_wireframe_model(&start.cast(), tp, local)?;
        } else if start.cast::<StepShapeFaceBasedSurfaceModel>().is_some() {
            builder.init_face_based_surface_model(&start.cast(), tp, local)?;
        } else if start.cast::<StepVisualTessellatedSolid>().is_some() {
            builder.init_tessellated_solid(
                &start.cast(),
                tp,
                only_no_brep,
                has_geom,
                local,
                range,
            )?;
        } else if start.cast::<StepVisualTessellatedShell>().is_some() {
            builder.init_tessellated_shell(
                &start.cast(),
                tp,
                only_no_brep,
                has_geom,
                local,
                range,
            )?;
        } else if start.cast::<StepVisualTessellatedFace>().is_some() {
            builder.init_tessellated_face(&start.cast(), tp, only_no_brep, has_geom, local)?;
        } else if start.cast::<StepVisualTessellatedSurfaceSet>().is_some() {
            builder.init_tessellated_surface_set(&start.cast(), tp, has_geom, local)?;
        } else {
            return Ok(false);
        }
        Ok(true)
    }
    /// `transfer_entity(start, TP, local_factors, isManifold, progress)` on a geometric item.
    pub(super) fn transfer_geometric_item(
        &self,
        git: &Handle<StepGeomGeometricRepresentationItem>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        is_manifold: bool,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<TransferBRepShapeBinder>, Failure> {
        let start: Handle<StandardTransient> = git.cast();
        let live_tp = step_live(tp, "StepControlActorRead")?;
        let nb_tp_items = live_tp.nb_mapped();
        let mut local = *factors;
        let old_context = self.state.read().sr_context.clone();
        self.factors_of_free_item(&start, tp, &mut local)?;
        let mut builder = StepToTopoDSBuilder::new();
        {
            let state = self.state.read();
            builder.root.set_precision(state.precision);
            builder.root.set_max_tol(state.max_tol);
        }
        let scope = ProgressScope::with_literal(
            progress,
            "Transfer stage",
            if is_manifold { 2.0 } else { 1.0 },
            false,
        );
        let only_no_brep = Self::tp_model(tp)?
            .internal_parameters()
            .read_tessellated
            .as_raw()
            == 2;
        let mut has_geom = true;
        let range = scope.next(1.0);
        let found = match self.run_shape_builder(
            &mut builder,
            &start,
            tp,
            &local,
            is_manifold,
            only_no_brep,
            &mut has_geom,
            &range,
        ) {
            Ok(found) => found,
            Err(_) => {
                live_tp.add_fail_text(
                    &start,
                    "Exception is raised. Entity was not translated.",
                    "",
                );
                let empty = Handle::<TransferBRepShapeBinder>::default();
                live_tp.bind(&start, &empty.cast())?;
                return Ok(empty);
            }
        };
        if scope.user_break() {
            return Ok(Handle::default());
        }
        let mut mapped = Shape::default();
        if found && builder.is_done() {
            mapped = builder.value()?.clone();
            if is_manifold && has_geom {
                let mut processor = self.new_shape_processor(false);
                let flags = self.shape_processing().flags().0;
                mapped = processor.process_shape(&mapped, flags, &scope.next(1.0))?;
                processor.merge_transfer_info(tp, nb_tp_items)?;
            }
        }
        let binder = if mapped.is_null() {
            Handle::default()
        } else {
            Handle::new(TransferBRepShapeBinder::with_result(&mapped))
        };
        if old_context.is_null() && self.state.read().sr_context.is_some() {
            self.prepare_units(&old_context, tp, &mut local)?; // :S4136
        }
        live_tp.bind(&start, &binder.cast())?;
        Ok(binder)
    }
}
