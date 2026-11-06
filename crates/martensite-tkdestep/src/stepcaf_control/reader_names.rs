//! The names, validation properties and layers of `STEPCAFControlReader`.
use super::reader::{
    PdFileMap, ReaderState, STEPCAFControlShapeLabelMap, WHERE, find_instance_label,
};
use crate::step_basic::{
    StepBasicProduct, StepBasicProductDefinition, StepBasicProductDefinitionWithAssociatedDocuments,
};
use crate::step_construct::StepConstructValidationProps;
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_entity::step_live;
use crate::step_repr::{
    StepReprNextAssemblyUsageOccurrence, StepReprProductDefinitionShape,
    StepReprPropertyDefinition, StepReprPropertyDefinitionRepresentation,
    StepReprRepresentationContext, StepReprShapeAspect,
};
use crate::step_shape::{
    StepShapeGeometricSet, StepShapeShapeDefinitionRepresentation, StepShapeShapeRepresentation,
    StepShapeShellBasedSurfaceModel, StepShapeSolidModel,
};
use crate::step_visual::{StepVisualInvisibility, StepVisualPresentationLayerAssignment};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkernel::tcollection::{ExtendedString, HAsciiString};
use martensite_tklcaf::tdata_std::TDataStdName;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::gp::Point3;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocArea, XCAFDocCentroid, XCAFDocDocumentTool, XCAFDocShapeTool, XCAFDocVolume,
};
use martensite_tkxsbase::interface::{InterfaceGraph, InterfaceModelEntities};
use martensite_tkxsbase::transfer::TransferBinder;
use martensite_tkxsbase::transfer_brep::TransferBRep;
use martensite_tkxsbase::xs_control::XSControlWorkSession;

/// The exact type test of the `dynamic_type() == STANDARD_TYPE(T)`.
pub(super) fn is_exactly<T: RuntimeTyped>(entity: &Handle<StandardTransient>) -> bool {
    entity
        .dynamic_type()
        .is_some_and(|kind| std::ptr::eq(kind, T::static_type()))
}

/// `UsefullLength` is above zero.
fn has_useful_text(text: &Handle<HAsciiString>) -> bool {
    text.get().is_some_and(|live| live.useful_length() > 0)
}

/// The label of an external file of a product definition, null when there is none.
fn label_of_extern_file(pd: &Handle<StepBasicProductDefinition>, pd_file: &PdFileMap) -> TdfLabel {
    pd_file
        .find(pd)
        .ok()
        .and_then(|file| file.get().map(|live| live.get_label()))
        .unwrap_or_default()
}

/// The label of a shape: from the map, else by a search.
fn label_of_shape(
    shape: &martensite_tkbrep::topods::Shape,
    tool: &XCAFDocShapeTool,
    map: &STEPCAFControlShapeLabelMap,
) -> Result<TdfLabel, Failure> {
    let mut label = map.find(shape).cloned().unwrap_or_default();
    if label.is_null() {
        tool.search(shape, &mut label, [true, true, false])?;
    }
    Ok(label)
}

/// `GetLabelFromPD` through the transfer process.
pub(super) fn label_from_pd(
    pd: &Handle<StepBasicProductDefinition>,
    tool: &XCAFDocShapeTool,
    tp: &martensite_tkxsbase::transfer::TransferTransientProcess,
    pd_file: &PdFileMap,
    map: &STEPCAFControlShapeLabelMap,
) -> Result<TdfLabel, Failure> {
    let label = label_of_extern_file(pd, pd_file);
    if !label.is_null() {
        return Ok(label);
    }
    let binder = tp.find(&pd.cast());
    match binder.get() {
        Some(live) if live.has_result() => {}
        _ => return Ok(TdfLabel::default()),
    }
    let shape = TransferBRep::shape_result(&binder);
    if shape.is_null() {
        return Ok(TdfLabel::default());
    }
    label_of_shape(&shape, tool, map)
}

/// `GetLabelFromPD` through the validation properties.
fn label_from_pd_of_props(
    pd: &Handle<StepBasicProductDefinition>,
    tool: &XCAFDocShapeTool,
    props: &StepConstructValidationProps,
    pd_file: &PdFileMap,
    map: &STEPCAFControlShapeLabelMap,
) -> Result<TdfLabel, Failure> {
    let label = label_of_extern_file(pd, pd_file);
    if !label.is_null() {
        return Ok(label);
    }
    let shape = props.get_prop_shape(pd)?;
    if shape.is_null() {
        return Ok(TdfLabel::default());
    }
    label_of_shape(&shape, tool, map)
}

