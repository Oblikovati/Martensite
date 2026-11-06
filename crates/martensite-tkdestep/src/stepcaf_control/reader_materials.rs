//! `read_materials` of `STEPCAFControlReader`: names and densities on solids.
use super::reader::{ReaderState, WHERE};
use super::reader_support::{SessionGraph, context_of_unit, mass_conversion_factor};
use crate::step_basic::{
    StepBasicConversionBasedUnitAndLengthUnit, StepBasicConversionBasedUnitAndMassUnit,
    StepBasicMeasureWithUnit, StepBasicSiUnitAndLengthUnit,
};
use crate::step_data::{StepDataFactors, StepDataSelectMember};
use crate::step_entity::{HSequence, step_live};
use crate::step_repr::{
    StepReprDescriptiveRepresentationItem, StepReprMeasureRepresentationItem,
    StepReprProductDefinitionShape, StepReprPropertyDefinition,
    StepReprPropertyDefinitionRepresentation, StepReprRepresentation,
    StepReprRepresentationRelationship,
};
use crate::step_shape::{
    StepShapeShapeDefinitionRepresentation, StepShapeShapeRepresentation, StepShapeSolidModel,
};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocMaterialFields};
use martensite_tkxsbase::xs_control::XSControlWorkSession;

/// The first solid model among the items of a representation, null without one.
fn solid_of_items(
    representation: &StepShapeShapeRepresentation,
) -> Result<Handle<StepShapeSolidModel>, Failure> {
    for rank in 1..=representation.nb_items() {
        let solid = representation
            .items_value(rank)?
            .cast::<StepShapeSolidModel>();
        if solid.is_some() {
            return Ok(solid);
        }
    }
    Ok(Handle::default())
}

/// `find_solid_for_pds`: the solid of a PDS, through its SDRs and representation relationships.
fn find_solid_for_pds(
    session: &SessionGraph,
    pds: &Handle<StepReprProductDefinitionShape>,
) -> Result<Handle<StepShapeSolidModel>, Failure> {
    for sharing in session.sharings(&pds.cast())? {
        let Some(sdr) = sharing
            .cast::<StepShapeShapeDefinitionRepresentation>()
            .get()
            .map(|live| live.used_representation())
        else {
            continue;
        };
        let representation = sdr.cast::<StepShapeShapeRepresentation>();
        let Some(live) = representation.get() else {
            continue;
        };
        let solid = solid_of_items(live)?;
        if solid.is_some() {
            return Ok(solid);
        }
        for second in session.sharings(&representation.cast())? {
            let relationship = second.cast::<StepReprRepresentationRelationship>();
            let Some(relationship) = relationship.get() else {
                continue;
            };
            let rep: Handle<StepReprRepresentation> = representation.cast();
            let other = if relationship.rep1() == rep {
                relationship.rep2()
            } else {
                relationship.rep1()
            };
            let other = other.cast::<StepShapeShapeRepresentation>();
            let Some(other) = other.get() else {
                continue;
            };
            let solid = solid_of_items(other)?;
            if solid.is_some() {
                return Ok(solid);
            }
        }
    }
    Ok(Handle::default())
}

/// The density of a measure, per the units of its derived unit: gram/(sm*sm*sm).
fn density_of(
    measure: &StepBasicMeasureWithUnit,
    factors: &StepDataFactors,
) -> Result<f64, Failure> {
    let mut density = measure.value_component();
    let unit = measure.unit_component();
    let derived = unit.derived_unit();
    let Some(derived) = derived.get() else {
        return Ok(density);
    };
    for rank in 1..=derived.nb_elements()? {
        let element = derived.elements_value(rank)?;
        let named = step_live(&element, WHERE)?.unit();
        if named.is_null() {
            continue;
        }
        if named
            .cast::<StepBasicConversionBasedUnitAndLengthUnit>()
            .is_some()
            || named.cast::<StepBasicSiUnitAndLengthUnit>().is_some()
        {
            let length = context_of_unit(&named, factors)?.length_factor();
            density /= length * length * length;
            // transfer length value for Density from millimeter to santimeter
            let cascade = factors.cascade_unit();
            density = density * 1000. / (cascade * cascade * cascade);
        }
        if named
            .cast::<StepBasicConversionBasedUnitAndMassUnit>()
            .is_some()
        {
            let mut factor = 1.;
            if mass_conversion_factor(&named, &mut factor)? {
                density *= factor;
            }
        }
    }
    Ok(density)
}

