//! `StepDataStepReaderTool`: drives the reading of a model from a `StepDataStepReaderData`.
use super::{
    StepDataFileRecognizer, StepDataProtocolOps, StepDataReadWriteModule, StepDataStepModel,
    StepDataStepReaderData, StepDataUndefinedEntity,
};
use martensite_tkernel::{
    message::Message,
    standard::{Failure, Handle, RuntimeTyped, SharedState, StandardTransient, exceptions},
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceFileReaderData, InterfaceFileReaderDataParams,
    InterfaceFileReaderTool, InterfaceFileReaderToolBase, InterfaceFileReaderToolLoad,
    InterfaceFileReaderToolOps, InterfaceGeneralLib, InterfaceInterfaceModel,
    InterfaceModelEntities, InterfaceModelReports, InterfaceReaderLib,
};
use std::fmt::Write as _;

/// ```
/// use martensite_tkdestep::step_data::{StepData, StepDataStepReaderData, StepDataStepReaderTool};
/// use martensite_tkernel::{resource::FormatType, standard::Handle};
/// let data = Handle::new(StepDataStepReaderData::new(0, 0, 0, FormatType::Utf8));
/// let tool = StepDataStepReaderTool::new(&data, &StepData::protocol());
/// assert!(tool.prepare(true).is_ok());
/// ```
pub struct StepDataStepReaderTool {
    base: InterfaceFileReaderToolBase,
    stepdat: Handle<StepDataStepReaderData>,
    thereco: SharedState<Handle<dyn StepDataFileRecognizer>>,
    libs: SharedState<(InterfaceGeneralLib, InterfaceReaderLib)>,
}

impl StepDataStepReaderTool {
    /// `StepDataStepReaderTool(reader, protocol)`.
    pub fn new(
        reader: &Handle<StepDataStepReaderData>,
        protocol: &Handle<dyn StepDataProtocolOps>,
    ) -> Self {
        let general = protocol.cast();
        let tool = Self {
            base: InterfaceFileReaderToolBase::default(),
            stepdat: reader.clone(),
            thereco: SharedState::new(Handle::default()),
            libs: SharedState::new((
                InterfaceGeneralLib::new(&general),
                InterfaceReaderLib::new(&general),
            )),
        };
        tool.set_data(&reader.cast(), &general);
        tool
    }
    fn data_ref(&self) -> Result<&StepDataStepReaderData, Failure> {
        self.stepdat.get().ok_or_else(|| {
            exceptions::null_object("StepDataStepReaderTool: null data; expected STEP reader data")
        })
    }
    /// `prepare(reco, optimize = true)`: works with a specific recognizer.
    pub fn prepare_with(
        &self,
        reco: &Handle<dyn StepDataFileRecognizer>,
        optimize: bool,
    ) -> Result<(), Failure> {
        *self.thereco.write() = reco.clone();
        self.prepare(optimize)
    }
    /// `prepare(optimize = true)`: resolves the references and binds empty entities. When
    /// errors are handled, a failure is reported and the reading continues.
    pub fn prepare(&self, optimize: bool) -> Result<(), Failure> {
        let result = self.prepare_entities(optimize);
        if !self.error_handle() {
            return result;
        }
        if let Err(failure) = result {
            let _ = write!(
                Message::send_info(),
                " Exception Raised during Preparation :\n{failure}\n Now, trying to continue, but with presomption of failure\n"
            );
        }
        Ok(())
    }
    fn prepare_entities(&self, optimize: bool) -> Result<(), Failure> {
        self.data_ref()?.set_entity_numbers(optimize)?;
        self.set_entities()
    }
    /// `prepare_header(reco)`: binds entities to the header records.
    pub fn prepare_header(&self, reco: &Handle<dyn StepDataFileRecognizer>) -> Result<(), Failure> {
        let data = self.data_ref()?;
        let mut rank = data.find_next_header_record(0);
        while rank != 0 {
            let mut ent = Handle::default();
            if let Some(live) = reco.get() {
                if !live.evaluate(&data.record_type(rank), &mut ent) {
                    ent = self.unknown_entity()?;
                }
            } else {
                let mut ach = Handle::new(InterfaceCheck::new());
                let mut libs = self.libs.write();
                let (glib, rlib) = &mut *libs;
                self.recognize_by_lib(rank, glib, rlib, &mut ach, &mut ent)?;
            }
            if ent.is_null() {
                ent = self.unknown_entity()?;
            }
            data.bind_entity(rank, &ent)?;
            rank = data.find_next_header_record(rank);
        }
        data.prepare_header();
        Ok(())
    }
    fn header_entity_read(
        &self,
        rank: i32,
        ent: &Handle<StandardTransient>,
        model: &StepDataStepModel,
    ) -> Result<(), Failure> {
        let data = self.data_ref()?;
        let mut ach = Handle::new(InterfaceCheck::with_entity(ent));
        self.analyse_record(rank, ent, &mut ach)?;
        if ent.is_kind(StepDataUndefinedEntity::static_type()) {
            let text = data.c_type(rank);
            ach.add_warning_text(
                &format!("Header Entity not Recognized, StepType: {text}"),
                "",
            );
        }
        if ach.has_failed() || ach.has_warnings() {
            let global = model.global_check(true);
            global.get_messages(&ach);
            model.set_global_check(&global);
        }
        model.add_header_entity(ent)?;
        trace_header_messages(rank, ent, &ach)
    }
}

