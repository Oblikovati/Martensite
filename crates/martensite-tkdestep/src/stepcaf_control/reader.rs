//! `STEPCAFControlReader`: reads a STEP file into an XDE document, translating shapes and then
//! the names, colours, layers, validation properties, SHUOs, GD&T, materials, views and metadata
//! each mode asks for.
use super::{STEPCAFControlController, STEPCAFControlExternFile};
use crate::de_step::DeStepParameters;
use crate::step_basic::StepBasicProductDefinition;
use crate::step_construct::StepConstructExternRefs;
use crate::step_control::StepControlReader;
use crate::step_data::{StepDataFactors, StepDataStepModel};
use crate::step_entity::step_live;
use crate::step_repr::{StepReprNextAssemblyUsageOccurrence, StepReprProductDefinitionShape};
use crate::step_shape::StepShapeShapeRepresentation;
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkbrep::toptools::{MapOfShape, ShapeMapHasher};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::{DataMap, HSequence};
use martensite_tkernel::osd::CadPath;
use martensite_tkernel::resource::FormatType;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::{AsciiString, ExtendedString, HAsciiString};
use martensite_tkernel::units_methods::LengthUnit;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::toploc::Location;
use martensite_tkshhealing::shape_process::ShapeProcessOperationsFlags;
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocShapeTool};
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::interface::InterfaceModelEntities;
use martensite_tkxsbase::transfer_brep::TransferBRep;
use martensite_tkxsbase::xs_algo::{
    DEShapeFixParameters, XSAlgoParameterMap, XSAlgoProcessingFlags, XSAlgoShapeProcessor,
};
use martensite_tkxsbase::xs_control::{XSControlReaderOps, XSControlWorkSession};
use std::io::Read;

mod transfer;
#[allow(unused_imports, reason = "what the sibling modules use")]
pub use transfer::*;

pub(super) const WHERE: &str = "STEPCAFControlReader";

/// The labels of the shapes that were transferred.
pub type STEPCAFControlShapeLabelMap = DataMap<Shape, TdfLabel, ShapeMapHasher>;
/// The product definitions of a transfer and the external files they refer to.
pub(super) type PdFileMap =
    DataMap<Handle<StepBasicProductDefinition>, Handle<STEPCAFControlExternFile>>;
/// The shapes originating from SDRs and their product definitions.
pub(super) type ShapePdMap = DataMap<Shape, Handle<StepBasicProductDefinition>, ShapeMapHasher>;

/// `fill_shapes_map`: all compounds and their components, without location.
fn fill_shapes_map(shape: &Shape, map: &mut MapOfShape) -> Result<(), Failure> {
    let mut located = shape.clone();
    located.set_location(&Location::default(), false)?;
    map.add(located);
    if shape.shape_type()? != ShapeEnum::Compound {
        return Ok(());
    }
    let mut components = ShapeIterator::new(shape, true, true);
    while components.more() {
        fill_shapes_map(components.value()?, map)?;
        components.next();
    }
    Ok(())
}

/// The state of the reader, apart from the reader of the main file: `transfer` runs also on the
/// readers of external files, so the transfer takes its reader as an argument.
pub(super) struct ReaderState {
    pub(super) files: DataMap<AsciiString, Handle<STEPCAFControlExternFile>>,
    pub(super) map: STEPCAFControlShapeLabelMap,
    color_mode: bool,
    name_mode: bool,
    layer_mode: bool,
    props_mode: bool,
    meta_mode: bool,
    product_meta_mode: bool,
    shuo_mode: bool,
    gdt_mode: bool,
    mat_mode: bool,
    view_mode: bool,
    pub(super) gdt_map: DataMap<Handle<StandardTransient>, TdfLabel>,
    /// The source code page of the main model is not `NoConversion`: names are UTF-8.
    names_are_multi_byte: bool,
    /// The session of the main reader: `expand_sub_shapes` reads it, even for an external file.
    pub(super) main_ws: Handle<XSControlWorkSession>,
}

/// ```no_run
/// use martensite_tkdestep::stepcaf_control::STEPCAFControlReader;
/// let mut reader = STEPCAFControlReader::new().unwrap();
/// assert!(reader.get_color_mode() && !reader.get_shuo_mode());
/// reader.set_shuo_mode(true);
/// ```
pub struct STEPCAFControlReader {
    my_reader: StepControlReader,
    state: ReaderState,
}

impl Default for ReaderState {
    fn default() -> Self {
        Self {
            files: DataMap::default(),
            map: DataMap::default(),
            color_mode: true,
            name_mode: true,
            layer_mode: true,
            props_mode: true,
            meta_mode: true,
            product_meta_mode: false,
            shuo_mode: false,
            gdt_mode: true,
            mat_mode: true,
            view_mode: true,
            gdt_map: DataMap::default(),
            names_are_multi_byte: false,
            main_ws: Handle::default(),
        }
    }
}

