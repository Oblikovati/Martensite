//! Reading the parameters of one entity: the state, the plain accesses and the messages.
//!
//! A read takes a cursor, feeds the check on what is wrong, and returns whether it gave a value.
//! The reads themselves are in `param_reader_read` and `param_reader_lists`.
use super::{IgesDataIgesEntity, IgesDataIgesReaderData, IgesDataParamCursor, IgesDataReadStage};
use martensite_tkernel::{
    message::Msg,
    standard::{Failure, Handle},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceFileParameter, InterfaceFileReaderDataParams, InterfaceParamList,
    InterfaceParamType,
};

/// What a read reports on a wrong parameter: a text it completes, or a catalog message.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataReadMessage;
/// assert!(matches!(IgesDataReadMessage::from("Radius"), IgesDataReadMessage::Text("Radius")));
/// ```
#[derive(Clone, Copy, Debug)]
pub enum IgesDataReadMessage<'a> {
    /// The label of the field; the read adds what is wrong with it.
    Text(&'a str),
    /// The message to send as it is.
    Msg(&'a Msg),
}

impl<'a> From<&'a str> for IgesDataReadMessage<'a> {
    fn from(text: &'a str) -> Self {
        Self::Text(text)
    }
}

impl<'a> From<&'a Msg> for IgesDataReadMessage<'a> {
    fn from(msg: &'a Msg) -> Self {
        Self::Msg(msg)
    }
}

impl<'a> From<&'a mut Msg> for IgesDataReadMessage<'a> {
    fn from(msg: &'a mut Msg) -> Self {
        Self::Msg(msg)
    }
}

/// The message of the exchange catalog under a key.
///
/// ```
/// use martensite_tkdeiges::iges_data::iges_msg;
/// assert!(iges_msg("XSTEP_27").value().length() > 0);
/// ```
pub fn iges_msg(key: &str) -> Msg {
    Msg::from_key(&AsciiString::new(key))
}

/// The reader of the parameters of one entity's record.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataParamReader;
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkxsbase::interface::{InterfaceCheck, InterfaceFileParameter, InterfaceParamList, InterfaceParamType};
/// let list = Handle::new(InterfaceParamList::new(4));
/// let mut type_number = InterfaceFileParameter::new();
/// type_number.init_text("110", InterfaceParamType::ParamInteger);
/// list.get().unwrap().set_value(1, &type_number).unwrap();
/// let check = Handle::new(InterfaceCheck::new());
/// let reader = IgesDataParamReader::new(&list, &check, 1, 1, 0);
/// assert_eq!(reader.nb_params(), 0);
/// ```
pub struct IgesDataParamReader {
    pub(super) theparams: Handle<InterfaceParamList>,
    pub(super) thecheck: Handle<InterfaceCheck>,
    pub(super) thebase: i32,
    pub(super) thenbpar: i32,
    pub(super) thecurr: i32,
    pub(super) thestage: IgesDataReadStage,
    pub(super) thelast: bool,
    pub(super) theindex: i32,
    pub(super) thenbitem: i32,
    pub(super) theitemsz: i32,
    pub(super) theoffset: i32,
    pub(super) thetermsz: i32,
    pub(super) themaxind: i32,
    pub(super) thenbterm: i32,
    thenum: i32,
}

