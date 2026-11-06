//! `STEPCAFControlWriter::transfer` and `transferExternFiles`: the shapes of the labels, then the
//! data of each mode.
use super::STEPCAFControlActorWrite;
use super::STEPCAFControlExternFile;
use super::writer::{WHERE, WriterState};
use super::writer_support::{get_label_name, model_of};
use crate::de_step::{
    DeStepWriteModeAssembly, DeStepWriteModeStepSchema, StepControlStepModelType,
};
use crate::step_control::StepControlWriter;
use crate::step_data::{StepDataFactors, StepDataStepModel};
use crate::step_entity::step_live;
use crate::step_repr::StepReprRepresentationItem;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Shape};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkernel::tcollection::{AsciiString, HAsciiString};
use martensite_tklcaf::tdf::{TdfChildIterator, TdfLabel};
use martensite_tkxcaf::xcaf_doc::{XCAFDocLengthUnit, XCAFDocShapeTool};
use martensite_tkxsbase::interface::InterfaceModelEntities;
use martensite_tkxsbase::transfer_brep::TransferBRep;
use martensite_tkxsbase::xs_algo::{XSAlgoShapeProcessor, xs_algo_units};
use martensite_tkxsbase::xs_control::XSControlWorkSession;

fn shape_of(label: &TdfLabel) -> Result<Shape, Failure> {
    let mut shape = Shape::default();
    XCAFDocShapeTool::get_shape(label, &mut shape)?;
    Ok(shape)
}

/// A new empty compound.
fn empty_compound() -> Result<Compound, Failure> {
    let mut compound = Compound::default();
    BRepBuilder::default().make_compound(&mut compound);
    Ok(compound)
}

/// `prepareUnit`: the length unit of the document, or of the session, in the model and the factors.
fn prepare_unit(
    label: &TdfLabel,
    model: &Handle<StepDataStepModel>,
    factors: &mut StepDataFactors,
) -> Result<(), Failure> {
    let live = step_live(model, WHERE)?;
    let mut length: Handle<XCAFDocLengthUnit> = Handle::default();
    if !label.is_null()
        && label
            .root()?
            .find_attribute_as(&XCAFDocLengthUnit::get_id(), &mut length)?
    {
        // convert to mm
        let unit = step_live(&length, WHERE)?.get_unit_value() * 1000.;
        live.set_local_length_unit(unit);
        factors.set_cascade_unit(unit);
        return Ok(());
    }
    XSAlgoShapeProcessor::prepare_for_transfer(); // update unit info
    let unit = xs_algo_units().get_cascade_length_unit_default();
    live.set_local_length_unit(unit);
    factors.set_cascade_unit(unit);
    Ok(())
}

