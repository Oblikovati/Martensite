//! `StepControlActorRead`: the constructive geometry and drawing relationships.
use super::StepControlActorRead;
use super::actor_read_support::{is_exactly, sharing_entities};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::StepGeomAxis2Placement3d;
use crate::step_repr::{
    StepReprConstructiveGeometryRepresentation,
    StepReprConstructiveGeometryRepresentationRelationship,
    StepReprMechanicalDesignAndDraughtingRelationship, StepReprRepresentation,
    StepReprRepresentationRelationship, StepReprShapeRepresentationRelationship,
};
use crate::step_shape::StepShapeShapeRepresentation;
use crate::step_to_geom::StepToGeom;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Face, Shape};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomPlane;
use martensite_tkmath::gp::Axis3;
use martensite_tkxsbase::transfer::{TransferActorOps, TransferBinder, TransferTransientProcess};
use martensite_tkxsbase::transfer_brep::{TransferBRep, TransferBRepShapeBinder};

impl StepControlActorRead {
    /// `transfer_entity(cgrr, tp, local_factors)`: axis placements become planar faces.
    pub(super) fn transfer_cgrr(
        &self,
        cgrr: &Handle<StepReprConstructiveGeometryRepresentationRelationship>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
    ) -> Result<Handle<TransferBRepShapeBinder>, Failure> {
        let Some(live) = cgrr.get() else {
            return Ok(Handle::default());
        };
        let live_tp = step_live(tp, "StepControlActorRead")?;
        let old_context = self.state.read().sr_context.clone();
        let mut local = factors.clone();
        let mut reset_units = false;
        let builder = BRepBuilder::default();
        let mut compound = Compound::default();
        builder.make_compound(&mut compound);
        for rank in 1..=2 {
            let rep = if rank == 1 { live.rep1() } else { live.rep2() };
            let constructive: Handle<StepReprConstructiveGeometryRepresentation> = rep.cast();
            let Some(live_rep) = constructive.get() else {
                continue;
            };
            let sr_context = self.state.read().sr_context.clone();
            if sr_context
                .get()
                .is_none_or(|known| live_rep.context_of_items() != known.context_of_items())
            {
                self.prepare_units(&rep, tp, &mut local)?;
                reset_units = true;
            }
            for item_rank in 1..=live_rep.nb_items() {
                let axis: Handle<StepGeomAxis2Placement3d> =
                    live_rep.items_value(item_rank)?.cast();
                if axis.is_null() {
                    continue;
                }
                let made = StepToGeom::make_axis2_placement(&axis, &local)?;
                let Some(placement) = made.get() else {
                    continue;
                };
                let plane = Handle::new(GeomPlane::new(Axis3::from_axis2(placement.ax2()?)));
                let mut face = Face::default();
                builder.make_face_on_surface(&mut face, &plane.cast(), precision::confusion())?;
                let axis_binder = Handle::new(TransferBRepShapeBinder::with_result(&face));
                live_tp.bind(&axis.cast(), &axis_binder.cast())?;
                builder.add(&mut compound, &face)?;
            }
        }
        let binder = Handle::new(TransferBRepShapeBinder::with_result(&compound));
        self.state.write().sr_context = old_context.clone();
        if old_context.is_null() || reset_units {
            self.prepare_units(&old_context, tp, &mut local)?;
        }
        live_tp.bind(&cgrr.cast(), &binder.cast())?;
        Ok(binder)
    }
    /// `transfer_entity(mdadr, tp, local_factors, progress)`.
    pub(super) fn transfer_mdadr(
        &self,
        mdadr: &Handle<StepReprMechanicalDesignAndDraughtingRelationship>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<TransferBRepShapeBinder>, Failure> {
        let Some(live) = mdadr.get() else {
            return Ok(Handle::default());
        };
        let live_tp = step_live(tp, "StepControlActorRead")?;
        let old_context = self.state.read().sr_context.clone();
        let mut local = factors.clone();
        let mut reset_units = false;
        let builder = BRepBuilder::default();
        let mut compound = Compound::default();
        builder.make_compound(&mut compound);
        let scope = ProgressScope::unnamed(progress, 2.0, false);
        for rank in 1..=2 {
            let range = scope.next(1.0);
            let rep: Handle<StepReprRepresentation> =
                if rank == 1 { live.rep1() } else { live.rep2() };
            let Some(live_rep) = rep.get() else {
                continue;
            };
            let sr_context = self.state.read().sr_context.clone();
            if sr_context
                .get()
                .is_none_or(|known| live_rep.context_of_items() != known.context_of_items())
            {
                self.prepare_units(&rep, tp, &mut local)?;
                reset_units = true;
            }
            let shape_rep: Handle<StepShapeShapeRepresentation> = rep.cast();
            let mut binder: Handle<dyn TransferBinder> = Handle::default();
            if shape_rep.is_some() {
                binder = if live_tp.is_bound(&shape_rep.cast()) {
                    live_tp.find(&shape_rep.cast())
                } else {
                    let mut is_bound = false;
                    self.transfer_shape_representation(
                        &shape_rep,
                        tp,
                        factors,
                        &mut is_bound,
                        false,
                        &range,
                    )?
                    .cast()
                };
            }
            let result = TransferBRep::shape_result(&binder);
            if !result.is_null() {
                builder.add(&mut compound, &result)?;
            }
        }
        let binder = Handle::new(TransferBRepShapeBinder::with_result(&compound));
        self.state.write().sr_context = old_context.clone();
        if old_context.is_null() || reset_units {
            self.prepare_units(&old_context, tp, &mut local)?;
        }
        live_tp.bind(&mdadr.cast(), &binder.cast())?;
        Ok(binder)
    }
    /// `transfer_related_srr(tp, rep, use_trsf, read_constructive_geom_rr, local_factors, cund, ps)`.
    #[allow(clippy::too_many_arguments, reason = "the member takes each of these")]
    pub(super) fn transfer_related_srr(
        &self,
        tp: &Handle<TransferTransientProcess>,
        rep: &Handle<StepShapeShapeRepresentation>,
        use_trsf: bool,
        read_constructive: bool,
        factors: &StepDataFactors,
        compound: &mut Compound,
        scope: &ProgressScope<'_>,
    ) -> Result<Shape, Failure> {
        let builder = BRepBuilder::default();
        let mut result = Shape::default();
        let as_rep: Handle<StepReprRepresentation> = rep.cast();
        for item in sharing_entities(tp, &rep.cast())? {
            if !scope.more() {
                break;
            }
            if item.cast::<StepReprRepresentationRelationship>().is_null() {
                continue;
            }
            let binder: Handle<dyn TransferBinder> =
                if is_exactly::<StepReprShapeRepresentationRelationship>(&item) {
                    let srr: Handle<StepReprShapeRepresentationRelationship> = item.cast();
                    let is_rep1 = as_rep == step_live(&srr, "StepControlActorRead")?.rep1();
                    let nb_rep = if is_rep1 { 2 } else { 1 };
                    self.transfer_srr(&srr, tp, factors, nb_rep, use_trsf, &scope.next(1.0))?
                        .cast()
                } else if is_exactly::<StepReprMechanicalDesignAndDraughtingRelationship>(&item) {
                    self.transfer_mdadr(&item.cast(), tp, factors, &scope.next(1.0))?
                        .cast()
                } else if read_constructive
                    && is_exactly::<StepReprConstructiveGeometryRepresentationRelationship>(&item)
                {
                    self.transfer_cgrr(&item.cast(), tp, factors)?.cast()
                } else {
                    Handle::default()
                };
            if binder.is_some() {
                result = TransferBRep::shape_result(&binder);
                builder.add(compound, &result)?;
            }
        }
        Ok(result)
    }
}
