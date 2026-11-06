//! The scanner of a STEP file. It applies the rules by longest match, the earlier
//! rule winning a tie, and all of them ignore case. Rules that look ahead count the
//! trailing context in the length. One limit is deliberately not kept: there a token
//! that fills the 16K input buffer (a long string or a long comment line) fails the whole
//! read; here the whole text is in memory, so a token of any length reads.
use super::StepFileReadData;
use martensite_tkxsbase::interface::InterfaceParamType;

/// The token kinds: the symbol numbers of the parser tables.
pub(super) mod token {
    pub const END_OF_FILE: i32 = 0;
    pub const STEP: i32 = 3;
    pub const HEADER: i32 = 4;
    pub const ENDSEC: i32 = 5;
    pub const DATA: i32 = 6;
    pub const ENDSTEP: i32 = 7;
    pub const SCOPE: i32 = 8;
    pub const ENDSCOPE: i32 = 9;
    pub const ENTITY: i32 = 10;
    pub const TYPE: i32 = 11;
    pub const IDENT: i32 = 14;
    pub const QUID: i32 = 19;
    pub const SEMICOLON: i32 = 21;
    pub const OPEN: i32 = 22;
    pub const CLOSE: i32 = 23;
    pub const COMMA: i32 = 24;
    pub const EQUAL: i32 = 25;
    pub const SLASH: i32 = 26;
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Condition {
    Initial,
    Comment,
    Text,
    End,
}

/// What a longest match selected, in the order of the rules.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Rule {
    Integer,
    Real,
    Hexa,
    Enum,
    Keyword(i32),
    EndIso,
    Scope,
    EndScope,
    Type,
    Misc,
}

fn is_word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// The length of the run of bytes from `from` that satisfy `accept`.
fn run(text: &[u8], from: usize, accept: impl Fn(u8) -> bool) -> usize {
    text.get(from..)
        .map_or(0, |tail| tail.iter().take_while(|b| accept(**b)).count())
}

/// `[-+0-9][0-9]*`.
fn match_integer(text: &[u8]) -> usize {
    match text.first() {
        Some(b'-' | b'+' | b'0'..=b'9') => 1 + run(text, 1, |b| b.is_ascii_digit()),
        _ => 0,
    }
}

/// `[-+\.0-9][\.0-9]+`.
fn match_real(text: &[u8]) -> usize {
    let is_number = |b: u8| b == b'.' || b.is_ascii_digit();
    match text.first() {
        Some(b'-' | b'+' | b'.' | b'0'..=b'9') => {
            let tail = run(text, 1, is_number);
            if tail == 0 { 0 } else { 1 + tail }
        }
        _ => 0,
    }
}

/// `[-+\.0-9][\.0-9]+E[-+0-9][0-9]*`.
fn match_real_exponent(text: &[u8]) -> usize {
    let mantissa = match_real(text);
    if mantissa == 0 || !matches!(text.get(mantissa), Some(b'E' | b'e')) {
        return 0;
    }
    match text.get(mantissa + 1) {
        Some(b'-' | b'+' | b'0'..=b'9') => {
            mantissa + 2 + run(text, mantissa + 2, |b| b.is_ascii_digit())
        }
        _ => 0,
    }
}

/// `["][0-9A-F]+["]`.
fn match_hexa(text: &[u8]) -> usize {
    if text.first() != Some(&b'"') {
        return 0;
    }
    let digits = run(text, 1, |b| b.is_ascii_hexdigit());
    if digits > 0 && text.get(digits + 1) == Some(&b'"') {
        digits + 2
    } else {
        0
    }
}

/// `[.]*[A-Z0-9_]+[.]`.
fn match_enum(text: &[u8]) -> usize {
    let dots = run(text, 0, |b| b == b'.');
    let word = run(text, dots, is_word);
    if word > 0 && text.get(dots + word) == Some(&b'.') {
        dots + word + 1
    } else {
        0
    }
}

/// `KEYWORD` then `[0-9\-]*;`, ignoring case; 0 when it does not start the text.
fn match_iso(text: &[u8], keyword: &[u8]) -> usize {
    let is_keyword = text
        .get(..keyword.len())
        .is_some_and(|head| head.eq_ignore_ascii_case(keyword));
    if !is_keyword {
        return 0;
    }
    let tail = run(text, keyword.len(), |b| b == b'-' || b.is_ascii_digit());
    if text.get(keyword.len() + tail) == Some(&b';') {
        keyword.len() + tail + 1
    } else {
        0
    }
}

