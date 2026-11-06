//! `StepControlActorRead`: the product_definition and next_assembly_usage_occurrence overloads of
//! `transfer_entity`, and what finds their shape_definition_representations.
use super::StepControlActorRead;
use super::actor_read_support::{apply_transformation, shared_entities, sharing_entities};
use crate::step_basic::StepBasicProductDefinition;
use crate::step_construct::StepConstructAssembly;
use crate::step_data::StepDataFactors;
use crate::step_dim_tol::{StepDimTolDatumFeature, StepDimTolGeometricTolerance};
use crate::step_entity::step_live;
use crate::step_repr::{
    StepReprNextAssemblyUsageOccurrence, StepReprProductDefinitionShape,
    StepReprPropertyDefinition, StepReprRepresentation, StepReprRepresentationRelationship,
    StepReprShapeAspect, StepReprShapeRepresentationRelationship,
};
use crate::step_shape::{
    StepShapeAdvancedBrepShapeRepresentation, StepShapeContextDependentShapeRepresentation,
    StepShapeEdgeBasedWireframeShapeRepresentation, StepShapeFacetedBrepShapeRepresentation,
    StepShapeGeometricallyBoundedSurfaceShapeRepresentation,
    StepShapeGeometricallyBoundedWireframeShapeRepresentation,
    StepShapeManifoldSurfaceShapeRepresentation, StepShapeShapeDefinitionRepresentation,
    StepShapeShapeRepresentation,
};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Shape, ShapeIterator};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Transform3;
use martensite_tkxsbase::transfer::{TransferBinder, TransferTransientProcess};
use martensite_tkxsbase::transfer_brep::{TransferBRep, TransferBRepShapeBinder};

/// The number `read.step.shape.repr` gives the kind of a representation (the Controller's
/// `eval` order).
fn find_shape_repr_type(start: &Handle<StepReprRepresentation>) -> i32 {
    if start
        .cast::<StepShapeAdvancedBrepShapeRepresentation>()
        .is_some()
    {
        return 2;
    }
    if start
        .cast::<StepShapeManifoldSurfaceShapeRepresentation>()
        .is_some()
    {
        return 3;
    }
    if start
        .cast::<StepShapeGeometricallyBoundedSurfaceShapeRepresentation>()
        .is_some()
    {
        return 4;
    }
    if start
        .cast::<StepShapeFacetedBrepShapeRepresentation>()
        .is_some()
    {
        return 5;
    }
    if start
        .cast::<StepShapeEdgeBasedWireframeShapeRepresentation>()
        .is_some()
    {
        return 6;
    }
    if start
        .cast::<StepShapeGeometricallyBoundedWireframeShapeRepresentation>()
        .is_some()
    {
        return 7;
    }
    8
}

/// The shape_definition_representations of a shape_aspect, unless it is a datum feature or has a
/// geometric tolerance.
fn get_list_sdr(
    aspect: &Handle<StepReprShapeAspect>,
    list: &mut Vec<Handle<StepShapeShapeDefinitionRepresentation>>,
    tp: &Handle<TransferTransientProcess>,
) -> Result<(), Failure> {
    if aspect.cast::<StepDimTolDatumFeature>().is_some() {
        return Ok(());
    }
    let sharings = sharing_entities(tp, &aspect.cast())?;
    if sharings
        .iter()
        .any(|ent| ent.cast::<StepDimTolGeometricTolerance>().is_some())
    {
        return Ok(());
    }
    for ent in &sharings {
        let definition: Handle<StepReprPropertyDefinition> = ent.cast();
        if definition.is_null() {
            continue;
        }
        for sharing in sharing_entities(tp, &definition.cast())? {
            let sdr: Handle<StepShapeShapeDefinitionRepresentation> = sharing.cast();
            if sdr.is_some() {
                list.push(sdr);
            }
        }
    }
    Ok(())
}

