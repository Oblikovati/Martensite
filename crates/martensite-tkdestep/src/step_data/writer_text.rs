//! The lines of the file: the current line, its margins, comments, entity brackets.
use super::StepDataStepWriter;
use super::writer::STEP_LONG;
use martensite_tkernel::{
    standard::{Failure, Handle},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkxsbase::interface::interface_mismatch;

impl StepDataStepWriter {
    /// `new_line(evenempty)`: flushes the current line; an empty one only with `evenempty`.
    pub fn new_line(&mut self, evenempty: bool) -> Result<(), Failure> {
        if evenempty || self.thecurr.length() > 0 {
            let line = self.thecurr.moved()?;
            self.thefile.push(line);
        }
        self.thecurr.set_initial(self.indentation())?;
        self.thecurr.clear()
    }
    pub(super) fn indentation(&self) -> i32 {
        let mut indst = self.thelevel * 2;
        if self.theindent {
            indst += self.theindval;
        }
        indst
    }
    /// `join_last(newline)`: what is added next is kept for the next line if this one is full.
    pub fn join_last(&mut self, _newline: bool) {
        self.thecurr.set_keep();
    }
    /// `indent(onent)`.
    pub fn indent(&mut self, onent: bool) {
        self.theindent = onent;
    }
    /// `send_ident(ident)`.
    pub fn send_ident(&mut self, ident: i32) -> Result<(), Failure> {
        self.thecurr.clear()?;
        self.thecurr.add_text(&format!("#{ident} ="))?;
        self.themult = false;
        Ok(())
    }
    /// `send_scope`.
    pub fn send_scope(&mut self) -> Result<(), Failure> {
        self.add_bytes(b" &SCOPE", 0)
    }
    /// `send_endscope`.
    pub fn send_endscope(&mut self) -> Result<(), Failure> {
        self.new_line(false)?;
        self.append_line("        ENDSCOPE");
        Ok(())
    }
    /// `comment(mode)`: begins or ends a comment zone.
    pub fn comment(&mut self, mode: bool) -> Result<(), Failure> {
        if mode && !self.thecomm {
            self.add_bytes(b"  /*  ", 20)?;
        }
        if !mode && self.thecomm {
            self.add_bytes(b"  */", 0)?;
        }
        self.thecomm = mode;
        Ok(())
    }
    /// `send_comment(const char*)`: an error outside a comment zone.
    pub fn send_comment(&mut self, text: &[u8]) -> Result<(), Failure> {
        if !self.thecomm {
            return Err(interface_mismatch(
                "StepWriter : Comment; expected a comment zone opened by Comment(true)",
            ));
        }
        self.add_bytes(text, 0)
    }
    /// `send_comment(handle)`.
    pub fn send_comment_h_string(&mut self, text: &Handle<HAsciiString>) -> Result<(), Failure> {
        let bytes = text
            .get()
            .map(|live| live.string().as_bytes().to_vec())
            .unwrap_or_default();
        self.send_comment(&bytes)
    }
    /// `start_entity(atype)`: its type, the open bracket; closes the previous component of a
    /// complex entity. Nothing for an empty type.
    pub fn start_entity(&mut self, atype: &AsciiString) -> Result<(), Failure> {
        if atype.is_empty() {
            return Ok(());
        }
        if self.themult {
            if self.thelevel != 1 {
                return Err(interface_mismatch(&format!(
                    "StepWriter : StartEntity; bracket level is {}, expected 1",
                    self.thelevel
                )));
            }
            self.add_bytes(b")", 0)?;
            self.add_bytes(b" ", 0)?;
        }
        self.themult = true;
        self.add_bytes(atype.as_bytes(), 0)?;
        self.thelevel = 0;
        self.theindval = self.thecurr.length();
        self.thecurr.set_initial(0)?;
        self.thefirst = true;
        self.open_sub()
    }
    /// `start_complex`.
    pub fn start_complex(&mut self) -> Result<(), Failure> {
        self.add_bytes(b"( ", 0)
    }
    /// `end_complex`.
    pub fn end_complex(&mut self) -> Result<(), Failure> {
        self.add_bytes(b") ", 0)
    }
    /// `end_entity`: the closing bracket and `;`; an error if the brackets do not balance.
    pub fn end_entity(&mut self) -> Result<(), Failure> {
        if self.thelevel != 1 {
            return Err(interface_mismatch(&format!(
                "StepWriter : EndEntity; bracket level is {}, expected 1",
                self.thelevel
            )));
        }
        self.add_bytes(b");", 0)?;
        self.thelevel = 0;
        let indent = self.theindent;
        self.theindent = false;
        self.new_line(false)?;
        self.theindent = indent;
        self.themult = false;
        Ok(())
    }
    fn flush_line(&mut self, required: i32) -> Result<(), Failure> {
        let line = self.thecurr.moved()?;
        self.thefile.push(line);
        let indst = self.indentation();
        let initial = if indst < STEP_LONG && indst + required <= STEP_LONG {
            indst
        } else {
            0
        };
        self.thecurr.set_initial(initial)
    }
    /// `add_string(astr, lnstr, more = 0)`: adds to the current line, first flushing it when
    /// less than `more` would remain; a text above a whole line is split.
    pub(super) fn add_bytes(&mut self, astr: &[u8], more: i32) -> Result<(), Failure> {
        let lnstr = astr.len() as i32;
        if lnstr + more <= STEP_LONG {
            while !self.thecurr.can_get(lnstr + more) {
                self.flush_line(lnstr + more)?;
            }
            return self.thecurr.add_bytes(astr, lnstr);
        }
        let mut start = 0usize;
        let mut rest = lnstr;
        while rest > 0 {
            if self.thecurr.can_get(rest + more) {
                return self.thecurr.add_bytes(&astr[start..], rest);
            }
            let room = STEP_LONG - self.thecurr.length();
            let fits_next = rest + more <= STEP_LONG;
            if room <= 0 || (fits_next && room < rest + more) {
                self.flush_line(if fits_next { rest + more } else { 0 })?;
                continue;
            }
            let chunk = rest.min(room);
            self.thecurr.add_bytes(&astr[start..], chunk)?;
            start += chunk as usize;
            rest -= chunk;
            if rest > 0 {
                let fits_next = rest + more <= STEP_LONG;
                self.flush_line(if fits_next { rest + more } else { 0 })?;
            }
        }
        Ok(())
    }
}