/// `KEYWORD`, ignoring case.
fn match_word(text: &[u8], keyword: &[u8]) -> usize {
    let found = text
        .get(..keyword.len())
        .is_some_and(|head| head.eq_ignore_ascii_case(keyword));
    if found { keyword.len() } else { 0 }
}

/// The section keywords, each with its `;`.
fn match_keyword(text: &[u8]) -> Option<(usize, Rule)> {
    const WORDS: [(&[u8], i32); 4] = [
        (b"STEP;", token::STEP),
        (b"HEADER;", token::HEADER),
        (b"ENDSEC;", token::ENDSEC),
        (b"DATA;", token::DATA),
    ];
    for (word, kind) in WORDS {
        let length = match_word(text, word);
        if length > 0 {
            return Some((length, Rule::Keyword(kind)));
        }
    }
    let endstep = match_word(text, b"ENDSTEP;");
    if endstep > 0 {
        let rest = run(text, endstep, |b| b != b'\n');
        return Some((endstep + rest, Rule::Keyword(token::ENDSTEP)));
    }
    let end_iso = match_iso(text, b"END-ISO");
    if end_iso > 0 {
        return Some((end_iso, Rule::EndIso));
    }
    let iso = match_iso(text, b"ISO");
    (iso > 0).then_some((iso, Rule::Keyword(token::STEP)))
}

/// The longest rule of the text; a tie goes to the first one listed.
fn longest_match(text: &[u8]) -> (usize, Rule) {
    let bang = if text.first() == Some(&b'!') {
        let word = run(text, 1, is_word);
        if word > 0 { 1 + word } else { 0 }
    } else {
        0
    };
    let keyword = match_keyword(text).unwrap_or((0, Rule::Misc));
    let candidates = [
        (match_integer(text), Rule::Integer),
        (match_real(text), Rule::Real),
        (match_real_exponent(text), Rule::Real),
        (match_hexa(text), Rule::Hexa),
        (match_enum(text), Rule::Enum),
        keyword,
        (match_word(text, b"&SCOPE"), Rule::Scope),
        (match_word(text, b"ENDSCOPE"), Rule::EndScope),
        (run(text, 0, is_word), Rule::Type),
        (bang, Rule::Type),
        (1, Rule::Misc),
    ];
    let mut best = (0, Rule::Misc);
    for candidate in candidates {
        if candidate.0 > best.0 {
            best = candidate;
        }
    }
    best
}

/// The scanner over the whole text of a file.
pub(super) struct StepFileScanner<'a> {
    text: &'a [u8],
    pos: usize,
    condition: Condition,
    /// `yylineno`.
    line: i32,
}