impl WriterState {
    /// `transfer`: the shapes of the labels through `writer`, then the data of each mode.
    pub(super) fn transfer(
        &mut self,
        writer: &StepControlWriter,
        labels: &[TdfLabel],
        mode: StepControlStepModelType,
        is_multi: Option<&str>,
        is_extern_file: bool,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        if labels.is_empty() {
            return Ok(false);
        }
        let ws = writer.ws();
        let live_ws = step_live(&ws, WHERE)?;
        let adaptor = live_ws.norm_adaptor();
        let actor: Handle<STEPCAFControlActorWrite> =
            step_live(&adaptor, WHERE)?.actor_write().cast();
        let model: Handle<StepDataStepModel> = live_ws.model().cast();
        let mut factors = StepDataFactors::default();
        prepare_unit(&labels[0], &model, &mut factors)?; // set local length unit to the model
        // translate free top-level shapes of the DECAF document
        let mut sub_labels = Vec::new();
        let scope = ProgressScope::with_literal(progress, "Labels", labels.len() as f64, false);
        // Iterate on requested shapes
        for label in labels {
            if !scope.more() {
                break;
            }
            let range = scope.next(1.0);
            if !self.transfer_one(
                writer,
                &actor,
                &model,
                label,
                mode,
                is_multi,
                &mut sub_labels,
                &factors,
                &range,
            )? {
                return Ok(false);
            }
        }
        if scope.user_break() {
            return Ok(false);
        }
        live_ws.compute_graph(true)?; // Setting to clear and regenerate graph
        self.write_each_mode(&ws, &model, &sub_labels, is_multi, is_extern_file, &factors)?;
        // refresh graph
        live_ws.compute_graph(true)?;
        if step_live(&model, WHERE)?
            .internal_parameters()
            .write_subshape_names
        {
            write_sub_shape_names(&ws, &sub_labels)?;
        }
        Ok(true)
    }
    /// One requested label: as a whole, or as multifile with extern refs. False on a user break.
    #[allow(clippy::too_many_arguments)]
    fn transfer_one(
        &mut self,
        writer: &StepControlWriter,
        actor: &Handle<STEPCAFControlActorWrite>,
        model: &Handle<StepDataStepModel>,
        label: &TdfLabel,
        mode: StepControlStepModelType,
        is_multi: Option<&str>,
        sub_labels: &mut Vec<TdfLabel>,
        factors: &StepDataFactors,
        range: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        if self.labels.contains_key(label) {
            return Ok(true); // already processed
        }
        let shape = shape_of(label)?;
        if shape.is_null() {
            return Ok(true);
        }
        let live_model = step_live(model, WHERE)?;
        let Some(prefix) = is_multi else {
            let actor = step_live(actor, WHERE)?;
            actor.set_std_mode(false);
            let current = self.collect_whole(actor, label, &shape, sub_labels)?;
            let parameters = live_model.internal_parameters().clone();
            writer.transfer_with(&current, mode, &parameters, false, range)?;
            actor.set_std_mode(true); // restore default behaviour
            return Ok(true);
        };
        // translate final solids
        let scope = ProgressScope::unnamed(range, 2.0, false);
        let assembly =
            self.transfer_extern_files(label, mode, sub_labels, factors, prefix, &scope.next(1.0))?;
        if scope.user_break() {
            return Ok(false);
        }
        let assembly_mode = live_model.internal_parameters().write_assembly;
        live_model.internal_parameters_mut().write_assembly = DeStepWriteModeAssembly::ON;
        let parameters = live_model.internal_parameters().clone();
        writer.transfer_with(
            &assembly,
            StepControlStepModelType::AS_IS,
            &parameters,
            true,
            &scope.next(1.0),
        )?;
        live_model.internal_parameters_mut().write_assembly = assembly_mode;
        Ok(true)
    }
    /// The (sub) shapes of a label written as a whole, the assemblies registered in the actor. Returns
    /// the shape to transfer.
    fn collect_whole(
        &mut self,
        actor: &STEPCAFControlActorWrite,
        label: &TdfLabel,
        shape: &Shape,
        sub_labels: &mut Vec<TdfLabel>,
    ) -> Result<Shape, Failure> {
        let mut current_label = label.clone();
        let mut current_shape = shape.clone();
        let mut comp_labels = Vec::new();
        // For case when only part of assembly structure should be written in the document
        // if specified label is component of the assembly then
        // in order to save location of this component in the high-level assembly
        // and save name of high-level assembly it is necessary to represent structure of high-level
        // assembly as assembly with one component specified by current label. For that compound
        // containing only specified component is binded to the label of the high-level assembly. The
        // such way full structure of high-level assembly was replaced on the assembly containing one
        // component. For case when free shape reference is (located root) also create an auxiliary
        // assembly.
        if XCAFDocShapeTool::is_reference(label)? {
            let mut compound: Shape = empty_compound()?.into();
            BRepBuilder::default().add(&mut compound, shape)?;
            current_shape = compound.clone();
            self.pure_ref_labels.insert(label.clone(), compound);
            comp_labels.push(label.clone());
            let mut referred = TdfLabel::default();
            if XCAFDocShapeTool::get_referred_shape(label, &mut referred)?
                && XCAFDocShapeTool::is_assembly(&referred)?
            {
                XCAFDocShapeTool::get_components(&referred, &mut comp_labels, true)?;
            }
            if !XCAFDocShapeTool::is_free(label)? {
                current_label = label.father();
            }
        } else if XCAFDocShapeTool::is_assembly(label)? {
            // fill sequence of (sub) shapes for which attributes should be written
            // and set actor to handle assemblies in a proper way
            XCAFDocShapeTool::get_components(label, &mut comp_labels, true)?;
        }
        for sub_label in &comp_labels {
            self.collect_component(actor, sub_label, sub_labels)?;
        }
        self.labels.insert(current_label.clone(), shape.clone());
        sub_labels.push(current_label.clone());
        if XCAFDocShapeTool::is_assembly(&current_label)?
            || XCAFDocShapeTool::is_reference(&current_label)?
        {
            actor.register_assembly(&current_shape)?;
        }
        Ok(current_shape)
    }
    /// A component, and the shape it refers to.
    fn collect_component(
        &mut self,
        actor: &STEPCAFControlActorWrite,
        sub_label: &TdfLabel,
        sub_labels: &mut Vec<TdfLabel>,
    ) -> Result<(), Failure> {
        if self.labels.contains_key(sub_label) {
            return Ok(());
        }
        let shape = shape_of(sub_label)?;
        if shape.is_null() {
            return Ok(());
        }
        self.labels.insert(sub_label.clone(), shape);
        sub_labels.push(sub_label.clone());
        let mut referred = TdfLabel::default();
        if !XCAFDocShapeTool::get_referred_shape(sub_label, &mut referred)? {
            return Ok(());
        }
        if !self.labels.contains_key(&referred) {
            let referred_shape = shape_of(&referred)?;
            self.labels.insert(referred.clone(), referred_shape.clone());
            sub_labels.push(referred.clone());
            if XCAFDocShapeTool::is_assembly(&referred)? {
                actor.register_assembly(&referred_shape)?;
            }
        }
        Ok(())
    }
    /// The writers of the modes that are on, in the order.
    fn write_each_mode(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        model: &Handle<StepDataStepModel>,
        labels: &[TdfLabel],
        is_multi: Option<&str>,
        is_extern_file: bool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        if self.name_mode {
            self.write_names(ws, labels)?;
        }
        if is_multi.is_none() {
            self.write_whole_file_modes(ws, model, labels, is_extern_file, factors)?;
        } else {
            // external refs
            self.write_extern_refs(ws, labels)?;
        }
        if self.props_mode {
            self.write_val_props(ws, labels, is_multi)?;
        }
        if self.metadata_mode {
            self.write_metadata(ws, labels)?;
        }
        Ok(())
    }
    /// The modes written only into a file that holds the shapes themselves.
    fn write_whole_file_modes(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        model: &Handle<StepDataStepModel>,
        labels: &[TdfLabel],
        is_extern_file: bool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        if self.color_mode {
            self.write_colors(ws, labels)?;
        }
        if self.layer_mode {
            self.write_layers(ws, labels)?;
        }
        // do not store SHUO for extern reference for the moment
        if self.shuo_mode && !is_extern_file {
            self.write_shuos(ws, labels)?;
        }
        if self.gdt_mode {
            let schema = step_live(model, WHERE)?.internal_parameters().write_schema;
            if schema == DeStepWriteModeStepSchema::AP242_DIS {
                self.write_dgts_ap242(ws, labels, factors)?;
            } else {
                self.write_dgts(ws, labels)?;
            }
        }
        if self.mat_mode {
            self.write_materials(ws, labels)?;
        }
        // register all MDGPRs in model
        let entities = model_of(ws)?;
        for index in 1..=self.map_comp_mdgpr.extent() {
            let mdgpr = self.map_comp_mdgpr.find_from_index(index)?;
            entities.add_with_refs(mdgpr, 0, false)?;
        }
        Ok(())
    }
    /// `transferExternFiles`: an assembly as a structure of empty compounds, each part in its own file.
    fn transfer_extern_files(
        &mut self,
        label: &TdfLabel,
        mode: StepControlStepModelType,
        labels: &mut Vec<TdfLabel>,
        factors: &StepDataFactors,
        prefix: &str,
        progress: &ProgressRange<'_>,
    ) -> Result<Shape, Failure> {
        // if label already translated, just return the shape
        if let Some(known) = self.labels.get(label) {
            return Ok(known.clone());
        }
        // if not assembly, write to separate file
        if !XCAFDocShapeTool::is_assembly(label)? && !XCAFDocShapeTool::is_component(label)? {
            return self.write_part_to_extern_file(label, mode, labels, prefix, progress);
        }
        let mut compound: Shape = empty_compound()?.into();
        let mut comp_labels = Vec::new();
        let mut assembly = label.clone();
        // if specified shape is component then high-level assembly is considered
        // to get valid structure with location
        if XCAFDocShapeTool::is_component(label)? {
            comp_labels.push(label.clone());
            assembly = label.father();
        } else if XCAFDocShapeTool::is_assembly(label)? {
            // else iterate on components add create structure of empty compounds
            // representing the assembly
            XCAFDocShapeTool::get_components(label, &mut comp_labels, false)?;
        }
        labels.push(assembly.clone());
        let scope = ProgressScope::unnamed(progress, comp_labels.len() as f64, false);
        // Iterate on requested shapes
        for component in &comp_labels {
            if !scope.more() {
                break;
            }
            let mut referred = TdfLabel::default();
            if !XCAFDocShapeTool::get_referred_shape(component, &mut referred)? {
                continue;
            }
            let mut sub_shape = self.transfer_extern_files(
                &referred,
                mode,
                labels,
                factors,
                prefix,
                &scope.next(1.0),
            )?;
            sub_shape.set_location(&XCAFDocShapeTool::get_location(component)?, false)?;
            BRepBuilder::default().add(&mut compound, &sub_shape)?;
        }
        self.labels.insert(assembly, compound.clone());
        Ok(compound)
    }
    /// A part in a file of its own; the result is an empty compound replacing the shape.
    fn write_part_to_extern_file(
        &mut self,
        label: &TdfLabel,
        mode: StepControlStepModelType,
        labels: &mut Vec<TdfLabel>,
        prefix: &str,
        progress: &ProgressRange<'_>,
    ) -> Result<Shape, Failure> {
        labels.push(label.clone());
        // prepare for transfer
        let new_ws = XSControlWorkSession::new();
        step_live(&new_ws, WHERE)?.select_norm("STEP")?;
        let writer = StepControlWriter::with_session(&new_ws, true)?;
        // construct the name for extern file
        let base_name = Handle::new(HAsciiString::new(prefix));
        get_label_name(label, &base_name)?;
        let new_name = self.free_extern_file_name(&base_name)?;
        // translate and record extern file
        let ext_file = Handle::new(STEPCAFControlExternFile::new());
        let live = step_live(&ext_file, WHERE)?;
        live.set_ws(&new_ws);
        live.set_name(&Handle::new(HAsciiString::new(new_name.as_bytes())));
        live.set_label(label);
        let model = writer.model(false)?;
        let live_model = step_live(&model, WHERE)?;
        let assembly_mode = live_model.internal_parameters().write_assembly;
        live_model.internal_parameters_mut().write_assembly = DeStepWriteModeAssembly::OFF;
        let done = self.transfer(
            &writer,
            std::slice::from_ref(label),
            mode,
            None,
            true,
            progress,
        )?;
        live.set_transfer_status(done);
        live_model.internal_parameters_mut().write_assembly = assembly_mode;
        self.lab_ef.insert(label.clone(), ext_file.clone());
        self.files.bind(new_name, ext_file.clone());
        // return empty compound as replacement for the shape
        let compound: Shape = empty_compound()?.into();
        self.labels.insert(label.clone(), compound.clone());
        Ok(compound)
    }
    /// The name `<base>.stp`, or `<base>_<k>.stp` to avoid confusions.
    fn free_extern_file_name(
        &self,
        base_name: &Handle<HAsciiString>,
    ) -> Result<AsciiString, Failure> {
        let base = step_live(base_name, WHERE)?.string().as_bytes().to_vec();
        let with_suffix = |suffix: &str| {
            let mut bytes = base.clone();
            bytes.extend_from_slice(suffix.as_bytes());
            AsciiString::from_bytes(&bytes)
        };
        let mut name = with_suffix(".stp");
        if self.files.is_bound(&name) {
            for k in 1..32000 {
                name = with_suffix(&format!("_{k}.stp"));
                if !self.files.is_bound(&name) {
                    break;
                }
            }
        }
        Ok(name)
    }
}

