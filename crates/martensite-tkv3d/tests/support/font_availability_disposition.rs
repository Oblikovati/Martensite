//! Availability records distinguish an original's early return from its gated assertions.
//! They describe branch execution only, never the font face that supplied a glyph.

#[derive(Clone, Copy)]
pub(super) enum FontAvailabilityGate {
    FontHandle,
    FontPair,
    CacheFontHandle,
    NamedInitialization,
    RepeatedGlyphPlan,
}

impl FontAvailabilityGate {
    fn text(self) -> &'static str {
        match self {
            Self::FontHandle => "font-handle",
            Self::FontPair => "font-pair",
            Self::CacheFontHandle => "cache-font-handle",
            Self::NamedInitialization => "named-initialization",
            Self::RepeatedGlyphPlan => "repeated-glyph-plan",
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum FontAvailabilityDisposition {
    AssertionsEntered,
    Skipped,
}

impl FontAvailabilityDisposition {
    fn text(self) -> &'static str {
        match self {
            Self::AssertionsEntered => "assertions-entered",
            Self::Skipped => "skipped",
        }
    }
}

pub(super) struct FontAvailabilityRecord {
    pub(super) module: &'static str,
    pub(super) case: &'static str,
    pub(super) gate: FontAvailabilityGate,
    pub(super) disposition: FontAvailabilityDisposition,
}

impl FontAvailabilityRecord {
    fn json_line(&self) -> String {
        // Only compiler identifier/module tokens and closed enum spellings reach this formatter.
        format!(
            "{{\"schema\":\"martensite-test-availability-v1\",\"module\":\"{}\",\"case\":\"{}\",\"gate\":\"{}\",\"disposition\":\"{}\"}}",
            self.module,
            self.case,
            self.gate.text(),
            self.disposition.text(),
        )
    }

    pub(super) fn emit(self) {
        let line: String = self.json_line();
        eprintln!("{line}");
    }
}

// Explicit tokens keep case identity independent of thread names and avoid rechecking the gate.
macro_rules! font_gate {
    ($case:ident => $gate:ident, $disposition:ident) => {
        $crate::font_availability_disposition::FontAvailabilityRecord {
            module: module_path!(),
            case: stringify!($case),
            gate: $crate::font_availability_disposition::FontAvailabilityGate::$gate,
            disposition:
                $crate::font_availability_disposition::FontAvailabilityDisposition::$disposition,
        }
        .emit()
    };
}
pub(crate) use font_gate;

#[cfg(test)]
mod tests {
    use super::{FontAvailabilityDisposition, FontAvailabilityGate, FontAvailabilityRecord};

    #[test]
    fn entered_record_retains_case_and_gate() {
        let record = FontAvailabilityRecord {
            module: "owned_control",
            case: "available",
            gate: FontAvailabilityGate::FontHandle,
            disposition: FontAvailabilityDisposition::AssertionsEntered,
        };
        assert_eq!(
            record.json_line(),
            concat!(
                "{\"schema\":\"martensite-test-availability-v1\",\"module\":\"owned_control\",",
                "\"case\":\"available\",\"gate\":\"font-handle\",\"disposition\":\"assertions-entered\"}",
            )
        );
        record.emit();
    }

    #[test]
    fn skipped_record_keeps_unavailable_disposition() {
        let record = FontAvailabilityRecord {
            module: "owned_control",
            case: "unavailable",
            gate: FontAvailabilityGate::RepeatedGlyphPlan,
            disposition: FontAvailabilityDisposition::Skipped,
        };
        assert_eq!(
            record.json_line(),
            concat!(
                "{\"schema\":\"martensite-test-availability-v1\",\"module\":\"owned_control\",",
                "\"case\":\"unavailable\",\"gate\":\"repeated-glyph-plan\",\"disposition\":\"skipped\"}",
            )
        );
        record.emit();
    }

    const EXPECTED_GATE_SPELLINGS: [&str; 5] = [
        "font-handle",
        "font-pair",
        "cache-font-handle",
        "named-initialization",
        "repeated-glyph-plan",
    ];

    #[test]
    fn gate_spellings_keep_distinct_case_predicates() {
        let gates = [
            FontAvailabilityGate::FontHandle,
            FontAvailabilityGate::FontPair,
            FontAvailabilityGate::CacheFontHandle,
            FontAvailabilityGate::NamedInitialization,
            FontAvailabilityGate::RepeatedGlyphPlan,
        ];
        assert_eq!(
            gates.map(FontAvailabilityGate::text),
            EXPECTED_GATE_SPELLINGS
        );
    }

    #[test]
    fn macro_records_its_explicit_call_site() {
        let caller_module = module_path!();
        assert_eq!(
            caller_module,
            "toolkit::font_availability_disposition::tests"
        );
        font_gate!(macro_records_its_explicit_call_site => FontPair, AssertionsEntered);
    }
}
