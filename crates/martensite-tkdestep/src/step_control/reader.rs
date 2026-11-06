//! `StepControlReader`: reads a STEP file into a work session and transfers its roots.
use super::StepControlController;
use crate::api_header_section::APIHeaderSectionMakeHeader;
use crate::de_step::DeStepParameters;
use crate::step_data::StepDataStepModel;
use crate::step_entity::step_live;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkshhealing::shape_process::{ShapeProcessOperation, ShapeProcessOperationsFlags};
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::interface::{InterfaceInterfaceModel, InterfaceProtocol};
use martensite_tkxsbase::xs_algo::DEShapeFixParameters;
use martensite_tkxsbase::xs_control::{XSControlReader, XSControlReaderOps, XSControlWorkSession};
use std::io::Read;

/// Reading through this class follows this flow:
/// ```no_run
/// use martensite_tkdestep::step_control::StepControlReader;
/// use martensite_tkernel::message::ProgressRange;
/// use martensite_tkxsbase::if_select::IFSelectReturnStatus;
/// use martensite_tkxsbase::xs_control::XSControlReaderOps;
/// let mut reader = StepControlReader::new().unwrap();
/// assert_eq!(reader.read_file("part.stp").unwrap(), IFSelectReturnStatus::RetDone);
/// reader.transfer_roots(&ProgressRange::default()).unwrap();
/// let shape = reader.xs_reader().one_shape();
/// ```
pub struct StepControlReader {
    reader: XSControlReader,
}

impl StepControlReader {
    /// `StepControlReader()`.
    pub fn new() -> Result<Self, Failure> {
        StepControlController::init()?;
        let mut reader = XSControlReader::new();
        reader.set_norm("STEP")?;
        Ok(Self { reader })
    }
    /// `StepControlReader(WS, scratch)`.
    pub fn with_session(ws: &Handle<XSControlWorkSession>, scratch: bool) -> Result<Self, Failure> {
        StepControlController::init()?;
        let mut reader = XSControlReader::new();
        reader.set_ws(ws, scratch)?;
        reader.set_norm("STEP")?;
        Ok(Self { reader })
    }
    /// `step_model`.
    pub fn step_model(&self) -> Result<Handle<StepDataStepModel>, Failure> {
        Ok(self.reader.model()?.cast())
    }
    /// The status of a read, recording the model on success. An exception in the read is the
    /// `RetFail` (its trace goes to the messenger).
    fn finish_read(
        &self,
        name: &str,
        model: Handle<dyn InterfaceInterfaceModel>,
        outcome: Result<i32, Failure>,
    ) -> Result<IFSelectReturnStatus, Failure> {
        let status = match outcome {
            Ok(0) => IFSelectReturnStatus::RetDone,
            Ok(stat) if stat < 0 => IFSelectReturnStatus::RetError,
            _ => IFSelectReturnStatus::RetFail,
        };
        if status != IFSelectReturnStatus::RetDone {
            return Ok(status);
        }
        let ws = self.reader.ws();
        let session = step_live(&ws, "StepControlReader")?;
        session.set_model(&model, true)?;
        session.set_loaded_file(name);
        session.init_transfer_reader(4)?;
        Ok(status)
    }
    /// A new model with the given parameters and the header of a new file.
    fn new_model_with(
        parameters: Option<&DeStepParameters>,
    ) -> Result<Handle<StepDataStepModel>, Failure> {
        let model = StepDataStepModel::new();
        let live = step_live(&model, "StepControlReader")?;
        match parameters {
            Some(given) => *live.internal_parameters_mut() = given.clone(),
            None => live.internal_parameters_mut().init_from_static(),
        }
        // `set_source_code_page(InternalParameters.ReadCodePage)`: the page is that parameter here.
        APIHeaderSectionMakeHeader::new(0)?.apply(live)?;
        Ok(model)
    }
    /// The library and the protocol of the session; `None` gives `RetVoid`.
    fn library_and_protocol(
        &self,
    ) -> Result<
        Option<(
            Handle<dyn martensite_tkxsbase::if_select::IFSelectWorkLibrary>,
            Handle<dyn InterfaceProtocol>,
        )>,
        Failure,
    > {
        let ws = self.reader.ws();
        let session = step_live(&ws, "StepControlReader")?;
        let (library, protocol) = (session.work_library(), session.protocol());
        Ok((library.is_some() && protocol.is_some()).then_some((library, protocol)))
    }
    fn read_named_file(
        &self,
        filename: &str,
        parameters: Option<&DeStepParameters>,
    ) -> Result<IFSelectReturnStatus, Failure> {
        let Some((library, protocol)) = self.library_and_protocol()? else {
            return Ok(IFSelectReturnStatus::RetVoid);
        };
        let model = Self::new_model_with(parameters)?;
        let mut generic: Handle<dyn InterfaceInterfaceModel> = model.cast();
        let outcome =
            step_live(&library, "StepControlReader")?.read_file(filename, &mut generic, &protocol);
        self.finish_read(filename, generic, outcome)
    }
    fn read_named_stream(
        &self,
        name: &str,
        parameters: Option<&DeStepParameters>,
        stream: &mut dyn Read,
    ) -> Result<IFSelectReturnStatus, Failure> {
        let Some((library, protocol)) = self.library_and_protocol()? else {
            return Ok(IFSelectReturnStatus::RetVoid);
        };
        let model = Self::new_model_with(parameters)?;
        let mut generic: Handle<dyn InterfaceInterfaceModel> = model.cast();
        let outcome = step_live(&library, "StepControlReader")?.read_stream(
            name,
            stream,
            &mut generic,
            &protocol,
        );
        self.finish_read(name, generic, outcome)
    }
    /// `read_file(filename, params)`.
    pub fn read_file_with(
        &self,
        filename: &str,
        parameters: &DeStepParameters,
    ) -> Result<IFSelectReturnStatus, Failure> {
        self.read_named_file(filename, Some(parameters))
    }
    /// `read_stream(name, params, i_stream)`.
    pub fn read_stream_with(
        &self,
        name: &str,
        parameters: &DeStepParameters,
        stream: &mut dyn Read,
    ) -> Result<IFSelectReturnStatus, Failure> {
        self.read_named_stream(name, Some(parameters), stream)
    }
    /// `set_system_length_unit(length_unit)`: nothing without a model.
    pub fn set_system_length_unit(&self, length_unit: f64) -> Result<(), Failure> {
        if let Some(live) = self.step_model()?.get() {
            live.set_local_length_unit(length_unit);
        }
        Ok(())
    }
    /// `system_length_unit`: 1 without a model.
    pub fn system_length_unit(&self) -> Result<f64, Failure> {
        Ok(self
            .step_model()?
            .get()
            .map_or(1.0, StepDataStepModel::local_length_unit))
    }
}