/// The fields of the material a PDS has: from the last representation items.
fn material_of(
    session: &SessionGraph,
    product_definition: &Handle<StandardTransient>,
    factors: &StepDataFactors,
) -> Result<XCAFDocMaterialFields, Failure> {
    let empty = || Handle::new(HAsciiString::new(""));
    let mut fields = XCAFDocMaterialFields {
        name: empty(),
        description: empty(),
        density: 0.,
        dens_name: empty(),
        dens_val_type: empty(),
    };
    for sharing in session.sharings(product_definition)? {
        if sharing.cast::<StepReprPropertyDefinition>().is_null() {
            continue;
        }
        for second in session.sharings(&sharing)? {
            let representation = second.cast::<StepReprPropertyDefinitionRepresentation>();
            let Some(representation) = representation.get().map(|live| live.used_representation())
            else {
                continue;
            };
            let Some(representation) = representation.get() else {
                continue;
            };
            for rank in 1..=representation.nb_items() {
                let item = representation.items_value(rank)?;
                // find name and description for material
                if let Some(descriptive) =
                    item.cast::<StepReprDescriptiveRepresentationItem>().get()
                {
                    fields.name = descriptive.name();
                    fields.description = descriptive.description();
                    if fields.name.is_null() {
                        fields.name = fields.description.clone();
                    }
                }
                // try to find density for material
                if let Some(measured) = item.cast::<StepReprMeasureRepresentationItem>().get() {
                    let measure = measured.measure();
                    let measure = step_live(&measure, WHERE)?;
                    fields.dens_name = measured.name();
                    let member = measure.value_component_member();
                    fields.dens_val_type =
                        Handle::new(HAsciiString::new(step_live(&member, WHERE)?.name()));
                    fields.density = density_of(measure, factors)?;
                }
            }
        }
    }
    Ok(fields)
}

impl ReaderState {
    /// `read_materials`: the material of each PDS with a name and a solid.
    pub(super) fn read_materials(
        &self,
        ws: &Handle<XSControlWorkSession>,
        doc: &Handle<TDocStdDocument>,
        seq_pds: &Handle<HSequence<Handle<StandardTransient>>>,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        let main = step_live(doc, WHERE)?.main()?;
        let shape_tool = XCAFDocDocumentTool::shape_tool(&main)?;
        let material_tool = XCAFDocDocumentTool::material_tool(&main)?;
        let Some(material_tool) = material_tool.get() else {
            return Ok(false);
        };
        let session = SessionGraph::of_process(ws)?;
        let all: Vec<Handle<StandardTransient>> =
            step_live(seq_pds, WHERE)?.sequence(|items| items.iter().cloned().collect());
        for entity in all {
            let pds = entity.cast::<StepReprProductDefinitionShape>();
            let Some(live) = pds.get() else {
                continue;
            };
            let product_definition = live.definition().product_definition();
            if product_definition.is_null() {
                continue;
            }
            let fields = material_of(&session, &product_definition.cast(), factors)?;
            if fields.name.get().is_none_or(HAsciiString::is_empty) {
                continue;
            }
            // find shape label amd create Material link
            let solid = find_solid_for_pds(&session, &pds)?;
            let shape = if solid.is_some() {
                session.shape_at(session.process()?.map_index(&solid.cast()))?
            } else {
                Default::default()
            };
            if shape.is_null() {
                continue;
            }
            let mut label = TdfLabel::default();
            if step_live(&shape_tool, WHERE)?.search(&shape, &mut label, [true, true, true])? {
                material_tool.set_material(&label, &fields)?;
            }
        }
        Ok(true)
    }
}
