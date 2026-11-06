//! `StepDataStepWriter`: builds the text of a STEP file, line by line (72 characters at
//! most), under the control of the model and of each entity's module. The sending of
//! parameter values is in `writer_send`.
use super::{StepDataProtocolOps, StepDataStepModel, StepDataUndefinedEntity, StepDataWriterLib};
use martensite_tkernel::{
    standard::{Failure, Handle, StandardTransient},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkxsbase::interface::{
    InterfaceCheckIterator, InterfaceFloatWriter, InterfaceLineBuffer, InterfaceModelEntities,
    InterfaceModelReports, interface_mismatch,
};
use std::io::{self, Write};

/// `StepLong`: the maximum length of a line.
pub(super) const STEP_LONG: i32 = 72;

/// The scope tables, numbered from 1 (the arrays).
#[derive(Default)]
struct ScopeTables {
    thescopebeg: Vec<i32>,
    thescopeend: Vec<i32>,
    thescopenext: Vec<i32>,
}

/// `StepDataStepWriter`. Its members return `result`: the standard ones raise
/// `an interface mismatch`, and the line buffer can fail.
///
/// ```
/// use martensite_tkdestep::step_data::{StepDataStepModel, StepDataStepWriter};
/// use martensite_tkernel::tcollection::AsciiString;
/// let model = StepDataStepModel::new();
/// let mut writer = StepDataStepWriter::new(&model);
/// writer.start_entity(&AsciiString::new("TEST")).unwrap();
/// writer.send_integer(9).unwrap();
/// writer.send_real(1.5).unwrap();
/// writer.end_entity().unwrap();
/// assert_eq!(writer.line(1).unwrap().string().as_bytes(), b"TEST(9,1.5);");
/// ```
pub struct StepDataStepWriter {
    pub(super) themodel: Handle<StepDataStepModel>,
    pub(super) thefile: Vec<Handle<HAsciiString>>,
    pub(super) thecurr: InterfaceLineBuffer,
    thesect: bool,
    pub(super) thecomm: bool,
    pub(super) thefirst: bool,
    pub(super) themult: bool,
    pub(super) thelevel: i32,
    pub(super) theindent: bool,
    pub(super) theindval: i32,
    thetypmode: i32,
    pub(super) thefloatw: InterfaceFloatWriter,
    pub(super) thechecks: InterfaceCheckIterator,
    pub(super) thenum: i32,
    pub(super) thelabmode: i32,
    scopes: Option<ScopeTables>,
}

/// The label of an entity for a label mode: (number, true ident, whether both are written).
pub(super) fn ident_label(num: i32, idtrue: i32, labmode: i32) -> (i32, bool) {
    let mut idnum = num;
    if labmode == 1 {
        idnum = idtrue;
    }
    if idnum == 0 {
        idnum = num;
    }
    (idnum, labmode >= 2 && idnum != idtrue)
}

impl StepDataStepWriter {
    /// `StepDataStepWriter(amodel)`.
    pub fn new(amodel: &Handle<StepDataStepModel>) -> Self {
        Self {
            themodel: amodel.clone(),
            thefile: Vec::new(),
            thecurr: InterfaceLineBuffer::new(STEP_LONG),
            thesect: false,
            thecomm: false,
            thefirst: true,
            themult: false,
            thelevel: 0,
            theindent: false,
            theindval: 0,
            thetypmode: 0,
            thefloatw: InterfaceFloatWriter::new(12),
            thechecks: InterfaceCheckIterator::new(),
            thenum: 0,
            thelabmode: 0,
            scopes: None,
        }
    }
    /// `label_mode`: 0 the entity number, 1 the recorded label (else its number).
    pub fn label_mode(&mut self) -> &mut i32 {
        &mut self.thelabmode
    }
    /// `type_mode`: 0 the long form of the types, 1 the short form when there is one.
    pub fn type_mode(&mut self) -> &mut i32 {
        &mut self.thetypmode
    }
    /// `float_writer`.
    pub fn float_writer(&mut self) -> &mut InterfaceFloatWriter {
        &mut self.thefloatw
    }
    fn model(&self) -> Result<&StepDataStepModel, Failure> {
        self.themodel.get().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::null_object(
                "StepDataStepWriter: null model; expected a STEP model",
            )
        })
    }
    /// `set_scope(numscope, numin)`: `numin` is in the scope of `numscope`.
    pub fn set_scope(&mut self, numscope: i32, numin: i32) -> Result<(), Failure> {
        let nb = self.model()?.nb_entities();
        if numscope <= 0 || numscope > nb || numin <= 0 || numin > nb {
            return Err(interface_mismatch(&format!(
                "StepWriter : SetScope, out of range: scope {numscope}, entity {numin}; expected [1, {nb}]"
            )));
        }
        let places = nb as usize + 1;
        let scopes = self.scopes.get_or_insert_with(|| ScopeTables {
            thescopebeg: vec![0; places],
            thescopeend: vec![0; places],
            thescopenext: vec![0; places],
        });
        let (numscope, numin) = (numscope as usize, numin as usize);
        if scopes.thescopenext[numin] != 0 {
            return Err(interface_mismatch(&format!(
                "StepWriter : SetScope, already set: entity {numin} is already in a scope"
            )));
        }
        scopes.thescopenext[numin] = -1;
        if scopes.thescopebeg[numscope] == 0 {
            scopes.thescopebeg[numscope] = numin as i32;
        }
        let lastin = scopes.thescopeend[numscope];
        if lastin > 0 {
            scopes.thescopenext[lastin as usize] = numin as i32;
        }
        scopes.thescopeend[numscope] = numin as i32;
        Ok(())
    }
    /// `is_in_scope(num)`.
    pub fn is_in_scope(&self, num: i32) -> bool {
        let Some(scopes) = &self.scopes else {
            return false;
        };
        usize::try_from(num)
            .ok()
            .and_then(|rank| scopes.thescopenext.get(rank))
            .is_some_and(|next| *next != 0)
    }
    /// `send_model(protocol, headeronly = false)`: the whole file, or the header section.
    pub fn send_model(
        &mut self,
        protocol: &Handle<dyn StepDataProtocolOps>,
        headeronly: bool,
    ) -> Result<(), Failure> {
        let lib = StepDataWriterLib::new(protocol);
        if !headeronly {
            self.append_line("ISO-10303-21;");
        }
        self.send_header()?;
        self.thenum = 0;
        for anent in self.model()?.header().content() {
            self.send_header_entity(&anent, &lib)?;
        }
        self.end_sec();
        if headeronly {
            return Ok(());
        }
        self.send_data()?;
        self.send_global_fails()?;
        for rank in 1..=self.model()?.nb_entities() {
            if !self.is_in_scope(rank) {
                self.send_entity_number(rank, &lib)?;
            }
        }
        self.end_sec();
        self.end_file()
    }
    fn send_header_entity(
        &mut self,
        anent: &Handle<StandardTransient>,
        lib: &StepDataWriterLib,
    ) -> Result<(), Failure> {
        if !self.write_by_module(anent, lib)? {
            let und = anent.cast::<StepDataUndefinedEntity>();
            if und.is_null() {
                return Ok(());
            }
            self.write_undefined(&und)?;
        }
        self.end_entity()
    }
    /// The entity by its module; false when the library does not know it.
    fn write_by_module(
        &mut self,
        anent: &Handle<StandardTransient>,
        lib: &StepDataWriterLib,
    ) -> Result<bool, Failure> {
        let Some((module, cn)) = lib.select(anent) else {
            return Ok(false);
        };
        let complex = module.is_complex(cn);
        if complex {
            self.start_complex()?;
        } else {
            let mut styp = AsciiString::default();
            if self.thetypmode > 0 {
                styp = module.short_type(cn);
            }
            if styp.is_empty() {
                styp = AsciiString::new(module.step_type(cn));
            }
            self.start_entity(&styp)?;
        }
        module.write_step(cn, self, anent)?;
        if complex {
            self.end_complex()?;
        }
        Ok(true)
    }
    fn write_undefined(&mut self, und: &Handle<StepDataUndefinedEntity>) -> Result<(), Failure> {
        let Some(live) = und.get() else {
            return Ok(());
        };
        let complex = live.is_complex();
        if complex {
            self.start_complex()?;
        }
        live.write_params(self)?;
        if complex {
            self.end_complex()?;
        }
        Ok(())
    }
    fn send_global_fails(&mut self) -> Result<(), Failure> {
        let check = self.model()?.global_check(true);
        let nbfails = check.nb_fails();
        if nbfails == 0 {
            return Ok(());
        }
        self.comment(true)?;
        self.send_comment(b"GLOBAL FAIL MESSAGES,  recorded at Read time :")?;
        for rank in 1..=nbfails {
            self.send_comment_h_string(&check.fail(rank, true)?)?;
        }
        self.comment(false)?;
        self.new_line(false)
    }
    pub(super) fn append_line(&mut self, text: &str) {
        self.thefile.push(Handle::new(HAsciiString::new(text)));
    }
    /// `send_header`.
    pub fn send_header(&mut self) -> Result<(), Failure> {
        self.new_line(false)?;
        self.append_line("HEADER;");
        self.thesect = true;
        Ok(())
    }
    /// `send_data`: an error if `end_sec` was not called.
    pub fn send_data(&mut self) -> Result<(), Failure> {
        if self.thesect {
            return Err(interface_mismatch(
                "StepWriter : Data section; expected EndSec before it",
            ));
        }
        self.new_line(false)?;
        self.append_line("DATA;");
        self.thesect = true;
        Ok(())
    }
    /// `end_sec`.
    pub fn end_sec(&mut self) {
        self.append_line("ENDSEC;");
        self.thesect = false;
    }
    /// `end_file`: an error if `end_sec` was not called.
    pub fn end_file(&mut self) -> Result<(), Failure> {
        if self.thesect {
            return Err(interface_mismatch(
                "StepWriter : EndFile; expected EndSec before it",
            ));
        }
        self.new_line(false)?;
        self.append_line("END-ISO-10303-21;");
        self.thesect = false;
        Ok(())
    }
    /// `send_entity(nument, lib)`: an entity of the data section, with its scope.
    pub fn send_entity_number(&mut self, num: i32, lib: &StepDataWriterLib) -> Result<(), Failure> {
        let anent = self.model()?.entity(num)?;
        let idtrue = if self.thelabmode > 0 {
            self.model()?.ident_label(&anent)
        } else {
            0
        };
        let (idnum, both) = ident_label(num, idtrue, self.thelabmode);
        let lident = if both {
            format!("{idnum}:#{idtrue} = ")
        } else {
            format!("#{idnum} = ")
        };
        self.thecurr.clear()?;
        self.thecurr.add_text(&lident)?;
        self.themult = false;
        self.send_scope_of(num, lib)?;
        self.thenum = num;
        if self.model()?.is_redefined_content(num) {
            return self.send_redefined(num);
        }
        if self.write_by_module(&anent, lib)? {
            return self.end_entity();
        }
        let und = anent.cast::<StepDataUndefinedEntity>();
        if und.is_null() {
            return Ok(());
        }
        self.write_undefined(&und)?;
        self.end_entity()
    }
    fn send_scope_of(&mut self, num: i32, lib: &StepDataWriterLib) -> Result<(), Failure> {
        let first = self
            .scopes
            .as_ref()
            .map_or(0, |scopes| scopes.thescopebeg[num as usize]);
        if first == 0 {
            return Ok(());
        }
        self.send_scope()?;
        let mut nument = first;
        while nument > 0 {
            self.send_entity_number(nument, lib)?;
            nument = self
                .scopes
                .as_ref()
                .map_or(0, |scopes| scopes.thescopenext[nument as usize]);
        }
        self.send_endscope()
    }
    /// An entity in error: its content, then its fails as comments.
    fn send_redefined(&mut self, num: i32) -> Result<(), Failure> {
        let rep = self.model()?.report_entity(num, false);
        let report = rep.get().ok_or_else(|| {
            interface_mismatch(&format!(
                "StepWriter : SendEntity {num}; expected the report of a redefined entity"
            ))
        })?;
        let und = report.content().cast::<StepDataUndefinedEntity>();
        let check = self.thechecks.c_check(num);
        match und.get() {
            None => {
                check.add_fail_text("Erroneous Entity, Content lost", "");
                self.start_entity(&AsciiString::new("!?LOST_DATA"))?;
            }
            Some(live) => {
                check.add_warning_text("Erroneous Entity, equivalent content", "");
                let complex = live.is_complex();
                if complex {
                    self.add_bytes(b" (", 0)?;
                }
                live.write_params(self)?;
                if complex {
                    self.add_bytes(b") ", 0)?;
                }
            }
        }
        self.end_entity()?;
        self.new_line(false)?;
        self.comment(true)?;
        if und.is_null() {
            self.send_comment(b"   ERRONEOUS ENTITY, DATA LOST")?;
        }
        self.send_comment(b"On Entity above, Fail Messages recorded at Read time :")?;
        let fails = report.check();
        for rank in 1..=fails.nb_fails() {
            self.send_comment_h_string(&fails.fail(rank, true)?)?;
        }
        self.comment(false)?;
        self.new_line(false)
    }
    /// `check_list`.
    pub fn check_list(&self) -> InterfaceCheckIterator {
        self.thechecks.clone()
    }
    /// `nb_lines`.
    pub fn nb_lines(&self) -> i32 {
        self.thefile.len() as i32
    }
    /// `Line(num)`: an out of range failure out of the lines.
    pub fn line(&self, num: i32) -> Result<Handle<HAsciiString>, Failure> {
        usize::try_from(num - 1)
            .ok()
            .and_then(|rank| self.thefile.get(rank).cloned())
            .ok_or_else(|| {
                martensite_tkernel::standard::exceptions::out_of_range(&format!(
                    "StepDataStepWriter::Line {num}; expected [1, {}]",
                    self.thefile.len()
                ))
            })
    }
    /// `print(S)`: the lines. The text stays in the writer.
    pub fn print(&mut self, s: &mut dyn Write) -> io::Result<bool> {
        for line in &self.thefile {
            let Some(text) = line.get() else {
                continue;
            };
            s.write_all(text.string().as_bytes())?;
            s.write_all(b"\n")?;
        }
        s.flush()?;
        Ok(true)
    }
}
