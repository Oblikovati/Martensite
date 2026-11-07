//! The splitting of a record into parameters: their end, their kind, the texts across records.
use super::structure::IgesFileStructure;
use martensite_tkxsbase::interface::InterfaceParamType;

/// The kinds of parameters the parser tells apart.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum IgesFileParamKind {
    Void,
    Misc,
    Text,
    Integer,
    SignedInteger,
    Real,
    IncompleteExponent,
    RealExponent,
    IntegerExponent,
}

impl IgesFileParamKind {
    /// The type of the parameter once in a model.
    pub(super) fn param_type(self) -> InterfaceParamType {
        match self {
            Self::Void => InterfaceParamType::ParamVoid,
            Self::Misc | Self::IncompleteExponent => InterfaceParamType::ParamMisc,
            Self::Text => InterfaceParamType::ParamText,
            Self::Integer | Self::SignedInteger => InterfaceParamType::ParamInteger,
            Self::Real | Self::RealExponent => InterfaceParamType::ParamReal,
            Self::IntegerExponent => InterfaceParamType::ParamEnum,
        }
    }
    /// The kind after a digit.
    fn with_digit(self) -> Self {
        match self {
            Self::Void => Self::Integer,
            Self::IncompleteExponent => Self::RealExponent,
            other => other,
        }
    }
    /// The kind after a sign.
    fn with_sign(self) -> Self {
        match self {
            Self::Void => Self::SignedInteger,
            Self::IncompleteExponent | Self::IntegerExponent => self,
            _ => Self::Misc,
        }
    }
    /// The kind after a decimal point.
    fn with_point(self) -> Self {
        match self {
            Self::Void | Self::Integer | Self::SignedInteger => Self::Real,
            _ => Self::Misc,
        }
    }
    /// The kind after an exponent mark.
    fn with_exponent(self) -> Self {
        match self {
            Self::Real => Self::IncompleteExponent,
            Self::Integer | Self::SignedInteger => Self::IntegerExponent,
            _ => Self::Misc,
        }
    }
}

/// Where the parsing of parameters stands from one record to the next.
#[derive(Default)]
pub(super) struct IgesFileParamState {
    /// 0 before anything, 1 at the end of a record, 2 more on this record, 3 in a text that
    /// goes on in the next record.
    pub(super) status: i32,
    hollerith: i32,
    offset: i32,
    /// Positive in a text across records, negative after a parameter that ends a record.
    continuation: i32,
}

/// The parsing of one call: one parameter of a record.
struct OneParam<'a> {
    line: &'a [u8],
    separator: u8,
    terminator: u8,
    length: i32,
    param: Vec<u8>,
    start: i32,
    kind: IgesFileParamKind,
}

impl OneParam<'_> {
    fn at(&self, offset: i32, index: i32) -> u8 {
        usize::try_from(offset + index)
            .ok()
            .and_then(|at| self.line.get(at))
            .copied()
            .unwrap_or(0)
    }
    fn put(&mut self, index: i32, byte: u8) {
        let at = index as usize;
        if at >= self.param.len() {
            self.param.resize(at + 1, 0);
        }
        self.param[at] = byte;
    }
    /// The parameter text from its start, with its terminator.
    fn value_to(&mut self, end: i32) -> Vec<u8> {
        self.put(end, 0);
        self.param[self.start as usize..=end as usize].to_vec()
    }
    /// Whether what follows `index` up to the end of the parameter is only blanks.
    fn blanks_to_delimiter(&self, offset: i32, index: i32) -> bool {
        let mut at = index + 1;
        loop {
            let byte = self.at(offset, at);
            if byte == 0 || byte == self.separator || byte == self.terminator {
                return true;
            }
            if byte != b' ' {
                return false;
            }
            at += 1;
        }
    }
    /// The value of the digits before the mark of a text, 0 when something else is there.
    fn hollerith_count(&self, index: i32) -> i32 {
        let mut count: i32 = 0;
        for at in self.start..index {
            let byte = self.param[at as usize];
            if !byte.is_ascii_digit() {
                return 0;
            }
            count = count.wrapping_mul(10).wrapping_add(i32::from(byte - b'0'));
        }
        count
    }
}

/// How one parameter of a record ended.
enum Delimited {
    /// By a separator or the terminator, at this index.
    Mark(i32, bool),
    /// In a text that goes on in the next record.
    TextContinues,
    /// At the end of the record, at this index.
    Line(i32),
}

