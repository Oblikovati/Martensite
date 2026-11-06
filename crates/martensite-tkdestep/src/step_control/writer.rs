//! `StepControlWriter`: transfers shapes into a STEP model and writes it.
use super::shape_fix_parameters::default_shape_fix_parameters;
use super::{StepControlActorWrite, StepControlController};
use crate::api_header_section::APIHeaderSectionMakeHeader;
use crate::de_step::{DeStepParameters, StepControlStepModelType};
use crate::step_data::{StepDataProtocolOps, StepDataStepModel, StepDataStepWriter};
use crate::step_entity::step_live;
use crate::step_tidy::StepTidyDuplicateCleaner;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkshhealing::shape_process::{ShapeProcessOperation, ShapeProcessOperationsFlags};
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::interface::InterfaceInterfaceModel;
use martensite_tkxsbase::transfer::TransferActorOfFinderProcessOps;
use martensite_tkxsbase::xs_algo::{
    DEShapeFixParameters, XSAlgoParameterMap, XSAlgoProcessingFlags, XSAlgoShapeProcessor,
    xs_algo_units,
};
use martensite_tkxsbase::xs_control::XSControlWorkSession;
use std::io::Write;

/// ```no_run
/// use martensite_tkdestep::de_step::StepControlStepModelType;
/// use martensite_tkdestep::step_control::StepControlWriter;
/// use martensite_tkbrep::topods::Shape;
/// use martensite_tkernel::message::ProgressRange;
/// let mut writer = StepControlWriter::new().unwrap();
/// writer.transfer(&Shape::default(), StepControlStepModelType::AS_IS, true, &ProgressRange::default()).unwrap();
/// writer.write("part.stp").unwrap();
/// ```
pub struct StepControlWriter {
    thesession: Handle<XSControlWorkSession>,
}