impl<'a> StepFileScanner<'a> {
    pub(super) fn new(text: &'a [u8]) -> Self {
        Self {
            text,
            pos: 0,
            condition: Condition::Initial,
            line: 1,
        }
    }
    /// `lineno`.
    pub(super) fn line(&self) -> i32 {
        self.line
    }
    /// `lex`: the kind of the next token; its text is given to `data`.
    pub(super) fn lex(&mut self, data: &mut StepFileReadData) -> i32 {
        loop {
            let found = match self.condition {
                Condition::Initial => self.lex_initial(data),
                Condition::Comment => self.lex_comment(),
                Condition::Text => self.lex_text(data),
                Condition::End => self.lex_end(),
            };
            if let Some(kind) = found {
                return kind;
            }
        }
    }
    fn lex_initial(&mut self, data: &mut StepFileReadData) -> Option<i32> {
        let start = self.pos;
        let Some(&byte) = self.text.get(start) else {
            return Some(token::END_OF_FILE);
        };
        match byte {
            b'/' if self.text.get(start + 1) == Some(&b'*') => {
                self.pos += 2;
                self.condition = Condition::Comment;
            }
            b'\'' => {
                self.condition = Condition::Text;
            }
            b' ' | b'\t' | b'\r' => self.pos += 1,
            b'\n' => {
                self.line += 1;
                self.pos += 1;
            }
            0 => self.pos += run(self.text, start, |b| b == 0),
            b'#' => return Some(self.lex_hash(data)),
            b'(' => return Some(self.single(token::OPEN)),
            b')' => return Some(self.single(token::CLOSE)),
            b',' => {
                data.prepare_new_arg();
                return Some(self.single(token::COMMA));
            }
            b'$' => {
                data.create_new_text(b"$");
                data.set_type_arg(InterfaceParamType::ParamVoid);
                return Some(self.single(token::QUID));
            }
            b'=' => return Some(self.single(token::EQUAL)),
            b';' => return Some(self.single(token::SEMICOLON)),
            b'/' => return Some(self.single(token::SLASH)),
            _ => return Some(self.lex_longest(data)),
        }
        None
    }
    fn single(&mut self, kind: i32) -> i32 {
        self.pos += 1;
        kind
    }
    /// `#[0-9]+` with or without its trailing `[ \t]*=`.
    fn lex_hash(&mut self, data: &mut StepFileReadData) -> i32 {
        let start = self.pos;
        let digits = run(self.text, start + 1, |b| b.is_ascii_digit());
        if digits == 0 {
            data.create_new_text(b"#");
            data.set_type_arg(InterfaceParamType::ParamMisc);
            return self.single(token::QUID);
        }
        let end = start + 1 + digits;
        let blanks = run(self.text, end, |b| b == b' ' || b == b'\t');
        data.create_new_text(&self.text[start..end]);
        self.pos = end;
        if self.text.get(end + blanks) == Some(&b'=') {
            token::ENTITY
        } else {
            token::IDENT
        }
    }
    fn lex_longest(&mut self, data: &mut StepFileReadData) -> i32 {
        let start = self.pos;
        let (length, rule) = longest_match(&self.text[start..]);
        self.pos = start + length;
        let matched = &self.text[start..start + length];
        let quid = |data: &mut StepFileReadData, kind: InterfaceParamType| {
            data.create_new_text(matched);
            data.set_type_arg(kind);
            token::QUID
        };
        match rule {
            Rule::Integer => quid(data, InterfaceParamType::ParamInteger),
            Rule::Real => quid(data, InterfaceParamType::ParamReal),
            Rule::Hexa => quid(data, InterfaceParamType::ParamHexa),
            Rule::Enum => quid(data, InterfaceParamType::ParamEnum),
            Rule::Misc => quid(data, InterfaceParamType::ParamMisc),
            Rule::Keyword(kind) => kind,
            Rule::EndIso => {
                self.condition = Condition::End;
                token::ENDSTEP
            }
            Rule::Scope => token::SCOPE,
            Rule::EndScope => token::ENDSCOPE,
            Rule::Type => {
                data.create_new_text(matched);
                token::TYPE
            }
        }
    }
    fn lex_comment(&mut self) -> Option<i32> {
        let start = self.pos;
        let Some(&byte) = self.text.get(start) else {
            return Some(token::END_OF_FILE);
        };
        if byte == b'\n' {
            self.line += 1;
            self.pos += 1;
            return None;
        }
        if byte != b'*' {
            self.pos += run(self.text, start, |b| b != b'*' && b != b'\n');
            return None;
        }
        let stars = run(self.text, start, |b| b == b'*');
        let after = start + stars;
        if self.text.get(after) == Some(&b'/') {
            self.pos = after + 1;
            self.condition = Condition::Initial;
            return None;
        }
        self.pos = after + run(self.text, after, |b| b != b'*' && b != b'/' && b != b'\n');
        None
    }
    /// The whole quoted text, from the opening quote at `self.pos`. A quote ends it only
    /// when followed by `[" \n\r]*` then `)` or `,`; the class holds the double quote.
    fn lex_text(&mut self, data: &mut StepFileReadData) -> Option<i32> {
        let start = self.pos;
        let mut cursor = start + 1;
        loop {
            match self.text.get(cursor) {
                None => {
                    self.pos = cursor;
                    return Some(token::END_OF_FILE);
                }
                Some(b'\n') => {
                    self.line += 1;
                    cursor += 1;
                }
                Some(b'\'') => {
                    let spaces = run(self.text, cursor + 1, |b| {
                        matches!(b, b'"' | b' ' | b'\n' | b'\r')
                    });
                    let look = cursor + 1 + spaces;
                    if matches!(self.text.get(look), Some(b')' | b',')) {
                        data.create_new_text(&self.text[start..=cursor]);
                        data.set_type_arg(InterfaceParamType::ParamText);
                        self.pos = cursor + 1;
                        self.condition = Condition::Initial;
                        return Some(token::QUID);
                    }
                    cursor += 1;
                }
                Some(_) => cursor += run(self.text, cursor, |b| b != b'\n' && b != b'\''),
            }
        }
    }
    fn lex_end(&mut self) -> Option<i32> {
        let start = self.pos;
        let Some(&byte) = self.text.get(start) else {
            return Some(token::END_OF_FILE);
        };
        if byte == b'\n' {
            self.line += 1;
        }
        self.pos += 1;
        None
    }
}
