//! The sending of parameter values. Overloads of the `send` are named by what they
//! take: [`send_integer`](StepDataStepWriter::send_integer),
//! [`send_real`](StepDataStepWriter::send_real), [`send_text`](StepDataStepWriter::send_text)
//! and [`send_entity`](StepDataStepWriter::send_entity).
use super::writer::{STEP_LONG, ident_label};
use super::{
    StepDataESDescr, StepDataField, StepDataFieldKind as Kind, StepDataFieldList, StepDataLogical,
    StepDataPDescr, StepDataSelectArrReal, StepDataSelectMember, StepDataStepWriter,
};
use martensite_tkernel::{
    ncollection::HArray1,
    standard::{Failure, Handle, RuntimeTyped, StandardTransient},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkxsbase::interface::InterfaceModelEntities;

/// The length of the control directive at `index`, 0 when there is none.
fn directive_length(text: &[u8], index: usize) -> usize {
    let at = |offset: usize| text.get(index + offset).copied();
    if at(1) == Some(b'X') && at(3) == Some(b'\\') {
        return wide_directive_length(text, index);
    }
    if at(1) == Some(b'X') && at(4) == Some(b'\\') {
        let hexa = |c: Option<u8>| c.is_some_and(|c| c.is_ascii_hexdigit());
        return if hexa(at(2)) && hexa(at(3)) { 5 } else { 0 };
    }
    if at(2) == Some(b'\\') {
        return if matches!(at(1), Some(b'S' | b'N' | b'T')) {
            3
        } else {
            0
        };
    }
    if at(1) == Some(b'P') && at(3) == Some(b'\\') {
        return if at(2).is_some_and(|c| c.is_ascii_alphabetic()) {
            4
        } else {
            0
        };
    }
    0
}

/// `\X0\`, or `\X2\`, `\X4\` up to their closing `\X0\`.
fn wide_directive_length(text: &[u8], index: usize) -> usize {
    let kind = text[index + 2];
    if !matches!(kind, b'2' | b'4' | b'0') {
        return 0;
    }
    if kind == b'0' {
        return 4;
    }
    let search_from = index + 4;
    (search_from..text.len().saturating_sub(3))
        .find(|place| &text[*place..*place + 4] == b"\\X0\\")
        .map_or(4, |place| place + 4 - index)
}

/// The last place in the first `STEP_LONG` bytes holding `mark` (1-based), 0 when none.
fn last_mark(text: &[u8], mark: u8) -> usize {
    let window = text.len().min(STEP_LONG as usize);
    text[..window]
        .iter()
        .rposition(|byte| *byte == mark)
        .map_or(0, |rank| rank + 1)
}

impl StepDataStepWriter {
    /// `clean_text_for_send(text)`: doubles quotes and backslashes, turns newlines and tabs to
    /// directives, and keeps the control directives already there.
    pub fn clean_text_for_send(text: &AsciiString) -> AsciiString {
        let bytes = text.as_bytes();
        let mut result = Vec::with_capacity(bytes.len());
        let mut index = 0;
        while index < bytes.len() {
            let byte = bytes[index];
            let directive = if byte == b'\\' {
                directive_length(bytes, index)
            } else {
                0
            };
            if directive > 0 {
                result.extend_from_slice(&bytes[index..index + directive]);
                index += directive;
                continue;
            }
            match byte {
                b'\'' => result.extend_from_slice(b"''"),
                b'\\' => result.extend_from_slice(b"\\\\"),
                b'\n' => result.extend_from_slice(b"\\N\\"),
                b'\t' => result.extend_from_slice(b"\\T\\"),
                _ => result.push(byte),
            }
            index += 1;
        }
        AsciiString::from_bytes(&result)
    }
    /// `send_field(fild, descr)`.
    pub fn send_field(
        &mut self,
        fild: &StepDataField,
        descr: &Handle<StepDataPDescr>,
    ) -> Result<(), Failure> {
        let kind = fild.kind(false);
        if kind == Kind::SELECT {
            return self.send_select(&fild.transient().cast(), descr);
        }
        match kind {
            Kind::UNDEFINED => return self.send_undef(),
            Kind::INTEGER => return self.send_integer(fild.integer(1, 1)?),
            Kind::BOOLEAN => return self.send_boolean(fild.boolean(1, 1)?),
            Kind::LOGICAL => return self.send_logical(fild.logical(1, 1)?),
            Kind::ENUM => return self.send_enum(fild.enum_text(1, 1)?.as_bytes()),
            Kind::REAL => return self.send_real(fild.real(1, 1)?),
            Kind::STRING => return self.send_text(&fild.string(1, 1)?),
            Kind::ENTITY => return self.send_entity(&fild.entity(1, 1)?),
            Kind::DERIVED => return self.send_derived(),
            _ => {}
        }
        match fild.arity() {
            1 => self.send_field_list(fild),
            2 => self.send_field_grid(fild),
            _ => self.send_undef(),
        }
    }
    fn send_field_list(&mut self, fild: &StepDataField) -> Result<(), Failure> {
        self.open_sub()?;
        let lower = fild.lower(1);
        for index in lower..lower + fild.length(1) {
            self.send_field_item(fild, index, 1)?;
        }
        self.close_sub()
    }
    fn send_field_grid(&mut self, fild: &StepDataField) -> Result<(), Failure> {
        self.open_sub()?;
        let (row0, column0) = (fild.lower(1), fild.lower(2));
        for row in row0..row0 + fild.length(1) {
            self.open_sub()?;
            for column in column0..column0 + fild.length(2) {
                self.send_field_item(fild, row, column)?;
            }
            self.close_sub()?;
        }
        self.close_sub()
    }
    fn send_field_item(&mut self, fild: &StepDataField, n1: i32, n2: i32) -> Result<(), Failure> {
        match fild.item_kind(n1, n2)? {
            Kind::INTEGER => self.send_integer(fild.integer(n1, n2)?),
            Kind::BOOLEAN => self.send_boolean(fild.boolean(n1, n2)?),
            Kind::LOGICAL => self.send_logical(fild.logical(n1, n2)?),
            Kind::ENUM => self.send_enum(fild.enum_text(n1, n2)?.as_bytes()),
            Kind::REAL => self.send_real(fild.real(n1, n2)?),
            Kind::STRING => self.send_text(&fild.string(n1, n2)?),
            Kind::ENTITY => self.send_entity(&fild.entity(n1, n2)?),
            _ => self.send_undef(),
        }
    }
    /// `send_select(sm, descr)`: its value, inside its name when it has one.
    pub fn send_select(
        &mut self,
        sm: &Handle<dyn StepDataSelectMember>,
        _descr: &Handle<StepDataPDescr>,
    ) -> Result<(), Failure> {
        let Some(member) = sm.get() else {
            return self.send_undef();
        };
        let named = member.has_name();
        if named {
            self.open_typed_sub(member.name().as_bytes())?;
        }
        match Kind::from_int(member.kind()) {
            _ if member.kind() > 0xFF || member.kind() < 0 => self.send_undef()?,
            Kind::INTEGER => self.send_integer(member.integer())?,
            Kind::BOOLEAN => self.send_boolean(member.boolean())?,
            Kind::LOGICAL => self.send_logical(member.logical())?,
            Kind::ENUM => self.send_enum(member.enum_text().as_bytes())?,
            Kind::REAL => self.send_real(member.real())?,
            Kind::STRING => self.send_text(&member.string())?,
            Kind::ANY => {
                let array = sm.cast::<StepDataSelectArrReal>();
                match array.get() {
                    Some(live) => self.send_arr_real(&live.arr_real())?,
                    None => self.send_undef()?,
                }
            }
            _ => self.send_undef()?,
        }
        if named {
            self.close_sub()?;
        }
        Ok(())
    }
    /// `send_list(list, descr)`: its fields, without the brackets.
    pub fn send_list(
        &mut self,
        list: &dyn StepDataFieldList,
        descr: &Handle<StepDataESDescr>,
    ) -> Result<(), Failure> {
        for rank in 1..=list.nb_fields() {
            let pde = descr
                .get()
                .map_or_else(Handle::default, |live| live.field(rank));
            self.send_field(list.field(rank)?, &pde)?;
        }
        Ok(())
    }
    /// `open_sub`.
    pub fn open_sub(&mut self) -> Result<(), Failure> {
        self.add_param()?;
        self.add_bytes(b"(", 0)?;
        self.thefirst = true;
        self.thelevel += 1;
        Ok(())
    }
    /// `open_typed_sub(subtype)`.
    pub fn open_typed_sub(&mut self, subtype: &[u8]) -> Result<(), Failure> {
        self.add_param()?;
        if !subtype.is_empty() {
            self.add_bytes(subtype, 0)?;
        }
        self.add_bytes(b"(", 0)?;
        self.thefirst = true;
        self.thelevel += 1;
        Ok(())
    }
    /// `close_sub`.
    pub fn close_sub(&mut self) -> Result<(), Failure> {
        self.add_bytes(b")", 0)?;
        self.thefirst = false;
        self.thelevel -= 1;
        Ok(())
    }
    /// `add_param`: the comma, except for the first parameter.
    pub fn add_param(&mut self) -> Result<(), Failure> {
        if !self.thefirst {
            self.add_bytes(b",", 0)?;
        }
        self.thefirst = false;
        Ok(())
    }
    /// `send(int)`.
    pub fn send_integer(&mut self, val: i32) -> Result<(), Failure> {
        self.add_param()?;
        self.add_bytes(val.to_string().as_bytes(), 0)
    }
    /// `send(double)`: by the float writer.
    pub fn send_real(&mut self, val: f64) -> Result<(), Failure> {
        let mut text = Vec::with_capacity(24);
        self.thefloatw.write(val, &mut text)?;
        self.add_param()?;
        self.add_bytes(&text, 0)
    }
    /// `send(AsciiString)`: a text between quotes, wrapped at blanks, backslashes
    /// or underscores when above a line.
    pub fn send_text(&mut self, val: &AsciiString) -> Result<(), Failure> {
        self.add_param()?;
        let cleaned = Self::clean_text_for_send(val);
        let mut quoted = Vec::with_capacity(cleaned.as_bytes().len() + 2);
        quoted.push(b'\'');
        quoted.extend_from_slice(cleaned.as_bytes());
        quoted.push(b'\'');
        let length = quoted.len() as i32;
        if self.thecurr.can_get(length) {
            return self.add_bytes(&quoted, 0);
        }
        let line = self.thecurr.moved()?;
        self.thefile.push(line);
        let indst = self.indentation();
        self.thecurr.set_initial(if indst + length <= STEP_LONG {
            indst
        } else {
            0
        })?;
        if self.thecurr.can_get(length) {
            return self.add_bytes(&quoted, 0);
        }
        self.send_wrapped_text(quoted)
    }
    fn send_wrapped_text(&mut self, mut text: Vec<u8>) -> Result<(), Failure> {
        while !text.is_empty() {
            if text.len() as i32 <= STEP_LONG {
                let length = text.len() as i32;
                self.thecurr.add_bytes(&text, length)?;
                self.thecurr.freeze_initial();
                return Ok(());
            }
            let mut stop = last_mark(&text, b' ');
            if stop == 0 {
                stop = last_mark(&text, b'\\');
            }
            if stop == 0 {
                stop = last_mark(&text, b'_');
            }
            if stop == 0 {
                stop = STEP_LONG as usize;
            }
            let rest = text.split_off(stop);
            self.thefile.push(Handle::new(HAsciiString::new(&text)));
            text = rest;
        }
        Ok(())
    }
    /// `Send(Handle<`StandardTransient`>)`: a reference to an entity, by its ident. A
    /// null value is undefined with a fail; a string or member outside the model is sent
    /// as its value.
    pub fn send_entity<T: ?Sized + RuntimeTyped>(
        &mut self,
        val: &Handle<T>,
    ) -> Result<(), Failure> {
        let val = val.cast::<StandardTransient>();
        if val.is_null() {
            return self.send_bad_reference("Null Reference", b" NUL REF ");
        }
        let model = self.themodel.clone();
        let num = model.get().map_or(0, |live| live.number(&val));
        if num != 0 {
            return self.send_reference(&val, num);
        }
        if let Some(text) = val.cast::<HAsciiString>().get() {
            let content = text.string().clone();
            return self.send_text(&content);
        }
        let member = val.cast::<dyn StepDataSelectMember>();
        if member.is_some() {
            return self.send_select(&member, &Handle::default());
        }
        self.send_bad_reference("UnknownReference", b" UNKNOWN REF ")
    }
    fn send_bad_reference(&mut self, fail: &str, comment: &[u8]) -> Result<(), Failure> {
        self.thechecks.c_check(self.thenum).add_fail_text(fail, "");
        self.send_undef()?;
        self.comment(true)?;
        self.send_comment(comment)?;
        self.comment(false)
    }
    fn send_reference(&mut self, val: &Handle<StandardTransient>, num: i32) -> Result<(), Failure> {
        let idtrue = if self.thelabmode > 0 {
            self.themodel.get().map_or(0, |live| live.ident_label(val))
        } else {
            0
        };
        let (idnum, both) = ident_label(num, idtrue, self.thelabmode);
        let lident = if both {
            format!("{idnum}:#{idtrue}")
        } else {
            format!("#{idnum}")
        };
        self.add_param()?;
        self.add_bytes(lident.as_bytes(), 0)
    }
    /// `send_boolean(val)`: `.T.` or `.F.`.
    pub fn send_boolean(&mut self, val: bool) -> Result<(), Failure> {
        self.send_string(if val { b".T." } else { b".F." })
    }
    /// `send_logical(val)`: `.T.`, `.F.` or `.U.`.
    pub fn send_logical(&mut self, val: StepDataLogical) -> Result<(), Failure> {
        self.send_string(match val {
            StepDataLogical::LTrue => b".T.",
            StepDataLogical::LFalse => b".F.",
            StepDataLogical::LUnknown => b".U.",
        })
    }
    /// `send_string(val)`: exactly as given.
    pub fn send_string(&mut self, val: &[u8]) -> Result<(), Failure> {
        self.add_param()?;
        self.add_bytes(val, 0)
    }
    /// `send_enum(val)`: adds the dots when absent; empty or `$` is undefined.
    pub fn send_enum(&mut self, val: &[u8]) -> Result<(), Failure> {
        if val.is_empty() || val == b"$" {
            return self.send_undef();
        }
        self.add_param()?;
        let mut value = Vec::with_capacity(val.len() + 2);
        if val[0] != b'.' {
            value.push(b'.');
        }
        value.extend_from_slice(val);
        if value.last() != Some(&b'.') {
            value.push(b'.');
        }
        self.add_bytes(&value, 2)
    }
    /// `send_arr_real(anArr)`.
    pub fn send_arr_real(&mut self, array: &Handle<HArray1<f64>>) -> Result<(), Failure> {
        let Some(live) = array.get() else {
            return self.send_undef();
        };
        self.open_sub()?;
        let values = live.array1().as_slice().to_vec();
        for value in values {
            self.send_real(value)?;
        }
        self.close_sub()
    }
    /// `send_undef`: `$`.
    pub fn send_undef(&mut self) -> Result<(), Failure> {
        self.add_param()?;
        self.add_bytes(b"$", 0)
    }
    /// `send_derived`: `*`.
    pub fn send_derived(&mut self) -> Result<(), Failure> {
        self.add_param()?;
        self.add_bytes(b"*", 0)
    }
}
