//! `STEPCAFControlWriter::writeLayers`: the layers of the documents written.
use super::writer::{WHERE, WriterState};
use super::writer_support::{find_entities, finder_process_of, get_label_name, model_of};
use crate::step_data::StepDataSelectType;
use crate::step_entity::{step_array1_set, step_live, step_new_array1};
use crate::step_visual::{
    StepVisualInvisibility, StepVisualInvisibleItem, StepVisualLayeredItem,
    StepVisualPresentationLayerAssignment,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tklcaf::tdata_std::TDataStdUAttribute;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::toploc::Location;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDoc, XCAFDocDocumentTool, XCAFDocLayerTool, XCAFDocShapeTool,
};
use martensite_tkxsbase::interface::InterfaceModelEntities;
use martensite_tkxsbase::xs_control::XSControlWorkSession;

/// The representation items of the shapes of a layer that were written during this transfer.
fn items_of_layer(
    ws: &Handle<XSControlWorkSession>,
    layer: &TdfLabel,
    labels: &[TdfLabel],
) -> Result<Vec<Handle<StandardTransient>>, Failure> {
    let fp = finder_process_of(ws)?;
    let mut items = Vec::new();
    for shape_label in XCAFDocLayerTool::get_shapes_of_layer(layer)? {
        if shape_label.is_null() {
            continue;
        }
        // there is no way to assign layer to instance in STEP
        if XCAFDocShapeTool::is_assembly(&shape_label)?
            || XCAFDocShapeTool::is_reference(&shape_label)?
        {
            continue;
        }
        // check that the shape is one of (uub)labels written during current transfer
        if !labels.iter().any(|label| shape_label.is_descendant(label)) {
            continue;
        }
        // get target STEP entity
        let mut shape = Shape::default();
        XCAFDocShapeTool::get_shape(&shape_label, &mut shape)?;
        let mut location = Location::default();
        find_entities(&fp, &shape, &mut location, &mut items)?;
    }
    Ok(items)
}

/// The layer entity of the items, and the invisibility of an invisible layer.
fn write_layer(
    ws: &Handle<XSControlWorkSession>,
    layer: &TdfLabel,
    items: &[Handle<StandardTransient>],
) -> Result<(), Failure> {
    // name of layer: if not set, is considered as being empty
    let name = Handle::new(HAsciiString::default());
    get_label_name(layer, &name)?;
    // analyze visibility
    let mut invisible: Handle<TDataStdUAttribute> = Handle::default();
    let is_invisible = layer.find_attribute_as(&XCAFDoc::invisible_guid(), &mut invisible)?;
    let description = if is_invisible { "invisible" } else { "visible" };
    // create layer entity
    let layered = step_new_array1::<StepVisualLayeredItem>(1, items.len() as i32)?;
    for (index, item) in items.iter().enumerate() {
        let mut select = StepVisualLayeredItem::default();
        select.set_value(item)?;
        step_array1_set(&layered, index as i32 + 1, select, WHERE)?;
    }
    let assignment = Handle::new(StepVisualPresentationLayerAssignment::new());
    let description = Handle::new(HAsciiString::new(description));
    step_live(&assignment, WHERE)?.init(&name, &description, &layered);
    let model = model_of(ws)?;
    model.add_with_refs(&assignment.cast(), 0, false)?;
    // PTV 23.01.2003 add the invisibility AFTER adding layer into the model.
    // add the invisibility for the layer
    if is_invisible {
        // Invisibility Item for containing invisible layers.
        let invisible_items = step_new_array1::<StepVisualInvisibleItem>(1, 1)?;
        let mut item = StepVisualInvisibleItem::default();
        item.set_value(&assignment.cast())?;
        step_array1_set(&invisible_items, 1, item, WHERE)?;
        let invisibility = Handle::new(StepVisualInvisibility::new());
        step_live(&invisibility, WHERE)?.init(&invisible_items);
        model.add_with_refs(&invisibility.cast(), 0, false)?;
    }
    Ok(())
}

impl WriterState {
    /// `writeLayers`.
    pub(super) fn write_layers(
        &self,
        ws: &Handle<XSControlWorkSession>,
        labels: &[TdfLabel],
    ) -> Result<bool, Failure> {
        if labels.is_empty() {
            return Ok(false);
        }
        // Iterate on requested shapes collect Tools
        let mut global_layers = Vec::new();
        for root in &self.root_labels {
            let tool = XCAFDocDocumentTool::layer_tool(root)?;
            global_layers.extend(step_live(&tool, WHERE)?.get_layer_labels()?);
        }
        for layer in &global_layers {
            // get labels of shapes in that layer
            let items = items_of_layer(ws, layer, labels)?;
            if !items.is_empty() {
                write_layer(ws, layer, &items)?;
            }
        }
        Ok(true)
    }
}
