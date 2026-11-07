//! The reading of the records of a file into entities.
use super::param_reader::{iges_atoi, iges_msg};
use super::{
    IgesDataFileRecognizer, IgesDataIgesEntity, IgesDataIgesModel, IgesDataIgesReaderData,
    IgesDataIgesType, IgesDataParamReader, IgesDataProtocolOps, IgesDataReadStage,
    IgesDataReadWriteModule, IgesDataUndefinedEntity, iges_live,
};
use martensite_tkernel::standard::SharedState;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceEntityList, InterfaceFileReaderDataParams, InterfaceFileReaderTool,
    InterfaceFileReaderToolBase, InterfaceFileReaderToolLoad, InterfaceFileReaderToolOps,
    InterfaceGeneralLib, InterfaceInterfaceModel, InterfaceModelReports, InterfaceParamList,
    InterfaceParamType, InterfaceProtocol, InterfaceReaderLib,
};

#[derive(Default)]
pub(super) struct ReaderToolFields {
    thelist: Handle<InterfaceParamList>,
    thereco: Handle<dyn IgesDataFileRecognizer>,
    pub(super) thecnum: i32,
    pub(super) thectyp: IgesDataIgesType,
    pub(super) thegradweight: i32,
    pub(super) themaxweight: f64,
    thedefweight: f64,
}

/// The tool that fills the entities of a model from the records of a file.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataIgesReaderData, IgesDataIgesReaderTool, IgesDataProtocol};
/// use martensite_tkernel::standard::Handle;
/// let data = Handle::new(IgesDataIgesReaderData::new(0, 1));
/// let protocol = Handle::new(IgesDataProtocol::default());
/// let _tool = IgesDataIgesReaderTool::new(&data, &protocol.cast());
/// ```
pub struct IgesDataIgesReaderTool {
    base: InterfaceFileReaderToolBase,
    pub(super) state: SharedState<ReaderToolFields>,
    theglib: SharedState<InterfaceGeneralLib>,
    therlib: SharedState<InterfaceReaderLib>,
}

