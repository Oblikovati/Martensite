//! The parser of a STEP file: the LALR(1) automaton of the grammar, with its error
//! recovery and its messages. The automaton's tables are the grammar's own, so the language
//! accepted, the recovery points and the "expecting" lists are the standard ones.
//!
//! The grammar:
//! ```text
//! stepf1: STEP HEADER headl ENDSEC endhead model ENDSEC finstep
//! stepf2: STEP HEADER ENDSEC endhead model ENDSEC ENDSTEP
//! stepf3: STEP HEADER ENDSEC endhead model error (stops at once)
//! headent: enttype listarg ';' | error
//! unarg: IDENT | QUID | listarg | listype listarg | error
//! listarg: deblist finlist | deblist arglist finlist
//! bloc: entlab '=' unent ';' | entlab '=' debscop model finscop unent ';'
//!         | entlab '=' debscop finscop unent ';' | error
//! unent: enttype listarg | '(' plex ')'
//! finscop: ENDSCOPE | ENDSCOPE debexp export '/'
//! ```
use super::StepFileReadData;
use super::read::step_file_interrupt;
use super::scanner::{StepFileScanner, token};
use martensite_tkernel::standard::Failure;
use martensite_tkxsbase::interface::InterfaceParamType;

const PACT_NINF: i32 = -24;
const TABLE_NINF: i32 = -9;
const LAST: i32 = 82;
const FINAL: i32 = 7;
const NTOKENS: i32 = 27;
const ERROR_TOKEN: i32 = 1;

#[rustfmt::skip]
const PACT: [i8; 83] = [
    26, 5, -24, -24, -24, 35, 29, -24, -24, 41, -24, 43, -24, 36, -24, 45, 41,
    -24, -24, 3, 38, -24, -24, 40, -24, 32, 45, -24, -24, -24, -24, -24, -24, 36,
    -24, -24, 9, -24, 60, 56, -24, -3, 51, -24, 17, -24, -24, -24, 53, 44, 6,
    36, 59, -24, 0, 36, -24, 42, 6, 2, -24, 47, -24, -24, 36, -24, -24, 55,
    2, 49, -24, 52, -24, -24, -24, -14, 50, -24, -24, 55, -24, -24, -24,
];
#[rustfmt::skip]
const DEFACT: [i8; 83] = [
    0, 0, 9, 10, 11, 0, 0, 1, 15, 0, 47, 0, 12, 0, 16, 0, 0, 13, 23, 0, 0,
    34, 46, 0, 29, 0, 0, 21, 22, 17, 18, 24, 27, 0, 25, 19, 0, 14, 34, 0, 30, 0,
    0, 20, 0, 26, 7, 39, 0, 0, 0, 0, 0, 28, 0, 0, 31, 44, 0, 0, 37, 4, 6,
    38, 0, 35, 43, 0, 0, 0, 2, 5, 36, 40, 41, 0, 0, 33, 3, 0, 45, 32, 42,
];
#[rustfmt::skip]
const PGOTO: [i8; 27] = [
    -24, -24, -24, -24, -24, -24, -24, -24, 62, 58, 31, -24, -24, 46, -13, -24,
    -23, -21, -24, -6, -24, -2, -24, -24, 18, -24, -5,
];
#[rustfmt::skip]
const DEFGOTO: [i8; 27] = [
    -1, 71, 62, 2, 3, 4, 5, 11, 12, 15, 32, 33, 19, 34, 35, 36, 23, 24, 54, 49,
    50, 74, 75, 67, 59, 25, 51,
];
#[rustfmt::skip]
const TABLE: [i8; 83] = [
    20, 13, 40, 42, 27, 47, 13, 21, 10, 6, 79, 10, 80, 10, 28, 57, 22, 29, 27, 48, 43,
    40, 30, 63, 48, 18, 31, 58, 28, 1, 8, 29, 31, 44, 9, 7, 30, 40, 60, 18, 10, 38,
    65, 55, 8, 39, 21, 14, 16, 64, 22, 72, 21, 69, 10, 22, 52, 41, 18, 37, -8, 22, 76,
    46, 10, 56, 61, 70, 66, 73, 77, 81, 78, 17, 26, 53, 68, 82, 0, 0, 0, 0, 45,
];
#[rustfmt::skip]
const CHECK: [i8; 83] = [
    13, 6, 23, 26, 1, 8, 11, 1, 11, 4, 24, 11, 26, 11, 11, 9, 10, 14, 1, 22, 33,
    42, 19, 23, 22, 22, 23, 50, 11, 3, 1, 14, 23, 24, 5, 0, 19, 58, 51, 22, 11, 1,
    55, 48, 1, 5, 1, 6, 5, 54, 10, 64, 1, 59, 11, 10, 5, 25, 22, 21, 0, 10, 68,
    7, 11, 21, 7, 20, 26, 14, 21, 21, 20, 11, 16, 44, 58, 79, -1, -1, -1, -1, 36,
];
#[rustfmt::skip]
const R1: [i8; 48] = [
    0, 27, 28, 28, 29, 29, 30, 31, 32, 33, 33, 33, 34, 34, 35, 35, 36, 37, 37, 37, 37, 37, 38, 39,
    40, 41, 41, 42, 42, 43, 43, 44, 44, 44, 44, 45, 45, 46, 46, 47, 48, 49, 49, 50, 51, 51, 52, 53,
];
#[rustfmt::skip]
const R2: [i8; 48] = [
    0, 2, 1, 2, 1, 2, 8, 7, 6, 1, 1, 1, 1, 2, 3, 1,
    1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 3, 1, 3, 1, 2, 4,
    7, 6, 1, 2, 3, 2, 3, 1, 1, 1, 3, 1, 1, 4, 1, 1,
];
/// The names of the tokens in the messages.
const TOKEN_NAMES: [&str; 27] = [
    "end of file",
    "error",
    "invalid token",
    "STEP",
    "HEADER",
    "ENDSEC",
    "DATA",
    "ENDSTEP",
    "SCOPE",
    "ENDSCOPE",
    "ENTITY",
    "TYPE",
    "INTEGER",
    "FLOAT",
    "IDENT",
    "TEXT",
    "NONDEF",
    "ENUM",
    "HEXA",
    "QUID",
    "' '",
    "';'",
    "'('",
    "')'",
    "','",
    "'='",
    "'/'",
];