/// The trace of the messages of a header entity.
fn trace_header_messages(
    rank: i32,
    ent: &Handle<StandardTransient>,
    ach: &InterfaceCheck,
) -> Result<(), Failure> {
    let name = ent.dynamic_type().map_or("", |kind| kind.name());
    let mut sout = Message::send_trace();
    if ach.has_warnings() {
        let nbmess = ach.nb_warnings();
        let _ = writeln!(
            sout,
            "{nbmess} Warnings on Reading Header Entity N0.{rank}:{name}"
        );
        for rank in 1..=nbmess {
            let text = ach.c_warning(rank, true)?;
            let _ = writeln!(sout, "{}", String::from_utf8_lossy(text.as_bytes()));
        }
    }
    if ach.has_failed() {
        let _ = writeln!(sout, " Fails on Reading Header Entity N0.{rank}:{name}");
        for rank in 1..=ach.nb_fails() {
            let text = ach.c_fail(rank, true)?;
            let _ = writeln!(sout, "{}", String::from_utf8_lossy(text.as_bytes()));
        }
    }
    Ok(())
}

impl InterfaceFileReaderTool for StepDataStepReaderTool {
    fn reader_tool_base(&self) -> &InterfaceFileReaderToolBase {
        &self.base
    }
    fn recognize(
        &self,
        num: i32,
        ach: &mut Handle<InterfaceCheck>,
        ent: &mut Handle<StandardTransient>,
    ) -> Result<bool, Failure> {
        let reco = self.thereco.read().clone();
        if let Some(live) = reco.get() {
            return Ok(live.evaluate(&self.data_ref()?.record_type(num), ent));
        }
        let mut libs = self.libs.write();
        let (glib, rlib) = &mut *libs;
        self.recognize_by_lib(num, glib, rlib, ach, ent)
    }
    fn begin_read(&self, amodel: &Handle<dyn InterfaceInterfaceModel>) -> Result<(), Failure> {
        let model = amodel.cast::<StepDataStepModel>();
        let live = model.get().ok_or_else(|| {
            exceptions::null_object("StepDataStepReaderTool::BeginRead: expected a STEP model")
        })?;
        let data = self.data_ref()?;
        live.clear_header();
        live.set_global_check(&data.global_check());
        let mut rank = data.find_next_header_record(0);
        while rank != 0 {
            self.header_entity_read(rank, &data.bound_entity(rank), live)?;
            rank = data.find_next_header_record(rank);
        }
        Ok(())
    }
    fn analyse_record(
        &self,
        num: i32,
        anent: &Handle<StandardTransient>,
        acheck: &mut Handle<InterfaceCheck>,
    ) -> Result<bool, Failure> {
        let selected = self.libs.read().1.select(anent);
        if let Some((module, cn)) = selected {
            let step = module.cast::<dyn StepDataReadWriteModule>();
            let live = step.get().ok_or_else(|| {
                exceptions::null_object(
                    "StepDataStepReaderTool::AnalyseRecord: expected a STEP read-write module",
                )
            })?;
            live.read_step(cn, &self.stepdat, num, acheck, anent)?;
        } else if let Some(undefined) = anent.cast::<StepDataUndefinedEntity>().get() {
            undefined.read_record(self.data_ref()?, num, acheck)?;
        } else {
            acheck.add_fail_text(
                "# Entity neither Recognized nor set as UndefinedEntity from StepData #",
                "",
            );
        }
        Ok(!acheck.has_failed())
    }
    fn end_read(&self, amodel: &Handle<dyn InterfaceInterfaceModel>) -> Result<(), Failure> {
        let model = amodel.cast::<StepDataStepModel>();
        let Some(live) = model.get() else {
            return Ok(());
        };
        let data = self.data_ref()?;
        // Ident labels of a whole file: one label array is filled in order, so reserve it.
        let mut rank = data.find_next_record(0);
        while rank != 0 {
            live.set_ident_label(&data.bound_entity(rank), data.record_ident(rank));
            rank = data.find_next_record(rank);
        }
        let _ = live.nb_entities();
        Ok(())
    }
}