impl XSControlReaderOps for StepControlReader {
    fn xs_reader(&self) -> &XSControlReader {
        &self.reader
    }
    fn xs_reader_mut(&mut self) -> &mut XSControlReader {
        &mut self.reader
    }
    /// `read_file(filename)`.
    fn read_file(&mut self, filename: &str) -> Result<IFSelectReturnStatus, Failure> {
        self.read_named_file(filename, None)
    }
    /// `read_stream(name, i_stream)`.
    fn read_stream(
        &mut self,
        name: &str,
        istream: &mut dyn Read,
    ) -> Result<IFSelectReturnStatus, Failure> {
        self.read_named_stream(name, None, istream)
    }
    /// `nb_roots_for_transfer`: the product_definitions that no assembly uses.
    fn nb_roots_for_transfer(&mut self) -> Result<i32, Failure> {
        if self.reader.therootsta {
            return Ok(self.reader.theroots.len() as i32);
        }
        self.reader.therootsta = true;
        let roots = super::reader_roots::roots_for_transfer(&self.reader.ws())?;
        self.reader.theroots.extend(roots);
        Ok(self.reader.theroots.len() as i32)
    }
    /// `GetDefaultShapeFixParameters`.
    fn default_shape_fix_parameters(&self) -> DEShapeFixParameters {
        super::shape_fix_parameters::default_shape_fix_parameters()
    }
    /// `GetDefaultShapeProcessFlags`: `fix_shape`.
    fn default_shape_process_flags(&self) -> ShapeProcessOperationsFlags {
        let mut flags = ShapeProcessOperationsFlags::default();
        flags.set(ShapeProcessOperation::FixShape);
        flags
    }
}

/// Roots are `StandardTransient`.
pub(super) type RootEntity = Handle<StandardTransient>;
