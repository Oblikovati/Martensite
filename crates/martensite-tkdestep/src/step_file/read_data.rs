//! `StepFileReadData`: collects what the parser finds. The current text, written by the
//! scanner, becomes an ident, a type or an argument; an argument is added to the current
//! record, and a sub-list gets its own record, recorded before the one that holds it. The
//! `next` of a record is first the record that holds it, then its follower in the file.
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkxsbase::interface::{InterfaceCheck, InterfaceParamType};
use std::sync::Arc;

/// A text of the file (a pointer into the character pages).
type Text = Arc<[u8]>;

const SUB_LIST: &[u8] = b"/* (SUB) */";
const SCOPE: &[u8] = b"SCOPE";
const NIL: &[u8] = b" ";
const ID_ZERO: &[u8] = b"#0";
const CARTESIAN_POINT: &[u8] = b"CARTESIAN_POINT";

fn text_of(bytes: &[u8]) -> Text {
    Arc::from(bytes)
}

/// `Argument`.
struct Argument {
    value: Text,
    kind: InterfaceParamType,
}

/// `Record`.
struct Record {
    next: Option<usize>,
    args: Vec<Argument>,
    ident: Text,
    record_type: Text,
}

/// `Scope`.
struct ScopeMark {
    previous: Option<usize>,
    record: Option<usize>,
}

/// ```
/// use martensite_tkdestep::step_file::StepFileReadData;
/// let mut data = StepFileReadData::new();
/// data.create_new_text(b"#1");
/// data.record_ident();
/// assert_eq!(data.nb_record(), 0);
/// ```
pub struct StepFileReadData {
    my_mode_print: i32,
    my_nb_rec: i32,
    my_nb_head: i32,
    my_nb_par: i32,
    my_ya_rec: bool,
    my_num_sub: i32,
    my_error_arg: bool,
    my_res_text: Option<Text>,
    my_curr_type: Text,
    my_sub_arg: Option<Text>,
    my_type_arg: InterfaceParamType,
    my_curr_arg: usize,
    records: Vec<Record>,
    my_first_rec: Option<usize>,
    my_cur_rec: Option<usize>,
    my_last_rec: Option<usize>,
    scopes: Vec<ScopeMark>,
    my_cur_scope: Option<usize>,
    my_errors: Vec<String>,
}

impl Default for StepFileReadData {
    fn default() -> Self {
        Self::new()
    }
}

fn no_record(member: &str) -> Failure {
    exceptions::null_object(&format!(
        "StepFileReadData::{member}: no current record; expected one opened by the parser"
    ))
}

