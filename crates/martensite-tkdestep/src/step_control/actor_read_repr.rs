//! `StepControlActorRead`: the representation and relationship overloads of `transfer_entity`.
use super::StepControlActorRead;
use super::actor_read_support::{apply_transformation, sharing_entities};
use crate::step_construct::StepConstructAssembly;
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::StepGeomAxis2Placement3d;
use crate::step_repr::{
    StepReprItemDefinedTransformation, StepReprRepresentationRelationship,
    StepReprShapeRepresentationRelationship,
};
use crate::step_shape::{
    StepShapeContextDependentShapeRepresentation, StepShapeNonManifoldSurfaceShapeRepresentation,
    StepShapeShapeRepresentation,
};
use crate::step_to_geom::StepToGeom;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Compound, Shape, Solid};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis3, Direction3, Point3, Transform3, TransformForm};
use martensite_tkmath::toploc::Location;
use martensite_tkxsbase::transfer::{
    TransferActorOfTransientProcessOps, TransferActorOps, TransferBinder, TransferTransientProcess,
};
use martensite_tkxsbase::transfer_brep::{TransferBRep, TransferBRepShapeBinder};

impl StepControlActorRead {
    /// Whether this shape_representation reads as non-manifold, and the tool made ready for it.
    fn prepare_manifold_mode(
        &self,
        sr: &Handle<StepShapeShapeRepresentation>,
        tp: &Handle<TransferTransientProcess>,
    ) -> Result<bool, Failure> {
        let model = Self::tp_model(tp)?;
        let (nm_mode, ideas_mode) = {
            let parameters = model.internal_parameters();
            (parameters.read_nonmanifold, parameters.read_ideas)
        };
        let mut state = self.state.write();
        let mut is_manifold = true;
        if nm_mode
            && sr
                .cast::<StepShapeNonManifoldSurfaceShapeRepresentation>()
                .is_some()
        {
            is_manifold = false;
            state.nm_detected = true;
        } else if nm_mode && state.nm_tool.is_ideas_case() && ideas_mode {
            is_manifold = false;
            state.nm_detected = true;
        }
        state.nm_tool.clean_up();
        state.nm_tool.set_active(!is_manifold && nm_mode);
        Ok(is_manifold)
    }
    /// The transformation of a placement item the root representation holds (`theUseTrsf`):
    /// `None` when the item is not such a placement and has to be transferred.
    fn root_placement_trsf(
        tp: &Handle<TransferTransientProcess>,
        item: &Handle<StandardTransient>,
        factors: &StepDataFactors,
        trsf: &mut Transform3,
    ) -> Result<Option<()>, Failure> {
        let placement: Handle<StepGeomAxis2Placement3d> = item.cast();
        if placement.is_null() {
            return Ok(None);
        }
        let used = sharing_entities(tp, item)?
            .iter()
            .any(|ent| ent.cast::<StepReprItemDefinedTransformation>().is_some());
        if used {
            return Ok(None);
        }
        let target = StepToGeom::make_axis2_placement(&placement, factors)?;
        let Some(live) = target.get() else {
            return Ok(Some(()));
        };
        let orig = Axis3::new(
            Point3::new(0.0, 0.0, 0.0),
            Direction3::new(0.0, 0.0, 1.0)?,
            Direction3::new(1.0, 0.0, 0.0)?,
        )?;
        let targ = Axis3::from_axis2(live.ax2()?);
        let same = targ.location().square_distance(&orig.location())
            < precision::square_confusion()
            && targ
                .direction()
                .is_equal(&orig.direction(), precision::angular())
            && targ
                .x_direction()
                .is_equal(&orig.x_direction(), precision::angular());
        if !same {
            trsf.set_transformation_between(targ, orig);
        }
        Ok(Some(()))
    }
    /// `transfer_entity(sr, TP, local_factors, isBound, use_trsf, progress)`.
    pub(super) fn transfer_shape_representation(
        &self,
        sr: &Handle<StepShapeShapeRepresentation>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        is_bound: &mut bool,
        use_trsf: bool,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<TransferBRepShapeBinder>, Failure> {
        self.state.write().nm_detected = false;
        if !self.recognize(&sr.cast()) {
            return Ok(Handle::default());
        }
        *is_bound = false;
        let live = step_live(sr, "StepControlActorRead")?;
        let live_tp = step_live(tp, "StepControlActorRead")?;
        let nb = live.nb_items();
        let nb_tp_items = live_tp.nb_mapped();
        let old_context = self.state.read().sr_context.clone(); // :S4136
        let mut local = factors.clone();
        self.prepare_units(&sr.cast(), tp, &mut local)?;
        let builder = BRepBuilder::default();
        let mut comp = Compound::default();
        builder.make_compound(&mut comp);
        let mut one_result = Shape::default();
        let mut nsh = 0;
        let is_manifold = self.prepare_manifold_mode(sr, tp)?;
        let mut trsf = Transform3::default();
        let root_scope = ProgressScope::with_literal(
            progress,
            "Sub-assembly",
            if is_manifold { 1.0 } else { 2.0 },
            false,
        );
        let root_range = root_scope.next(1.0);
        let scope = ProgressScope::with_literal(&root_range, "Transfer", f64::from(nb), false);
        let mut compounded = IndexedMapOfShape::default();
        let mut rank = 1;
        while rank <= nb && scope.more() {
            let range = scope.next(1.0);
            let item: Handle<StandardTransient> = live.items_value(rank)?.cast();
            rank += 1;
            if item.is_null() {
                continue;
            }
            if use_trsf && Self::root_placement_trsf(tp, &item, &local, &mut trsf)?.is_some() {
                continue;
            }
            let binder = if live_tp.is_bound(&item) {
                *is_bound = true;
                live_tp.find(&item)
            } else {
                self.transfer_shape(&item, tp, &local, is_manifold, false, &range)?
            };
            let result = TransferBRep::shape_result(&binder);
            if !result.is_null() {
                one_result = result.clone();
                if !compounded.contains(&result) {
                    compounded.add(result.clone());
                    topexp::map_shapes(&result, &mut compounded, false, false);
                    builder.add(&mut comp, &result)?;
                    nsh += 1;
                }
            }
        }
        if !is_manifold {
            let process_range = root_scope.next(1.0);
            comp = self.process_non_manifold(comp, tp, nb_tp_items, &process_range)?;
        }
        let binder = if nsh == 0 {
            Handle::default()
        } else if nsh == 1 {
            if trsf.form() != TransformForm::Identity {
                one_result.move_by(&Location::new(trsf), false)?;
            }
            Handle::new(TransferBRepShapeBinder::with_result(&one_result))
        } else {
            if trsf.form() != TransformForm::Identity {
                comp.move_by(&Location::new(trsf), false)?;
            }
            Handle::new(TransferBRepShapeBinder::with_result(&comp))
        };
        self.prepare_units(&old_context, tp, &mut local)?; // :S4136
        live_tp.bind(&sr.cast(), &binder.cast())?;
        Ok(binder)
    }
    /// The healed compound of a non-manifold representation, with its closed shells made solids.
    fn process_non_manifold(
        &self,
        comp: Compound,
        tp: &Handle<TransferTransientProcess>,
        nb_tp_items: i32,
        progress: &ProgressRange<'_>,
    ) -> Result<Compound, Failure> {
        let builder = BRepBuilder::default();
        let scope = ProgressScope::with_literal(progress, "Process", 1.0, false);
        let mut processor = self.new_shape_processor(true);
        let flags = self.shape_processing().flags().0;
        let fixed = processor.process_shape(&comp, flags, &scope.next(1.0))?;
        processor.merge_transfer_info(tp, nb_tp_items)?;
        let mut current = if fixed.shape_type()? == ShapeEnum::Compound {
            topods::compound(&fixed)?
        } else {
            let mut made = Compound::default();
            builder.make_compound(&mut made);
            builder.add(&mut made, &fixed)?;
            made
        };
        if self.state.read().nm_tool.is_ideas_case() {
            current = self.ideas_closed_compound(&current)?;
        }
        let mut reconstructed = Compound::default();
        builder.make_compound(&mut reconstructed);
        let mut shells = Explorer::new(&current, ShapeEnum::Shell, ShapeEnum::Shape)?;
        while shells.more() {
            let shell = shells.current();
            if shell.closed()? {
                let mut solid = Solid::default();
                builder.make_solid(&mut solid);
                builder.add(&mut solid, shell)?;
                builder.add(&mut reconstructed, &solid)?;
            } else {
                builder.add(&mut reconstructed, shell)?;
            }
            shells.next()?;
        }
        Ok(reconstructed)
    }
    /// `transfer_entity(CDSR, TP, local_factors, progress)`.
    pub(super) fn transfer_cdsr(
        &self,
        cdsr: &Handle<StepShapeContextDependentShapeRepresentation>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<TransferBRepShapeBinder>, Failure> {
        let live = step_live(cdsr, "StepControlActorRead")?;
        let srr: Handle<StepReprShapeRepresentationRelationship> =
            live.representation_relation().cast();
        let Some(live_srr) = srr.get() else {
            return Ok(Handle::default());
        };
        let live_tp = step_live(tp, "StepControlActorRead")?;
        let reversed = {
            let guard = live_tp.graph()?;
            let graph = guard.get();
            StepConstructAssembly::check_srr_reverses_nauo(&graph, cdsr)?
        };
        let rep1 = if reversed {
            live_srr.rep2()
        } else {
            live_srr.rep1()
        };
        let rep: Handle<StepShapeShapeRepresentation> = rep1.cast();
        if reversed {
            let text = "SRR reverses relation defined by NAUO; NAUO definition is taken";
            live_tp.add_warning_text(&srr.cast(), text, "");
        }
        let mut trsf = Transform3::default();
        let relation: Handle<StepReprRepresentationRelationship> = srr.cast();
        let iatrsf = self.compute_srrwt(&relation, tp, &mut trsf, factors)?;
        let binder: Handle<dyn TransferBinder> = if live_tp.is_bound(&rep.cast()) {
            live_tp.find(&rep.cast())
        } else {
            let mut is_bound = false;
            self.transfer_shape_representation(&rep, tp, factors, &mut is_bound, false, progress)?
                .cast()
        };
        let mut result = TransferBRep::shape_result(&binder);
        let made = if result.is_null() {
            Handle::default()
        } else {
            if iatrsf {
                let applied = if reversed { trsf.inverted()? } else { trsf };
                apply_transformation(&mut result, &applied)?;
            }
            Handle::new(TransferBRepShapeBinder::with_result(&result))
        };
        live_tp.bind(&cdsr.cast(), &made.cast())?;
        Ok(made)
    }
    /// `transfer_entity(und, TP, local_factors, nbrep, use_trsf, progress)`.
    pub(super) fn transfer_srr(
        &self,
        und: &Handle<StepReprShapeRepresentationRelationship>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        nb_rep: i32,
        use_trsf: bool,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<TransferBRepShapeBinder>, Failure> {
        let Some(live) = und.get() else {
            return Ok(Handle::default());
        };
        let live_tp = step_live(tp, "StepControlActorRead")?;
        let builder = BRepBuilder::default();
        let mut compound = Compound::default();
        builder.make_compound(&mut compound);
        let mut one_result = Shape::default();
        let mut nsh = 0;
        let mut trsf = Transform3::default();
        let relation: Handle<StepReprRepresentationRelationship> = und.cast();
        let iatrsf = self.compute_srrwt(&relation, tp, &mut trsf, factors)?;
        let scope = ProgressScope::unnamed(progress, 2.0, false);
        for rank in 1..=2 {
            if !scope.more() {
                break;
            }
            let range = scope.next(1.0);
            if nb_rep != 0 && nb_rep != rank {
                continue;
            }
            let rep = if rank == 1 { live.rep1() } else { live.rep2() };
            let item: Handle<StepShapeShapeRepresentation> = rep.cast();
            let binder: Handle<dyn TransferBinder> = if live_tp.is_bound(&item.cast()) {
                live_tp.find(&item.cast())
            } else {
                let mut is_bound = false;
                self.transfer_shape_representation(
                    &item,
                    tp,
                    factors,
                    &mut is_bound,
                    use_trsf,
                    &range,
                )?
                .cast()
            };
            let result = TransferBRep::shape_result(&binder);
            if !result.is_null() {
                one_result = result.clone();
                builder.add(&mut compound, &result)?;
                nsh += 1;
            }
        }
        if iatrsf {
            if nsh == 1 {
                apply_transformation(&mut one_result, &trsf)?;
            } else if nsh > 1 {
                apply_transformation(&mut compound, &trsf)?;
            }
        }
        let binder = match nsh {
            0 => Handle::default(),
            1 => Handle::new(TransferBRepShapeBinder::with_result(&one_result)),
            _ => Handle::new(TransferBRepShapeBinder::with_result(&compound)),
        };
        live_tp.bind(&und.cast(), &binder.cast())?;
        Ok(binder)
    }
}
