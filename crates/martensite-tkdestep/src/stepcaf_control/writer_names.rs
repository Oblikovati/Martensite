//! `STEPCAFControlWriter`: the names of products and assembly links, the metadata of products and
//! the validation properties.
use super::writer::{WHERE, WriterState};
use super::writer_support::{finder_process_of, get_label_name, model_of, typed_result_of};
use crate::step_basic::{
    StepBasicGeneralProperty, StepBasicGeneralPropertyAssociation, StepBasicProductDefinition,
};
use crate::step_construct::StepConstructValidationProps;
use crate::step_data::StepDataSelectType;
use crate::step_entity::{step_array1_set, step_live, step_new_array1};
use crate::step_repr::{
    StepReprCharacterizedDefinition, StepReprDescriptiveRepresentationItem,
    StepReprIntegerRepresentationItem, StepReprPropertyDefinition,
    StepReprPropertyDefinitionRepresentation, StepReprRealRepresentationItem,
    StepReprRepresentation, StepReprRepresentationItem, StepReprRepresentedDefinition,
};
use crate::step_shape::{
    StepShapeContextDependentShapeRepresentation, StepShapeShapeDefinitionRepresentation,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::{AsciiString, ExtendedString, HAsciiString};
use martensite_tklcaf::tdata_std::{TDataStdNamedData, TDataStdNamedMap};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkxcaf::xcaf_doc::{XCAFDocArea, XCAFDocCentroid, XCAFDocShapeTool, XCAFDocVolume};
use martensite_tkxsbase::interface::InterfaceModelEntities;
use martensite_tkxsbase::xs_control::XSControlWorkSession;

fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

/// The string of a key or value in the conversion of an extended string.
fn ascii_of(value: &ExtendedString) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(
        AsciiString::from_extended(value, None).as_bytes(),
    ))
}

/// The product definition of an SDR, null when it has none.
fn product_definition_of(
    sdr: &Handle<StepShapeShapeDefinitionRepresentation>,
) -> Result<Handle<StepBasicProductDefinition>, Failure> {
    let property = step_live(sdr, WHERE)?.definition().property_definition();
    let Some(property) = property.get() else {
        return Ok(Handle::default());
    };
    Ok(property.definition().product_definition())
}