impl IgesFileParamState {
    /// Parses parameters from one record, `length` characters long. Until the status is other
    /// than 2, this is to be called again on the same record.
    pub(super) fn param(
        &mut self,
        data: &mut IgesFileStructure,
        line: &[u8],
        separator: u8,
        terminator: u8,
        length: i32,
    ) {
        if self.status == 0 {
            self.continuation = 0;
        }
        if self.status != 2 {
            self.offset = 0;
        }
        if self.status < 3 {
            self.hollerith = 0;
        } else {
            self.offset = self.hollerith;
            if self.offset > length {
                data.add_param(&line[..length as usize]);
                self.hollerith -= length;
                return;
            }
            data.add_param(&line[..self.hollerith.max(0) as usize]);
            self.hollerith = 0;
        }
        let mut one = OneParam {
            line,
            separator,
            terminator,
            length,
            param: vec![0; 128],
            start: 0,
            kind: IgesFileParamKind::Void,
        };
        match self.scan(&mut one) {
            Delimited::Mark(end, is_separator) => self.end_mark(data, &mut one, end, is_separator),
            Delimited::TextContinues => {
                let end = (length - self.offset).max(0);
                one.put(end, 0);
                let to = (length - one.start).max(0) as usize;
                let from = one.start as usize;
                let value = one.param[from..(from + to).min(one.param.len())].to_vec();
                self.status = 3;
                data.new_param(one.kind, &value);
                self.continuation = 1;
            }
            Delimited::Line(end) => {
                self.status = 1;
                self.continuation = -1;
                if end > one.start {
                    let value = one.value_to(end);
                    data.new_param(one.kind, &value);
                }
            }
        }
    }
    /// Scans one parameter: fills it, finds its kind, up to what ends it.
    fn scan(&mut self, one: &mut OneParam<'_>) -> Delimited {
        let mut index = 0;
        loop {
            let byte = one.at(self.offset, index);
            if byte == 0 {
                return Delimited::Line(index);
            }
            if byte == one.separator {
                return Delimited::Mark(index, true);
            }
            if byte == one.terminator {
                return Delimited::Mark(index, false);
            }
            one.put(index, byte);
            match byte {
                b'0'..=b'9' => one.kind = one.kind.with_digit(),
                b'+' | b'-' => one.kind = one.kind.with_sign(),
                b'.' => one.kind = one.kind.with_point(),
                b'E' | b'e' | b'D' | b'd' => one.kind = one.kind.with_exponent(),
                b'H' => {
                    if one.kind == IgesFileParamKind::Integer {
                        one.kind = IgesFileParamKind::Text;
                        self.hollerith = one.hollerith_count(index);
                        if self.offset + index + self.hollerith >= one.length {
                            self.copy_end_of_line(one, index);
                            return Delimited::TextContinues;
                        }
                        for ahead in 1..=self.hollerith {
                            let text = one.at(self.offset, index + ahead);
                            one.put(index + ahead, text);
                        }
                        index += self.hollerith;
                    } else {
                        one.kind = IgesFileParamKind::Misc;
                    }
                }
                b' ' => {
                    if one.kind == IgesFileParamKind::Void {
                        one.start = index + 1;
                    } else if !one.blanks_to_delimiter(self.offset, index) {
                        one.kind = IgesFileParamKind::Misc;
                    }
                }
                _ => one.kind = IgesFileParamKind::Misc,
            }
            index += 1;
        }
    }
    /// Copies the text of a parameter up to the end of the record, and sets what remains of it.
    fn copy_end_of_line(&mut self, one: &mut OneParam<'_>, index: i32) {
        for ahead in 1..(one.length - self.offset - index) {
            let text = one.at(self.offset, index + ahead);
            one.put(index + ahead, text);
        }
        self.hollerith = self.offset + index + self.hollerith + 1 - one.length;
    }
    fn end_mark(
        &mut self,
        data: &mut IgesFileStructure,
        one: &mut OneParam<'_>,
        end: i32,
        is_separator: bool,
    ) {
        let value = one.value_to(end);
        if self.continuation == 0 {
            data.new_param(one.kind, &value);
        } else if self.continuation > 0 {
            data.add_param(&value);
        }
        self.continuation = 0;
        if !is_separator {
            self.status = 1;
            return;
        }
        let mut ahead = end + 1;
        loop {
            match one.at(self.offset, ahead) {
                0 => {
                    self.status = 1;
                    return;
                }
                b' ' => ahead += 1,
                _ => {
                    self.status = 2;
                    self.offset += end + 1;
                    return;
                }
            }
        }
    }
}