/// The name of a NAUO, from the last PDS sharing it: its relationship's description, name or id.
/// `name` stays when no PDS has a relationship.
fn name_of_nauo(
    graph: &InterfaceGraph,
    nauo: &Handle<StandardTransient>,
    name: &mut Handle<HAsciiString>,
) -> Result<(), Failure> {
    for sharing in graph.sharings(nauo)?.content() {
        let pds = sharing.cast::<StepReprProductDefinitionShape>();
        let Some(pds) = pds.get() else {
            continue;
        };
        let relationship = pds.definition().product_definition_relationship();
        let Some(relationship) = relationship.get() else {
            continue;
        };
        *name = if relationship.has_description() && has_useful_text(&relationship.description()) {
            relationship.description()
        } else if has_useful_text(&relationship.name()) {
            relationship.name()
        } else if relationship.id().is_some() {
            relationship.id()
        } else {
            Handle::new(HAsciiString::default())
        };
    }
    Ok(())
}

/// The name of a product: its name or id.
fn name_of_product(product: &Handle<StepBasicProduct>) -> Handle<HAsciiString> {
    let Some(live) = product.get() else {
        return Handle::new(HAsciiString::default());
    };
    if has_useful_text(&live.name()) {
        return live.name();
    }
    if live.id().is_some() {
        return live.id();
    }
    Handle::new(HAsciiString::default())
}