/// Writes names for the sub-shapes: the name of each label on its STEP representation item.
fn write_sub_shape_names(
    ws: &Handle<XSControlWorkSession>,
    labels: &[TdfLabel],
) -> Result<(), Failure> {
    let writer = step_live(ws, WHERE)?.transfer_writer();
    let fp = step_live(&writer, WHERE)?.finder_process();
    let process = step_live(&fp, WHERE)?;
    // Iterate on requested sub shapes
    for label in labels {
        let mut children = TdfChildIterator::new(label, true)?;
        while children.more() {
            let sub_label = children.value().clone();
            children.next();
            // Access name recorded in OCAF TDataStdName attribute
            let name = Handle::new(HAsciiString::default());
            if !get_label_name(&sub_label, &name)? {
                continue;
            }
            // Access topological data
            let sub_shape = shape_of(&sub_label)?;
            if sub_shape.is_null() {
                continue;
            }
            // Access the correspondent STEP Representation Item
            let mapper = TransferBRep::shape_mapper(&fp, &sub_shape)?;
            let mut item: Handle<StepReprRepresentationItem> = Handle::default();
            if !process.find_typed_transient_as(
                &mapper.cast(),
                StepReprRepresentationItem::static_type(),
                &mut item,
            ) {
                continue;
            }
            // Record the name
            step_live(&item, WHERE)?.set_name(&name);
        }
    }
    Ok(())
}