/// The entities a product_definition transfers.
#[derive(Default)]
struct ProductParts {
    sdr: Vec<Handle<StepShapeShapeDefinitionRepresentation>>,
    nauo: Vec<Handle<StepReprNextAssemblyUsageOccurrence>>,
    sdr_aspect: Vec<Handle<StepShapeShapeDefinitionRepresentation>>,
}

/// `getSDR`: fills the lists from a product_definition_shape, keeping for
/// `read.step.shape.repr` the representations nearest the mode.
fn get_sdr(
    pds: &Handle<StepReprProductDefinitionShape>,
    parts: &mut ProductParts,
    tp: &Handle<TransferTransientProcess>,
) -> Result<(), Failure> {
    let model = StepControlActorRead::tp_model(tp)?;
    let ics = model.internal_parameters().read_shape_repr.as_raw();
    let nb_sdr0 = parts.sdr.len();
    let mut delta = 100;
    for sharing in sharing_entities(tp, &pds.cast())? {
        let sdr: Handle<StepShapeShapeDefinitionRepresentation> = sharing.cast();
        if let Some(live) = sdr.get() {
            let rep = live.used_representation();
            if rep.is_null() {
                continue;
            }
            if ics == 1 {
                // translate all SDRs
                parts.sdr.push(sdr);
                continue;
            }
            let difference = (find_shape_repr_type(&rep) - ics).abs();
            if difference < delta {
                parts.sdr.truncate(nb_sdr0);
                delta = difference;
            }
            if difference <= delta {
                parts.sdr.push(sdr);
            }
            continue;
        }
        let aspect: Handle<StepReprShapeAspect> = sharing.cast();
        if aspect.is_some() {
            get_list_sdr(&aspect, &mut parts.sdr_aspect, tp)?;
            continue;
        }
        let nauo: Handle<StepReprNextAssemblyUsageOccurrence> = sharing.cast();
        if let Some(live) = nauo.get() {
            let definition = step_live(pds, "getSDR")?.definition().product_definition();
            if definition == live.relating_product_definition() {
                parts.nauo.push(nauo);
            }
        }
    }
    Ok(())
}

/// Adds a result to the compound: the children of a compound when the file is non-manifold.
fn add_to_compound(
    compound: &mut Compound,
    result: &Shape,
    nm_detected: bool,
) -> Result<(), Failure> {
    let builder = BRepBuilder::default();
    if nm_detected && result.shape_type()? == ShapeEnum::Compound {
        let mut children = ShapeIterator::new(result, true, true);
        while children.more() {
            builder.add(compound, children.value()?)?;
            children.next();
        }
        return Ok(());
    }
    builder.add(compound, result)
}

/// The upcast of a binder's result; a null binder gives a null shape.
fn binder_result(binder: &Handle<dyn TransferBinder>) -> Shape {
    TransferBRep::shape_result(binder)
}

