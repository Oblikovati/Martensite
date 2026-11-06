//! `StepControlActorWrite::transfer_compound` and `transfer_sub_shape`: compounds to assemblies.
use super::StepControlActorWrite;
use super::actor_write::roots_of;
use super::actor_write_items::used_tolerance;
use super::actor_write_shape::{is_manifold_shape, shared_sequence};
use crate::geom_to_step::{
    GeomToStepMakeAxis2Placement3d, GeomToStepMakeCartesianTransformationOperator, shared_array1,
};
use crate::step_construct::{StepConstructAssembly, StepConstructPart, StepConstructUnitContext};
use crate::step_data::{StepDataFactors, StepDataStepModel};
use crate::step_entity::step_live;
use crate::step_geom::StepGeomGeometricRepresentationItem;
use crate::step_repr::StepReprRepresentationItem;
use crate::step_shape::{StepShapeShapeDefinitionRepresentation, StepShapeShapeRepresentation};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{self, Compound, Shape, ShapeIterator};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::HSequence;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Transform3;
use martensite_tkmath::toploc::Location;
use martensite_tkxsbase::transfer::{
    TransferActorOps, TransferBinder, TransferFinder, TransferFinderProcess,
    TransferSimpleBinderOfTransient,
};
use martensite_tkxsbase::transfer_brep::{TransferBRep, TransferBRepShapeMapper};

const WHERE: &str = "StepControlActorWrite";

/// The children to write of a compound, its free vertices in one compound at the end, and the
/// shells that a non-manifold shape groups.
fn compound_children(
    shape: &Shape,
    is_manifold: bool,
    separate_vertices: bool,
) -> Result<(Vec<Shape>, Vec<Shape>), Failure> {
    let builder = BRepBuilder::default();
    let mut vertices = Compound::default();
    builder.make_compound(&mut vertices);
    let (mut items, mut group) = (Vec::new(), Vec::new());
    let mut nb_free_vertices = 0;
    let mut children = ShapeIterator::new(shape, true, true);
    while children.more() {
        let sub = children.value()?.clone();
        children.next();
        let kind = sub.shape_type()?;
        if kind == ShapeEnum::Vertex && separate_vertices {
            builder.add(&mut vertices, &sub)?;
            nb_free_vertices += 1;
            continue;
        }
        if !is_manifold && kind == ShapeEnum::Solid {
            let mut shells = ShapeIterator::new(&sub, true, true);
            while shells.more() {
                let shell = topods::shell(shells.value()?)?;
                shell.set_closed(true)?;
                items.push(shell.clone().into_shape());
                group.push(shell.into_shape());
                shells.next();
            }
        } else if !is_manifold && matches!(kind, ShapeEnum::Shell | ShapeEnum::Face) {
            items.push(sub.clone());
            group.push(sub);
        } else {
            items.push(sub);
        }
    }
    if nb_free_vertices > 0 {
        items.push(vertices.into_shape());
    }
    Ok((items, group))
}

