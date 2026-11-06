//! The reads of single values by the parameter reader.
use super::param_reader::iges_atoi;
use super::{IgesDataParamCursor, IgesDataParamReader, IgesDataReadMessage};
use martensite_tkernel::{
    standard::{Failure, Handle, c_string::atof_bytes},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkmath::gp::{Xy, Xyz};
use martensite_tkxsbase::interface::{InterfaceFileParameter, InterfaceParamType};

/// The text of a real parameter, with a `D` exponent written `e`.
fn real_text(parameter: &InterfaceFileParameter) -> Vec<u8> {
    parameter
        .c_value()
        .iter()
        .take(50)
        .map(|byte| {
            if matches!(*byte, b'D' | b'd') {
                b'e'
            } else {
                *byte
            }
        })
        .collect()
}

impl IgesDataParamReader {
    /// Prepares a read and sets the fields the list reads go on with; false when it cannot be done.
    /// Reports only when `mess` is a text: a catalog message is left to the read.
    pub(super) fn prepare_read(
        &mut self,
        pc: &IgesDataParamCursor,
        mess: Option<&str>,
        several: bool,
        size: i32,
    ) -> bool {
        self.theindex = pc.start();
        self.themaxind = pc.limit();
        self.thenbitem = pc.count();
        self.theitemsz = pc.item_size();
        self.theoffset = pc.offset();
        self.thetermsz = pc.term_size();
        if !several && self.thenbitem > 1 {
            self.report_prepare(mess, true, " : List not allowed");
            return false;
        }
        if size > 1 && self.thetermsz % size != 0 {
            self.report_prepare(mess, true, " : term size mismatch");
            return false;
        }
        if self.theindex <= 0 || (self.themaxind - 1) > self.nb_params() {
            if self.thenbitem == 1 {
                self.report_prepare(mess, true, " : Parameter number out of range");
            } else {
                self.report_prepare(mess, false, " : too many values to read");
            }
            return false;
        }
        if pc.advance() {
            self.set_current_number(self.themaxind);
        }
        self.thelast = true;
        true
    }
    fn report_prepare(&mut self, mess: Option<&str>, fail: bool, what: &str) {
        if let Some(label) = mess {
            self.labeled(fail, label, what, "");
        }
    }
    /// The first parameter of a list read.
    pub(super) fn first_read(&mut self) -> i32 {
        self.theindex += self.theoffset;
        let res = self.theindex;
        self.thenbterm = 1;
        if self.thenbterm >= self.thetermsz {
            self.theindex += self.theitemsz;
            self.thenbterm = 0;
        }
        res
    }
    /// The next parameter of a list read, 0 at its end.
    pub(super) fn next_read(&mut self) -> i32 {
        let res = if self.theindex >= self.themaxind {
            0
        } else {
            self.theindex
        };
        self.thenbterm += 1;
        if self.thenbterm >= self.thetermsz {
            self.theindex += self.theitemsz;
            self.thenbterm = 0;
        }
        res
    }
    /// Reads an integer; a void parameter gives 0. Nothing is reported.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_integer(
        &mut self,
        pc: &IgesDataParamCursor,
        val: &mut i32,
    ) -> Result<bool, Failure> {
        if !self.prepare_read(pc, None, false, 1) {
            return Ok(false);
        }
        self.read_integer_at(val).map(|outcome| outcome.is_some())
    }
    /// Reads an integer; a void parameter gives 0. What is wrong is reported under `mess`.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_integer_text(
        &mut self,
        pc: &IgesDataParamCursor,
        mess: &str,
        val: &mut i32,
    ) -> Result<bool, Failure> {
        if !self.prepare_read(pc, Some(mess), false, 1) {
            return Ok(false);
        }
        let read = self.read_integer_at(val)?.is_some();
        if !read {
            self.labeled(true, mess, " : not given as an Integer", "");
        }
        Ok(read)
    }
    fn read_integer_at(&mut self, val: &mut i32) -> Result<Option<()>, Failure> {
        let parameter = self.param_at(self.theindex)?;
        match parameter.param_type() {
            InterfaceParamType::ParamInteger => *val = iges_atoi(parameter.c_value()),
            InterfaceParamType::ParamVoid => *val = 0,
            _ => return Ok(None),
        }
        Ok(Some(()))
    }
    /// Reads a boolean, given as 0 or 1; a void parameter gives false. Another integer is a
    /// failure when `exact`, else a warning and the value is whether it is positive.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_boolean<'a>(
        &mut self,
        pc: &IgesDataParamCursor,
        mess: impl Into<IgesDataReadMessage<'a>>,
        val: &mut bool,
        exact: bool,
    ) -> Result<bool, Failure> {
        let mess = mess.into();
        if !self.prepare_read(pc, label_of(mess), false, 1) {
            return Ok(false);
        }
        let parameter = self.param_at(self.theindex)?;
        if parameter.param_type() != InterfaceParamType::ParamInteger {
            if parameter.param_type() == InterfaceParamType::ParamVoid {
                *val = false;
                return Ok(true);
            }
            self.report_fail(mess, " : not an Integer (for Boolean)", "");
            return Ok(false);
        }
        let flag = iges_atoi(parameter.c_value());
        if flag != 0 && flag != 1 {
            let what = format!(
                " : Value is not 0/1, but {}",
                String::from_utf8_lossy(parameter.c_value())
            );
            if exact {
                self.report_fail(mess, &what, " : Value is not 0/1, but %s");
                self.thelast = true;
                return Ok(false);
            }
            self.report_warning(mess, &what, " : Value is not 0/1, but %s");
        }
        *val = flag > 0;
        Ok(true)
    }
    /// Reads a real; a void parameter gives 0. Nothing is reported.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_real(&mut self, pc: &IgesDataParamCursor, val: &mut f64) -> Result<bool, Failure> {
        if !self.prepare_read(pc, None, false, 1) {
            return Ok(false);
        }
        self.reading_real(self.theindex, val)
    }
    /// Reads a real; a void parameter gives 0. What is wrong is reported under `mess`.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_real_text(
        &mut self,
        pc: &IgesDataParamCursor,
        mess: &str,
        val: &mut f64,
    ) -> Result<bool, Failure> {
        if !self.prepare_read(pc, Some(mess), false, 1) {
            return Ok(false);
        }
        self.reading_real_text(self.theindex, mess, val)
    }
    /// Reads two coordinates into `val`, which stays as it is unless both are there. A catalog
    /// message is not used.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_xy<'a>(
        &mut self,
        pc: &IgesDataParamCursor,
        mess: impl Into<IgesDataReadMessage<'a>>,
        val: &mut Xy,
    ) -> Result<bool, Failure> {
        let label = label_of(mess.into());
        if !self.prepare_read(pc, label, false, 2) {
            return Ok(false);
        }
        let (mut x, mut y) = (0.0, 0.0);
        let stat = self.reading_labeled(self.theindex, label, &mut x)?
            && self.reading_labeled(self.theindex + 1, label, &mut y)?;
        if stat {
            *val = Xy::new(x, y);
        }
        Ok(stat)
    }
    /// Reads three coordinates into `val`, which stays as it is unless all are there. It returns
    /// true even then, as long as the read could start. A catalog message is not used.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_xyz<'a>(
        &mut self,
        pc: &IgesDataParamCursor,
        mess: impl Into<IgesDataReadMessage<'a>>,
        val: &mut Xyz,
    ) -> Result<bool, Failure> {
        let label = label_of(mess.into());
        if !self.prepare_read(pc, label, false, 3) {
            return Ok(false);
        }
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        let stat = self.reading_labeled(self.theindex, label, &mut x)?
            && self.reading_labeled(self.theindex + 1, label, &mut y)?
            && self.reading_labeled(self.theindex + 2, label, &mut z)?;
        if stat {
            *val = Xyz::new(x, y, z);
        }
        Ok(true)
    }
    fn reading_labeled(
        &mut self,
        num: i32,
        label: Option<&str>,
        val: &mut f64,
    ) -> Result<bool, Failure> {
        match label {
            Some(mess) => self.reading_real_text(num, mess, val),
            None => self.reading_real(num, val),
        }
    }
    /// Reads a text given in its counted form `nHtext`; a void parameter gives an empty text.
    ///
    /// # Errors
    /// An out of range failure when the parameter list is short.
    pub fn read_text<'a>(
        &mut self,
        pc: &IgesDataParamCursor,
        mess: impl Into<IgesDataReadMessage<'a>>,
        val: &mut Handle<HAsciiString>,
    ) -> Result<bool, Failure> {
        let mess = mess.into();
        if !self.prepare_read(pc, label_of(mess), false, 1) {
            return Ok(false);
        }
        let parameter = self.param_at(self.theindex)?;
        match mess {
            IgesDataReadMessage::Msg(_) => self.read_text_catalog(&parameter, mess, val),
            IgesDataReadMessage::Text(label) => self.read_text_labeled(&parameter, label, val),
        }
    }
    fn read_text_catalog(
        &mut self,
        parameter: &InterfaceFileParameter,
        mess: IgesDataReadMessage<'_>,
        val: &mut Handle<HAsciiString>,
    ) -> Result<bool, Failure> {
        if parameter.param_type() != InterfaceParamType::ParamText {
            *val = Handle::new(HAsciiString::new(""));
            if parameter.param_type() == InterfaceParamType::ParamVoid {
                return Ok(true);
            }
            self.report_fail(mess, "", "");
            return Ok(false);
        }
        let text = parameter.c_value();
        let length = text.len() as i32;
        let symbol = text
            .iter()
            .position(|byte| *byte == b'H')
            .map_or(0, |at| at as i32 + 1);
        if symbol <= 1 || symbol > length {
            *val = Handle::new(HAsciiString::new(""));
            self.report_fail(mess, "", "");
            return Ok(false);
        }
        let count = AsciiString::from_bytes(&text[..symbol as usize - 1]);
        if !count.is_integer_value() {
            *val = Handle::new(HAsciiString::new(""));
            self.report_fail(mess, "", "");
            return Ok(false);
        }
        if count.integer_value()? != length - symbol {
            self.report_warning(mess, "", "");
        }
        *val = Handle::new(HAsciiString::new(&text[symbol as usize..]));
        Ok(true)
    }
    fn read_text_labeled(
        &mut self,
        parameter: &InterfaceFileParameter,
        mess: &str,
        val: &mut Handle<HAsciiString>,
    ) -> Result<bool, Failure> {
        if parameter.param_type() != InterfaceParamType::ParamText {
            if parameter.param_type() == InterfaceParamType::ParamVoid {
                *val = Handle::new(HAsciiString::new(""));
                return Ok(true);
            }
            self.labeled(true, mess, " : not given as a Text", "");
            return Ok(false);
        }
        match self.counted_text(parameter.c_value(), IgesDataReadMessage::Text(mess)) {
            Some(text) => {
                *val = text;
                Ok(true)
            }
            None => Ok(false),
        }
    }
    /// The content of a counted text, or the report of a text that is not one; a wrong count is
    /// only a warning. The form used by the labeled reads and the list reads.
    pub(super) fn counted_text(
        &mut self,
        text: &[u8],
        mess: IgesDataReadMessage<'_>,
    ) -> Option<Handle<HAsciiString>> {
        let length = text.len() as i32;
        let symbol = text
            .iter()
            .position(|byte| *byte == b'H')
            .map_or(0, |at| at as i32 + 1);
        if symbol <= 1 || symbol >= length {
            self.report_fail(mess, " : not in Hollerith Form", "");
            return None;
        }
        if iges_atoi(&text[..symbol as usize - 1]) != length - symbol {
            self.report_warning(mess, " : bad Hollerith count ", "");
        }
        Some(Handle::new(HAsciiString::new(&text[symbol as usize..])))
    }
    /// Reads the real at a parameter number; nothing is reported.
    ///
    /// # Errors
    /// An out of range failure for a bad number.
    pub fn reading_real(&mut self, num: i32, val: &mut f64) -> Result<bool, Failure> {
        let parameter = self.param_at(num)?;
        match parameter.param_type() {
            InterfaceParamType::ParamInteger => *val = f64::from(iges_atoi(parameter.c_value())),
            InterfaceParamType::ParamReal | InterfaceParamType::ParamEnum => {
                *val = atof_bytes(&real_text(&parameter));
            }
            InterfaceParamType::ParamVoid => *val = 0.0,
            _ => return Ok(false),
        }
        Ok(true)
    }
    /// Reads the real at a parameter number; what is wrong is reported under `mess`.
    ///
    /// # Errors
    /// An out of range failure for a bad number.
    pub fn reading_real_text(
        &mut self,
        num: i32,
        mess: &str,
        val: &mut f64,
    ) -> Result<bool, Failure> {
        if self.reading_real(num, val)? {
            return Ok(true);
        }
        *val = 0.0;
        self.labeled(
            true,
            mess,
            &format!(": not given as Real, rank {num}"),
            ": not given as Real, rank %d",
        );
        Ok(false)
    }
}

/// The label of a read's message: the reads report by the label, a catalog message otherwise.
pub(super) fn label_of(mess: IgesDataReadMessage<'_>) -> Option<&str> {
    match mess {
        IgesDataReadMessage::Text(label) => Some(label),
        IgesDataReadMessage::Msg(_) => None,
    }
}