impl IgesDataParamReader {
    /// A reader over `list`, from the rank `base`; `nbpar` counts the parameters with the type
    /// number that starts the record, 0 for the whole list. The entity number is not kept.
    pub fn new(
        list: &Handle<InterfaceParamList>,
        ach: &Handle<InterfaceCheck>,
        base: i32,
        nbpar: i32,
        _num: i32,
    ) -> Self {
        let length = list.get().map_or(0, InterfaceParamList::length);
        Self {
            theparams: list.clone(),
            thecheck: ach.clone(),
            thebase: base,
            thenbpar: if nbpar > 0 { nbpar } else { length },
            thecurr: 1,
            thestage: IgesDataReadStage::ReadOwn,
            thelast: true,
            theindex: 0,
            thenbitem: 0,
            theitemsz: 0,
            theoffset: 0,
            thetermsz: 0,
            themaxind: 0,
            thenbterm: 0,
            thenum: 0,
        }
    }
    /// The entity number; it stays 0.
    pub fn entity_number(&self) -> i32 {
        self.thenum
    }
    /// Comes back to the first parameter and the first stage.
    pub fn clear(&mut self) {
        self.thecurr = 1;
        self.thestage = IgesDataReadStage::ReadOwn;
    }
    /// The current parameter number.
    pub fn current_number(&self) -> i32 {
        self.thecurr
    }
    /// Sets the current parameter number.
    pub fn set_current_number(&mut self, num: i32) {
        self.thecurr = num;
    }
    /// The current stage.
    pub fn stage(&self) -> IgesDataReadStage {
        self.thestage
    }
    /// Goes to the next stage.
    pub fn next_stage(&mut self) {
        self.thestage = self.thestage.next();
    }
    /// Goes to the last stage.
    pub fn end_all(&mut self) {
        self.thestage = IgesDataReadStage::ReadEnd;
    }
    /// The count of parameters, without the type number.
    pub fn nb_params(&self) -> i32 {
        self.thenbpar - 1
    }
    pub(super) fn param_at(&self, num: i32) -> Result<InterfaceFileParameter, Failure> {
        super::iges_live(&self.theparams, "IGES param reader")?.value(num + self.thebase)
    }
    /// The type of a parameter.
    ///
    /// # Errors
    /// An out of range failure for a bad number.
    pub fn param_type(&self, num: i32) -> Result<InterfaceParamType, Failure> {
        Ok(self.param_at(num)?.param_type())
    }
    /// The text of a parameter.
    ///
    /// # Errors
    /// An out of range failure for a bad number.
    pub fn param_value(&self, num: i32) -> Result<AsciiString, Failure> {
        Ok(AsciiString::from_bytes(self.param_at(num)?.c_value()))
    }
    /// Whether a parameter is there and not void.
    ///
    /// # Errors
    /// An out of range failure for a bad number.
    pub fn is_param_defined(&self, num: i32) -> Result<bool, Failure> {
        if num >= self.thenbpar {
            return Ok(false);
        }
        Ok(self.param_type(num)? != InterfaceParamType::ParamVoid)
    }
    /// Whether a parameter designates an entity.
    ///
    /// # Errors
    /// An out of range failure for a bad number.
    pub fn is_param_entity(&self, num: i32) -> Result<bool, Failure> {
        Ok(self.param_number(num)? != 0)
    }
    /// The entity number a parameter designates, 0 when none.
    ///
    /// # Errors
    /// An out of range failure for a bad number.
    pub fn param_number(&self, num: i32) -> Result<i32, Failure> {
        Ok(self.param_at(num)?.entity_number())
    }
    /// The entity a parameter designates; a parameter that designates none adds a failure.
    ///
    /// # Errors
    /// An out of range failure for a bad number.
    pub fn param_entity(
        &mut self,
        ir: &Handle<IgesDataIgesReaderData>,
        num: i32,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        let n = self.param_number(num)?;
        if n == 0 {
            self.check_text(
                true,
                "IGES parameter reader: the parameter does not designate an entity",
                "",
            );
        }
        Ok(bound_entity(ir, n))
    }
    /// The cursor of the current parameter.
    pub fn current(&self) -> IgesDataParamCursor {
        IgesDataParamCursor::new(self.thecurr)
    }
    /// The cursor of `nb` items of `size` parameters from the current one.
    pub fn current_list(&self, nb: i32, size: i32) -> IgesDataParamCursor {
        IgesDataParamCursor::with_items(self.thecurr, nb, size)
    }
    /// Whether the current parameter is defined; a void one is skipped.
    ///
    /// # Errors
    /// An out of range failure for a bad number.
    pub fn defined_else_skip(&mut self) -> Result<bool, Failure> {
        if self.thecurr > self.nb_params() {
            return Ok(false);
        }
        if self.is_param_defined(self.thecurr)? {
            return Ok(true);
        }
        self.thecurr += 1;
        Ok(false)
    }
    /// Sends a failure message.
    pub fn send_fail(&mut self, amsg: &Msg) {
        if let Some(check) = self.thecheck.get() {
            check.send_fail(amsg);
        }
        self.thelast = false;
    }
    /// Sends a warning message.
    pub fn send_warning(&mut self, amsg: &Msg) {
        if let Some(check) = self.thecheck.get() {
            check.send_warning(amsg);
        }
        self.thelast = false;
    }
    /// Adds a failure text, with its original form when it is not empty.
    pub fn add_fail(&mut self, afail: &str, bfail: &str) {
        self.thelast = false;
        self.check_text(true, afail, bfail);
    }
    /// Adds a failure, from shared strings.
    pub fn add_fail_h(&mut self, afail: &Handle<HAsciiString>, bfail: &Handle<HAsciiString>) {
        self.thelast = false;
        if let Some(check) = self.thecheck.get() {
            check.add_fail_with_original(afail, bfail);
        }
    }
    /// Adds a warning text, with its original form when it is not empty.
    pub fn add_warning(&mut self, awarn: &str, bwarn: &str) {
        self.check_text(false, awarn, bwarn);
    }
    /// Adds a warning, from shared strings.
    pub fn add_warning_h(&mut self, awarn: &Handle<HAsciiString>, bwarn: &Handle<HAsciiString>) {
        if let Some(check) = self.thecheck.get() {
            check.add_warning_with_original(awarn, bwarn);
        }
    }
    /// Turns the failures into warnings with a prefix.
    ///
    /// # Errors
    /// A failure of the check.
    pub fn mend(&mut self, pref: &str) -> Result<(), Failure> {
        if let Some(check) = self.thecheck.get() {
            check.mend(Some(pref), 0)?;
        }
        self.thelast = true;
        Ok(())
    }
    /// Whether the last read has failed.
    pub fn has_failed(&self) -> bool {
        !self.thelast
    }
    /// The check.
    pub fn check(&self) -> &Handle<InterfaceCheck> {
        &self.thecheck
    }
    /// The check, to feed it or replace it.
    pub fn c_check(&mut self) -> &mut Handle<InterfaceCheck> {
        &mut self.thecheck
    }
    /// Whether the check holds nothing.
    pub fn is_check_empty(&self) -> bool {
        self.thecheck
            .get()
            .is_none_or(|check| !check.has_failed() && !check.has_warnings())
    }
    fn check_text(&self, fail: bool, text: &str, orig: &str) {
        let Some(check) = self.thecheck.get() else {
            return;
        };
        if fail {
            check.add_fail_text(text, orig);
        } else {
            check.add_warning_text(text, orig);
        }
    }
    /// Reports a wrong parameter by a failure: the message as it is, or the label and what is
    /// wrong. `orig` is the text before its value was put in, empty when it is `what` itself.
    pub(super) fn report_fail(&mut self, mess: IgesDataReadMessage<'_>, what: &str, orig: &str) {
        match mess {
            IgesDataReadMessage::Msg(msg) => self.send_fail(msg),
            IgesDataReadMessage::Text(label) => self.labeled(true, label, what, orig),
        }
    }
    /// Reports a wrong parameter by a warning, as [`Self::report_fail`].
    pub(super) fn report_warning(&mut self, mess: IgesDataReadMessage<'_>, what: &str, orig: &str) {
        match mess {
            IgesDataReadMessage::Msg(msg) => self.send_warning(msg),
            IgesDataReadMessage::Text(label) => self.labeled(false, label, what, orig),
        }
    }
    /// A label followed by what is wrong; a failure marks the last read as failed.
    pub(super) fn labeled(&mut self, fail: bool, label: &str, what: &str, orig: &str) {
        let text = format!("{label}{what}");
        let original = if orig.is_empty() {
            String::new()
        } else {
            format!("{label}{orig}")
        };
        self.check_text(fail, &text, &original);
        if fail {
            self.thelast = false;
        }
    }
}

