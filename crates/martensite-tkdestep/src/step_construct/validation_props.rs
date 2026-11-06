//! `StepConstructValidationProps`: writes and reads the validation properties of shapes in a STEP
//! file: surface area, solid volume and centroid.
use super::{StepConstructTool, StepConstructUnitContext};
use crate::api_header_section::APIHeaderSectionMakeHeader;
use crate::step_basic::{
    StepBasicDerivedUnit, StepBasicDerivedUnitElement, StepBasicMeasureValueMember,
    StepBasicProductDefinition, StepBasicSiPrefix, StepBasicSiUnitAndLengthUnit,
    StepBasicSiUnitName, StepBasicUnit,
};
use crate::step_data::{
    StepDataFactors, StepDataLogical, StepDataSelectMember, StepDataSelectType, StepDataStepModel,
};
use crate::step_entity::{step_array1_set, step_live, step_new_array1};
use crate::step_geom::{
    StepGeomCartesianPoint, StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx,
    StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext,
    StepGeomGeometricRepresentationItem,
};
use crate::step_repr::{
    StepReprCharacterizedDefinition, StepReprGlobalUnitAssignedContext,
    StepReprMeasureRepresentationItem, StepReprNextAssemblyUsageOccurrence,
    StepReprProductDefinitionShape, StepReprPropertyDefinition,
    StepReprPropertyDefinitionRepresentation, StepReprRepresentation,
    StepReprRepresentationContext, StepReprRepresentationItem, StepReprRepresentationRelationship,
    StepReprRepresentedDefinition, StepReprShapeAspect,
};
use crate::step_shape::{StepShapeShapeDefinitionRepresentation, StepShapeShapeRepresentation};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkxsbase::interface::{InterfaceGraph, InterfaceModelEntities};
use martensite_tkxsbase::transfer::{TransferBinder, TransferSimpleBinderOfTransient};
use martensite_tkxsbase::transfer_brep::TransferBRep;
use std::ops::Deref;

mod loading;
#[allow(unused_imports, reason = "what the sibling modules use")]
pub use loading::*;

const WHERE: &str = "StepConstructValidationProps";

fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

/// `transient_result`: a binder of a result, null for a null one.
fn transient_result(res: &Handle<StandardTransient>) -> Handle<dyn TransferBinder> {
    if res.is_null() {
        return Handle::default();
    }
    let binder = TransferSimpleBinderOfTransient::new();
    binder.set_result(res);
    Handle::new(binder).cast()
}

/// The first of `entities` of type `T`, null without one.
fn first_of<T: ?Sized + RuntimeTyped>(entities: &[Handle<StandardTransient>]) -> Handle<T> {
    entities
        .iter()
        .map(|entity| entity.cast::<T>())
        .find(Handle::is_some)
        .unwrap_or_default()
}

/// The PDS of the first SDR among the entities sharing `rep`, null without one.
fn pds_of_sharing_sdr(
    graph: &InterfaceGraph,
    rep: &Handle<StandardTransient>,
) -> Result<Handle<StepReprProductDefinitionShape>, Failure> {
    for entity in graph.sharings(rep)?.content() {
        if let Some(sdr) = entity
            .cast::<StepShapeShapeDefinitionRepresentation>()
            .get()
        {
            let pds = sdr
                .definition()
                .property_definition()
                .cast::<StepReprProductDefinitionShape>();
            if pds.is_some() {
                return Ok(pds);
            }
        }
    }
    Ok(Handle::default())
}

/// The PDS of an SR found through a RepresentationRelationship with another SR.
fn pds_through_relationship(
    graph: &InterfaceGraph,
    sr: &Handle<StepShapeShapeRepresentation>,
) -> Result<Handle<StepReprProductDefinitionShape>, Failure> {
    for entity in graph.sharings(&sr.cast())?.content() {
        let relationship = entity.cast::<StepReprRepresentationRelationship>();
        let Some(relationship) = relationship.get() else {
            continue;
        };
        let rep = sr.cast::<StepReprRepresentation>();
        let other = if relationship.rep1() == rep {
            relationship.rep2()
        } else {
            relationship.rep1()
        };
        let other = other.cast::<StepShapeShapeRepresentation>();
        if other.is_some() {
            let pds = pds_of_sharing_sdr(graph, &other.cast())?;
            if pds.is_some() {
                return Ok(pds);
            }
        }
    }
    Ok(Handle::default())
}