macro_rules! reader_mode {
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

impl STEPCAFControlReader {
    /// `STEPCAFControlReader()`: all modes on but SHUO and product metadata.
    pub fn new() -> Result<Self, Failure> {
        STEPCAFControlController::init()?;
        let mut reader = Self {
            my_reader: StepControlReader::new()?,
            state: ReaderState::default(),
        };
        let ws = reader.my_reader.xs_reader().ws();
        if ws.is_some() {
            reader.init(&ws, true)?;
        }
        Ok(reader)
    }
    /// `STEPCAFControlReader(WS, scratch)`.
    pub fn with_session(ws: &Handle<XSControlWorkSession>, scratch: bool) -> Result<Self, Failure> {
        STEPCAFControlController::init()?;
        let mut reader = Self {
            my_reader: StepControlReader::with_session(ws, scratch)?,
            state: ReaderState::default(),
        };
        reader.init(ws, scratch)?;
        Ok(reader)
    }
    /// `init`: clears the internal data and sets the session; necessary only in the writer, to set
    /// good actor: `WS->SelectNorm("STEP")`.
    pub fn init(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        scratch: bool,
    ) -> Result<(), Failure> {
        self.my_reader.xs_reader_mut().set_ws(ws, scratch)?;
        self.state.files.clear(true);
        self.state.map.clear(true);
        Ok(())
    }
    /// `read_file(file_name)`.
    pub fn read_file(&mut self, file_name: &str) -> Result<IFSelectReturnStatus, Failure> {
        self.my_reader.read_file(file_name)
    }
    /// `read_file(file_name, params)`.
    pub fn read_file_with(
        &mut self,
        file_name: &str,
        parameters: &DeStepParameters,
    ) -> Result<IFSelectReturnStatus, Failure> {
        self.my_reader.read_file_with(file_name, parameters)
    }
    /// `read_stream(name, i_stream)`.
    pub fn read_stream(
        &mut self,
        name: &str,
        stream: &mut dyn Read,
    ) -> Result<IFSelectReturnStatus, Failure> {
        self.my_reader.read_stream(name, stream)
    }
    /// `read_stream(name, params, i_stream)`.
    pub fn read_stream_with(
        &mut self,
        name: &str,
        parameters: &DeStepParameters,
        stream: &mut dyn Read,
    ) -> Result<IFSelectReturnStatus, Failure> {
        self.my_reader.read_stream_with(name, parameters, stream)
    }
    /// `nb_roots_for_transfer`.
    pub fn nb_roots_for_transfer(&mut self) -> Result<i32, Failure> {
        self.my_reader.nb_roots_for_transfer()
    }
    /// The name conversion of the main model is taken before each transfer.
    fn prepare_names(&mut self) -> Result<(), Failure> {
        self.state.main_ws = self.my_reader.xs_reader().ws();
        let model = self.my_reader.step_model()?;
        self.state.names_are_multi_byte =
            step_live(&model, WHERE)?.source_code_page() != FormatType::NoConversion;
        Ok(())
    }
    /// `transfer_one_root`: one root into the document.
    pub fn transfer_one_root(
        &mut self,
        num: i32,
        doc: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.prepare_names()?;
        let mut labels = Vec::new();
        self.state
            .transfer(&mut self.my_reader, num, doc, &mut labels, false, progress)
    }
    /// `transfer(doc)`: all roots into the document.
    pub fn transfer(
        &mut self,
        doc: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.transfer_one_root(0, doc, progress)
    }
    /// `perform(filename, doc)`.
    pub fn perform(
        &mut self,
        filename: &str,
        doc: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        if self.read_file(filename)? != IFSelectReturnStatus::RetDone {
            return Ok(false);
        }
        self.transfer(doc, progress)
    }
    /// `perform(filename, doc, params)`.
    pub fn perform_with(
        &mut self,
        filename: &str,
        doc: &Handle<TDocStdDocument>,
        parameters: &DeStepParameters,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        if self.read_file_with(filename, parameters)? != IFSelectReturnStatus::RetDone {
            return Ok(false);
        }
        self.transfer(doc, progress)
    }
    /// `extern_files`.
    pub fn extern_files(&self) -> &DataMap<AsciiString, Handle<STEPCAFControlExternFile>> {
        &self.state.files
    }
    /// `extern_file`: the data of an external file, false when it was not read.
    pub fn extern_file(&self, name: &str, file: &mut Handle<STEPCAFControlExternFile>) -> bool {
        *file = Handle::default();
        let key = AsciiString::new(name);
        match self.state.files.find(&key) {
            Ok(found) => {
                *file = found.clone();
                true
            }
            Err(_) => false,
        }
    }
    /// `change_reader`.
    pub fn change_reader(&mut self) -> &mut StepControlReader {
        &mut self.my_reader
    }
    /// `Reader`.
    pub fn reader(&self) -> &StepControlReader {
        &self.my_reader
    }
    reader_mode! {
        /// `SetColorMode` and `GetColorMode`.
        set_color_mode, get_color_mode, color_mode;
        /// `SetNameMode` and `GetNameMode`.
        set_name_mode, get_name_mode, name_mode;
        /// `SetLayerMode` and `GetLayerMode`.
        set_layer_mode, get_layer_mode, layer_mode;
        /// `SetPropsMode` and `GetPropsMode`.
        set_props_mode, get_props_mode, props_mode;
        /// `SetMetaMode` and `GetMetaMode`.
        set_meta_mode, get_meta_mode, meta_mode;
        /// `SetProductMetaMode` and `GetProductMetaMode`.
        set_product_meta_mode, get_product_meta_mode, product_meta_mode;
        /// `SetSHUOMode` and `GetSHUOMode`.
        set_shuo_mode, get_shuo_mode, shuo_mode;
        /// `SetGDTMode` and `GetGDTMode`.
        set_gdt_mode, get_gdt_mode, gdt_mode;
        /// `SetMatMode` and `GetMatMode`.
        set_mat_mode, get_mat_mode, mat_mode;
        /// `SetViewMode` and `GetViewMode`.
        set_view_mode, get_view_mode, view_mode;
    }
    /// `get_shape_label_map`.
    pub fn get_shape_label_map(&self) -> &STEPCAFControlShapeLabelMap {
        &self.state.map
    }
    /// `set_shape_fix_parameters(parameters)`.
    pub fn set_shape_fix_parameters(&self, parameters: XSAlgoParameterMap) -> Result<(), Failure> {
        self.my_reader
            .xs_reader()
            .set_shape_fix_parameters(parameters);
        Ok(())
    }
    /// `set_shape_fix_parameters(parameters, additional_parameters)`.
    pub fn set_shape_fix_parameters_from(
        &self,
        parameters: &DEShapeFixParameters,
        additional: &XSAlgoParameterMap,
    ) -> Result<(), Failure> {
        self.my_reader
            .xs_reader()
            .set_shape_fix_parameters_from(parameters, additional);
        Ok(())
    }
    /// `GetShapeFixParameters`.
    pub fn shape_fix_parameters(&self) -> Result<XSAlgoParameterMap, Failure> {
        Ok(self.my_reader.xs_reader().shape_fix_parameters())
    }
    /// `set_shape_process_flags`.
    pub fn set_shape_process_flags(
        &self,
        flags: ShapeProcessOperationsFlags,
    ) -> Result<(), Failure> {
        self.my_reader.xs_reader().set_shape_process_flags(flags);
        Ok(())
    }
    /// `GetShapeProcessFlags`.
    pub fn shape_process_flags(&self) -> Result<XSAlgoProcessingFlags, Failure> {
        Ok(self.my_reader.xs_reader().shape_process_flags())
    }
}

/// What the shapes of a transfer say about the product definitions in the model.
struct TransferredProducts {
    new_shapes: MapOfShape,
    shape_pd: ShapePdMap,
    pd_file: PdFileMap,
    seq_pds: Handle<HSequence<Handle<StandardTransient>>>,
}

impl ReaderState {
    /// `convertName`: when the source code page is not `NoConversion` the string is treated as
    /// UTF-8, else each character is copied to an extended character.
    pub(super) fn convert_name(&self, name: &AsciiString) -> ExtendedString {
        ExtendedString::from_ascii_string(name, self.names_are_multi_byte)
    }
    /// `prepareUnits`: the length unit of the document, set from the session when it has none.
    fn prepare_units(
        model: &Handle<StepDataStepModel>,
        doc: &Handle<TDocStdDocument>,
        factors: &mut StepDataFactors,
    ) -> Result<(), Failure> {
        let mut scale = 1.;
        if !XCAFDocDocumentTool::get_length_unit_in(doc, &mut scale, LengthUnit::Millimeter)? {
            XSAlgoShapeProcessor::prepare_for_transfer(); // update unit info
            scale = martensite_tkxsbase::xs_algo::xs_algo_units().get_cascade_length_unit_default();
            // Sets length unit to the document
            XCAFDocDocumentTool::set_length_unit_in(doc, scale, LengthUnit::Millimeter)?;
        }
        factors.set_cascade_unit(scale);
        step_live(model, WHERE)?.set_local_length_unit(scale);
        Ok(())
    }
    /// Transfers the roots with the reader; false when the transfer stops at a user break.
    fn transfer_roots(
        reader: &mut StepControlReader,
        nroot: i32,
        roots: i32,
        root_scope: &ProgressScope<'_>,
    ) -> Result<bool, Failure> {
        if nroot != 0 {
            if nroot > roots {
                return Ok(false);
            }
            let range = root_scope.next(1.0);
            reader.transfer_one_root(nroot, &range)?;
        } else {
            let range = root_scope.next(1.0);
            let scope = ProgressScope::unnamed(&range, f64::from(roots), false);
            let mut i = 1;
            while i <= roots && scope.more() {
                let step = scope.next(1.0);
                reader.transfer_one_root(i, &step)?;
                i += 1;
            }
        }
        Ok(!root_scope.user_break())
    }
}