/// The entity of a directory number of the reader data, null when none or not an entity.
pub(super) fn bound_entity(
    ir: &Handle<IgesDataIgesReaderData>,
    num: i32,
) -> Handle<IgesDataIgesEntity> {
    ir.get()
        .map_or_else(Handle::default, |data| data.bound_entity(num).cast())
}

/// The integer a C text starts with: blanks, a sign and digits; 0 when there are none.
///
/// ```
/// use martensite_tkdeiges::iges_data::iges_atoi;
/// assert_eq!(iges_atoi(b" -12H"), -12);
/// ```
pub fn iges_atoi(text: &[u8]) -> i32 {
    let start = text
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(text.len());
    let rest = &text[start..];
    let (negative, digits) = match rest.first() {
        Some(b'-') => (true, &rest[1..]),
        Some(b'+') => (false, &rest[1..]),
        _ => (false, rest),
    };
    let mut magnitude: i64 = 0;
    for digit in digits.iter().take_while(|byte| byte.is_ascii_digit()) {
        magnitude = (magnitude * 10 + i64::from(digit - b'0')).min(i64::from(u32::MAX));
    }
    // Wrapping keeps the value a C atoi gives on the usual platforms for out of range text.
    let signed = if negative { -magnitude } else { magnitude };
    signed as i32
}