impl StepControlWriter {
    /// `StepControlWriter()`.
    pub fn new() -> Result<Self, Failure> {
        StepControlController::init()?;
        let mut writer = Self {
            thesession: Handle::default(),
        };
        writer.set_ws(&XSControlWorkSession::new(), false)?;
        Ok(writer)
    }
    /// `StepControlWriter(WS, scratch)`.
    pub fn with_session(ws: &Handle<XSControlWorkSession>, scratch: bool) -> Result<Self, Failure> {
        StepControlController::init()?;
        let mut writer = Self {
            thesession: Handle::default(),
        };
        writer.set_ws(ws, scratch)?;
        Ok(writer)
    }
    /// `set_ws(WS, scratch)`.
    pub fn set_ws(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        scratch: bool,
    ) -> Result<(), Failure> {
        self.thesession = ws.clone();
        let live = step_live(ws, "StepControlWriter")?;
        live.select_norm("STEP")?;
        live.init_transfer_reader(0)?;
        self.model(scratch)?;
        Ok(())
    }
    /// `WS`.
    pub fn ws(&self) -> Handle<XSControlWorkSession> {
        self.thesession.clone()
    }
    fn session(&self) -> Result<&XSControlWorkSession, Failure> {
        step_live(&self.thesession, "StepControlWriter")
    }
    /// `model(newone)`.
    pub fn model(&self, new_one: bool) -> Result<Handle<StepDataStepModel>, Failure> {
        let session = self.session()?;
        let current: Handle<StepDataStepModel> = session.model().cast();
        if new_one || current.is_null() {
            return Ok(session.new_model()?.cast());
        }
        Ok(current)
    }
    fn write_actor(&self) -> Result<Handle<dyn TransferActorOfFinderProcessOps>, Failure> {
        let controller = self.session()?.norm_adaptor();
        Ok(controller
            .get()
            .map(|live| live.actor_write())
            .unwrap_or_default())
    }
    /// `set_tolerance(Tol)`.
    pub fn set_tolerance(&self, tolerance: f64) -> Result<(), Failure> {
        let actor: Handle<StepControlActorWrite> = self.write_actor()?.cast();
        if let Some(live) = actor.get() {
            live.set_tolerance(tolerance);
        }
        Ok(())
    }
    /// `unset_tolerance`.
    pub fn unset_tolerance(&self) -> Result<(), Failure> {
        self.set_tolerance(-1.0)
    }
    /// `transfer(sh, mode, compgraph, progress)`: with the parameters of the statics.
    pub fn transfer(
        &self,
        shape: &Shape,
        mode: StepControlStepModelType,
        compgraph: bool,
        progress: &ProgressRange<'_>,
    ) -> Result<IFSelectReturnStatus, Failure> {
        let model: Handle<StepDataStepModel> = self.session()?.model().cast();
        let live = step_live(&model, "StepControlWriter::Transfer")?;
        live.internal_parameters_mut().init_from_static();
        let parameters = live.internal_parameters().clone();
        self.transfer_with(shape, mode, &parameters, compgraph, progress)
    }
    /// `transfer(sh, mode, params, compgraph, progress)`.
    pub fn transfer_with(
        &self,
        shape: &Shape,
        mode: StepControlStepModelType,
        parameters: &DeStepParameters,
        compgraph: bool,
        progress: &ProgressRange<'_>,
    ) -> Result<IFSelectReturnStatus, Failure> {
        let mws = match mode {
            StepControlStepModelType::AS_IS => 0,
            StepControlStepModelType::FACETED_BREP => 1,
            StepControlStepModelType::SHELL_BASED_SURFACE_MODEL => 2,
            StepControlStepModelType::MANIFOLD_SOLID_BREP => 3,
            StepControlStepModelType::GEOMETRIC_CURVE_SET => 4,
            _ => return Ok(IFSelectReturnStatus::RetError), // cas non reconnu
        };
        let session = self.session()?;
        let writer = session.transfer_writer();
        step_live(&writer, "StepControlWriter::Transfer")?.set_transfer_mode(mws);
        let model = self.model(false)?;
        let live = step_live(&model, "StepControlWriter::Transfer")?;
        if !live.is_initialized_unit() {
            XSAlgoShapeProcessor::prepare_for_transfer(); // update unit info
            live.set_local_length_unit(xs_algo_units().get_cascade_length_unit_default());
        }
        *live.internal_parameters_mut() = parameters.clone();
        APIHeaderSectionMakeHeader::new(0)?.apply(live)?;
        let actor: Handle<StepControlActorWrite> = self.write_actor()?.cast();
        step_live(&actor, "StepControlWriter::Transfer")?
            .set_group_mode(live.internal_parameters().write_assembly.as_raw());
        self.initialize_missing_parameters()?;
        session.transfer_write_shape(shape, compgraph, progress)
    }
    /// `Write(theFileName)`.
    pub fn write(&self, file_name: &str) -> Result<IFSelectReturnStatus, Failure> {
        let model = self.model(false)?;
        let Some(live) = model.get() else {
            return Ok(IFSelectReturnStatus::RetFail);
        };
        APIHeaderSectionMakeHeader::new(0)?.apply(live)?;
        self.session()?.send_all(file_name, false)
    }
    /// `write_stream(o_stream)`.
    pub fn write_stream(&self, stream: &mut dyn Write) -> Result<IFSelectReturnStatus, Failure> {
        let model = self.model(false)?;
        let Some(live) = model.get() else {
            return Ok(IFSelectReturnStatus::RetFail);
        };
        let protocol: Handle<dyn StepDataProtocolOps> =
            InterfaceInterfaceModel::protocol(live).cast();
        if protocol.is_null() {
            return Ok(IFSelectReturnStatus::RetFail);
        }
        let mut writer = StepDataStepWriter::new(&model);
        writer.send_model(&protocol, false)?;
        APIHeaderSectionMakeHeader::new(0)?.apply(live)?;
        let printed = writer
            .print(stream)
            .map_err(|error| Failure::new(&format!("StepControlWriter::WriteStream: {error}")))?;
        Ok(if printed {
            IFSelectReturnStatus::RetDone
        } else {
            IFSelectReturnStatus::RetFail
        })
    }
    /// `print_stats_transfer(what, mode)`.
    pub fn print_stats_transfer(&self, what: i32, mode: i32) -> Result<(), Failure> {
        let writer = self.session()?.transfer_writer();
        step_live(&writer, "StepControlWriter")?.print_stats(what, mode)
    }
    /// `clean_duplicate_entities`.
    pub fn clean_duplicate_entities(&self) -> Result<(), Failure> {
        StepTidyDuplicateCleaner::new(&self.thesession).perform()
    }
    /// `set_shape_fix_parameters(parameters)`.
    pub fn set_shape_fix_parameters(&self, parameters: XSAlgoParameterMap) -> Result<(), Failure> {
        if let Some(live) = self.write_actor()?.get() {
            live.shape_processing().set_shape_fix_parameters(parameters);
        }
        Ok(())
    }
    /// `set_shape_fix_parameters(parameters, additional_parameters)`.
    pub fn set_shape_fix_parameters_from(
        &self,
        parameters: &DEShapeFixParameters,
        additional: XSAlgoParameterMap,
    ) -> Result<(), Failure> {
        if let Some(live) = self.write_actor()?.get() {
            live.shape_processing()
                .set_shape_fix_parameters_from(parameters, &additional);
        }
        Ok(())
    }
    /// `GetShapeFixParameters`: empty without an actor.
    pub fn shape_fix_parameters(&self) -> Result<XSAlgoParameterMap, Failure> {
        let actor = self.write_actor()?;
        Ok(actor
            .get()
            .map(|live| live.shape_processing().shape_fix_parameters())
            .unwrap_or_default())
    }
    /// `set_shape_process_flags(flags)`.
    pub fn set_shape_process_flags(
        &self,
        flags: ShapeProcessOperationsFlags,
    ) -> Result<(), Failure> {
        if let Some(live) = self.write_actor()?.get() {
            live.shape_processing().set_flags(flags);
        }
        Ok(())
    }
    /// `GetShapeProcessFlags`: empty without an actor.
    pub fn shape_process_flags(&self) -> Result<XSAlgoProcessingFlags, Failure> {
        let actor = self.write_actor()?;
        Ok(actor
            .get()
            .map(|live| live.shape_processing().flags())
            .unwrap_or_default())
    }
    /// `initialize_missing_parameters`.
    fn initialize_missing_parameters(&self) -> Result<(), Failure> {
        if self.shape_fix_parameters()?.is_empty() {
            self.set_shape_fix_parameters_from(
                &default_shape_fix_parameters(),
                XSAlgoParameterMap::default(),
            )?;
        }
        if !self.shape_process_flags()?.1 {
            let mut flags = ShapeProcessOperationsFlags::default();
            flags.set(ShapeProcessOperation::SplitCommonVertex);
            flags.set(ShapeProcessOperation::DirectFaces);
            self.set_shape_process_flags(flags)?;
        }
        Ok(())
    }
}
