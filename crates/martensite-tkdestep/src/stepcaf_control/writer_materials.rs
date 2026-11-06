//! `STEPCAFControlWriter::writeMaterials`: the materials of the documents written.
use super::writer::{WHERE, WriterState};
use super::writer_gdt_common::{bytes_of, graph_of, one_item, sharings_of, text};
use super::writer_support::{find_entities, finder_process_of, model_of};
use crate::step_basic::{
    StepBasicDerivedUnit, StepBasicDerivedUnitElement, StepBasicMeasureValueMember,
    StepBasicProductDefinition, StepBasicSiPrefix, StepBasicSiUnitAndLengthUnit,
    StepBasicSiUnitAndMassUnit, StepBasicSiUnitName, StepBasicUnit,
};
use crate::step_data::{StepDataSelectMember, StepDataSelectType};
use crate::step_entity::{step_array1_set, step_live, step_new_array1};
use crate::step_repr::{
    StepReprCharacterizedDefinition, StepReprDescriptiveRepresentationItem,
    StepReprMeasureRepresentationItem, StepReprProductDefinitionShape, StepReprPropertyDefinition,
    StepReprPropertyDefinitionRepresentation, StepReprRepresentation,
    StepReprRepresentationContext, StepReprRepresentationItem, StepReprRepresentedDefinition,
};
use crate::step_shape::{StepShapeShapeDefinitionRepresentation, StepShapeShapeRepresentation};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tklcaf::tdata_std::TDataStdTreeNode;
use martensite_tklcaf::tdf::{TdfAttribute, TdfLabel};
use martensite_tkmath::toploc::Location;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDoc, XCAFDocDocumentTool, XCAFDocMaterialFields, XCAFDocMaterialTool, XCAFDocShapeTool,
};
use martensite_tkxsbase::interface::{InterfaceHGraph, InterfaceModelEntities};
use martensite_tkxsbase::xs_control::XSControlWorkSession;
use std::collections::HashMap;

type RepresentationMap = HashMap<Vec<u8>, Handle<StepReprRepresentation>>;

/// `FindPDSforRI`: the product definition shape and context of a representation item.
fn find_pds_for_item(
    graph: &Handle<InterfaceHGraph>,
    entity: &Handle<StandardTransient>,
    pds: &mut Handle<StepReprProductDefinitionShape>,
    context: &mut Handle<StepReprRepresentationContext>,
) -> Result<(), Failure> {
    if entity.cast::<StepReprRepresentationItem>().is_null() {
        return Ok(());
    }
    for sharing in sharings_of(graph, entity)? {
        let representation: Handle<StepShapeShapeRepresentation> = sharing.cast();
        if representation.is_null() {
            continue;
        }
        *context = step_live(&representation, WHERE)?.context_of_items();
        for user in sharings_of(graph, &representation)? {
            let sdr: Handle<StepShapeShapeDefinitionRepresentation> = user.cast();
            let Some(sdr) = sdr.get() else {
                continue;
            };
            *pds = sdr.definition().property_definition().cast();
            if pds.is_some() {
                return Ok(());
            }
        }
    }
    Ok(())
}

/// The representation of a density: in g/cm3, under the name of its value type.
fn density_representation(
    ws: &Handle<XSControlWorkSession>,
    fields: &XCAFDocMaterialFields,
    context: &Handle<StepReprRepresentationContext>,
) -> Result<Handle<StepReprRepresentation>, Failure> {
    let model = model_of(ws)?;
    // mass
    let mass = Handle::new(StepBasicSiUnitAndMassUnit::new());
    mass.set_name(StepBasicSiUnitName::SunGram);
    let mass_element = Handle::new(StepBasicDerivedUnitElement::new());
    mass_element.init(&mass.cast(), 3.0);
    // length
    let length = Handle::new(StepBasicSiUnitAndLengthUnit::new());
    length.init(
        true,
        StepBasicSiPrefix::SpCenti,
        StepBasicSiUnitName::SunMetre,
    );
    let length_element = Handle::new(StepBasicDerivedUnitElement::new());
    length_element.init(&length.cast(), 2.0);
    // other
    let elements = step_new_array1::<Handle<StepBasicDerivedUnitElement>>(1, 2)?;
    step_array1_set(&elements, 1, mass_element, WHERE)?;
    step_array1_set(&elements, 2, length_element, WHERE)?;
    let derived = Handle::new(StepBasicDerivedUnit::new());
    derived.init(&elements);
    model.add_with_refs(&derived.cast(), 0, false)?;
    let mut unit = StepBasicUnit::default();
    unit.set_value(&derived.cast())?;
    let member = Handle::new(StepBasicMeasureValueMember::new());
    let type_name = bytes_of(&fields.dens_val_type)?;
    member.set_name(&String::from_utf8_lossy(&type_name));
    member.set_real(fields.density);
    let measure = Handle::new(StepReprMeasureRepresentationItem::new());
    measure.init(&fields.dens_name, &member, &unit)?;
    let representation = Handle::new(StepReprRepresentation::new());
    representation.init(&text("density"), &one_item(&measure.cast())?, context);
    model.add_with_refs(&representation.cast(), 0, false)?;
    Ok(representation)
}

