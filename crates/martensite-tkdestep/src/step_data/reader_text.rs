//! The cleaning of a quoted STEP string: quotes off, doubled characters single, control
//! directives (`\X\`, `\X2\`, `\X4\`, `\S\`, `\P\`, `\N\`, `\T\`) decoded to UTF-8, and
//! raw line ends removed. Positions are 1-based, as the algorithm's.
use super::StepDataStepReaderData;
use martensite_tkernel::{
    resource::{FormatType, ResourceUnicode},
    standard::{Failure, Handle},
    tcollection::{ExtendedString, HAsciiString},
};
use martensite_tkxsbase::interface::InterfaceCheck;

/// The value of a hexadecimal digit, 0 for anything else.
fn hex_value(unit: u16) -> u16 {
    char::from_u32(u32::from(unit))
        .and_then(|digit| digit.to_digit(16))
        .map_or(0, |digit| digit as u16)
}

/// The UTF-16 units of `text`.
fn units_of(text: &ExtendedString) -> Vec<u16> {
    let units = text.as_utf16_with_nul();
    units[..units.len().saturating_sub(1)].to_vec()
}

/// The ISO 8859 code page designated by `\P{id}\`.
fn iso_page(id: u8) -> Option<FormatType> {
    let rank = usize::from(id.checked_sub(b'A')?);
    if rank > 8 {
        return None;
    }
    let first = FormatType::ALL
        .iter()
        .position(|known| *known == FormatType::Iso8859_1)?;
    FormatType::ALL.get(first + rank).copied()
}

/// The working state of one string, with the indices.
struct TextCleaner<'a> {
    units: Vec<u16>,
    size: i32,
    pos: i32,
    set_pos: i32,
    temp: Vec<u16>,
    local_format: FormatType,
    check: &'a Handle<InterfaceCheck>,
}