impl WriterState {
    /// `writeNames`.
    pub(super) fn write_names(
        &self,
        ws: &Handle<XSControlWorkSession>,
        labels: &[TdfLabel],
    ) -> Result<bool, Failure> {
        if labels.is_empty() {
            return Ok(false);
        }
        // Iterate on requested shapes
        for label in labels {
            self.write_name_of(ws, label)?;
        }
        Ok(true)
    }
    fn write_name_of(
        &self,
        ws: &Handle<XSControlWorkSession>,
        label: &TdfLabel,
    ) -> Result<(), Failure> {
        // find target STEP entity for the current shape
        let Some(shape) = self.labels.get(label) else {
            return Ok(()); // not recorded as translated, skip
        };
        // get name
        let name = Handle::new(HAsciiString::default());
        if !get_label_name(label, &name)? {
            return Ok(());
        }
        let fp = finder_process_of(ws)?;
        let is_component =
            XCAFDocShapeTool::is_component(label)? || self.pure_ref_labels.contains_key(label);
        let mut sdr: Handle<StepShapeShapeDefinitionRepresentation> = Handle::default();
        let cdsr = if is_component {
            typed_result_of::<StepShapeContextDependentShapeRepresentation>(&fp, shape)?
        } else {
            Handle::default()
        };
        if let Some(live) = cdsr.get() {
            let pds = live.represented_product_relation();
            let nauo = step_live(&pds, WHERE)?
                .definition()
                .product_definition_relationship();
            if let Some(nauo) = nauo.get() {
                nauo.set_name(&name);
            }
            if let Some(internal) = self.pure_ref_labels.get(label) {
                sdr = typed_result_of(&fp, internal)?;
            }
        }
        if sdr.is_null() && cdsr.is_null() {
            sdr = typed_result_of(&fp, shape)?;
        }
        if sdr.is_null() {
            return Ok(());
        }
        // set the name to the PRODUCT
        let definition = product_definition_of(&sdr)?;
        let Some(definition) = definition.get() else {
            return Ok(());
        };
        let formation = definition.formation();
        let product = step_live(&formation, WHERE)?.of_product();
        let product = step_live(&product, WHERE)?;
        product.set_id(&name);
        product.set_name(&name);
        Ok(())
    }
    /// `writeMetadata`.
    pub(super) fn write_metadata(
        &self,
        ws: &Handle<XSControlWorkSession>,
        labels: &[TdfLabel],
    ) -> Result<bool, Failure> {
        if labels.is_empty() {
            return Ok(false);
        }
        // Iterate on requested shapes.
        for label in labels {
            self.write_metadata_for_label(ws, label)?;
        }
        Ok(true)
    }
    /// `writeMetadataForLabel`: a label without metadata ends the walk of its children.
    fn write_metadata_for_label(
        &self,
        ws: &Handle<XSControlWorkSession>,
        label: &TdfLabel,
    ) -> Result<bool, Failure> {
        if label.is_null() {
            return Ok(false);
        }
        // Check if label has metadata (NamedData):
        let mut named_data: Handle<TDataStdNamedData> = Handle::default();
        if !label.find_attribute_as(&TDataStdNamedData::get_id(), &mut named_data)? {
            return Ok(false); // No metadata on this label
        }
        // Find target STEP entity for the current shape:
        let Some(shape) = self.labels.get(label) else {
            return Ok(false); // Not recorded as translated, skip
        };
        let fp = finder_process_of(ws)?;
        let sdr = typed_result_of::<StepShapeShapeDefinitionRepresentation>(&fp, shape)?;
        if sdr.is_null() {
            return Ok(false); // Cannot find STEP representation
        }
        // Get the product definition from the shape definition representation:
        let definition = product_definition_of(&sdr)?;
        if definition.is_null() {
            return Ok(false);
        }
        write_named_values(ws, step_live(&named_data, WHERE)?, &sdr, &definition)?;
        // Process label children recursively:
        if label.has_child() {
            for child in 1..=label.nb_children()? {
                self.write_metadata_for_label(ws, &label.find_child(child, true)?)?;
            }
        }
        Ok(true)
    }
    /// `writeValProps`.
    pub(super) fn write_val_props(
        &self,
        ws: &Handle<XSControlWorkSession>,
        labels: &[TdfLabel],
        is_multi: Option<&str>,
    ) -> Result<bool, Failure> {
        if labels.is_empty() {
            return Ok(false);
        }
        // Iterate on requested shapes
        for label in labels {
            write_props_for_label(ws, label, is_multi)?;
        }
        Ok(true)
    }
}

/// The strings, integers and reals of a label, one representation item each.
fn write_named_values(
    ws: &Handle<XSControlWorkSession>,
    data: &TDataStdNamedData,
    sdr: &Handle<StepShapeShapeDefinitionRepresentation>,
    definition: &Handle<StepBasicProductDefinition>,
) -> Result<(), Failure> {
    if data.has_strings() {
        let map = data.get_strings_container();
        for (key, value) in step_live(&map, WHERE)?.map().items() {
            // Create descriptive representation item for the value.
            let item = Handle::new(StepReprDescriptiveRepresentationItem::new());
            let live = step_live(&item, WHERE)?;
            live.set_name(&ascii_of(key));
            live.set_description(&ascii_of(value));
            write_metadata_representation_item(ws, key, sdr, definition, &item.cast())?;
        }
    }
    if data.has_integers() {
        let map = data.get_integers_container();
        for (key, value) in step_live(&map, WHERE)?.map().items() {
            // Create integer representation item for the value.
            let item = Handle::new(StepReprIntegerRepresentationItem::new());
            step_live(&item, WHERE)?.init(&ascii_of(key), *value);
            write_metadata_representation_item(ws, key, sdr, definition, &item.cast())?;
        }
    }
    if data.has_reals() {
        let map = data.get_reals_container();
        for (key, value) in step_live(&map, WHERE)?.map().items() {
            // Create real representation item for the value.
            let item = Handle::new(StepReprRealRepresentationItem::new());
            step_live(&item, WHERE)?.init(&ascii_of(key), *value);
            write_metadata_representation_item(ws, key, sdr, definition, &item.cast())?;
        }
    }
    Ok(())
}