impl IgesDataIgesReaderTool {
    /// A tool over the records of `reader`, with the modules of `protocol`.
    pub fn new(
        reader: &Handle<IgesDataIgesReaderData>,
        protocol: &Handle<dyn IgesDataProtocolOps>,
    ) -> Self {
        let general: Handle<dyn InterfaceProtocol> = protocol.cast();
        let tool = Self {
            base: InterfaceFileReaderToolBase::default(),
            state: SharedState::new(ReaderToolFields::default()),
            theglib: SharedState::new(InterfaceGeneralLib::new(&general)),
            therlib: SharedState::new(InterfaceReaderLib::new(&general)),
        };
        tool.set_data(&reader.cast(), &general);
        tool
    }
    /// Sets the recognizer up and creates the empty entities.
    ///
    /// # Errors
    /// A failure of the creation of the entities.
    pub fn prepare(&self, reco: &Handle<dyn IgesDataFileRecognizer>) -> Result<(), Failure> {
        self.state.write().thereco = reco.clone();
        self.set_entities()?;
        let data = self.data();
        let params = iges_live(&data, "IGES reader tool")?.params(0)?;
        self.state.write().thelist = params;
        Ok(())
    }
    fn iges_data(&self) -> Handle<IgesDataIgesReaderData> {
        self.data().cast()
    }
    /// Reads the own parameters, by the module of the entity.
    ///
    /// # Errors
    /// A failure of the module.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDataIgesEntity>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let selected = self.therlib.read().select(&ent.cast::<StandardTransient>());
        if let Some((imodule, cn)) = selected {
            let module = imodule.cast::<dyn IgesDataReadWriteModule>();
            return iges_live(&module, "IGES reader tool: read own")?
                .read_own_params(cn, ent, ir, pr);
        }
        if let Some(undefined) = ent.cast::<IgesDataUndefinedEntity>().get() {
            return undefined.read_own_params(ir, pr);
        }
        // The check of these two messages is local to this call: they go nowhere.
        Ok(())
    }
    /// Reads the properties.
    ///
    /// # Errors
    /// A failure of a parameter list.
    pub fn read_props(
        &self,
        ent: &Handle<IgesDataIgesEntity>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let msg = self.located_msg("XSTEP_38")?;
        let ncur = pr.current_number();
        if ncur == pr.nb_params() + 1 {
            pr.end_all();
            return Ok(());
        }
        if !pr.defined_else_skip()? {
            return Ok(());
        }
        let mut nbprops = 0;
        if !pr.read_integer(&ncur.into(), &mut nbprops)? {
            pr.send_fail(&iges_msg("XSTEP_221"));
            return Ok(());
        }
        if nbprops == 0 {
            return Ok(());
        }
        let mut props = InterfaceEntityList::new();
        if pr.read_ent_list(ir, &pr.current_list(nbprops, 1), &msg, &mut props, false)? {
            iges_live(ent, "IGES reader tool: read props")?.load_properties(&props);
        }
        Ok(())
    }
    /// Reads the associativities.
    ///
    /// # Errors
    /// A failure of a parameter list.
    pub fn read_assocs(
        &self,
        ent: &Handle<IgesDataIgesEntity>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let msg = self.located_msg("XSTEP_37")?;
        if pr.current_number() == pr.nb_params() + 1 {
            pr.end_all();
            return Ok(());
        }
        if !pr.defined_else_skip()? {
            return Ok(());
        }
        let mut nbassocs = 0;
        if !pr.read_integer(&pr.current(), &mut nbassocs)? {
            pr.send_fail(&iges_msg("XSTEP_220"));
            return Ok(());
        }
        if nbassocs == 0 {
            return Ok(());
        }
        let mut assocs = InterfaceEntityList::new();
        if pr.read_ent_list(ir, &pr.current_list(nbassocs, 1), &msg, &mut assocs, false)? {
            iges_live(ent, "IGES reader tool: read assocs")?.load_associativities(&assocs);
        }
        Ok(())
    }
    fn analyse_directory(
        &self,
        num: i32,
        ent: &Handle<IgesDataIgesEntity>,
        igesdat: &Handle<IgesDataIgesReaderData>,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let data = iges_live(igesdat, "IGES reader tool")?;
        let mut part = data.dir_part(num);
        if let Some(undefined) = ent.cast::<IgesDataUndefinedEntity>().get() {
            undefined.read_dir(igesdat, &mut part, ach)?;
        }
        self.read_dir(ent, igesdat, &part, ach)
    }
    /// The first parameter of a record and the check that it starts with the entity's type.
    fn parameter_header(
        &self,
        num: i32,
        live: &IgesDataIgesEntity,
        nbpar: i32,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<Option<(Handle<InterfaceParamList>, i32)>, Failure> {
        let data = self.data();
        let reader = iges_live(&data, "IGES reader tool")?;
        let n0par = if num == 1 {
            1
        } else {
            reader.param_first_rank(num - 1)? + 1
        };
        let list = self.state.read().thelist.clone();
        let parameter = iges_live(&list, "IGES reader tool")?.value(n0par)?;
        if parameter.param_type() != InterfaceParamType::ParamInteger
            || iges_atoi(parameter.c_value()) != live.type_number()
        {
            self.send_record_fail("XSTEP_28", ach)?;
            return Ok(None);
        }
        debug_assert!(nbpar >= 1);
        Ok(Some((list, n0par)))
    }
    fn send_record_fail(&self, key: &str, ach: &Handle<InterfaceCheck>) -> Result<(), Failure> {
        let mut msg = iges_msg(key);
        msg.arg_integer(self.state.read().thecnum)?;
        if let Some(check) = ach.get() {
            check.send_fail(&msg);
        }
        Ok(())
    }
    /// Reads the parameter stages; the stage where reading ends decides whether the next ones run.
    fn read_stages(
        &self,
        ent: &Handle<IgesDataIgesEntity>,
        igesdat: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        self.read_own_params(ent, igesdat, pr)?;
        if pr.stage() == IgesDataReadStage::ReadOwn {
            pr.next_stage();
        } else if pr.stage() == IgesDataReadStage::ReadEnd {
            return Ok(());
        }
        self.read_assocs(ent, igesdat, pr)?;
        if pr.stage() == IgesDataReadStage::ReadAssocs {
            pr.next_stage();
        } else if pr.stage() == IgesDataReadStage::ReadEnd {
            return Ok(());
        }
        self.read_props(ent, igesdat, pr)
    }
}

