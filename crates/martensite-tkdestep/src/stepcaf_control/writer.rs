//! `STEPCAFControlWriter`: writes an XDE document to a STEP file: the shapes through
//! `StepControlWriter`, then the names, colours, layers, SHUOs, GD&T, materials, validation
//! properties and metadata that each mode asks for, and the external files of a multi-file write.
use super::{STEPCAFControlController, STEPCAFControlExternFile};
use crate::de_step::{DeStepParameters, StepControlStepModelType};
use crate::step_control::StepControlWriter;
use crate::step_data::StepDataStepModel;
use crate::step_entity::step_live;
use crate::step_repr::{StepReprProductDefinitionShape, StepReprRepresentationItem};
use crate::step_visual::{StepVisualDraughtingModel, StepVisualPresentationStyleAssignment};
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::{DataMap, HArray1, IndexedDataMap};
use martensite_tkernel::osd::CadPath;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkshhealing::shape_process::ShapeProcessOperationsFlags;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::xs_algo::{
    DEShapeFixParameters, XSAlgoParameterMap, XSAlgoProcessingFlags,
};
use martensite_tkxsbase::xs_control::XSControlWorkSession;
use std::collections::HashMap;
use std::io::Write;

pub(super) const WHERE: &str = "STEPCAFControlWriter";

/// The state of the writer, apart from the writer of the main file: the transfer also runs on the
/// writers of external files, so it takes its writer as an argument.
pub(super) struct WriterState {
    pub(super) files: DataMap<AsciiString, Handle<STEPCAFControlExternFile>>,
    /// `myRootLabels`: the roots of the documents written, each once.
    pub(super) root_labels: Vec<TdfLabel>,
    pub(super) labels: HashMap<TdfLabel, Shape>,
    pub(super) lab_ef: HashMap<TdfLabel, Handle<STEPCAFControlExternFile>>,
    pub(super) pure_ref_labels: HashMap<TdfLabel, Shape>,
    pub(super) color_mode: bool,
    pub(super) name_mode: bool,
    pub(super) layer_mode: bool,
    pub(super) props_mode: bool,
    pub(super) metadata_mode: bool,
    pub(super) shuo_mode: bool,
    /// `myMapCompMDGPR`: a map by address, here in the order of creation, so a file does not
    /// change from run to run.
    pub(super) map_comp_mdgpr: IndexedDataMap<Shape, Handle<StandardTransient>, ShapeMapHasher>,
    pub(super) gdt_mode: bool,
    pub(super) mat_mode: bool,
    pub(super) vis_mat_mode: bool,
    pub(super) is_clean_duplicates: bool,
    pub(super) gdt_annotations: Vec<Handle<StepReprRepresentationItem>>,
    pub(super) gdt_presentation_dm: Handle<StepVisualDraughtingModel>,
    pub(super) gdt_prs_curve_style: Handle<HArray1<Handle<StepVisualPresentationStyleAssignment>>>,
    pub(super) gdt_common_pds: Handle<StepReprProductDefinitionShape>,
}

impl Default for WriterState {
    fn default() -> Self {
        Self {
            files: DataMap::default(),
            root_labels: Vec::new(),
            labels: HashMap::new(),
            lab_ef: HashMap::new(),
            pure_ref_labels: HashMap::new(),
            color_mode: true,
            name_mode: true,
            layer_mode: true,
            props_mode: true,
            metadata_mode: true,
            shuo_mode: true,
            map_comp_mdgpr: IndexedDataMap::default(),
            gdt_mode: true,
            mat_mode: true,
            vis_mat_mode: false,
            is_clean_duplicates: false,
            gdt_annotations: Vec::new(),
            gdt_presentation_dm: Handle::default(),
            gdt_prs_curve_style: Handle::default(),
            gdt_common_pds: Handle::default(),
        }
    }
}

impl WriterState {
    /// Records the root of a label, once.
    fn add_root(&mut self, label: &TdfLabel) -> Result<(), Failure> {
        let root = label.root()?;
        if !self.root_labels.contains(&root) {
            self.root_labels.push(root);
        }
        Ok(())
    }
}