impl StepControlActorRead {
    /// `transfer_entity(PD, TP, local_factors, use_trsf, progress)`.
    pub(super) fn transfer_product_definition(
        &self,
        pd: &Handle<StepBasicProductDefinition>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        use_trsf: bool,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<TransferBRepShapeBinder>, Failure> {
        let mut parts = ProductParts::default();
        for sharing in sharing_entities(tp, &pd.cast())? {
            let pds: Handle<StepReprProductDefinitionShape> = sharing.cast();
            if pds.is_some() {
                get_sdr(&pds, &mut parts, tp)?;
                continue;
            }
            let nauo: Handle<StepReprNextAssemblyUsageOccurrence> = sharing.cast();
            if let Some(live) = nauo.get()
                && *pd == live.relating_product_definition()
            {
                parts.nauo.push(nauo);
            }
        }
        let model = Self::tp_model(tp)?;
        let (read_srr, read_constructive, read_aspect, read_assembly) = {
            let parameters = model.internal_parameters();
            (
                parameters.read_relationship,
                parameters.read_constr_relation,
                parameters.read_shape_aspect,
                parameters.read_assembly_level.as_raw(),
            )
        };
        if !read_aspect {
            parts.sdr_aspect.clear();
        }
        let nb_not_aspect = parts.sdr.len();
        parts.sdr.append(&mut parts.sdr_aspect);
        if read_assembly == 3 || (read_assembly == 2 && !parts.nauo.is_empty()) {
            parts.sdr.clear();
        } else if read_assembly == 4 {
            parts.nauo.clear();
        }
        let nb_ent = parts.sdr.len() + parts.nauo.len();
        if nb_ent == 0 {
            return Ok(Handle::default());
        }
        let scope = ProgressScope::with_literal(progress, "Part", nb_ent as f64, false);
        let builder = BRepBuilder::default();
        let mut compound = Compound::default();
        builder.make_compound(&mut compound);
        let mut result1 = Shape::default();
        let mut nb_components = 0;
        let live_tp = step_live(tp, "StepControlActorRead")?;
        for nauo in &parts.nauo {
            if !scope.more() {
                break;
            }
            let range = scope.next(1.0);
            let start: Handle<StandardTransient> = nauo.cast();
            let binder: Handle<dyn TransferBinder> = if live_tp.is_bound(&start) {
                live_tp.find(&start)
            } else {
                self.transfer_nauo(nauo, tp, factors, &range)?.cast()
            };
            let result = binder_result(&binder);
            if !result.is_null() {
                result1 = result.clone();
                add_to_compound(&mut compound, &result, self.state.read().nm_detected)?;
                nb_components += 1;
            }
        }
        for (index, sdr) in parts.sdr.iter().enumerate() {
            if !scope.more() {
                break;
            }
            let aspect_entry = index >= nb_not_aspect;
            let rep: Handle<StepShapeShapeRepresentation> = step_live(sdr, "StepControlActorRead")?
                .used_representation()
                .cast();
            if rep.is_null() {
                continue;
            }
            let part_range = scope.next(1.0);
            let sub_scope = ProgressScope::unnamed(&part_range, 2.0, false);
            let mut is_bound = true;
            let trsf = use_trsf && !aspect_entry;
            let mut binder: Handle<dyn TransferBinder> = live_tp.find(&rep.cast());
            if binder.is_null() {
                let range = sub_scope.next(1.0);
                binder = self
                    .transfer_shape_representation(&rep, tp, factors, &mut is_bound, trsf, &range)?
                    .cast();
            }
            if aspect_entry && is_bound {
                continue;
            }
            if binder.is_some() {
                let result = binder_result(&binder);
                if !result.is_null() {
                    result1 = result.clone();
                    add_to_compound(&mut compound, &result, self.state.read().nm_detected)?;
                }
            }
            if read_srr && !aspect_entry {
                let related = self.transfer_related_srr(
                    tp,
                    &rep,
                    trsf,
                    read_constructive,
                    factors,
                    &mut compound,
                    &sub_scope,
                )?;
                if !related.is_null() {
                    result1 = related;
                }
            }
        }
        let extra = compound.nb_children() - nb_components;
        if extra > 0 && nb_components > 0 {
            live_tp.add_warning_text(
                &pd.cast(),
                "Product has both sub-assemblies and directly assigned shape",
                "",
            );
        }
        let binder = if extra == 1 && nb_components == 0 {
            Handle::new(TransferBRepShapeBinder::with_result(&result1))
        } else {
            Handle::new(TransferBRepShapeBinder::with_result(&compound))
        };
        live_tp.bind(&pd.cast(), &binder.cast())?;
        Ok(binder)
    }
    /// `transfer_entity(NAUO, TP, local_factors, progress)`.
    pub(super) fn transfer_nauo(
        &self,
        nauo: &Handle<StepReprNextAssemblyUsageOccurrence>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<Handle<TransferBRepShapeBinder>, Failure> {
        let mut pd = Handle::<StepBasicProductDefinition>::default();
        let mut srr = Handle::<StepReprShapeRepresentationRelationship>::default();
        let mut trsf = Transform3::default();
        let (mut iatrsf, mut srr_reversed, mut is_depend) = (false, false, false);
        for sharing in sharing_entities(tp, &nauo.cast())? {
            let pds: Handle<StepReprProductDefinitionShape> = sharing.cast();
            if pds.is_null() {
                continue;
            }
            for sub in sharing_entities(tp, &pds.cast())? {
                let cdsr: Handle<StepShapeContextDependentShapeRepresentation> = sub.cast();
                let Some(live) = cdsr.get() else {
                    continue;
                };
                is_depend = true;
                let relation: Handle<StepReprRepresentationRelationship> =
                    live.representation_relation().cast();
                let Some(live_relation) = relation.get() else {
                    continue;
                };
                srr_reversed = {
                    let guard = step_live(tp, "StepControlActorRead")?.graph()?;
                    let graph = guard.get();
                    StepConstructAssembly::check_srr_reverses_nauo(&graph, &cdsr)?
                };
                let rep = if srr_reversed {
                    live_relation.rep2()
                } else {
                    live_relation.rep1()
                };
                if rep.is_null() {
                    continue;
                }
                iatrsf = self.compute_srrwt(&relation, tp, &mut trsf, factors)?;
                self.find_definition_of_rep(tp, &rep, &mut pd, &mut srr)?;
            }
        }
        let mut binder = Handle::<TransferBRepShapeBinder>::default();
        let live_tp = step_live(tp, "StepControlActorRead")?;
        if is_depend {
            let scope = ProgressScope::unnamed(progress, 2.0, false);
            let mut result = Shape::default();
            if pd.is_some() {
                let mut found: Handle<dyn TransferBinder> = live_tp.find(&pd.cast());
                if found.is_null() {
                    let range = scope.next(1.0);
                    found = self
                        .transfer_product_definition(&pd, tp, factors, false, &range)?
                        .cast();
                }
                result = binder_result(&found);
                if !result.is_null() {
                    if iatrsf {
                        let applied = if srr_reversed { trsf.inverted()? } else { trsf };
                        apply_transformation(&mut result, &applied)?;
                    }
                    binder = Handle::new(TransferBRepShapeBinder::with_result(&result));
                }
            }
            if result.is_null() && srr.is_some() {
                let found: Handle<dyn TransferBinder> = live_tp.find(&srr.cast());
                if found.is_null() {
                    let range = scope.next(1.0);
                    let made = self.transfer_srr(&srr, tp, factors, 0, false, &range)?;
                    let made_binder: Handle<dyn TransferBinder> = made.cast();
                    let shape = binder_result(&made_binder);
                    if !shape.is_null() {
                        binder = Handle::new(TransferBRepShapeBinder::with_result(&shape));
                    }
                }
            }
        }
        live_tp.bind(&nauo.cast(), &binder.cast())?;
        Ok(binder)
    }
    /// The product_definition behind a representation, and the relationship that shares it.
    fn find_definition_of_rep(
        &self,
        tp: &Handle<TransferTransientProcess>,
        rep: &Handle<StepReprRepresentation>,
        pd: &mut Handle<StepBasicProductDefinition>,
        srr: &mut Handle<StepReprShapeRepresentationRelationship>,
    ) -> Result<(), Failure> {
        for sharing in sharing_entities(tp, &rep.cast())? {
            let sdr: Handle<StepShapeShapeDefinitionRepresentation> = sharing.cast();
            if let Some(live) = sdr.get() {
                let pds: Handle<StepReprProductDefinitionShape> =
                    live.definition().property_definition().cast();
                if pds.is_null() {
                    continue;
                }
                for shared in shared_entities(tp, &pds.cast())? {
                    if pd.is_some() {
                        break;
                    }
                    *pd = shared.cast();
                }
            } else if sharing
                .cast::<StepReprShapeRepresentationRelationship>()
                .is_some()
            {
                *srr = sharing.cast();
            }
        }
        Ok(())
    }
}