impl StepControlActorWrite {
    /// `transfer_compound(start, SDR0, FP, local_factors, progress)`.
    pub fn transfer_compound(
        &self,
        start: &Handle<dyn TransferFinder>,
        sdr0: &Handle<StepShapeShapeDefinitionRepresentation>,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<dyn TransferBinder>, Failure> {
        let mapper: Handle<TransferBRepShapeMapper> = start.cast();
        let Some(live_mapper) = mapper.get() else {
            return Ok(Handle::default());
        };
        let the_shape = live_mapper.value().clone();
        let model: Handle<StepDataStepModel> = step_live(fp, WHERE)?.model().cast();
        let live_model = step_live(&model, WHERE)?;
        let (nm_mode, separate) = {
            let parameters = live_model.internal_parameters();
            (
                parameters.write_nonmanifold,
                parameters.write_vertex_mode.as_raw() == 0,
            )
        };
        let is_manifold = !nm_mode || is_manifold_shape(&the_shape)?;
        let (rep_items, group) = compound_children(&the_shape, is_manifold, separate)?;
        let group_handle: Handle<HSequence<Shape>> = shared_sequence(group);
        let live_sdr = step_live(sdr0, WHERE)?;
        let mut shape_rep: Handle<StepShapeShapeRepresentation> =
            live_sdr.used_representation().cast();
        if shape_rep.is_null() {
            shape_rep = Handle::new(StepShapeShapeRepresentation::new());
            live_sdr.set_used_representation(&shape_rep.cast()); // to be used by MakeItem
        }
        // set SDR as first item in order to be found first (but not SDR of subshape!)
        let binder: Handle<dyn TransferBinder> = self.transient_result(&sdr0.cast()).cast();
        binder.add_result(&self.transient_result(&shape_rep.cast()).cast());
        let mut item_seq: Vec<Handle<StepReprRepresentationItem>> =
            vec![self.default_axis()?.cast()];
        self.state.write().my_context.next_level();
        let scope = ProgressScope::unnamed(progress, rep_items.len() as f64, false);
        for item in &rep_items {
            if !scope.more() {
                break;
            }
            let subs: Handle<dyn TransferFinder> = TransferBRep::shape_mapper(fp, item)?.cast();
            let mut ax1 = Handle::<StepGeomGeometricRepresentationItem>::default();
            let mut bnd = self.transfer_sub_shape(
                &subs,
                sdr0,
                &mut ax1,
                fp,
                factors,
                &group_handle,
                is_manifold,
                &scope.next(1.0),
            )?;
            if ax1.is_some() {
                item_seq.push(ax1.cast());
            }
            // copy binders so as to have only roots in upper binder (shape mapping is lost)
            while bnd.is_some() {
                let simple: Handle<TransferSimpleBinderOfTransient> = bnd.cast();
                if let Some(live) = simple.get() {
                    binder.add_result(&self.transient_result(&live.result()).cast());
                }
                bnd = bnd.next_result();
            }
        }
        self.state.write().my_context.prev_level();
        let live_rep = step_live(&shape_rep, WHERE)?;
        live_rep.set_items(&shared_array1(item_seq));
        let tolerance = used_tolerance(live_model, self.state.read().mytoler, &the_shape)?;
        let mut context = StepConstructUnitContext::new();
        context.init(tolerance, &model, factors)?;
        live_rep.set_context_of_items(&context.value().cast()); // la tolerance, voir au debut
        live_rep.set_name(&Handle::new(HAsciiString::new("")));
        Ok(binder)
    }
    /// The sub-shape's transformation made an assembly link to `sdr0`; the placement goes to `ax1`.
    #[allow(clippy::too_many_arguments, reason = "the block reads each of these")]
    fn make_assembly_link(
        &self,
        assembly: &mut StepConstructAssembly,
        sdr: &Handle<StepShapeShapeDefinitionRepresentation>,
        sdr0: &Handle<StepShapeShapeDefinitionRepresentation>,
        ax1: &mut Handle<StepGeomGeometricRepresentationItem>,
        location: &Transform3,
        start: &Handle<dyn TransferFinder>,
        fp: &Handle<TransferFinderProcess>,
        model: &StepDataStepModel,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let process = step_live(fp, WHERE)?;
        let write_scaling = model.internal_parameters().write_scaling_trsf;
        let scale_gap = (location.scale_factor() - 1.0).abs();
        if scale_gap > precision::confusion() {
            let text = if write_scaling {
                "The shape has a scaling factor, exported as cartesian_transformation_operator"
            } else {
                "The shape has a scaling factor, skipped"
            };
            process.add_warning_text(start, text, "");
        }
        if scale_gap < precision::confusion() || !write_scaling {
            let placed = GeomToStepMakeAxis2Placement3d::from_trsf(location, factors)?.value()?;
            assembly.init(sdr, sdr0, &self.default_axis()?, &placed)?;
            *ax1 = placed.cast();
        } else {
            let maker = GeomToStepMakeCartesianTransformationOperator::new(location, factors)?;
            if !maker.is_done() {
                process.add_fail_text(start, "The transformation relation creation failed", "");
            }
            let operator = maker.value()?;
            assembly.init_trsf(sdr, sdr0, &operator.cast())?;
            *ax1 = operator.cast();
        }
        assembly.make_relationship()
    }
    /// `transfer_sub_shape(start, SDR0, AX1, FP, local_factors, shapeGroup, isManifold, progress)`.
    #[allow(clippy::too_many_arguments, reason = "the member takes each of these")]
    pub fn transfer_sub_shape(
        &self,
        start: &Handle<dyn TransferFinder>,
        sdr0: &Handle<StepShapeShapeDefinitionRepresentation>,
        ax1: &mut Handle<StepGeomGeometricRepresentationItem>,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
        shape_group: &Handle<HSequence<Shape>>,
        is_manifold: bool,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<dyn TransferBinder>, Failure> {
        let start_mapper: Handle<TransferBRepShapeMapper> = start.cast();
        let Some(live_start) = start_mapper.get() else {
            return Ok(self.null_result());
        };
        // Shape without location, transferred once: the location goes into the assembly link.
        let mut sh0 = live_start.value().clone();
        let mut location = Transform3::default();
        let mut is_located = false;
        let mut mapper: Handle<dyn TransferFinder> = start.clone();
        if self.group_mode() > 0 {
            let shape_location = live_start.value().location();
            is_located = !shape_location.is_identity();
            location = shape_location.transformation();
            sh0.set_location(&Location::default(), false)?;
            mapper = TransferBRep::shape_mapper(fp, &sh0)?.cast();
            mapper.same_attributes(start);
        }
        let process = step_live(fp, WHERE)?;
        let mut result = process.find(&mapper);
        let model: Handle<StepDataStepModel> = process.model().cast();
        let live_model = step_live(&model, WHERE)?;
        let mut sdr = Handle::<StepShapeShapeDefinitionRepresentation>::default();
        let atype = StepShapeShapeDefinitionRepresentation::static_type();
        let has_sdr = process.get_typed_transient_as(&result, atype, &mut sdr);
        let mut part = StepConstructPart::new();
        if has_sdr {
            part.read_sdr(&sdr);
        } else {
            self.make_part_sdr(&mut part, &model)?;
            sdr = part.sdr_value();
        }
        let product: Handle<dyn TransferBinder> = self.transient_result(&sdr.cast()).cast(); // KA - bug 7141(skl 10.11.2004)
        let mut just_transferred = false;
        if !has_sdr || result.is_null() {
            let made = self.transfer_shape(
                &mapper,
                &sdr,
                fp,
                factors,
                shape_group,
                is_manifold,
                progress,
            )?;
            if made.is_null() || step_live(&sdr, WHERE)?.used_representation().is_null() {
                return Ok(Handle::default());
            }
            result = made;
            let previous = process.find(&mapper);
            if previous.is_some() {
                result.add_result(&previous);
            }
            just_transferred = true;
        }
        let mut assembly = StepConstructAssembly::new();
        self.make_assembly_link(
            &mut assembly,
            &sdr,
            sdr0,
            ax1,
            &location,
            start,
            fp,
            live_model,
            factors,
        )?;
        let mut roots: Vec<Handle<StandardTransient>> = roots_of(
            &self
                .state
                .write()
                .my_context
                .get_roots_for_assembly_link(&assembly)?,
        );
        if !has_sdr {
            product.add_result(&self.transient_result(&sdr.cast()).cast());
            if product != result {
                result.add_result(&self.transient_result(&sdr.cast()).cast()); // KA - bug 7141(skl 10.11.2004)
            }
            let of_part = self.state.write().my_context.get_roots_for_part(&part)?;
            roots.extend(roots_of(&of_part));
        }
        for root in &roots {
            product.add_result(&self.transient_result(root).cast());
            if product != result {
                result.add_result(&self.transient_result(root).cast()); // KA - bug 7141(skl 10.11.2004)
            }
        }
        if just_transferred {
            product.add_result(&result);
            process.bind(&mapper, &product)?;
        }
        self.state.write().my_context.next_index();
        if mapper != *start && is_located {
            let mut located = process.find(start);
            for root in &roots {
                let root_binder: Handle<dyn TransferBinder> = self.transient_result(root).cast();
                match located.get() {
                    None => located = root_binder,
                    Some(live) => live.add_result(&root_binder),
                }
            }
            process.bind(start, &located)?;
        }
        Ok(product)
    }
}
