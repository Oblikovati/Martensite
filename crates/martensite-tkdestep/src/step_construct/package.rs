//! `STEPConstruct`: finding the entities and shapes a transfer recorded.
use crate::step_entity::step_live;
use crate::step_repr::StepReprRepresentationItem;
use crate::step_shape::{
    StepShapeContextDependentShapeRepresentation, StepShapeShapeDefinitionRepresentation,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkmath::toploc::Location;
use martensite_tkxsbase::transfer::{
    TransferBinder, TransferFinderProcess, TransferSimpleBinderOfTransient,
    TransferTransientProcess,
};
use martensite_tkxsbase::transfer_brep::TransferBRep;

/// `STEPConstruct`: the static members.
///
/// ```
/// use martensite_tkdestep::step_construct::StepConstruct;
/// use martensite_tkbrep::topods::Shape;
/// use martensite_tkernel::standard::Handle;
/// let found = StepConstruct::find_entity(&Handle::default(), &Shape::default());
/// assert!(found.is_err());
/// ```
pub struct StepConstruct;

impl StepConstruct {
    /// `find_entity(FinderProcess, Shape)`: the representation item a shape was translated into,
    /// null when none is recorded.
    pub fn find_entity(
        finder_process: &Handle<TransferFinderProcess>,
        shape: &Shape,
    ) -> Result<Handle<StepReprRepresentationItem>, Failure> {
        let mut item = Handle::default();
        Self::find_item_of(finder_process, shape, &mut item)?;
        Ok(item)
    }
    /// `find_entity(FinderProcess, Shape, Loc)`: as `find_entity`, repeating the search on the shape
    /// without location when nothing is found; `loc` receives the location the result was found with.
    pub fn find_entity_located(
        finder_process: &Handle<TransferFinderProcess>,
        shape: &Shape,
        loc: &mut Location,
    ) -> Result<Handle<StepReprRepresentationItem>, Failure> {
        let mut item = Handle::default();
        *loc = shape.location().clone();
        if !Self::find_item_of(finder_process, shape, &mut item)? && !loc.is_identity() {
            *loc = Location::default();
            let mut located = shape.clone();
            located.set_location(loc, false)?;
            Self::find_item_of(finder_process, &located, &mut item)?;
        }
        Ok(item)
    }
    /// The typed search of `find_entity`.
    fn find_item_of(
        finder_process: &Handle<TransferFinderProcess>,
        shape: &Shape,
        item: &mut Handle<StepReprRepresentationItem>,
    ) -> Result<bool, Failure> {
        let mapper = TransferBRep::shape_mapper(finder_process, shape)?;
        let process = step_live(finder_process, "STEPConstruct::FindEntity")?;
        Ok(process.find_typed_transient_as(
            &mapper.cast(),
            StepReprRepresentationItem::static_type(),
            item,
        ))
    }
    /// `find_shape`: the shape resulting from a STEP entity, null when not mapped.
    pub fn find_shape(
        transient_process: &Handle<TransferTransientProcess>,
        item: &Handle<StepReprRepresentationItem>,
    ) -> Result<Shape, Failure> {
        let process = step_live(transient_process, "STEPConstruct::FindShape")?;
        let binder = process.find(&item.cast());
        match binder.get() {
            Some(live) if live.has_result() => Ok(TransferBRep::shape_result(&binder)),
            _ => Ok(Shape::default()),
        }
    }
    /// `find_cdsr`: the CDSR of a component in an assembly, searched in the chain of its binders.
    pub fn find_cdsr(
        component_binder: &Handle<dyn TransferBinder>,
        assembly_sdr: &Handle<StepShapeShapeDefinitionRepresentation>,
        component_cdsr: &mut Handle<StepShapeContextDependentShapeRepresentation>,
    ) -> Result<bool, Failure> {
        let sdr = step_live(assembly_sdr, "STEPConstruct::FindCDSR")?;
        let prop = sdr.definition().property_definition();
        let Some(prop) = prop.get() else {
            return Ok(false);
        };
        let assembly_pd = prop.definition().product_definition();
        if assembly_pd.is_null() {
            return Ok(false);
        }
        let mut binder = component_binder.clone();
        while let Some(live) = binder.get() {
            if binder_names_assembly(&binder, &assembly_pd, component_cdsr)? {
                return Ok(true);
            }
            let next = live.next_result();
            binder = next;
        }
        Ok(false)
    }
}

/// One step of the `find_cdsr` chain: `component_cdsr` takes the result of a simple binder even when
/// it is not the wanted one.
fn binder_names_assembly(
    binder: &Handle<dyn TransferBinder>,
    assembly_pd: &Handle<crate::step_basic::StepBasicProductDefinition>,
    component_cdsr: &mut Handle<StepShapeContextDependentShapeRepresentation>,
) -> Result<bool, Failure> {
    let simple = binder.cast::<TransferSimpleBinderOfTransient>();
    let Some(simple) = simple.get() else {
        return Ok(false);
    };
    *component_cdsr = simple.result().cast();
    let Some(cdsr) = component_cdsr.get() else {
        return Ok(false);
    };
    let pds = cdsr.represented_product_relation();
    let Some(pds) = pds.get() else {
        return Ok(false);
    };
    let nauo = pds.definition().product_definition_relationship();
    let Some(nauo) = nauo.get() else {
        return Ok(false);
    };
    Ok(nauo.relating_product_definition() == *assembly_pd)
}