/// ```no_run
/// use martensite_tkdestep::stepcaf_control::STEPCAFControlWriter;
/// let mut writer = STEPCAFControlWriter::new().unwrap();
/// assert!(writer.get_color_mode() && !writer.get_visual_material_mode());
/// writer.set_shuo_mode(false);
/// ```
pub struct STEPCAFControlWriter {
    my_writer: StepControlWriter,
    state: WriterState,
}

macro_rules! writer_mode {
    ($($(#[$meta:meta])* $set:ident, $get:ident, $field:ident;)+) => {
        $(
            $(#[$meta])*
            pub fn $set(&mut self, mode: bool) {
                self.state.$field = mode;
            }
            $(#[$meta])*
            pub fn $get(&self) -> bool {
                self.state.$field
            }
        )+
    };
}

impl STEPCAFControlWriter {
    /// `STEPCAFControlWriter()`: all modes on but the visual materials and the cleaning of duplicates.
    pub fn new() -> Result<Self, Failure> {
        STEPCAFControlController::init()?;
        Self::with_controller_ready(&XSControlWorkSession::new(), true)
    }
    /// `STEPCAFControlWriter(WS, scratch)`.
    pub fn with_session(ws: &Handle<XSControlWorkSession>, scratch: bool) -> Result<Self, Failure> {
        STEPCAFControlController::init()?;
        Self::with_controller_ready(ws, scratch)
    }
    fn with_controller_ready(
        ws: &Handle<XSControlWorkSession>,
        scratch: bool,
    ) -> Result<Self, Failure> {
        let mut writer = Self {
            my_writer: StepControlWriter::with_session(ws, scratch)?,
            state: WriterState::default(),
        };
        writer.init(ws, scratch)?;
        Ok(writer)
    }
    /// `init`: clears the internal data and sets the session.
    pub fn init(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        scratch: bool,
    ) -> Result<(), Failure> {
        step_live(ws, WHERE)?.select_norm("STEP")?;
        self.my_writer.set_ws(ws, scratch)?;
        self.state.files.clear(true);
        self.state.lab_ef.clear();
        self.state.labels.clear();
        self.state.pure_ref_labels.clear();
        self.state.root_labels.clear();
        self.state.gdt_presentation_dm = Handle::new(StepVisualDraughtingModel::new());
        self.state.gdt_prs_curve_style = crate::step_entity::step_new_array1::<
            Handle<StepVisualPresentationStyleAssignment>,
        >(1, 1)?;
        Ok(())
    }
    /// `Write(theFileName)`: the file, then the external files not written yet.
    pub fn write(&self, file_name: &str) -> Result<IFSelectReturnStatus, Failure> {
        if self.state.is_clean_duplicates {
            // remove duplicates
            self.my_writer.clean_duplicate_entities()?;
        }
        let mut status = self.my_writer.write(file_name)?;
        if status != IFSelectReturnStatus::RetDone {
            return Ok(status);
        }
        // get directory name of the main file
        let mut main_file = CadPath::new(file_name.as_bytes());
        main_file.set_name("");
        main_file.set_extension("");
        let dir_path = main_file.system_name();
        for (_, ext_file) in self.state.files.items() {
            let live = step_live(ext_file, WHERE)?;
            if live.get_write_status() != IFSelectReturnStatus::RetVoid {
                continue;
            }
            // construct extern file name
            let name = live.get_name();
            let name = step_live(&name, WHERE)?.string().as_bytes().to_vec();
            let mut full_name = CadPath::absolute_path(dir_path.as_bytes(), &name)?;
            if full_name.is_empty() {
                full_name = AsciiString::from_bytes(&name);
            }
            let path = String::from_utf8_lossy(full_name.as_bytes()).into_owned();
            let written = step_live(&live.get_ws(), WHERE)?.send_all(&path, false)?;
            live.set_write_status(written);
            if written != IFSelectReturnStatus::RetDone {
                status = written;
            }
        }
        Ok(status)
    }
    /// `write_stream`: `RetError` with external files, which a stream cannot hold.
    pub fn write_stream(&self, stream: &mut dyn Write) -> Result<IFSelectReturnStatus, Failure> {
        if !self.state.files.is_empty() {
            // writing external files is unsupported via stream interface
            return Ok(IFSelectReturnStatus::RetError);
        }
        self.my_writer.write_stream(stream)
    }
    fn model(&self) -> Result<Handle<StepDataStepModel>, Failure> {
        let ws = self.my_writer.ws();
        Ok(step_live(&ws, WHERE)?.model().cast())
    }
    /// The parameters of the statics, as the `InternalParameters.InitFromStatic`.
    fn parameters_of_statics(&self) -> Result<DeStepParameters, Failure> {
        let model = self.model()?;
        let live = step_live(&model, WHERE)?;
        live.internal_parameters_mut().init_from_static();
        let parameters = live.internal_parameters().clone();
        Ok(parameters)
    }
    /// `transfer(doc, mode, is_multi, progress)`: the free shapes of a document.
    pub fn transfer_document(
        &mut self,
        doc: &Handle<TDocStdDocument>,
        mode: StepControlStepModelType,
        multi: Option<&str>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let parameters = self.parameters_of_statics()?;
        self.transfer_document_with(doc, &parameters, mode, multi, progress)
    }
    /// `transfer(doc, params, mode, is_multi, progress)`.
    pub fn transfer_document_with(
        &mut self,
        doc: &Handle<TDocStdDocument>,
        parameters: &DeStepParameters,
        mode: StepControlStepModelType,
        multi: Option<&str>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let main = step_live(doc, WHERE)?.main()?;
        let tool = XCAFDocDocumentTool::shape_tool(&main)?;
        let Some(tool) = tool.get() else {
            return Ok(false);
        };
        self.state.root_labels.clear();
        self.state.add_root(&main)?;
        let labels = tool.get_free_shapes()?;
        self.transfer_labels_prepared(&labels, parameters, mode, multi, progress)
    }
    /// `transfer(label, mode, is_multi, progress)`.
    pub fn transfer_label(
        &mut self,
        label: &TdfLabel,
        mode: StepControlStepModelType,
        multi: Option<&str>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let parameters = self.parameters_of_statics()?;
        self.transfer_label_with(label, &parameters, mode, multi, progress)
    }
    /// `transfer(label, params, mode, is_multi, progress)`: false for a null label.
    pub fn transfer_label_with(
        &mut self,
        label: &TdfLabel,
        parameters: &DeStepParameters,
        mode: StepControlStepModelType,
        multi: Option<&str>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        if label.is_null() {
            return Ok(false);
        }
        self.state.root_labels.clear();
        self.state.add_root(label)?;
        let labels = [label.clone()];
        self.transfer_labels_prepared(&labels, parameters, mode, multi, progress)
    }
    /// `transfer(label_seq, mode, is_multi, progress)`.
    pub fn transfer_labels(
        &mut self,
        labels: &[TdfLabel],
        mode: StepControlStepModelType,
        multi: Option<&str>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let parameters = self.parameters_of_statics()?;
        self.transfer_labels_with(labels, &parameters, mode, multi, progress)
    }
    /// `transfer(label_seq, params, mode, is_multi, progress)`.
    pub fn transfer_labels_with(
        &mut self,
        labels: &[TdfLabel],
        parameters: &DeStepParameters,
        mode: StepControlStepModelType,
        multi: Option<&str>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.state.root_labels.clear();
        for label in labels.iter().filter(|label| !label.is_null()) {
            self.state.add_root(label)?;
        }
        self.transfer_labels_prepared(labels, parameters, mode, multi, progress)
    }
    fn transfer_labels_prepared(
        &mut self,
        labels: &[TdfLabel],
        parameters: &DeStepParameters,
        mode: StepControlStepModelType,
        multi: Option<&str>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let model = self.model()?;
        *step_live(&model, WHERE)?.internal_parameters_mut() = parameters.clone();
        self.state
            .transfer(&self.my_writer, labels, mode, multi, false, progress)
    }
    /// `perform(doc, file_name, progress)`.
    pub fn perform(
        &mut self,
        doc: &Handle<TDocStdDocument>,
        file_name: &str,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let parameters = self.parameters_of_statics()?;
        self.perform_with(doc, file_name, &parameters, progress)
    }
    /// `perform(doc, file_name, params, progress)`.
    pub fn perform_with(
        &mut self,
        doc: &Handle<TDocStdDocument>,
        file_name: &str,
        parameters: &DeStepParameters,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        if !self.transfer_document_with(
            doc,
            parameters,
            StepControlStepModelType::AS_IS,
            None,
            progress,
        )? {
            return Ok(false);
        }
        Ok(self.write(file_name)? == IFSelectReturnStatus::RetDone)
    }
    /// `extern_files`.
    pub fn extern_files(&self) -> &DataMap<AsciiString, Handle<STEPCAFControlExternFile>> {
        &self.state.files
    }
    /// `extern_file(label, ext_file)`: the data of the external file of a label.
    pub fn extern_file_of_label(
        &self,
        label: &TdfLabel,
        file: &mut Handle<STEPCAFControlExternFile>,
    ) -> bool {
        match self.state.lab_ef.get(label) {
            Some(found) => {
                *file = found.clone();
                true
            }
            None => {
                *file = Handle::default();
                false
            }
        }
    }
    /// `extern_file(name, ext_file)`: the test is `!IsEmpty || !IsBound`, so this finds
    /// nothing, whatever the name.
    pub fn extern_file(&self, _name: &str, file: &mut Handle<STEPCAFControlExternFile>) -> bool {
        *file = Handle::default();
        false
    }
    /// `change_writer`.
    pub fn change_writer(&mut self) -> &mut StepControlWriter {
        &mut self.my_writer
    }
    /// `writer`.
    pub fn writer(&self) -> &StepControlWriter {
        &self.my_writer
    }
    writer_mode! {
        /// `SetColorMode`, `GetColorMode`.
        set_color_mode, get_color_mode, color_mode;
        /// `SetNameMode`, `GetNameMode`.
        set_name_mode, get_name_mode, name_mode;
        /// `SetLayerMode`, `GetLayerMode`.
        set_layer_mode, get_layer_mode, layer_mode;
        /// `SetPropsMode`, `GetPropsMode`.
        set_props_mode, get_props_mode, props_mode;
        /// `SetMetadataMode`, `GetMetadataMode`.
        set_metadata_mode, get_metadata_mode, metadata_mode;
        /// `SetSHUOMode`, `GetSHUOMode`.
        set_shuo_mode, get_shuo_mode, shuo_mode;
        /// `SetDimTolMode`, `GetDimTolMode`.
        set_dim_tol_mode, get_dim_tol_mode, gdt_mode;
        /// `SetMaterialMode`, `GetMaterialMode`.
        set_material_mode, get_material_mode, mat_mode;
        /// `SetVisualMaterialMode`, `GetVisualMaterialMode`.
        set_visual_material_mode, get_visual_material_mode, vis_mat_mode;
        /// `SetCleanDuplicates`, `GetCleanDuplicates`.
        set_clean_duplicates, get_clean_duplicates, is_clean_duplicates;
    }
    /// `set_shape_fix_parameters(parameters)`.
    pub fn set_shape_fix_parameters(&self, parameters: XSAlgoParameterMap) -> Result<(), Failure> {
        self.my_writer.set_shape_fix_parameters(parameters)
    }
    /// `set_shape_fix_parameters(parameters, additional_parameters)`.
    pub fn set_shape_fix_parameters_from(
        &self,
        parameters: &DEShapeFixParameters,
        additional: XSAlgoParameterMap,
    ) -> Result<(), Failure> {
        self.my_writer
            .set_shape_fix_parameters_from(parameters, additional)
    }
    /// `GetShapeFixParameters`.
    pub fn shape_fix_parameters(&self) -> Result<XSAlgoParameterMap, Failure> {
        self.my_writer.shape_fix_parameters()
    }
    /// `set_shape_process_flags`.
    pub fn set_shape_process_flags(
        &self,
        flags: ShapeProcessOperationsFlags,
    ) -> Result<(), Failure> {
        self.my_writer.set_shape_process_flags(flags)
    }
    /// `GetShapeProcessFlags`.
    pub fn shape_process_flags(&self) -> Result<XSAlgoProcessingFlags, Failure> {
        self.my_writer.shape_process_flags()
    }
}