/// A shape aspect of `pds` already recorded in the chain of `binder`, null without one.
fn aspect_in_binder(
    binder: &Handle<dyn TransferBinder>,
    pds: &Handle<StepReprProductDefinitionShape>,
) -> Handle<StepReprShapeAspect> {
    let mut current = binder.clone();
    while let Some(live) = current.get() {
        if let Some(simple) = current.cast::<TransferSimpleBinderOfTransient>().get() {
            let aspect = simple.result().cast::<StepReprShapeAspect>();
            if aspect.get().is_some_and(|asp| asp.of_shape() == *pds) {
                return aspect;
            }
        }
        let next = live.next_result();
        current = next;
    }
    Handle::default()
}

/// A derived unit of a power of millimetres: the unit written for areas and volumes.
fn millimetre_power_unit(exponent: f64) -> Result<Handle<StepBasicDerivedUnit>, Failure> {
    let mm = Handle::new(StepBasicSiUnitAndLengthUnit::new());
    mm.init(
        true,
        StepBasicSiPrefix::SpMilli,
        StepBasicSiUnitName::SunMetre,
    );
    let element = Handle::new(StepBasicDerivedUnitElement::new());
    element.init(&mm.cast(), exponent);
    let elements = step_new_array1::<Handle<StepBasicDerivedUnitElement>>(1, 1)?;
    step_array1_set(&elements, 1, element, WHERE)?;
    let unit = Handle::new(StepBasicDerivedUnit::new());
    unit.init(&elements);
    Ok(unit)
}

/// ```
/// use martensite_tkdestep::step_construct::StepConstructValidationProps;
/// let mut props = StepConstructValidationProps::new();
/// assert!(!props.init(&Default::default()).unwrap());
/// assert!(props.ws().is_null());
/// ```
#[derive(Clone, Default)]
pub struct StepConstructValidationProps {
    tool: StepConstructTool,
    area_unit: StepBasicUnit,
    vol_unit: StepBasicUnit,
    my_assembly_pd: Handle<StepBasicProductDefinition>,
}

impl Deref for StepConstructValidationProps {
    type Target = StepConstructTool;
    fn deref(&self) -> &StepConstructTool {
        &self.tool
    }
}