impl TextCleaner<'_> {
    fn at(&self, index: i32) -> u16 {
        usize::try_from(index - 1)
            .ok()
            .and_then(|rank| self.units.get(rank).copied())
            .unwrap_or(0)
    }
    /// `set_value(where, what)` of an extended string: overwrites, growing when needed.
    fn set_from(&mut self, place: i32, what: &[u16]) {
        let start = (place - 1).max(0) as usize;
        if self.units.len() < start + what.len() {
            self.units.resize(start + what.len(), 0);
        }
        self.units[start..start + what.len()].copy_from_slice(what);
    }
    /// `set_value(where, ToExtString() + offset)`: moves the tail of the text.
    fn move_tail(&mut self, place: i32, offset: i32) {
        let start = (offset.max(0) as usize).min(self.units.len());
        let end = self.units[start..]
            .iter()
            .position(|unit| *unit == 0)
            .map_or(self.units.len(), |rank| start + rank);
        let tail = self.units[start..end].to_vec();
        self.set_from(place, &tail);
    }
    fn trunc(&mut self, length: i32) {
        self.units.truncate(length.max(0) as usize);
    }
    /// `\P{N}\`: the code page of the next `\S\`.
    fn page_directive(&mut self) {
        let id = (self.at(self.pos + 2) & 255) as u8;
        match iso_page(id.to_ascii_uppercase()) {
            Some(format) => self.local_format = format,
            None => self.check.add_warning_text(
                "String control directive \\P*\\ with an unsupported symbol in place of *",
                "",
            ),
        }
        self.pos += 3;
    }
    /// `\S\`: the following Latin code point, in the current code page.
    fn latin_directive(&mut self) -> Result<(), Failure> {
        let byte = (self.at(self.pos + 3) | 0x80) as u8;
        let mut converted = ExtendedString::default();
        ResourceUnicode::convert_format_to_unicode(self.local_format, &[byte], &mut converted)?;
        self.temp = units_of(&converted);
        self.pos += 3;
        Ok(())
    }
    /// `\X\HH`: one byte, without conversion. A zero byte gives nothing.
    fn byte_directive(&mut self) {
        let value = (hex_value(self.at(self.pos + 3)) << 4) | hex_value(self.at(self.pos + 4));
        let byte = value as u8;
        self.temp = if byte == 0 {
            Vec::new()
        } else {
            vec![u16::from(byte)]
        };
        self.pos += 4;
    }
    /// `\X2\...\X0\`, `\X4\...\X0\`: UTF-16 or UTF-32 hexadecimal digits.
    fn wide_directive(&mut self) {
        let first = self.pos + 3;
        let mut last = self.pos;
        let mut closed = false;
        while self.pos <= self.size && !closed {
            if self.size - self.pos > 2
                && self.at(self.pos) == u16::from(b'\\')
                && self.at(self.pos + 1) == u16::from(b'X')
                && self.at(self.pos + 2) == u16::from(b'0')
                && self.at(self.pos + 3) == u16::from(b'\\')
            {
                last = self.pos - 1;
                self.pos += 2;
                closed = true;
            }
            self.pos += 1;
        }
        if !closed {
            self.pos = self.size;
            last = self.size;
        }
        let length = last - first;
        let kind = self.at(first - 1);
        if kind == u16::from(b'2') && self.size - first > 3 {
            self.decode_utf16(first, length);
        } else if kind == u16::from(b'4') && self.size - first > 7 {
            self.decode_utf32(first, length);
        }
    }
    fn decode_utf16(&mut self, first: i32, length: i32) {
        let step = if length % 4 == 0 { 4 } else { 3 };
        if length % step != 0 {
            self.temp.push(u16::from(b'?'));
            self.check.add_warning_text(
                "String control directive \\X2\\ is followed by number of digits not multiple of 4",
                "",
            );
            return;
        }
        let mut unit = 0u16;
        for rank in 1..=length {
            unit |= hex_value(self.at(rank + first));
            if rank % step == 0 {
                self.temp.push(unit);
                unit = 0;
            }
            unit <<= 4;
        }
    }
    fn decode_utf32(&mut self, first: i32, length: i32) {
        if length % 8 != 0 {
            self.temp.push(u16::from(b'?'));
            self.check.add_warning_text(
                "String control directive \\X4\\ is followed by number of digits not multiple of 8",
                "",
            );
            return;
        }
        let mut code = 0u32;
        for rank in 1..=length {
            code |= u32::from(hex_value(self.at(rank + first)));
            if rank % 8 == 0 {
                let decoded = char::from_u32(code).unwrap_or(char::REPLACEMENT_CHARACTER);
                let mut buffer = [0u16; 2];
                self.temp
                    .extend_from_slice(decoded.encode_utf16(&mut buffer));
                code = 0;
            }
            code <<= 4;
        }
    }
    /// The directive at the current position, when there is one.
    fn try_directive(&mut self) -> Result<bool, Failure> {
        if self.at(self.pos) != u16::from(b'\\') || self.pos > self.size - 3 {
            return Ok(false);
        }
        let dir = self.at(self.pos + 1);
        let second_slash = self.at(self.pos + 2) == u16::from(b'\\');
        let third_slash = self.at(self.pos + 3) == u16::from(b'\\');
        if dir == u16::from(b'P') && third_slash {
            self.page_directive();
        } else if dir == u16::from(b'S') && second_slash {
            self.latin_directive()?;
        } else if dir == u16::from(b'X') && self.pos <= self.size - 4 && second_slash {
            self.byte_directive();
        } else if dir == u16::from(b'X') && third_slash {
            self.wide_directive();
        } else {
            return Ok(false);
        }
        Ok(true)
    }
    /// Replaces the directive by its text, when the file is converted.
    fn apply_directive(&mut self, to_conversion: bool) {
        if to_conversion {
            let temp = std::mem::take(&mut self.temp);
            let length = temp.len() as i32;
            self.size -= self.pos - self.set_pos - length + 1;
            self.set_from(self.set_pos, &temp);
            self.set_pos += length;
            self.move_tail(self.set_pos, self.pos);
            self.pos = self.set_pos - 1;
            self.trunc(self.size);
        }
        self.temp.clear();
    }
    /// The doubled characters and `\N\`, `\T\`; true when the character is done.
    fn doubled_or_control(&mut self, unit: u16) -> bool {
        if self.pos > self.size - 1 {
            return false;
        }
        let next = self.at(self.pos + 1);
        if next == unit && (unit == u16::from(b'\'') || unit == u16::from(b'\\')) {
            self.move_tail(self.set_pos, self.pos);
            self.size -= 1;
            self.trunc(self.size);
            return false;
        }
        let is_directive = unit == u16::from(b'\\')
            && self.pos <= self.size - 2
            && self.at(self.pos + 2) == u16::from(b'\\');
        let replacement = match next {
            n if is_directive && n == u16::from(b'N') => u16::from(b'\n'),
            n if is_directive && n == u16::from(b'T') => u16::from(b'\t'),
            _ => return false,
        };
        self.set_from(self.set_pos, &[replacement]);
        self.set_pos += 1;
        self.move_tail(self.set_pos, self.pos + 2);
        self.size -= 2;
        self.trunc(self.size);
        true
    }
    fn run(&mut self, to_conversion: bool) -> Result<(), Failure> {
        self.pos = 1;
        while self.pos <= self.size {
            let unit = self.at(self.pos);
            self.set_pos = self.pos;
            if self.try_directive()? {
                self.apply_directive(to_conversion);
                self.pos += 1;
                continue;
            }
            if self.doubled_or_control(unit) {
                self.pos += 1;
                continue;
            }
            if unit == u16::from(b'\n') || unit == u16::from(b'\r') {
                self.move_tail(self.set_pos, self.pos);
                self.size -= 1;
                self.trunc(self.size);
                self.pos -= 1;
            }
            self.pos += 1;
        }
        Ok(())
    }
}

impl StepDataStepReaderData {
    /// `cleanText`: the text of a quoted parameter (with its quotes).
    pub(super) fn clean_text(&self, value: &[u8]) -> Result<Handle<HAsciiString>, Failure> {
        if value.len() == 2 {
            return Ok(Handle::new(HAsciiString::new(b"")));
        }
        let mut converted = ExtendedString::default();
        let body = value.get(1..).unwrap_or_default();
        ResourceUnicode::convert_format_to_unicode(self.my_source_code_page, body, &mut converted)?;
        let units = units_of(&converted);
        let mut cleaner = TextCleaner {
            size: units.len() as i32 - 1,
            units,
            pos: 1,
            set_pos: 1,
            temp: Vec::new(),
            local_format: FormatType::Iso8859_1,
            check: &self.thecheck,
        };
        cleaner.run(self.my_source_code_page != FormatType::NoConversion)?;
        let size = cleaner.size;
        cleaner.trunc(size);
        let text = ExtendedString::from_raw(&cleaner.units);
        Ok(Handle::new(HAsciiString::new(text.to_utf8())))
    }
}
