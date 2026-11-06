//! The making of an IGES file from a model: the sections, the parameters sent by the modules.
use super::writer_directory::directory_values;
use super::{
    IgesDataDirPart, IgesDataGlobalSection, IgesDataIgesEntity, IgesDataIgesModel,
    IgesDataProtocolOps, IgesDataReadStage, IgesDataTextLines, IgesDataUndefinedEntity,
    IgesDataWriterLib, iges_entity_of,
};
use martensite_tkernel::{
    message::{Gravity, Message},
    standard::{Failure, Handle, RuntimeTyped, StandardTransient},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkxsbase::interface::{
    InterfaceFloatWriter, InterfaceLineBuffer, InterfaceModelEntities, InterfaceModelReports,
    interface_error,
};

use super::writer_param::IgesDataWriterParam;

pub(super) const MAXCARS_G: i32 = 72;
pub(super) const MAXCARS_P: i32 = 64;

/// The writer of an IGES file: fed with a model section by section, then printed.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataIgesModel, IgesDataIgesWriter, IgesDataProtocol};
/// use martensite_tkernel::standard::Handle;
/// let model = IgesDataIgesModel::new();
/// let mut writer = IgesDataIgesWriter::new(&model);
/// let protocol = Handle::new(IgesDataProtocol::default());
/// writer.send_model(&protocol.cast()).unwrap();
/// let mut file = Vec::new();
/// assert!(writer.print(&mut file).unwrap());
/// assert!(file.ends_with(b"T0000001\n"));
/// ```
pub struct IgesDataIgesWriter {
    pub(super) themodel: Handle<IgesDataIgesModel>,
    pub(super) thestar: Handle<IgesDataTextLines>,
    pub(super) thehead: Handle<IgesDataTextLines>,
    thesep: u8,
    theendm: u8,
    pub(super) thedirs: Vec<IgesDataDirPart>,
    pub(super) thepnum: Vec<i32>,
    pub(super) thepars: Handle<IgesDataTextLines>,
    pub(super) thesect: i32,
    thestep: IgesDataReadStage,
    thecurr: InterfaceLineBuffer,
    pub(super) themodew: i32,
    thefloatw: InterfaceFloatWriter,
}

impl Default for IgesDataIgesWriter {
    fn default() -> Self {
        Self {
            themodel: Handle::default(),
            thestar: Handle::default(),
            thehead: Handle::default(),
            thesep: b',',
            theendm: b';',
            thedirs: vec![IgesDataDirPart::default()],
            thepnum: vec![0],
            thepars: Handle::default(),
            thesect: 0,
            thestep: IgesDataReadStage::ReadDir,
            thecurr: InterfaceLineBuffer::new(MAXCARS_G + 1),
            themodew: 0,
            thefloatw: InterfaceFloatWriter::new(9),
        }
    }
}

fn append_line(lines: &Handle<IgesDataTextLines>, line: Handle<HAsciiString>) {
    if let Some(live) = lines.get() {
        live.append(line);
    }
}

impl IgesDataIgesWriter {
    /// A writer for a model, ready for [`Self::send_model`].
    pub fn new(amodel: &Handle<IgesDataIgesModel>) -> Self {
        let nbent = amodel.get().map_or(0, |model| model.nb_entities()).max(0) as usize;
        Self {
            themodel: amodel.clone(),
            thehead: Handle::new(IgesDataTextLines::default()),
            thedirs: vec![IgesDataDirPart::default(); nbent + 1],
            thepnum: vec![0; nbent + 1],
            thepars: Handle::new(IgesDataTextLines::default()),
            ..Self::default()
        }
    }
    /// The writer of the reals, to set its format.
    pub fn float_writer(&mut self) -> &mut InterfaceFloatWriter {
        &mut self.thefloatw
    }
    /// The write mode; from 10 the file is written in its enciphered form.
    pub fn write_mode(&mut self) -> &mut i32 {
        &mut self.themodew
    }
    /// Sends a line of the start section; a long one is cut.
    pub fn send_start_line(&mut self, startline: &[u8]) {
        if startline.is_empty() {
            return;
        }
        if self.thestar.is_null() {
            self.thestar = Handle::new(IgesDataTextLines::default());
        }
        for piece in startline.chunks(MAXCARS_G as usize) {
            append_line(&self.thestar, Handle::new(HAsciiString::new(piece)));
        }
    }
    /// Sends the whole model.
    ///
    /// # Errors
    /// An interface failure when the writer was already fed, a failure of a module.
    pub fn send_model(
        &mut self,
        protocol: &Handle<dyn IgesDataProtocolOps>,
    ) -> Result<(), Failure> {
        let lib = IgesDataWriterLib::new(protocol);
        let model = self.themodel.clone();
        let live = super::iges_live(&model, "IGES writer")?;
        self.section_s()?;
        for i in 1..=live.nb_start_lines() {
            self.send_start_line(&live.start_line(i));
        }
        self.section_g(&live.global_section())?;
        self.sections_dp()?;
        for i in 1..=live.nb_entities() {
            self.send_one_entity(live, i, &lib)?;
        }
        self.section_t()
    }
    fn send_one_entity(
        &mut self,
        live: &IgesDataIgesModel,
        i: i32,
        lib: &IgesDataWriterLib,
    ) -> Result<(), Failure> {
        let ent = live.entity(i);
        let cnt = content_of(live, i, &ent);
        self.dir_part(&cnt)?;
        self.own_params(&ent)?;
        let content: Handle<StandardTransient> = cnt.cast();
        if let Some((module, cn)) = lib.select(&content) {
            super::iges_live(&module, "IGES writer")?.write_own_params(cn, &cnt, self)?;
        } else if let Some(undefined) = cnt.cast::<IgesDataUndefinedEntity>().get() {
            undefined.write_own_params(self)?;
        } else if let Some(entity) = cnt.get() {
            Message::send_text(
                &AsciiString::new(&format!(
                    " -- IGES writer: not processed for n0.{i} in file, type {}, form {}\n",
                    entity.type_number(),
                    entity.form_number()
                )),
                Gravity::Info,
            );
        }
        self.associativities(&cnt)?;
        self.properties(&cnt)?;
        self.end_entity()
    }
    /// Starts the start section.
    ///
    /// # Errors
    /// An interface failure out of the order of the sections.
    pub fn section_s(&mut self) -> Result<(), Failure> {
        self.enter_section(0, 1, "SectionS")
    }
    fn enter_section(&mut self, from: i32, to: i32, name: &str) -> Result<(), Failure> {
        if self.thesect != from {
            return Err(interface_error(&format!(
                "IGES writer: {name}: at section {}; expected {from}",
                self.thesect
            )));
        }
        self.thesect = to;
        Ok(())
    }
    /// Sends the global section.
    ///
    /// # Errors
    /// An interface failure out of the order of the sections, a failure of the line buffer.
    pub fn section_g(&mut self, header: &IgesDataGlobalSection) -> Result<(), Failure> {
        self.enter_section(1, 2, "SectionG")?;
        self.thesep = header.separator();
        self.theendm = header.end_mark();
        self.thecurr.set_max(MAXCARS_G)?;
        let params = header.params()?;
        let set = super::iges_live(&params, "IGES writer")?;
        let nb = set.nb_params();
        for i in 1..=nb {
            let parameter = set.param(i)?;
            self.add_bytes(parameter.c_value(), 0)?;
            let closing = if i < nb { self.thesep } else { self.theendm };
            self.add_char(closing, 0)?;
        }
        if self.thecurr.length() > 0 {
            append_line(&self.thehead, self.thecurr.moved()?);
        }
        Ok(())
    }
    /// Starts the directory and parameter sections.
    ///
    /// # Errors
    /// An interface failure out of the order of the sections, a failure of the line buffer.
    pub fn sections_dp(&mut self) -> Result<(), Failure> {
        self.enter_section(2, 3, "SectionsDP")?;
        self.thecurr.set_max(MAXCARS_P)?;
        self.thestep = IgesDataReadStage::ReadEnd;
        Ok(())
    }
    /// Ends the sections.
    ///
    /// # Errors
    /// An interface failure out of the order of the sections.
    pub fn section_t(&mut self) -> Result<(), Failure> {
        self.enter_section(3, 4, "SectionT")?;
        let lines = self
            .thepars
            .get()
            .map_or(0, |pars| pars.sequence(|items| items.length()));
        if let Some(last) = self.thepnum.last_mut() {
            *last = lines + 1;
        }
        Ok(())
    }
    fn check_stage(&self, stage: IgesDataReadStage, name: &str) -> Result<(), Failure> {
        if self.thesect != 3 && self.thestep != stage {
            return Err(interface_error(&format!(
                "IGES writer: {name}: at section {}, stage {:?}; expected section 3 or stage {stage:?}",
                self.thesect, self.thestep
            )));
        }
        Ok(())
    }
    /// Fills the directory entry of an entity; nothing for an entity not in the model.
    ///
    /// # Errors
    /// An interface failure out of the order of the sections.
    pub fn dir_part<T: ?Sized + RuntimeTyped>(&mut self, anent: &Handle<T>) -> Result<(), Failure> {
        self.check_stage(IgesDataReadStage::ReadEnd, "DirPart")?;
        let entity = iges_entity_of(anent);
        let model = self.themodel.clone();
        let (Some(live), Some(ent)) = (model.get(), entity.get()) else {
            return Ok(());
        };
        let nument = live.number(&entity.cast());
        if nument == 0 {
            return Ok(());
        }
        let values = directory_values(live, ent);
        if let Some(slot) = self.thedirs.get_mut(nument as usize) {
            slot.init(&values);
        }
        self.thestep = IgesDataReadStage::ReadDir;
        Ok(())
    }
    /// Starts the parameters of an entity, by its type number.
    ///
    /// # Errors
    /// An interface failure out of the order of the sections, a failure of the line buffer.
    pub fn own_params<T: ?Sized + RuntimeTyped>(
        &mut self,
        anent: &Handle<T>,
    ) -> Result<(), Failure> {
        self.check_stage(IgesDataReadStage::ReadDir, "OwnParams")?;
        let entity = iges_entity_of(anent);
        let number = self
            .themodel
            .get()
            .map_or(0, |model| model.number(&entity.cast()));
        let lines = self
            .thepars
            .get()
            .map_or(0, |pars| pars.sequence(|items| items.length()));
        if let Some(slot) = usize::try_from(number - 1)
            .ok()
            .and_then(|at| self.thepnum.get_mut(at))
        {
            *slot = lines + 1;
        }
        self.thecurr.clear()?;
        let text = entity
            .get()
            .map_or(0, IgesDataIgesEntity::type_number)
            .to_string();
        self.add_bytes(text.as_bytes(), 0)?;
        self.thestep = IgesDataReadStage::ReadOwn;
        Ok(())
    }
    /// Sends the properties of an entity, when it has some.
    ///
    /// # Errors
    /// An interface failure out of the order of the sections, a failure of the line buffer.
    pub fn properties<T: ?Sized + RuntimeTyped>(
        &mut self,
        anent: &Handle<T>,
    ) -> Result<(), Failure> {
        self.check_stage(IgesDataReadStage::ReadOwn, "Properties")?;
        self.thestep = IgesDataReadStage::ReadProps;
        let entity = iges_entity_of(anent);
        let Some(live) = entity.get() else {
            return Ok(());
        };
        if !live.are_present_properties() {
            return Ok(());
        }
        self.send_integer(live.nb_properties())?;
        self.send_list(&live.properties())
    }
    /// Sends the associativities of an entity, when it has some or when properties follow.
    ///
    /// # Errors
    /// An interface failure out of the order of the sections, a failure of the line buffer.
    pub fn associativities<T: ?Sized + RuntimeTyped>(
        &mut self,
        anent: &Handle<T>,
    ) -> Result<(), Failure> {
        self.check_stage(IgesDataReadStage::ReadOwn, "Associativities")?;
        self.thestep = IgesDataReadStage::ReadAssocs;
        let entity = iges_entity_of(anent);
        let Some(live) = entity.get() else {
            return Ok(());
        };
        if !live.are_present_associativities() && !live.are_present_properties() {
            return Ok(());
        }
        self.send_integer(live.nb_associativities())?;
        self.send_list(&live.associativities())
    }
    fn send_list(
        &mut self,
        iter: &martensite_tkxsbase::interface::InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.start();
        while iter.more() {
            self.send_entity(&iter.value()?.cast::<IgesDataIgesEntity>(), false)?;
            iter.next();
        }
        Ok(())
    }
    /// Ends the parameters of an entity.
    ///
    /// # Errors
    /// An interface failure out of the order of the sections, a failure of the line buffer.
    pub fn end_entity(&mut self) -> Result<(), Failure> {
        self.check_stage(IgesDataReadStage::ReadOwn, "EndEntity")?;
        self.add_char(self.theendm, 0)?;
        if self.thecurr.length() > 0 {
            append_line(&self.thepars, self.thecurr.moved()?);
        }
        self.thestep = IgesDataReadStage::ReadEnd;
        Ok(())
    }
    fn section_lines(&self) -> &Handle<IgesDataTextLines> {
        if self.thesect < 3 {
            &self.thehead
        } else {
            &self.thepars
        }
    }
    fn add_bytes(&mut self, val: &[u8], more: i32) -> Result<(), Failure> {
        let mut lnstr = val.len() as i32;
        if !self.thecurr.can_get(lnstr + more + 1) {
            let line = self.thecurr.moved()?;
            append_line(self.section_lines(), line);
        }
        let maxcars = if self.thesect == 3 {
            MAXCARS_P
        } else {
            MAXCARS_G
        };
        let mut from = 0usize;
        while lnstr > maxcars {
            self.thecurr.add_bytes(&val[from..], lnstr)?;
            let line = self.thecurr.moved()?;
            append_line(self.section_lines(), line);
            from += maxcars as usize;
            lnstr -= maxcars;
        }
        self.thecurr.add_bytes(&val[from..], lnstr)
    }
    fn add_char(&mut self, val: u8, more: i32) -> Result<(), Failure> {
        if !self.thecurr.can_get(1 + more) {
            let line = self.thecurr.moved()?;
            append_line(self.section_lines(), line);
        }
        self.thecurr.add_bytes(&[val], 1)
    }
    /// Sends an empty parameter.
    ///
    /// # Errors
    /// A failure of the line buffer.
    pub fn send_void(&mut self) -> Result<(), Failure> {
        self.add_char(self.thesep, 0)
    }
    /// Sends a parameter: an integer, a real, coordinates, a counted text, or an entity by its
    /// directory number.
    ///
    /// # Errors
    /// A failure of the line buffer or of the formatting of a real.
    pub fn send(&mut self, val: impl IgesDataWriterParam) -> Result<(), Failure> {
        val.send_to(self)
    }
    pub(super) fn send_integer(&mut self, val: i32) -> Result<(), Failure> {
        self.add_char(self.thesep, 0)?;
        self.add_bytes(val.to_string().as_bytes(), 0)
    }
    pub(super) fn send_real(&mut self, val: f64) -> Result<(), Failure> {
        self.add_char(self.thesep, 0)?;
        let mut text = Vec::new();
        self.thefloatw.write(val, &mut text)?;
        self.add_bytes(&text, 0)
    }
    pub(super) fn send_text(&mut self, val: &Handle<HAsciiString>) -> Result<(), Failure> {
        self.add_char(self.thesep, 0)?;
        let Some(text) = val.get() else {
            return Ok(());
        };
        let content = text.string();
        if content.is_empty() {
            return Ok(());
        }
        let mut counted = format!("{}H", content.len()).into_bytes();
        counted.extend_from_slice(content.as_bytes());
        self.add_bytes(&counted, 0)
    }
    /// Sends a boolean as 1 or 0.
    ///
    /// # Errors
    /// A failure of the line buffer.
    pub fn send_boolean(&mut self, val: bool) -> Result<(), Failure> {
        self.add_char(self.thesep, 0)?;
        self.add_bytes(if val { b"1" } else { b"0" }, 0)
    }
    /// Sends an entity by its directory number, 0 for null, negated when `negative`.
    ///
    /// # Errors
    /// A failure of the line buffer.
    pub fn send_entity<T: ?Sized + RuntimeTyped>(
        &mut self,
        val: &Handle<T>,
        negative: bool,
    ) -> Result<(), Failure> {
        let entity = iges_entity_of(val);
        let num = match (entity.is_some(), self.themodel.get()) {
            (true, Some(model)) => model.d_num(&entity),
            _ => 0,
        };
        self.send_integer(if negative { -num } else { num })
    }
    /// Sends a text as it is.
    ///
    /// # Errors
    /// A failure of the line buffer.
    pub fn send_string(&mut self, val: &Handle<HAsciiString>) -> Result<(), Failure> {
        self.add_char(self.thesep, 0)?;
        match val.get() {
            Some(text) => self.add_bytes(text.string().as_bytes(), 0),
            None => Ok(()),
        }
    }
    /// The lines of a section: 1 start, 2 global, from 3 parameters; null for another number.
    pub fn section_strings(&self, numsec: i32) -> Handle<IgesDataTextLines> {
        match numsec {
            1 => self.thestar.clone(),
            2 => self.thehead.clone(),
            num if num >= 3 => self.thepars.clone(),
            _ => Handle::default(),
        }
    }
}

/// The entity that gives the content of a model's entity: the one a report redefines it with.
fn content_of(
    live: &IgesDataIgesModel,
    i: i32,
    ent: &Handle<IgesDataIgesEntity>,
) -> Handle<IgesDataIgesEntity> {
    if !live.is_redefined_content(i) {
        return ent.clone();
    }
    Message::send_text(
        &AsciiString::new(&format!(" --  IGES writer: erroneous entity n0.{i}  --\n")),
        Gravity::Info,
    );
    let report = live.report_entity(i, false);
    let content = report.get().map_or_else(Handle::default, |rep| {
        rep.content().cast::<IgesDataIgesEntity>()
    });
    if content.is_null() {
        ent.clone()
    } else {
        content
    }
}