impl ReaderState {
    /// `TDataStdName::Set` with the converted name.
    fn set_name(&self, label: &TdfLabel, name: &Handle<HAsciiString>) -> Result<(), Failure> {
        let live = step_live(name, WHERE)?;
        TDataStdName::set(label, &self.convert_name(&live.string()))?;
        Ok(())
    }
    /// `read_names`: the names of instances and products on their labels.
    pub(super) fn read_names(
        &self,
        ws: &Handle<XSControlWorkSession>,
        doc: &Handle<TDocStdDocument>,
        pd_file: &PdFileMap,
    ) -> Result<bool, Failure> {
        // get starting data
        let live_ws = step_live(ws, WHERE)?;
        let model = live_ws.model();
        let tr = live_ws.transfer_reader();
        let tp = step_live(&tr, WHERE)?.transient_process();
        let main = step_live(doc, WHERE)?.main()?;
        let tool = XCAFDocDocumentTool::shape_tool(&main)?;
        let Some(live_tool) = tool.get() else {
            return Ok(false);
        };
        // iterate on model to find all SDRs and CDSRs
        let mut name = Handle::<HAsciiString>::default();
        for i in 1..=model.nb_entities() {
            let entity = model.value(i)?;
            // get description of NAUO
            if is_exactly::<StepReprNextAssemblyUsageOccurrence>(&entity) {
                let hgraph = live_ws.graph()?;
                name_of_nauo(&step_live(&hgraph, WHERE)?.graph(), &entity, &mut name)?;
                // find proper label
                let label = find_instance_label(&entity.cast(), &tool, &tp, &self.map)?;
                if !label.is_null() {
                    self.set_name(&label, &name)?;
                }
            }
            // for PD get name of associated product
            if is_exactly::<StepBasicProductDefinition>(&entity)
                || is_exactly::<StepBasicProductDefinitionWithAssociatedDocuments>(&entity)
            {
                let pd: Handle<StepBasicProductDefinition> = entity.cast();
                let product = step_live(&pd, WHERE)?
                    .formation()
                    .get()
                    .map(|formation| formation.of_product())
                    .unwrap_or_default();
                name = name_of_product(&product);
                let label =
                    label_from_pd(&pd, live_tool, step_live(&tp, WHERE)?, pd_file, &self.map)?;
                if !label.is_null() {
                    self.set_name(&label, &name)?;
                }
            }
        }
        Ok(true)
    }
    /// The label a PDS of a validation property refers to.
    fn label_of_property_pds(
        &self,
        pds: &Handle<StandardTransient>,
        ws: &Handle<XSControlWorkSession>,
        tool: &Handle<XCAFDocShapeTool>,
        props: &StepConstructValidationProps,
        pd_file: &PdFileMap,
    ) -> Result<TdfLabel, Failure> {
        let hgraph = props.graph(false)?;
        let shareds = step_live(&hgraph, WHERE)?.graph().shareds(pds)?.content();
        // find corresponding NAUO
        let nauo = shareds
            .iter()
            .map(|shared| shared.cast::<StepReprNextAssemblyUsageOccurrence>())
            .find(Handle::is_some);
        if let Some(nauo) = nauo {
            let tr = step_live(ws, WHERE)?.transfer_reader();
            let tp = step_live(&tr, WHERE)?.transient_process();
            return find_instance_label(&nauo, tool, &tp, &self.map);
        }
        // find corresponding ProductDefinition:
        let prod_def = shareds
            .iter()
            .map(|shared| shared.cast::<StepBasicProductDefinition>())
            .find(Handle::is_some);
        let Some(prod_def) = prod_def else {
            return Ok(TdfLabel::default());
        };
        label_from_pd_of_props(
            &prod_def,
            step_live(tool, WHERE)?,
            props,
            pd_file,
            &self.map,
        )
    }
    /// The first shape representation of an SDR over the properties of a shape aspect.
    fn sr_of_shape_aspect(
        graph: &InterfaceGraph,
        aspect: &Handle<StandardTransient>,
    ) -> Result<Handle<StepShapeShapeRepresentation>, Failure> {
        for sharing in graph.sharings(aspect)?.content() {
            if sharing.cast::<StepReprPropertyDefinition>().is_null() {
                continue;
            }
            for second in graph.sharings(&sharing)?.content() {
                if let Some(sdr) = second
                    .cast::<StepShapeShapeDefinitionRepresentation>()
                    .get()
                {
                    let sr = sdr
                        .used_representation()
                        .cast::<StepShapeShapeRepresentation>();
                    if sr.is_some() {
                        return Ok(sr);
                    }
                }
            }
        }
        Ok(Handle::default())
    }
    /// The label a shape aspect of a validation property refers to.
    fn label_of_property_aspect(
        &self,
        aspect: &Handle<StandardTransient>,
        ws: &Handle<XSControlWorkSession>,
        tool: &XCAFDocShapeTool,
        props: &StepConstructValidationProps,
    ) -> Result<TdfLabel, Failure> {
        let hgraph = props.graph(false)?;
        // find ShapeRepresentation
        let sr = Self::sr_of_shape_aspect(&step_live(&hgraph, WHERE)?.graph(), aspect)?;
        let Some(sr) = sr.get() else {
            return Ok(TdfLabel::default());
        };
        let tr = step_live(ws, WHERE)?.transfer_reader();
        let tp = step_live(&tr, WHERE)?.transient_process();
        let tp = step_live(&tp, WHERE)?;
        let mut binder = Handle::<dyn TransferBinder>::default();
        let mut rank = 1;
        while rank <= sr.nb_items() && binder.is_null() {
            let item = sr.items_value(rank)?;
            step_live(&item, WHERE)?;
            if item.cast::<StepShapeSolidModel>().is_some()
                || item.cast::<StepShapeShellBasedSurfaceModel>().is_some()
                || item.cast::<StepShapeGeometricSet>().is_some()
            {
                binder = tp.find(&item.cast());
            }
            rank += 1;
        }
        match binder.get() {
            Some(live) if live.has_result() => {}
            _ => return Ok(TdfLabel::default()),
        }
        let shape = TransferBRep::shape_result(&binder);
        if shape.is_null() {
            return Ok(TdfLabel::default());
        }
        let mut label = self.map.find(&shape).cloned().unwrap_or_default();
        if label.is_null() {
            tool.search(&shape, &mut label, [true, true, true])?;
        }
        Ok(label)
    }
    /// The label of a property definition, from what it shares.
    fn label_of_property(
        &self,
        definition: &Handle<StepReprPropertyDefinition>,
        ws: &Handle<XSControlWorkSession>,
        tool: &Handle<XCAFDocShapeTool>,
        props: &StepConstructValidationProps,
        pd_file: &PdFileMap,
    ) -> Result<TdfLabel, Failure> {
        let hgraph = props.graph(false)?;
        let shareds = step_live(&hgraph, WHERE)?
            .graph()
            .shareds(&definition.cast())?
            .content();
        let mut label = TdfLabel::default();
        for shared in shareds {
            if !label.is_null() {
                break;
            }
            if shared.cast::<StepReprProductDefinitionShape>().is_some() {
                label = self.label_of_property_pds(&shared, ws, tool, props, pd_file)?;
                if label.is_null() {
                    continue;
                }
            }
            if shared.cast::<StepReprShapeAspect>().is_some() {
                label =
                    self.label_of_property_aspect(&shared, ws, step_live(tool, WHERE)?, props)?;
            }
        }
        Ok(label)
    }
    /// Decodes the validation properties of a representation on a label.
    fn set_properties(
        props: &StepConstructValidationProps,
        pdr: &StepReprPropertyDefinitionRepresentation,
        label: &TdfLabel,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let representation = pdr.used_representation();
        let representation = step_live(&representation, WHERE)?;
        let context: Handle<StepReprRepresentationContext> = representation.context_of_items();
        for j in 1..=representation.nb_items() {
            let item = representation.items_value(j)?;
            let (mut is_area, mut value) = (false, 0.);
            let mut position = Point3::default();
            if props.get_prop_real(&item, &mut value, &mut is_area, factors)? {
                if is_area {
                    XCAFDocArea::set(label, value)?;
                } else {
                    XCAFDocVolume::set(label, value)?;
                }
            } else if props.get_prop_pnt(&item, &context, &mut position, factors)? {
                XCAFDocCentroid::set(label, &position)?;
            }
        }
        Ok(())
    }
    /// `read_val_props`: area, volume and centroid of the model on the labels.
    pub(super) fn read_val_props(
        &self,
        ws: &Handle<XSControlWorkSession>,
        doc: &Handle<TDocStdDocument>,
        pd_file: &PdFileMap,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        // get starting data
        let main = step_live(doc, WHERE)?.main()?;
        let tool = XCAFDocDocumentTool::shape_tool(&main)?;
        if tool.is_null() {
            return Ok(false);
        }
        // load props from the STEP model
        let mut found = Vec::new();
        let props = StepConstructValidationProps::with_ws(ws)?;
        if !props.load_props(&mut found)? {
            return Ok(false);
        }
        // interpret props one by one
        for prop in found {
            let pdr = prop.cast::<StepReprPropertyDefinitionRepresentation>();
            let Some(pdr) = pdr.get() else {
                continue;
            };
            let definition = pdr.definition().property_definition();
            let label = self.label_of_property(&definition, ws, &tool, &props, pd_file)?;
            if !label.is_null() {
                Self::set_properties(&props, pdr, &label, factors)?;
            }
        }
        Ok(true)
    }
    /// `read_layers`: the presentation layer assignments of the model on the labels.
    pub(super) fn read_layers(
        &self,
        ws: &Handle<XSControlWorkSession>,
        doc: &Handle<TDocStdDocument>,
    ) -> Result<bool, Failure> {
        let live_ws = step_live(ws, WHERE)?;
        let model = live_ws.model();
        let tr = live_ws.transfer_reader();
        let tp = step_live(&tr, WHERE)?.transient_process();
        let tp = step_live(&tp, WHERE)?;
        let main = step_live(doc, WHERE)?.main()?;
        let shape_tool = XCAFDocDocumentTool::shape_tool(&main)?;
        let layer_tool = XCAFDocDocumentTool::layer_tool(&main)?;
        let (Some(shape_tool), Some(layer_tool)) = (shape_tool.get(), layer_tool.get()) else {
            return Ok(false);
        };
        for i in 1..=model.nb_entities() {
            let entity = model.value(i)?;
            let assignment = entity.cast::<StepVisualPresentationLayerAssignment>();
            let Some(assignment) = assignment
                .get()
                .filter(|live| live.assigned_items().is_some())
            else {
                continue;
            };
            let layer_name = {
                let name = assignment.name();
                ExtendedString::from_ascii(&step_live(&name, WHERE)?.string())
            };
            // check invisibility
            let hgraph = live_ws.graph()?;
            let sharings = step_live(&hgraph, WHERE)?
                .graph()
                .sharings(&entity)?
                .content();
            let visible = !sharings
                .iter()
                .any(|sharing| sharing.cast::<StepVisualInvisibility>().is_some());
            let mut layer_label = TdfLabel::default();
            // find a target shape and its label in the document
            for j in 1..=assignment.nb_assigned_items() {
                let binder = tp.find(&assignment.assigned_items_value(j)?.value());
                match binder.get() {
                    Some(live) if live.has_result() => {}
                    _ => continue,
                }
                let shape = TransferBRep::shape_result(&binder);
                if shape.is_null() {
                    continue;
                }
                let mut shape_label = TdfLabel::default();
                if !shape_tool.search(&shape, &mut shape_label, [true, true, true])? {
                    continue;
                }
                if layer_label.is_null() {
                    layer_label = layer_tool.add_layer_visible(&layer_name, visible)?;
                }
                layer_tool.set_layer_label(&shape_label, &layer_label, false)?;
            }
            if !layer_label.is_null() {
                layer_tool.set_visibility(&layer_label, visible)?;
            }
        }
        Ok(true)
    }
}
