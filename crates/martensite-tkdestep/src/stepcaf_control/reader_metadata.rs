//! The metadata of `STEPCAFControlReader`: general properties and product data as named data.
use super::reader::{ReaderState, WHERE};
use super::reader_support::{SessionGraph, context_of_unit, mass_conversion_factor};
use crate::step_ap242::{
    StepAP242DraughtingModelItemAssociation, StepAP242ItemIdentifiedRepresentationUsage,
};
use crate::step_basic::{
    StepBasicConversionBasedUnitAndLengthUnit, StepBasicConversionBasedUnitAndMassUnit,
    StepBasicDerivedUnit, StepBasicGeneralProperty, StepBasicGeneralPropertyAssociation,
    StepBasicMeasureWithUnit, StepBasicNamedUnit, StepBasicProductDefinition,
    StepBasicSiUnitAndLengthUnit, StepBasicUnit,
};
use crate::step_data::{StepDataFactors, StepDataSelectMember, StepDataSelectType};
use crate::step_entity::{step_array1_item, step_live};
use crate::step_repr::{
    StepReprBooleanRepresentationItem, StepReprDescriptiveRepresentationItem,
    StepReprIntegerRepresentationItem, StepReprMeasureRepresentationItem,
    StepReprNextAssemblyUsageOccurrence, StepReprProductDefinitionShape,
    StepReprPropertyDefinition, StepReprPropertyDefinitionRelationship,
    StepReprPropertyDefinitionRepresentation, StepReprRealRepresentationItem,
    StepReprRepresentationItem, StepReprShapeAspect, StepReprValueRepresentationItem,
};
use crate::step_shape::StepShapeShapeDefinitionRepresentation;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tklcaf::tdata_std::TDataStdNamedData;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
use martensite_tkxsbase::interface::{InterfaceModelEntities, InterfaceParamType};
use martensite_tkxsbase::transfer::TransferBinder;
use martensite_tkxsbase::transfer_brep::TransferBRep;
use martensite_tkxsbase::xs_control::XSControlWorkSession;

/// A binder that is not null goes to the list.
fn push_binder(binders: &mut Vec<Handle<dyn TransferBinder>>, binder: Handle<dyn TransferBinder>) {
    if binder.is_some() {
        binders.push(binder);
    }
}

/// `findReprItems`: the binders of the items of a representation of an SDR.
fn find_repr_items(
    session: &SessionGraph,
    sdr: &StepShapeShapeDefinitionRepresentation,
    binders: &mut Vec<Handle<dyn TransferBinder>>,
) -> Result<bool, Failure> {
    let representation = sdr.used_representation();
    let Some(representation) = representation.get() else {
        return Ok(false);
    };
    for rank in 1..=representation.nb_items() {
        push_binder(
            binders,
            session
                .process()?
                .find(&representation.items_value(rank)?.cast()),
        );
    }
    Ok(true)
}

/// The binders of a shape aspect a property is on. An association of a draughting model ends the
/// search.
fn collect_aspect_binders(
    session: &SessionGraph,
    aspect: &Handle<StandardTransient>,
    source: &Handle<StepReprPropertyDefinition>,
    binders: &mut Vec<Handle<dyn TransferBinder>>,
) -> Result<(), Failure> {
    let process = session.process()?;
    for sharing in session.sharings(aspect)? {
        if sharing
            .cast::<StepAP242DraughtingModelItemAssociation>()
            .is_some()
        {
            break;
        }
        if let Some(usage) = sharing
            .cast::<StepAP242ItemIdentifiedRepresentationUsage>()
            .get()
        {
            for rank in 1..=usage.nb_identified_item() {
                let item = usage.identified_item_value(rank)?;
                if item.is_some() {
                    push_binder(binders, process.find(&item.cast()));
                }
            }
            continue;
        }
        let definition = sharing.cast::<StepReprPropertyDefinition>();
        if definition.is_some() && definition != *source {
            for second in session.sharings(&sharing)? {
                if let Some(sdr) = second
                    .cast::<StepShapeShapeDefinitionRepresentation>()
                    .get()
                {
                    find_repr_items(session, sdr, binders)?;
                }
            }
            continue;
        }
        if let Some(sdr) = sharing
            .cast::<StepShapeShapeDefinitionRepresentation>()
            .get()
        {
            find_repr_items(session, sdr, binders)?;
        }
    }
    Ok(())
}

