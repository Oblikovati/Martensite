//! `StepDataEnumTool`: the conversion between an enumeration value and its dotted text.
use martensite_tkernel::tcollection::AsciiString;

/// `StepDataEnumTool`. The constructor's forty optional definitions are a slice.
///
/// ```
/// use martensite_tkdestep::step_data::StepDataEnumTool;
/// let tool = StepDataEnumTool::new(&["ALPHA", ".BETA."]);
/// assert_eq!(tool.value(b".BETA."), 1);
/// assert_eq!(tool.null_value(), -1);
/// assert_eq!(tool.text(0).as_bytes(), b".ALPHA.");
/// ```
#[derive(Clone, Debug, Default)]
pub struct StepDataEnumTool {
    thetexts: Vec<AsciiString>,
    theinit: i32,
    theopt: bool,
}

/// Closes a term with its final dot, when absent.
fn close_term(term: &mut Vec<u8>) {
    if term.last() != Some(&b'.') {
        term.push(b'.');
    }
}

impl StepDataEnumTool {
    /// `StepDataEnumTool(e0, e1...)`.
    pub fn new(definitions: &[&str]) -> Self {
        let mut tool = Self::default();
        for definition in definitions {
            tool.add_definition(definition);
        }
        tool.theinit = tool.thetexts.len() as i32;
        tool.theopt = true;
        tool
    }
    /// `add_definition(term)`: splits at blanks; dots are added when absent. Quirks
    /// kept: `$` is recorded as `$.` (so `null_value` never finds it, see the unit test), and
    /// the blank that ends a term starts the next one, as `. NEXT.`.
    pub fn add_definition(&mut self, term: &str) {
        if term.is_empty() {
            return;
        }
        let mut text: Vec<u8> = Vec::new();
        for &byte in term.as_bytes() {
            // A signed char: the bytes from 0x80 compare below 32.
            if (byte as i8) <= 32 {
                if text.is_empty() {
                    continue;
                }
                if text.len() > 1 || text[0] != b'$' {
                    close_term(&mut text);
                }
                self.thetexts.push(AsciiString::from_bytes(&text));
                text.clear();
            }
            if text.is_empty() && byte != b'.' && byte != b'$' {
                text.push(b'.');
            }
            text.push(byte);
        }
        close_term(&mut text);
        self.thetexts.push(AsciiString::from_bytes(&text));
    }
    /// `is_set`: a definition was added after the creation.
    pub fn is_set(&self) -> bool {
        self.thetexts.len() as i32 > self.theinit
    }
    /// `max_value`.
    pub fn max_value(&self) -> i32 {
        self.thetexts.len() as i32 - 1
    }
    /// `optional(mode)`.
    pub fn optional(&mut self, mode: bool) {
        self.theopt = mode;
    }
    /// `null_value`: the rank of `$`, -1 when none. Not optional gives
    /// `false`, read as 0.
    pub fn null_value(&self) -> i32 {
        if self.theopt { self.value(b"$") } else { 0 }
    }
    /// `Text(num)`: empty out of range.
    pub fn text(&self, num: i32) -> AsciiString {
        usize::try_from(num)
            .ok()
            .and_then(|rank| self.thetexts.get(rank).cloned())
            .unwrap_or_default()
    }
    /// `Value(txt)`: the rank of the exact text, -1 when none.
    pub fn value(&self, txt: &[u8]) -> i32 {
        self.thetexts
            .iter()
            .position(|text| text.as_bytes() == txt)
            .map_or(-1, |rank| rank as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dollar_definition_is_recorded_with_a_dot() {
        let tool = StepDataEnumTool::new(&["$"]);
        assert_eq!(tool.text(0).as_bytes(), b"$.");
        assert_eq!(tool.null_value(), -1);
    }

    #[test]
    fn blank_starts_the_next_term() {
        let tool = StepDataEnumTool::new(&["A B"]);
        assert_eq!(tool.text(0).as_bytes(), b".A.");
        assert_eq!(tool.text(1).as_bytes(), b". B.");
    }

    #[test]
    fn not_optional_null_value_reads_zero() {
        let mut tool = StepDataEnumTool::new(&["A"]);
        tool.optional(false);
        assert_eq!(tool.null_value(), 0);
        assert!(!tool.is_set());
        tool.add_definition("B");
        assert!(tool.is_set());
        assert_eq!(tool.max_value(), 1);
        assert!(tool.text(9).is_empty());
    }
}
