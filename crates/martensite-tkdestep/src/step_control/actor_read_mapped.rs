//! `StepControlActorRead`: the mapped item and face surface overloads of `transfer_entity`.
use super::StepControlActorRead;
use super::actor_read_support::{apply_transformation, find_context, sharing_entities};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::{StepGeomAxis2Placement3d, StepGeomCartesianTransformationOperator3d};
use crate::step_repr::{StepReprMappedItem, StepReprRepresentationItem};
use crate::step_shape::{
    StepShapeFaceSurface, StepShapeShapeRepresentation, StepShapeTopologicalRepresentationItem,
};
use crate::step_to_geom::StepToGeom;
use crate::step_to_topo_ds::{StepToTopoDSNMTool, StepToTopoDSTool, StepToTopoDSTranslateFace};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Shape};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::Transform3;
use martensite_tkxsbase::transfer::{
    TransferActorOfTransientProcessOps, TransferBinder, TransferTransientProcess,
};
use martensite_tkxsbase::transfer_brep::{TransferBRep, TransferBRepShapeBinder};

impl StepControlActorRead {
    /// `transfer_entity(mapit, TP, local_factors, progress)`.
    pub(super) fn transfer_mapped_item(
        &self,
        mapit: &Handle<StepReprMappedItem>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<TransferBRepShapeBinder>, Failure> {
        let live = step_live(mapit, "StepControlActorRead")?;
        let live_tp = step_live(tp, "StepControlActorRead")?;
        let source = live.mapping_source();
        let live_source = step_live(&source, "StepControlActorRead")?;
        let maprep: Handle<StepShapeShapeRepresentation> =
            live_source.mapped_representation().cast();
        let root = ProgressScope::unnamed(progress, 2.0, false);
        let mut found: Handle<dyn TransferBinder> = live_tp.find(&maprep.cast());
        if found.is_null() {
            // The whole range goes to this call too.
            let mut is_bound = false;
            found = self
                .transfer_shape_representation(
                    &maprep,
                    tp,
                    factors,
                    &mut is_bound,
                    false,
                    progress,
                )?
                .cast();
        }
        let mut binder: Handle<TransferBRepShapeBinder> = found.cast();
        match binder.get() {
            None => live_tp.add_warning_text(&mapit.cast(), "No Shape Produced", ""),
            Some(live_binder) => {
                let mut mapped = live_binder.result();
                if !mapped.is_null() {
                    let (ok, trsf) =
                        self.mapped_item_transformation(live, live_source, &maprep, tp, factors)?;
                    if ok {
                        apply_transformation(&mut mapped, &trsf)?;
                    } else {
                        let text = "Mapped Item, case not recognized, location ignored";
                        live_tp.add_warning_text(&mapit.cast(), text, "");
                    }
                    binder = Handle::new(TransferBRepShapeBinder::with_result(&mapped));
                }
            }
        }
        let shape_builder = BRepBuilder::default();
        let mut compound = Compound::default();
        shape_builder.make_compound(&mut compound);
        let mut result = Shape::default();
        if binder.is_some() {
            let as_binder: Handle<dyn TransferBinder> = binder.cast();
            result = TransferBRep::shape_result(&as_binder);
            shape_builder.add(&mut compound, &result)?;
        }
        let model = Self::tp_model(tp)?;
        let (read_srr, read_constructive) = {
            let parameters = model.internal_parameters();
            (
                parameters.read_relationship,
                parameters.read_constr_relation,
            )
        };
        if read_srr {
            let nb_srr = sharing_entities(tp, &maprep.cast())?.len();
            let part_range = root.next(1.0);
            let scope = ProgressScope::with_literal(&part_range, "Part", nb_srr as f64, false);
            let related = self.transfer_related_srr(
                tp,
                &maprep,
                false,
                read_constructive,
                factors,
                &mut compound,
                &scope,
            )?;
            if !related.is_null() {
                result = related;
            }
            let children = compound.nb_children();
            if children == 1 {
                binder = Handle::new(TransferBRepShapeBinder::with_result(&result));
            } else if children > 1 {
                binder = Handle::new(TransferBRepShapeBinder::with_result(&compound));
            }
        }
        live_tp.bind(&mapit.cast(), &binder.cast())?;
        Ok(binder)
    }
    /// The transformation of a mapped_item, and whether its case was recognized.
    pub(super) fn mapped_item_transformation(
        &self,
        live: &StepReprMappedItem,
        source: &crate::step_repr::StepReprRepresentationMap,
        maprep: &Handle<StepShapeShapeRepresentation>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
    ) -> Result<(bool, Transform3), Failure> {
        let mut trsf = Transform3::default();
        let target: Handle<StepReprRepresentationItem> = live.mapping_target();
        let operator: Handle<StepGeomCartesianTransformationOperator3d> = target.cast();
        if operator.is_some() {
            return Ok(
                match StepToGeom::make_transformation_3d(&operator, factors)? {
                    Some(made) => (true, made),
                    None => (false, trsf),
                },
            );
        }
        let origin: Handle<StepGeomAxis2Placement3d> = source.mapping_origin().cast();
        let placed: Handle<StepGeomAxis2Placement3d> = target.cast();
        if origin.is_null() || placed.is_null() {
            return Ok((false, trsf));
        }
        let rep = self.state.read().sr_context.clone(); // NOTE: copy of handle !
        self.compute_transformation(
            &origin,
            &placed,
            &maprep.cast(),
            &rep,
            tp,
            &mut trsf,
            factors,
        )?;
        Ok((true, trsf))
    }
    /// `transfer_entity(fs, TP, local_factors, progress)`.
    pub(super) fn transfer_face_surface(
        &self,
        fs: &Handle<StepShapeFaceSurface>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<TransferBRepShapeBinder>, Failure> {
        let live_tp = step_live(tp, "StepControlActorRead")?;
        let nb_tp_items = live_tp.nb_mapped();
        match self.translate_face_surface(fs, tp, factors, nb_tp_items, progress) {
            Ok(binder) => {
                live_tp.bind(&fs.cast(), &binder.cast())?;
                Ok(binder)
            }
            Err(_) => {
                let text = "Exception is raised. Entity was not translated.";
                live_tp.add_fail_text(&fs.cast(), text, "");
                let empty = Handle::<TransferBRepShapeBinder>::default();
                live_tp.bind(&fs.cast(), &empty.cast())?;
                Ok(empty)
            }
        }
    }
    pub(super) fn translate_face_surface(
        &self,
        fs: &Handle<StepShapeFaceSurface>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        nb_tp_items: i32,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<TransferBRepShapeBinder>, Failure> {
        let mut tool = StepToTopoDSTool::new();
        let map: DataMap<Handle<StepShapeTopologicalRepresentationItem>, Shape> =
            DataMap::default();
        tool.init(&map, tp);
        let mut translator = StepToTopoDSTranslateFace::new();
        {
            let state = self.state.read();
            translator.root.set_precision(state.precision);
            translator.root.set_max_tol(state.max_tol);
        }
        let mut dummy = StepToTopoDSNMTool::new();
        translator.init(fs, &mut tool, &mut dummy, factors)?;
        let old_context = self.state.read().sr_context.clone();
        let mut local = factors.clone();
        self.factors_of_free_item(&fs.cast(), tp, &mut local)?;
        let live_tp = step_live(tp, "StepControlActorRead")?;
        let found: Handle<TransferBRepShapeBinder> = live_tp.find(&fs.cast()).cast();
        if let Some(live) = found.get() {
            let shape = live.result();
            if !shape.is_null() {
                let mut processor = self.new_shape_processor(false);
                let flags = self.shape_processing().flags().0;
                let processed = processor.process_shape(&shape, flags, progress)?;
                processor.merge_transfer_info(tp, nb_tp_items)?;
                if processed != shape {
                    live.set_result(&processed)?;
                }
                processor.merge_transfer_info(tp, nb_tp_items)?;
            }
        }
        if old_context.is_null() && self.state.read().sr_context.is_some() {
            self.prepare_units(&old_context, tp, &mut local)?; // :S4136
        }
        Ok(found)
    }
    /// The factors of an item out of a representation: if no context, try to find it (ex:
    /// r0701_ug.stp #4790).
    pub(super) fn factors_of_free_item(
        &self,
        start: &Handle<StandardTransient>,
        tp: &Handle<TransferTransientProcess>,
        local: &mut StepDataFactors,
    ) -> Result<(), Failure> {
        if self.state.read().sr_context.is_some() {
            return Ok(());
        }
        let context = find_context(start, tp, 10)?;
        if context.is_null() {
            let text = "Entity with no unit context; default units taken";
            step_live(tp, "StepControlActorRead")?.add_warning_text(start, text, "");
            let model = Self::tp_model(tp)?;
            return self.reset_units(&model, local);
        }
        self.prepare_units(&context, tp, local)
    }
}