impl InterfaceFileReaderTool for IgesDataIgesReaderTool {
    fn reader_tool_base(&self) -> &InterfaceFileReaderToolBase {
        &self.base
    }
    fn recognize(
        &self,
        num: i32,
        ach: &mut Handle<InterfaceCheck>,
        ent: &mut Handle<StandardTransient>,
    ) -> Result<bool, Failure> {
        let igesdat = self.iges_data();
        let data = iges_live(&igesdat, "IGES reader tool")?;
        let ctyp = data.dir_type(num);
        let reco = {
            let mut state = self.state.write();
            state.thecnum = num;
            state.thectyp = ctyp;
            state.thereco.clone()
        };
        if let Some(recognizer) = reco.get() {
            let mut anent = Handle::default();
            if recognizer.evaluate(&ctyp, &mut anent) {
                *ent = anent.cast();
                return Ok(true);
            }
        }
        let mut glib = self.theglib.write();
        let mut rlib = self.therlib.write();
        self.recognize_by_lib(num, &mut glib, &mut rlib, ach, ent)
    }
    fn begin_read(&self, amodel: &Handle<dyn InterfaceInterfaceModel>) -> Result<(), Failure> {
        let amod = amodel.cast::<IgesDataIgesModel>();
        let model = iges_live(&amod, "IGES reader tool: begin read")?;
        let igesdat = self.iges_data();
        let data = iges_live(&igesdat, "IGES reader tool: begin read")?;
        let gs = data.global_section();
        model.set_start_section(&data.start_section(), false);
        model.set_global_section(&gs);
        let glob = model.global_check(true);
        if let (Some(global), Some(read)) = (glob.get(), data.global_check().get()) {
            global.get_messages(read);
        }
        model.set_global_check(&glob);
        let mut state = self.state.write();
        state.themaxweight = gs.max_line_weight();
        state.thegradweight = gs.line_weight_grad();
        if state.thegradweight > 0 {
            state.themaxweight /= f64::from(state.thegradweight);
            state.thegradweight = 1;
        }
        state.thedefweight = data.default_line_weight();
        Ok(())
    }
    fn analyse_record(
        &self,
        num: i32,
        anent: &Handle<StandardTransient>,
        acheck: &mut Handle<InterfaceCheck>,
    ) -> Result<bool, Failure> {
        let ent = anent.cast::<IgesDataIgesEntity>();
        let igesdat = self.iges_data();
        let live = iges_live(&ent, "IGES reader tool: analyse")?;
        live.clear();
        self.analyse_directory(num, &ent, &igesdat, acheck)?;
        let data = self.data();
        let nbpar = iges_live(&data, "IGES reader tool")?.nb_params(num)?;
        if nbpar < 1 {
            if ent.cast::<IgesDataUndefinedEntity>().is_some() {
                return Ok(true);
            }
            self.send_record_fail("XSTEP_27", acheck)?;
            return Ok(false);
        }
        let Some((list, n0par)) = self.parameter_header(num, live, nbpar, acheck)? else {
            return Ok(false);
        };
        let mut pr = IgesDataParamReader::new(&list, acheck, n0par, nbpar, num);
        self.read_stages(&ent, &igesdat, &mut pr)?;
        if !pr.is_check_empty() {
            *acheck = pr.check().clone();
        }
        Ok(acheck.get().is_none_or(|check| !check.has_failed()))
    }
}