/// `collectBinders`: the binders of what a property is on.
fn collect_binders(
    session: &SessionGraph,
    source: &Handle<StepReprPropertyDefinition>,
) -> Result<Vec<Handle<dyn TransferBinder>>, Failure> {
    let process = session.process()?;
    let mut binders = Vec::new();
    for shared in session.shareds(&source.cast())? {
        // Acquire binder from StepBasicProductDefinition.
        if shared.cast::<StepBasicProductDefinition>().is_some() {
            push_binder(&mut binders, process.find(&shared));
        } else if let Some(shape) = shared.cast::<StepReprProductDefinitionShape>().get() {
            // Acquire binder from StepReprProductDefinitionShape.
            push_binder(
                &mut binders,
                process.find(&shape.definition().product_definition().cast()),
            );
        } else if shared
            .cast::<StepReprNextAssemblyUsageOccurrence>()
            .is_some()
        {
            // Acquire binder from StepReprNextAssemblyUsageOccurrence.
            push_binder(&mut binders, process.find(&shared));
        } else if shared.cast::<StepReprShapeAspect>().is_some() {
            collect_aspect_binders(session, &shared, source, &mut binders)?;
        }
    }
    Ok(binders)
}

/// The scale of a derived unit on a measure of the type `value_type`.
fn derived_unit_scale(
    derived: &StepBasicDerivedUnit,
    value_type: &str,
    factors: &StepDataFactors,
) -> Result<f64, Failure> {
    let mut scale = 1.;
    for rank in 1..=derived.nb_elements()? {
        let element = derived.elements_value(rank)?;
        let Some(element) = element.get() else {
            continue;
        };
        let exponent = element.exponent();
        let named = element.unit();
        if named.is_null() {
            continue;
        }
        if named
            .cast::<StepBasicConversionBasedUnitAndLengthUnit>()
            .is_some()
            || named.cast::<StepBasicSiUnitAndLengthUnit>().is_some()
        {
            if value_type == "VOLUME_MEASURE" {
                scale = martensite_cmath::pow(factors.length_factor(), exponent);
            } else {
                scale /= martensite_cmath::pow(factors.length_factor(), exponent);
            }
        } else if named
            .cast::<StepBasicConversionBasedUnitAndMassUnit>()
            .is_some()
        {
            let mut factor = 1.;
            if mass_conversion_factor(&named, &mut factor)? {
                scale *= factor;
            }
        } else {
            let context = context_of_unit(&named, factors)?;
            if context.area_done() {
                scale = context.area_factor();
            }
            if context.length_done() {
                scale *= martensite_cmath::pow(context.length_factor(), exponent);
            }
        }
    }
    Ok(scale)
}

/// The scale of a named unit.
fn named_unit_scale(
    named: &Handle<StepBasicNamedUnit>,
    factors: &StepDataFactors,
) -> Result<f64, Failure> {
    if named
        .cast::<StepBasicConversionBasedUnitAndMassUnit>()
        .is_some()
    {
        let mut factor = 1.;
        mass_conversion_factor(named, &mut factor)?;
        return Ok(factor);
    }
    let context = context_of_unit(named, factors)?;
    let mut scale = 1.;
    if context.area_done() {
        scale = context.area_factor();
    }
    if context.volume_done() {
        scale = context.volume_factor();
    }
    if context.length_done() {
        scale = context.length_factor();
    }
    Ok(scale)
}

/// The value of a measure in the units of the document.
fn measure_value(
    measure: &StepBasicMeasureWithUnit,
    value_type: &str,
    factors: &StepDataFactors,
) -> Result<f64, Failure> {
    let value = measure.value_component();
    let unit: StepBasicUnit = measure.unit_component();
    if unit.is_null() {
        return Ok(value);
    }
    if unit.value().cast::<StepBasicDerivedUnit>().is_some() {
        let derived = unit.derived_unit();
        return derived.get().map_or(Ok(value), |live| {
            Ok::<f64, Failure>(value * derived_unit_scale(live, value_type, factors)?)
        });
    }
    let named = unit.named_unit();
    if named.is_null() {
        return Ok(value);
    }
    Ok(value * named_unit_scale(&named, factors)?)
}