/// The semantic action of a rule; true for the action that ends the parse (rule `stepf3`).
fn reduce_action(
    rule: i32,
    data: &mut StepFileReadData,
    error_status: &mut i32,
) -> Result<bool, Failure> {
    match rule {
        11 => return Ok(true),
        16 => data.final_of_head(),
        17 | 40 => {
            data.set_type_arg(InterfaceParamType::ParamIdent);
            data.create_new_arg()?;
        }
        18..=20 => data.create_new_arg()?,
        21 => data.create_error_arg()?,
        22 => data.record_type_text(),
        23 | 43 => data.record_list_start(),
        24 => {
            if data.get_mode_print() > 0 {
                print!("Record no : {} -- ", data.nb_record() + 1);
                data.print_current_record();
            }
            data.record_new_entity()?;
            *error_status = 0;
        }
        39 => data.add_new_scope(),
        44 => data.final_of_scope()?,
        45 => {
            println!("***  Warning : Export List not yet processed");
            data.record_new_entity()?;
            data.final_of_scope()?;
        }
        46 => data.record_ident(),
        47 => data.record_type(),
        _ => {}
    }
    Ok(false)
}

fn table_at(index: i32) -> Option<(i32, i32)> {
    let rank = usize::try_from(index).ok().filter(|_| index <= LAST)?;
    Some((i32::from(CHECK[rank]), i32::from(TABLE[rank])))
}

/// `yy_lr_goto_state_`.
fn goto_state(state: i32, symbol: i32) -> i32 {
    let rank = (symbol - NTOKENS) as usize;
    let index = i32::from(PGOTO[rank]) + state;
    match table_at(index) {
        Some((check, target)) if check == state => target,
        _ => i32::from(DEFGOTO[rank]),
    }
}

/// `expected_tokens`: at most `limit`; none when there are more.
fn expected_tokens(state: i32, limit: usize) -> Vec<i32> {
    let base = i32::from(PACT[state as usize]);
    if base == PACT_NINF {
        return Vec::new();
    }
    let begin = if base < 0 { -base } else { 0 };
    let end = (LAST - base + 1).min(NTOKENS);
    let mut found = Vec::new();
    for kind in begin..end {
        let usable = table_at(kind + base).is_some_and(|(check, value)| {
            check == kind && kind != ERROR_TOKEN && value != TABLE_NINF
        });
        if usable {
            if found.len() == limit {
                return Vec::new();
            }
            found.push(kind);
        }
    }
    found
}