impl StepConstructValidationProps {
    /// `StepConstructValidationProps()`: an empty tool.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepConstructValidationProps(WS)`: a tool loaded with a session.
    pub fn with_ws(
        ws: &Handle<martensite_tkxsbase::xs_control::XSControlWorkSession>,
    ) -> Result<Self, Failure> {
        Ok(Self {
            tool: StepConstructTool::with_ws(ws)?,
            ..Self::default()
        })
    }
    /// `init`: loads a session; true on success.
    pub fn init(
        &mut self,
        ws: &Handle<martensite_tkxsbase::xs_control::XSControlWorkSession>,
    ) -> Result<bool, Failure> {
        self.tool.set_ws(ws)
    }
    /// The graph of the tool, read through its handle.
    fn hgraph(&self) -> Result<Handle<martensite_tkxsbase::interface::InterfaceHGraph>, Failure> {
        self.tool.graph(false)
    }
    /// The target of a compound: the PDS of its product definition, or of an SR through its SDR.
    fn target_of_compound(
        &self,
        mapper: &Handle<dyn martensite_tkxsbase::transfer::TransferFinder>,
        target: &mut StepReprCharacterizedDefinition,
        context: &mut Handle<StepReprRepresentationContext>,
    ) -> Result<(), Failure> {
        let process = step_live(self.finder_process(), WHERE)?;
        let hgraph = self.hgraph()?;
        let hgraph = step_live(&hgraph, WHERE)?;
        let graph = hgraph.graph();
        let mut prod_def = Handle::<StepBasicProductDefinition>::default();
        if process.find_typed_transient_as(
            mapper,
            StepBasicProductDefinition::static_type(),
            &mut prod_def,
        ) {
            let pds: Handle<StepReprProductDefinitionShape> =
                first_of(&graph.sharings(&prod_def.cast())?.content());
            target.set_value(&pds.cast())?;
            return Ok(());
        }
        let mut sr = Handle::<StepShapeShapeRepresentation>::default();
        if !process.find_typed_transient_as(
            mapper,
            StepShapeShapeRepresentation::static_type(),
            &mut sr,
        ) {
            return Ok(());
        }
        let mut pds = pds_of_sharing_sdr(&graph, &sr.cast())?;
        if pds.is_null() {
            pds = pds_through_relationship(&graph, &sr)?;
        }
        if pds.is_some() {
            target.set_value(&pds.cast())?;
            *context = step_live(&sr, WHERE)?.context_of_items();
        }
        Ok(())
    }
    /// The PDS of an item (GRI <- SR <- SDR -> PDS), taking the context of the SR on the way.
    fn pds_of_item(
        &self,
        graph: &InterfaceGraph,
        item: &Handle<StepGeomGeometricRepresentationItem>,
        context: &mut Handle<StepReprRepresentationContext>,
    ) -> Result<Handle<StepReprProductDefinitionShape>, Failure> {
        let mut pds = Handle::<StepReprProductDefinitionShape>::default();
        for sharing in graph.sharings(&item.cast())?.content() {
            if pds.is_some() {
                break;
            }
            let sr = sharing.cast::<StepShapeShapeRepresentation>();
            let Some(sr) = sr.get() else {
                continue;
            };
            *context = sr.context_of_items();
            // The last SDR wins: this loop has no stop on a found PDS.
            for second in graph.sharings(&sharing)?.content() {
                if let Some(sdr) = second
                    .cast::<StepShapeShapeDefinitionRepresentation>()
                    .get()
                {
                    pds = sdr.definition().property_definition().cast();
                }
            }
        }
        Ok(pds)
    }
    /// A new shape aspect of `pds` and its related data: the aspect is recorded in the map, and the
    /// SDR with all its data is added to the model.
    fn create_aspect(
        &self,
        binder: &Handle<dyn TransferBinder>,
        pds: &Handle<StepReprProductDefinitionShape>,
        item: &Handle<StepGeomGeometricRepresentationItem>,
        context: &Handle<StepReprRepresentationContext>,
    ) -> Result<Handle<StepReprShapeAspect>, Failure> {
        let aspect = Handle::new(StepReprShapeAspect::new());
        aspect.init(&text(""), &text(""), pds, StepDataLogical::LFalse);
        let mut characterized = StepReprCharacterizedDefinition::default();
        characterized.set_value(&aspect.cast())?;
        let propdef = Handle::new(StepReprPropertyDefinition::new());
        propdef.init(
            &text("shape with specific properties"),
            true,
            &text("properties for subshape"),
            &characterized,
        );
        let items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, 1)?;
        step_array1_set(&items, 1, item.cast(), WHERE)?;
        let sr = Handle::new(StepShapeShapeRepresentation::new());
        sr.init(&text(""), &items, context);
        let sdr = Handle::new(StepShapeShapeDefinitionRepresentation::new());
        let mut represented = StepReprRepresentedDefinition::default();
        represented.set_value(&propdef.cast())?;
        sdr.init(&represented, &sr.cast());
        // record SHAPE_ASPECT in the map
        step_live(binder, WHERE)?.add_result(&transient_result(&aspect.cast()));
        // add SDR and all the data into model
        self.model()?.add_with_refs(&sdr.cast(), 0, false)?;
        Ok(aspect)
    }
    /// The context of an aspect found already: from the first SDR sharing its PDS.
    fn context_of_aspect(
        &self,
        graph: &InterfaceGraph,
        aspect: &StepReprShapeAspect,
        context: &mut Handle<StepReprRepresentationContext>,
    ) -> Result<(), Failure> {
        for sharing in graph.sharings(&aspect.of_shape().cast())?.content() {
            if context.is_some() {
                break;
            }
            if let Some(sdr) = sharing
                .cast::<StepShapeShapeDefinitionRepresentation>()
                .get()
            {
                *context = step_live(&sdr.used_representation(), WHERE)?.context_of_items();
            }
        }
        Ok(())
    }
    /// The target of a subshape: the SHAPE_ASPECT of its item, created if not yet existing.
    fn target_of_subshape(
        &self,
        mapper: &Handle<dyn martensite_tkxsbase::transfer::TransferFinder>,
        binder: &Handle<dyn TransferBinder>,
        target: &mut StepReprCharacterizedDefinition,
        context: &mut Handle<StepReprRepresentationContext>,
    ) -> Result<(), Failure> {
        let process = step_live(self.finder_process(), WHERE)?;
        let mut item = Handle::<StepGeomGeometricRepresentationItem>::default();
        if !process.find_typed_transient_as(
            mapper,
            StepGeomGeometricRepresentationItem::static_type(),
            &mut item,
        ) {
            return Ok(());
        }
        let hgraph = self.hgraph()?;
        let hgraph = step_live(&hgraph, WHERE)?;
        let graph = hgraph.graph();
        let pds = self.pds_of_item(&graph, &item, context)?;
        if pds.is_null() {
            return Ok(());
        }
        // find SHAPE_ASPECT or create it with all associated info if not yet exists
        let mut aspect = aspect_in_binder(binder, &pds);
        match aspect.get() {
            // SHAPE_ASPECT found, but we also need context: FIND IT !!!!
            Some(found) => self.context_of_aspect(&graph, found, context)?,
            None => {
                drop(graph);
                aspect = self.create_aspect(binder, &pds, &item, context)?;
            }
        }
        target.set_value(&aspect.cast())?;
        Ok(())
    }
}