/// The name of an entry of the named data.
fn key(name: &AsciiString) -> ExtendedString {
    ExtendedString::from_ascii(name)
}

/// One item of a property as a value of the named data.
fn fill_item(
    item: &Handle<StepReprRepresentationItem>,
    name: &AsciiString,
    attribute: &TDataStdNamedData,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    if let Some(descriptive) = item.cast::<StepReprDescriptiveRepresentationItem>().get() {
        let description = descriptive.description();
        if let Some(description) = description.get() {
            attribute.set_string(
                &key(name),
                ExtendedString::from_ascii(&description.string()),
            )?;
        }
    } else if let Some(measured) = item.cast::<StepReprMeasureRepresentationItem>().get() {
        let measure = measured.measure();
        let Some(measure) = measure.get() else {
            return Ok(());
        };
        let member = measure.value_component_member();
        let Some(member) = member.get() else {
            return Ok(());
        };
        attribute.set_real(&key(name), measure_value(measure, &member.name(), factors)?)?;
    } else if let Some(valued) = item.cast::<StepReprValueRepresentationItem>().get() {
        let member = valued.value_component_member();
        if let Some(member) = member.get() {
            let kind = member.param_type();
            if kind == InterfaceParamType::ParamInteger {
                attribute.set_integer(&key(name), member.integer())?;
            } else if kind == InterfaceParamType::ParamReal {
                attribute.set_real(&key(name), member.real())?;
            }
        }
    } else if let Some(integer) = item.cast::<StepReprIntegerRepresentationItem>().get() {
        attribute.set_integer(&key(name), integer.value())?;
    } else if let Some(real) = item.cast::<StepReprRealRepresentationItem>().get() {
        attribute.set_real(&key(name), real.value())?;
    } else if let Some(boolean) = item.cast::<StepReprBooleanRepresentationItem>().get() {
        attribute.set_integer(&key(name), i32::from(boolean.value()))?;
    }
    Ok(())
}

/// `fillAttributes`: the values of a property definition in the named data.
fn fill_attributes(
    session: &SessionGraph,
    definition: &Handle<StepReprPropertyDefinition>,
    factors: &StepDataFactors,
    attribute: &TDataStdNamedData,
) -> Result<bool, Failure> {
    // Skip if property definition or name is null
    let Some(live) = definition.get() else {
        return Ok(false);
    };
    let name = live.name();
    let Some(name) = name.get() else {
        return Ok(false);
    };
    let name = name.string().clone();
    for sharing in session.sharings(&definition.cast())? {
        let representation = sharing.cast::<StepReprPropertyDefinitionRepresentation>();
        let Some(representation) = representation.get().map(|live| live.used_representation())
        else {
            continue;
        };
        let Some(representation) = representation.get() else {
            continue;
        };
        let items = representation.items();
        let Some(live_items) = items.get() else {
            continue;
        };
        let (lower, upper) = {
            let array = live_items.array1();
            (array.lower(), array.upper())
        };
        for rank in lower..=upper {
            let item = step_array1_item(&items, rank, WHERE)?;
            if item.is_some() {
                fill_item(&item, &name, attribute, factors)?;
            }
        }
    }
    Ok(true)
}