/// `yysyntax_error_`: at most four expected tokens are listed.
fn syntax_message(state: i32, lookahead: Option<i32>) -> String {
    let Some(kind) = lookahead else {
        return "syntax error".to_owned();
    };
    let mut message = format!("syntax error, unexpected {}", TOKEN_NAMES[kind as usize]);
    for (rank, expected) in expected_tokens(state, 4).into_iter().enumerate() {
        message.push_str(if rank == 0 { ", expecting " } else { " or " });
        message.push_str(TOKEN_NAMES[expected as usize]);
    }
    message
}

/// `step::parser::error`.
fn report_error(message: &str, line: i32, data: &mut StepFileReadData) {
    let line = line + 1;
    let text = if !message.starts_with("syntax error") {
        format!("Undefined Parsing: Line {line}: {message}")
    } else if message.len() > 13 {
        let detail = message.get(14..).unwrap_or_default();
        format!("Undefined Parsing: Line {line}: Incorrect syntax: {detail}")
    } else {
        format!("Undefined Parsing: Line {line}: Incorrect syntax")
    };
    step_file_interrupt(Some(&text), false);
    data.add_error(&text);
}

/// The states and the state of the recovery.
struct Automaton {
    stack: Vec<i32>,
    lookahead: Option<i32>,
    error_status: i32,
}

/// What follows a step of the automaton.
enum Next {
    Continue,
    Accept,
    Abort,
}

impl Automaton {
    fn state(&self) -> i32 {
        self.stack.last().copied().unwrap_or(0)
    }
    /// `yyerrlab`, `yyerrlab1`: reports, then pops until a state shifts the error token.
    fn recover(&mut self, scanner: &StepFileScanner<'_>, data: &mut StepFileReadData) -> Next {
        if self.error_status == 0 {
            let message = syntax_message(self.state(), self.lookahead);
            report_error(&message, scanner.line(), data);
        }
        if self.error_status == 3 {
            if self.lookahead == Some(token::END_OF_FILE) {
                return Next::Abort;
            }
            self.lookahead = None;
        }
        self.error_status = 3;
        loop {
            let base = i32::from(PACT[self.state() as usize]);
            if base != PACT_NINF
                && let Some((check, target)) = table_at(base + ERROR_TOKEN)
                && check == ERROR_TOKEN
                && target > 0
            {
                self.stack.push(target);
                return Next::Continue;
            }
            if self.stack.len() == 1 {
                return Next::Abort;
            }
            self.stack.pop();
        }
    }
    /// `yyreduce`.
    fn reduce(&mut self, rule: i32, data: &mut StepFileReadData) -> Result<Next, Failure> {
        if reduce_action(rule, data, &mut self.error_status)? {
            return Ok(Next::Accept);
        }
        let length = i32::from(R2[rule as usize]) as usize;
        self.stack.truncate(self.stack.len() - length);
        let target = goto_state(self.state(), i32::from(R1[rule as usize]));
        self.stack.push(target);
        Ok(Next::Continue)
    }
    fn step(
        &mut self,
        scanner: &mut StepFileScanner<'_>,
        data: &mut StepFileReadData,
    ) -> Result<Next, Failure> {
        let state = self.state();
        if state == FINAL {
            return Ok(Next::Accept);
        }
        let base = i32::from(PACT[state as usize]);
        if base != PACT_NINF {
            let kind = match self.lookahead {
                Some(kind) => kind,
                None => {
                    let kind = scanner.lex(data);
                    self.lookahead = Some(kind);
                    kind
                }
            };
            if let Some((check, value)) = table_at(base + kind)
                && check == kind
            {
                if value == TABLE_NINF {
                    return Ok(self.recover(scanner, data));
                }
                if value <= 0 {
                    return self.reduce(-value, data);
                }
                self.error_status = (self.error_status - 1).max(0);
                self.stack.push(value);
                self.lookahead = None;
                return Ok(Next::Continue);
            }
        }
        let rule = i32::from(DEFACT[state as usize]);
        if rule == 0 {
            return Ok(self.recover(scanner, data));
        }
        self.reduce(rule, data)
    }
}

/// `step::parser::parse`: 0 when accepted, 1 when aborted; `Err` for what the code
/// raises from the scanner or the actions.
pub(super) fn parse(text: &[u8], data: &mut StepFileReadData) -> Result<i32, Failure> {
    let mut scanner = StepFileScanner::new(text);
    let mut automaton = Automaton {
        stack: vec![0],
        lookahead: None,
        error_status: 0,
    };
    loop {
        match automaton.step(&mut scanner, data)? {
            Next::Continue => {}
            Next::Accept => return Ok(0),
            Next::Abort => return Ok(1),
        }
    }
}