impl StepFileReadData {
    /// `StepFileReadData()`.
    pub fn new() -> Self {
        Self {
            my_mode_print: 0,
            my_nb_rec: 0,
            my_nb_head: 0,
            my_nb_par: 0,
            my_ya_rec: false,
            my_num_sub: 0,
            my_error_arg: false,
            my_res_text: None,
            my_curr_type: text_of(SUB_LIST),
            my_sub_arg: None,
            my_type_arg: InterfaceParamType::ParamSub,
            my_curr_arg: 0,
            records: Vec::new(),
            my_first_rec: None,
            my_cur_rec: None,
            my_last_rec: None,
            scopes: Vec::new(),
            my_cur_scope: None,
            my_errors: Vec::new(),
        }
    }
    /// `create_new_text(text, len)`: the current text; the copy stops at a NUL. After an
    /// error argument, the text is appended to the current one.
    pub fn create_new_text(&mut self, text: &[u8]) {
        let end = text.iter().position(|b| *b == 0).unwrap_or(text.len());
        let text = &text[..end];
        if text == CARTESIAN_POINT {
            self.my_res_text = Some(text_of(CARTESIAN_POINT));
            return;
        }
        if self.my_error_arg {
            let mut joined = self.my_res_text.as_deref().unwrap_or_default().to_vec();
            joined.extend_from_slice(text);
            self.my_res_text = Some(text_of(&joined));
            return;
        }
        self.my_res_text = Some(text_of(text));
    }
    fn result_text(&self) -> Text {
        self.my_res_text.clone().unwrap_or_else(|| text_of(b""))
    }
    fn new_record(&mut self, ident: Text, record_type: Text, next: Option<usize>) -> usize {
        self.records.push(Record {
            next,
            args: Vec::new(),
            ident,
            record_type,
        });
        self.records.len() - 1
    }
    fn add_new_record(&mut self, record: usize) {
        self.my_nb_rec += 1;
        if self.my_first_rec.is_none() {
            self.my_first_rec = Some(record);
        }
        if let Some(last) = self.my_last_rec {
            self.records[last].next = Some(record);
        }
        self.my_last_rec = Some(record);
    }
    /// `record_new_entity`: adds the current record; the record that holds it becomes current.
    pub fn record_new_entity(&mut self) -> Result<(), Failure> {
        self.my_error_arg = false;
        let current = self
            .my_cur_rec
            .ok_or_else(|| no_record("RecordNewEntity"))?;
        self.add_new_record(current);
        self.set_type_arg(InterfaceParamType::ParamSub);
        self.my_sub_arg = Some(self.records[current].ident.clone());
        self.my_cur_rec = self.records[current].next;
        self.records[current].next = None;
        Ok(())
    }
    /// `record_ident`: a new record with the current text as ident.
    pub fn record_ident(&mut self) {
        let ident = self.result_text();
        self.my_cur_rec = Some(self.new_record(ident, text_of(b""), None));
        self.my_ya_rec = true;
    }
    /// `record_type`: the type of the current record, or of a new one.
    pub fn record_type(&mut self) {
        let record_type = self.result_text();
        let current = match self.my_cur_rec {
            Some(current) if self.my_ya_rec => current,
            _ => self.new_record(text_of(ID_ZERO), text_of(b""), None),
        };
        self.records[current].record_type = record_type;
        self.my_cur_rec = Some(current);
        self.my_ya_rec = false;
        self.my_num_sub = 0;
    }
    /// `record_list_start`: a sub-list gets a new record, which the current one will hold.
    pub fn record_list_start(&mut self) {
        if self.my_num_sub > 0 {
            let ident = match self.my_num_sub {
                1 => text_of(b"$1"),
                2 => text_of(b"$2"),
                num => self.record_new_text(format!("${num}").as_bytes()),
            };
            let record_type = std::mem::replace(&mut self.my_curr_type, text_of(SUB_LIST));
            self.my_cur_rec = Some(self.new_record(ident, record_type, self.my_cur_rec));
        }
        self.my_error_arg = false;
        self.my_num_sub += 1;
    }
    /// `create_new_arg`: an argument of the last type, its value the current text, or the
    /// ident of the last sub-list.
    pub fn create_new_arg(&mut self) -> Result<(), Failure> {
        self.my_nb_par += 1;
        let value = if self.my_type_arg == InterfaceParamType::ParamSub {
            self.my_sub_arg.clone().unwrap_or_else(|| text_of(b""))
        } else {
            self.result_text()
        };
        if self.my_type_arg == InterfaceParamType::ParamMisc {
            self.my_error_arg = true;
        }
        let current = self.my_cur_rec.ok_or_else(|| no_record("CreateNewArg"))?;
        self.records[current].args.push(Argument {
            value,
            kind: self.my_type_arg,
        });
        Ok(())
    }
    /// `create_error_arg`: one argument for a sequence of errors, with the updated text.
    pub fn create_error_arg(&mut self) -> Result<(), Failure> {
        if !self.my_error_arg {
            self.set_type_arg(InterfaceParamType::ParamMisc);
            self.create_new_arg()?;
            self.my_error_arg = true;
            return Ok(());
        }
        let text = self.result_text();
        let current = self.my_cur_rec.ok_or_else(|| no_record("CreateErrorArg"))?;
        let last = self.records[current]
            .args
            .last_mut()
            .ok_or_else(|| no_record("CreateErrorArg"))?;
        last.value = text;
        Ok(())
    }
    /// `add_new_scope`: a scope containing the current record.
    pub fn add_new_scope(&mut self) {
        self.scopes.push(ScopeMark {
            previous: self.my_cur_scope,
            record: self.my_cur_rec,
        });
        self.my_cur_scope = Some(self.scopes.len() - 1);
        let record = self.new_record(text_of(SCOPE), text_of(NIL), None);
        self.add_new_record(record);
    }
    /// `final_of_scope`: ends the scope, taking a last sub-list as its export list.
    /// Its record is identified as `SCOPE` too.
    pub fn final_of_scope(&mut self) -> Result<(), Failure> {
        let Some(scope) = self.my_cur_scope else {
            return Ok(());
        };
        let record = self.new_record(text_of(SCOPE), text_of(NIL), None);
        if self.my_sub_arg.as_deref().and_then(|t| t.first()) == Some(&b'$') {
            if self.my_mode_print > 0 {
                println!("Export List : (List in Record n0 {}) -- ", self.my_nb_rec);
                self.print_record(self.my_last_rec);
            }
            self.my_cur_rec = Some(record);
            self.my_type_arg = InterfaceParamType::ParamSub;
            self.create_new_arg()?;
        }
        self.add_new_record(record);
        self.my_cur_rec = self.scopes[scope].record;
        self.my_ya_rec = true;
        self.my_cur_scope = self.scopes[scope].previous;
        Ok(())
    }
    /// `clear_recorder(mode)`: 1 the records and arguments, 2 the text, 3 all.
    pub fn clear_recorder(&mut self, mode: i32) {
        if mode & 1 != 0 {
            self.my_curr_type = text_of(b"");
            self.my_sub_arg = None;
            self.my_curr_arg = 0;
            self.my_first_rec = None;
            self.my_cur_rec = None;
            self.my_last_rec = None;
            self.my_cur_scope = None;
            self.records.clear();
            self.scopes.clear();
        }
        if mode & 2 != 0 {
            self.my_res_text = None;
        }
    }
    /// `get_arg_description`: the next argument of the current record.
    pub fn get_arg_description(&mut self) -> Option<(InterfaceParamType, Text)> {
        let record = &self.records[self.my_cur_rec?];
        let argument = record.args.get(self.my_curr_arg)?;
        self.my_curr_arg += 1;
        Some((argument.kind, argument.value.clone()))
    }
    /// `get_file_nb_r`: (records of the header, records, parameters); starts the iteration.
    pub fn get_file_nb_r(&mut self) -> (i32, i32, i32) {
        self.my_cur_rec = self.my_first_rec;
        (self.my_nb_head, self.my_nb_rec, self.my_nb_par)
    }
    /// `get_record_description`: (ident, type, whether it has arguments) of the current record.
    pub fn get_record_description(&mut self) -> Option<(Text, Text, i32)> {
        let record = &self.records[self.my_cur_rec?];
        self.my_curr_arg = 0;
        Some((
            record.ident.clone(),
            record.record_type.clone(),
            i32::from(!record.args.is_empty()),
        ))
    }
    /// `record_type_text`: the type of the next typed list.
    pub fn record_type_text(&mut self) {
        self.my_curr_type = self.result_text();
    }
    /// `next_record`.
    pub fn next_record(&mut self) {
        if let Some(current) = self.my_cur_rec {
            self.my_cur_rec = self.records[current].next;
        }
    }
    /// `print_current_record`.
    pub fn print_current_record(&self) {
        self.print_record(self.my_cur_rec);
    }
    /// `prepare_new_arg`: a comma ends an error argument.
    pub fn prepare_new_arg(&mut self) {
        self.my_error_arg = false;
    }
    /// `final_of_head`: the records so far are the header.
    pub fn final_of_head(&mut self) {
        self.my_nb_head = self.my_nb_rec;
    }
    /// `set_type_arg(type)`.
    pub fn set_type_arg(&mut self, kind: InterfaceParamType) {
        self.my_type_arg = kind;
    }
    /// `set_mode_print(mode)`: 0 nothing, 1 the records, 2 with their arguments.
    pub fn set_mode_print(&mut self, mode: i32) {
        self.my_mode_print = mode;
    }
    /// `get_mode_print`.
    pub fn get_mode_print(&self) -> i32 {
        self.my_mode_print
    }
    /// `GetNbRecord`.
    pub fn nb_record(&self) -> i32 {
        self.my_nb_rec
    }
    /// `add_error(message)`.
    pub fn add_error(&mut self, message: &str) {
        self.my_errors.push(message.to_owned());
    }
    /// `error_handle(check)`: the errors go to the check; true when there is none.
    pub fn error_handle(&self, check: &Handle<InterfaceCheck>) -> bool {
        for message in &self.my_errors {
            check.add_fail_text(message, "Undefined Parsing");
        }
        self.my_errors.is_empty()
    }
    /// `GetLastError`.
    pub fn last_error(&self) -> Option<&str> {
        self.my_errors.last().map(String::as_str)
    }
    fn record_new_text(&mut self, text: &[u8]) -> Text {
        let saved = self.my_res_text.clone();
        self.create_new_text(text);
        let created = self.result_text();
        self.my_res_text = saved;
        created
    }
    fn print_record(&self, record: Option<usize>) {
        let Some(record) = record.map(|rank| &self.records[rank]) else {
            println!("Not defined");
            return;
        };
        let lossy = |text: &[u8]| String::from_utf8_lossy(text).into_owned();
        println!(
            "Ident : {}  Type : {}  Nb.Arg.s : {}",
            lossy(&record.ident),
            lossy(&record.record_type),
            record
                .args
                .first()
                .map_or_else(String::new, |a| lossy(&a.value))
        );
        if self.my_mode_print < 2 {
            return;
        }
        let mut line = 0;
        let mut length = 0;
        for (rank, argument) in record.args.iter().enumerate() {
            length = argument.value.len() + 18;
            line += length;
            if line > 132 {
                println!();
                line = length;
            }
            let code = argument.kind as usize;
            print!(
                "  - Arg.{}[{}{}] : {}",
                rank + 1,
                char::from(b"(IF#TnEHBx"[code.min(9)]),
                char::from(b")nlIxdnxix"[code.min(9)]),
                lossy(&argument.value)
            );
        }
        if length > 0 {
            println!();
        }
    }
}