/// `writeMetadataRepresentationItem`: the property of one key of the metadata of a product.
fn write_metadata_representation_item(
    ws: &Handle<XSControlWorkSession>,
    key: &ExtendedString,
    sdr: &Handle<StepShapeShapeDefinitionRepresentation>,
    definition: &Handle<StepBasicProductDefinition>,
    item: &Handle<StepReprRepresentationItem>,
) -> Result<(), Failure> {
    // Empty string to use for empty values:
    let empty = text("");
    let name = ascii_of(key);
    // Create property_definition:
    let property = Handle::new(StepReprPropertyDefinition::new());
    let live_property = step_live(&property, WHERE)?;
    live_property.set_name(&name);
    live_property.set_description(&text("user defined attribute"));
    // Create a general_property:
    let general = Handle::new(StepBasicGeneralProperty::new());
    let live_general = step_live(&general, WHERE)?;
    live_general.set_id(&empty);
    live_general.set_name(&name);
    live_general.set_description(&empty);
    // Create a general_property_association:
    let association = Handle::new(StepBasicGeneralPropertyAssociation::new());
    let live_association = step_live(&association, WHERE)?;
    live_association.set_name(&empty);
    live_association.set_description(&empty);
    live_association.set_property_definition(&property);
    live_association.set_general_property(&general);
    // Set the definition to point to the product definition:
    let mut characterized = StepReprCharacterizedDefinition::default();
    characterized.set_value(&definition.cast())?;
    live_property.set_definition(&characterized);
    // Create property_definition_representation:
    let representation = Handle::new(StepReprPropertyDefinitionRepresentation::new());
    let mut represented = StepReprRepresentedDefinition::default();
    represented.set_value(&property.cast())?;
    let live_representation = step_live(&representation, WHERE)?;
    live_representation.set_definition(&represented);
    // Create representation with descriptive_representation_item:
    let values = Handle::new(StepReprRepresentation::new());
    let live_values = step_live(&values, WHERE)?;
    live_values.set_name(&name);
    // Add item to representation:
    let items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, 1)?;
    step_array1_set(&items, 1, item.clone(), WHERE)?;
    live_values.set_items(&items);
    // Set representation context (reuse from shape representation):
    let used = step_live(sdr, WHERE)?.used_representation();
    live_values.set_context_of_items(&step_live(&used, WHERE)?.context_of_items());
    live_representation.set_used_representation(&values);
    // Add entities to the model:
    let model = model_of(ws)?;
    model.add_with_refs(&property.cast(), 0, false)?;
    model.add_with_refs(&general.cast(), 0, false)?;
    model.add_with_refs(&association.cast(), 0, false)?;
    model.add_with_refs(&representation.cast(), 0, false)?;
    model.add_with_refs(&values.cast(), 0, false)?;
    model.add_with_refs(&item.cast(), 0, false)
}

/// `write_props_for_label`: the area, volume and centroid of a label and, for a compound or assembly,
/// of its children.
fn write_props_for_label(
    ws: &Handle<XSControlWorkSession>,
    label: &TdfLabel,
    is_multi: Option<&str>,
) -> Result<bool, Failure> {
    if label.is_null() {
        return Ok(false);
    }
    let mut props = StepConstructValidationProps::with_ws(ws)?;
    let mut shape = Shape::default();
    XCAFDocShapeTool::get_shape(label, &mut shape)?;
    if shape.is_null() {
        return Ok(false);
    }
    if is_multi.is_none() || XCAFDocShapeTool::is_assembly(label)? {
        // write area
        let mut area: Handle<XCAFDocArea> = Handle::default();
        if label.find_attribute_as(&XCAFDocArea::get_id(), &mut area)? {
            props.add_area(&shape, step_live(&area, WHERE)?.get())?;
        }
        // write volume
        let mut volume: Handle<XCAFDocVolume> = Handle::default();
        if label.find_attribute_as(&XCAFDocVolume::get_id(), &mut volume)? {
            props.add_volume(&shape, step_live(&volume, WHERE)?.get())?;
        }
    }
    // write centroid
    let mut centroid: Handle<XCAFDocCentroid> = Handle::default();
    if label.find_attribute_as(&XCAFDocCentroid::get_id(), &mut centroid)? {
        props.add_centroid(&shape, &step_live(&centroid, WHERE)?.get(), false)?;
    }
    if (XCAFDocShapeTool::is_compound(label)? || XCAFDocShapeTool::is_assembly(label)?)
        && label.has_child()
    {
        for child in 1..=label.nb_children()? {
            write_props_for_label(ws, &label.find_child(child, true)?, is_multi)?;
        }
    }
    Ok(true)
}