/// A property of the product definition with the representation.
fn write_material_property(
    ws: &Handle<XSControlWorkSession>,
    definition: &Handle<StepBasicProductDefinition>,
    description: &str,
    representation: &Handle<StepReprRepresentation>,
) -> Result<(), Failure> {
    let model = model_of(ws)?;
    let mut characterized = StepReprCharacterizedDefinition::default();
    characterized.set_value(&definition.cast())?;
    let property = Handle::new(StepReprPropertyDefinition::new());
    property.init(
        &text("material property"),
        true,
        &text(description),
        &characterized,
    );
    model.add_with_refs(&property.cast(), 0, false)?;
    let mut represented = StepReprRepresentedDefinition::default();
    represented.set_value(&property.cast())?;
    let link = Handle::new(StepReprPropertyDefinitionRepresentation::new());
    link.init(&represented, representation);
    model.add_with_refs(&link.cast(), 0, false)
}

impl WriterState {
    /// `writeMaterials`.
    pub(super) fn write_materials(
        &self,
        ws: &Handle<XSControlWorkSession>,
        labels: &[TdfLabel],
    ) -> Result<bool, Failure> {
        if labels.is_empty() {
            return Ok(false);
        }
        // get working data
        let graph = graph_of(ws)?;
        if graph.is_null() {
            return Ok(false);
        }
        let (mut map_dri, mut map_mri) = (RepresentationMap::new(), RepresentationMap::new());
        // Iterate on requested shapes collect Tools
        let mut top_labels = Vec::new();
        for root in &self.root_labels {
            let tool = XCAFDocDocumentTool::shape_tool(root)?;
            top_labels.extend(step_live(&tool, WHERE)?.get_shapes()?);
        }
        for shape_label in &top_labels {
            write_material_of_shape(ws, &graph, shape_label, &mut map_dri, &mut map_mri)?;
        }
        Ok(true)
    }
}

fn write_material_of_shape(
    ws: &Handle<XSControlWorkSession>,
    graph: &Handle<InterfaceHGraph>,
    shape_label: &TdfLabel,
    map_dri: &mut RepresentationMap,
    map_mri: &mut RepresentationMap,
) -> Result<(), Failure> {
    let mut node: Handle<TDataStdTreeNode> = Handle::default();
    if !shape_label.find_attribute_as(&XCAFDoc::material_ref_guid(), &mut node)?
        || !step_live(&node, WHERE)?.has_father()
    {
        return Ok(());
    }
    // find PDS for current shape
    let fp = finder_process_of(ws)?;
    let mut shape = Shape::default();
    XCAFDocShapeTool::get_shape(shape_label, &mut shape)?;
    let mut location = Location::default();
    let mut items = Vec::new();
    find_entities(&fp, &shape, &mut location, &mut items)?;
    let Some(entity) = items.first() else {
        return Ok(());
    };
    let (mut pds, mut context) = (Handle::default(), Handle::default());
    find_pds_for_item(graph, entity, &mut pds, &mut context)?;
    let Some(live_pds) = pds.get() else {
        return Ok(());
    };
    let definition = live_pds.definition().product_definition();
    if definition.is_null() {
        return Ok(());
    }
    // write material entities
    let father = step_live(&node, WHERE)?.father();
    let material = step_live(&father, WHERE)?.attribute_base().label();
    let Some(fields) = XCAFDocMaterialTool::get_material(&material)? else {
        return Ok(());
    };
    if step_live(&fields.name, WHERE)?.length() == 0 {
        return Ok(());
    }
    let key = bytes_of(&fields.name)?;
    if !map_dri.contains_key(&key) {
        // write DRI
        let item = Handle::new(StepReprDescriptiveRepresentationItem::new());
        item.init(&fields.name, &fields.description);
        let representation = Handle::new(StepReprRepresentation::new());
        representation.init(&text("material name"), &one_item(&item.cast())?, &context);
        model_of(ws)?.add_with_refs(&representation.cast(), 0, false)?;
        map_dri.insert(key.clone(), representation);
        // write MRI
        if fields.density > 0.0 {
            map_mri.insert(key.clone(), density_representation(ws, &fields, &context)?);
        }
    }
    write_material_property(ws, &definition, "material name", &map_dri[&key])?;
    if let Some(density) = map_mri.get(&key) {
        write_material_property(ws, &definition, "density", density)?;
    }
    Ok(())
}
