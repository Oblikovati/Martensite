//! `StepConstructAssembly`: creates and checks an item of an assembly from its basic data, a
//! ShapeRepresentation and a location.
//!
//! :k8 abv 06.01.99: TR10: writing unique names for NAUOs
//! :j4 16.03.99 gka S4134
//! abv 18.11.99 renamed from the assembly item
use crate::step_basic::StepBasicProductDefinition;
use crate::step_data::StepDataSelectType;
use crate::step_entity::step_live;
use crate::step_geom::{StepGeomAxis2Placement3d, StepGeomCartesianTransformationOperator3d};
use crate::step_repr::{
    StepReprCharacterizedDefinition, StepReprItemDefinedTransformation,
    StepReprNextAssemblyUsageOccurrence, StepReprProductDefinitionShape, StepReprRepresentation,
    StepReprShapeRepresentationRelationshipWithTransformation, StepReprTransformation,
};
use crate::step_shape::{
    StepShapeContextDependentShapeRepresentation, StepShapeShapeDefinitionRepresentation,
    StepShapeShapeRepresentation,
};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::InterfaceGraph;
use std::sync::atomic::{AtomicI32, Ordering};

const WHERE: &str = "StepConstructAssembly";

/// The identifier of the NAUOs: a process-wide counter (`static int id`), so the numbering
/// of a file depends on what the process wrote before it.
static NEXT_NAUO_ID: AtomicI32 = AtomicI32::new(0);

fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

/// The product definition of an SDR: null at any null link but the SDR and its definition.
fn sdr_product_definition(
    sdr: &Handle<StepShapeShapeDefinitionRepresentation>,
) -> Result<Handle<StepBasicProductDefinition>, Failure> {
    let prop = step_live(sdr, WHERE)?.definition().property_definition();
    Ok(step_live(&prop, WHERE)?.definition().product_definition())
}

/// `StepConstructAssembly`: an item, coded from a ShapeRepresentation as nothing (assembly data
/// ignored) or as a RepresentationRelationship with a transformation.
///
/// ```
/// use martensite_tkdestep::step_construct::StepConstructAssembly;
/// let assembly = StepConstructAssembly::new();
/// assert!(assembly.item_value().is_null() && assembly.item_location().is_null());
/// assert!(assembly.get_nauo().unwrap().is_null());
/// ```
#[derive(Clone, Default)]
pub struct StepConstructAssembly {
    thesdr: Handle<StepShapeShapeDefinitionRepresentation>,
    thesdr0: Handle<StepShapeShapeDefinitionRepresentation>,
    thesr: Handle<StepShapeShapeRepresentation>,
    thesr0: Handle<StepShapeShapeRepresentation>,
    theval: Handle<StandardTransient>,
    theloc: Handle<StepGeomAxis2Placement3d>,
    theax0: Handle<StepGeomAxis2Placement3d>,
    my_trsf_op: Handle<StepGeomCartesianTransformationOperator3d>,
    my_is_cartesian_trsf: bool,
}