impl ReaderState {
    /// `getShapeLabelFromProductDefinition`.
    fn shape_label_of_product_definition(
        &self,
        session: &SessionGraph,
        definition: &Handle<StepBasicProductDefinition>,
    ) -> Result<TdfLabel, Failure> {
        let binder = session.process()?.find(&definition.cast());
        if binder.is_null() {
            return Ok(TdfLabel::default());
        }
        let shape = TransferBRep::shape_result(&binder);
        if shape.is_null() {
            return Ok(TdfLabel::default());
        }
        Ok(self.map.find(&shape).cloned().unwrap_or_default())
    }
    /// `collectShapeLabels`: the labels of the shapes a property is on.
    fn collect_shape_labels(
        &self,
        session: &SessionGraph,
        source: &Handle<StepReprPropertyDefinition>,
    ) -> Result<Vec<TdfLabel>, Failure> {
        let mut labels = Vec::new();
        for binder in collect_binders(session, source)? {
            let shape = TransferBRep::shape_result(&binder);
            if !shape.is_null()
                && let Ok(label) = self.map.find(&shape)
            {
                labels.push(label.clone());
            }
        }
        Ok(labels)
    }
    /// `read_metadata`: the values of general properties on the labels of what they are on.
    pub(super) fn read_metadata(
        &self,
        ws: &Handle<XSControlWorkSession>,
        doc: &Handle<TDocStdDocument>,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        let main = step_live(doc, WHERE)?.main()?;
        if XCAFDocDocumentTool::shape_tool(&main)?.is_null() {
            return Ok(false);
        }
        let model = step_live(ws, WHERE)?.model();
        let session = SessionGraph::of_session(ws)?;
        for i in 1..=model.nb_entities() {
            let entity = model.value(i)?;
            if entity.cast::<StepBasicGeneralProperty>().is_null() {
                continue;
            }
            // collectPropertyDefinitions
            for sharing in session.sharings(&entity)? {
                let association = sharing.cast::<StepBasicGeneralPropertyAssociation>();
                let Some(association) = association.get() else {
                    continue;
                };
                let definition = association.property_definition();
                // Collect all the related PropertyDefinitions.
                let mut grouped = vec![definition.clone()];
                for related in session.sharings(&definition.cast())? {
                    if let Some(relationship) = related
                        .cast::<StepReprPropertyDefinitionRelationship>()
                        .get()
                    {
                        let found = relationship.related_property_definition();
                        if found.is_some() {
                            grouped.push(found);
                        }
                    }
                }
                // Collect all the shapes labels, and fill all attributes for each.
                for label in self.collect_shape_labels(&session, &definition)? {
                    let attribute = TDataStdNamedData::set(&label)?;
                    let attribute = step_live(&attribute, WHERE)?;
                    for item in &grouped {
                        fill_attributes(&session, item, factors, attribute)?;
                    }
                }
            }
        }
        Ok(true)
    }
    /// `read_product_metadata`: the fields of each product on the label of its shape.
    pub(super) fn read_product_metadata(
        &self,
        ws: &Handle<XSControlWorkSession>,
        doc: &Handle<TDocStdDocument>,
    ) -> Result<bool, Failure> {
        let main = step_live(doc, WHERE)?.main()?;
        if XCAFDocDocumentTool::shape_tool(&main)?.is_null() {
            return Ok(false);
        }
        let model = step_live(ws, WHERE)?.model();
        let session = SessionGraph::of_process(ws)?;
        for i in 1..=model.nb_entities() {
            // Processing only StepBasicProductDefinition entities.
            let definition = model.value(i)?.cast::<StepBasicProductDefinition>();
            let Some(live) = definition.get() else {
                continue;
            };
            // Prepare required data for processing: ShapeLabel and Product entity.
            let label = self.shape_label_of_product_definition(&session, &definition)?;
            if label.is_null() {
                continue;
            }
            let formation = live.formation();
            let Some(product) = formation.get().map(|live| live.of_product()) else {
                continue;
            };
            let Some(product) = product.get() else {
                continue;
            };
            // Find or create NamedData attribute for ShapeLabel.
            let attribute = TDataStdNamedData::set(&label)?;
            let attribute = step_live(&attribute, WHERE)?;
            // Fill NamedData attribute with product metadata.
            for (entry, value) in [
                ("ProductID", product.id()),
                ("ProductName", product.name()),
                ("Description", product.description()),
                ("ProductDefinition", live.description()),
            ] {
                if let Some(value) = value.get().filter(|text| !text.is_empty()) {
                    attribute.set_string(
                        &ExtendedString::new(entry),
                        ExtendedString::from_ascii(&value.string()),
                    )?;
                }
            }
        }
        Ok(true)
    }
}