impl StepConstructAssembly {
    /// `StepConstructAssembly()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// The part common to both `init`.
    fn init_common(
        &mut self,
        sdr: &Handle<StepShapeShapeDefinitionRepresentation>,
        sdr0: &Handle<StepShapeShapeDefinitionRepresentation>,
    ) -> Result<(), Failure> {
        self.thesdr = sdr.clone();
        self.thesdr0 = sdr0.clone();
        self.thesr = step_live(sdr, WHERE)?.used_representation().cast();
        self.thesr0 = step_live(sdr0, WHERE)?.used_representation().cast();
        self.theval = Handle::default();
        Ok(())
    }
    /// `init(sr, SDR0, Ax0, Loc)`: `ax0` is the origin axis, `loc` the location to place the item.
    pub fn init(
        &mut self,
        sdr: &Handle<StepShapeShapeDefinitionRepresentation>,
        sdr0: &Handle<StepShapeShapeDefinitionRepresentation>,
        ax0: &Handle<StepGeomAxis2Placement3d>,
        loc: &Handle<StepGeomAxis2Placement3d>,
    ) -> Result<(), Failure> {
        self.init_common(sdr, sdr0)?;
        self.theax0 = ax0.clone();
        self.theloc = loc.clone();
        self.my_is_cartesian_trsf = false;
        Ok(())
    }
    /// `init(sr, sdr0, trsf_op)`: a local transformation, which may have a scaling factor.
    pub fn init_trsf(
        &mut self,
        sdr: &Handle<StepShapeShapeDefinitionRepresentation>,
        sdr0: &Handle<StepShapeShapeDefinitionRepresentation>,
        trsf_op: &Handle<StepGeomCartesianTransformationOperator3d>,
    ) -> Result<(), Failure> {
        self.init_common(sdr, sdr0)?;
        self.my_trsf_op = trsf_op.clone();
        self.my_is_cartesian_trsf = true;
        Ok(())
    }
    /// The item of the transformation of `make_relationship`.
    fn transformation(&self) -> Result<StepReprTransformation, Failure> {
        let item: Handle<StandardTransient> = if self.my_is_cartesian_trsf {
            self.my_trsf_op.cast()
        } else {
            let item_def = Handle::new(StepReprItemDefinedTransformation::new());
            item_def.init(
                &text(""),
                &text(""),
                &self.theax0.cast(),
                &self.theloc.cast(),
            );
            item_def.cast()
        };
        let mut transformation = StepReprTransformation::default();
        transformation.set_value(&item)?;
        Ok(transformation)
    }
    /// `make_relationship`: a ShapeRepresentationRelationship with a transformation, with its NAUO and
    /// the CDSR that is the resulting value.
    pub fn make_relationship(&mut self) -> Result<(), Failure> {
        // get PDs for assembly (sdr0) and component (sdr)
        let pded = sdr_product_definition(&self.thesdr)?;
        let pding = sdr_product_definition(&self.thesdr0)?;
        // :k8 abv 06 Jan 99: TR10: writing unique names for NAUOs !!!!!
        let nauo = Handle::new(StepReprNextAssemblyUsageOccurrence::new());
        let id = NEXT_NAUO_ID.fetch_add(1, Ordering::Relaxed) + 1;
        let ocid = Handle::new(HAsciiString::from_integer(id));
        nauo.init(
            &ocid,
            &text(""),
            true,
            &text(""),
            &pding,
            &pded,
            false,
            &Handle::default(),
        );
        // create PDS for link CDSR->PDS->NAUO
        let mut characterized = StepReprCharacterizedDefinition::default();
        characterized.set_value(&nauo.cast())?;
        let pds = Handle::new(StepReprProductDefinitionShape::new());
        pds.init(
            &text("Placement"),
            true,
            &text("Placement of an item"),
            &characterized,
        );
        let srrwt = Handle::new(StepReprShapeRepresentationRelationshipWithTransformation::new());
        srrwt.init(
            &text(""),
            &text(""),
            &self.thesr.cast(),
            &self.thesr0.cast(),
            &self.transformation()?,
        );
        // create CDSR (final result, root)
        let cdsr = Handle::new(StepShapeContextDependentShapeRepresentation::new());
        cdsr.init(&srrwt.cast(), &pds);
        self.theval = cdsr.cast();
        Ok(())
    }
    /// `item_value`: the value, or the starting SR when no `Make...` has been called.
    pub fn item_value(&self) -> Handle<StandardTransient> {
        if self.theval.is_null() {
            return self.thesr.cast();
        }
        self.theval.clone()
    }
    /// `ItemLocation`.
    pub fn item_location(&self) -> Handle<StepGeomAxis2Placement3d> {
        self.theloc.clone()
    }
    /// `get_nauo`: the NAUO describing the assembly link.
    pub fn get_nauo(&self) -> Result<Handle<StepReprNextAssemblyUsageOccurrence>, Failure> {
        let cdsr = self
            .item_value()
            .cast::<StepShapeContextDependentShapeRepresentation>();
        let Some(cdsr) = cdsr.get() else {
            return Ok(Handle::default());
        };
        let pds = cdsr.represented_product_relation();
        Ok(step_live(&pds, WHERE)?
            .definition()
            .product_definition_relationship()
            .cast())
    }
    /// The product definition of the first SDR sharing a representation.
    fn pd_of_representation(
        graph: &InterfaceGraph,
        rep: &Handle<StepReprRepresentation>,
    ) -> Result<Handle<StepBasicProductDefinition>, Failure> {
        let target = StepShapeShapeDefinitionRepresentation::static_type();
        for enti in graph.sharings(&rep.cast())?.content() {
            if enti
                .dynamic_type()
                .is_none_or(|kind| !std::ptr::eq(kind, target))
            {
                continue;
            }
            let sdr = enti.cast::<StepShapeShapeDefinitionRepresentation>();
            let sdr = step_live(&sdr, WHERE)?;
            if sdr.used_representation() != *rep {
                continue;
            }
            let found = sdr
                .definition()
                .property_definition()
                .get()
                .map(|prop| prop.definition().product_definition())
                .unwrap_or_default();
            if found.is_some() {
                return Ok(found);
            }
        }
        Ok(Handle::default())
    }
    /// `check_srr_reverses_nauo`: whether the SRR's definition of assembly and component contradicts
    /// the NAUO, according to the model schema (AP214 or AP203).
    pub fn check_srr_reverses_nauo(
        graph: &InterfaceGraph,
        cdsr: &Handle<StepShapeContextDependentShapeRepresentation>,
    ) -> Result<bool, Failure> {
        let cdsr = step_live(cdsr, WHERE)?;
        let pds = cdsr.represented_product_relation();
        let nauo = step_live(&pds, WHERE)?
            .definition()
            .product_definition_relationship()
            .cast::<StepReprNextAssemblyUsageOccurrence>();
        let Some(nauo) = nauo.get() else {
            return Ok(false);
        };
        let relation = cdsr.representation_relation();
        let relation = step_live(&relation, WHERE)?;
        let (rep1, rep2) = (relation.rep1(), relation.rep2());
        if rep1.is_null() || rep2.is_null() {
            return Ok(false);
        }
        // find SDRs corresponding to Rep1 and Rep2 and remember their PDs
        let pd1 = Self::pd_of_representation(graph, &rep1)?;
        let pd2 = Self::pd_of_representation(graph, &rep2)?;
        let (related, relating) = (
            nauo.related_product_definition(),
            nauo.relating_product_definition(),
        );
        if pd1 == related && pd2 == relating {
            return Ok(false); // OK
        }
        // Reversed, or :k3 abv 25 Nov 98: rp1sd.stp - bad assemblies
        Ok(pd2 == related || pd1 == relating)
    }
}
