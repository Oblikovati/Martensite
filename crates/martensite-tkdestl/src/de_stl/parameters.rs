//! Format settings; integer-backed modes retain configured values.

/// Internal settings of the STL transfer configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct DeStlInternalSection {
    /// Resource `read.merge.angle`; default `90.0`.
    pub read_merge_angle: f64,
    /// Resource `read.brep`; default `false`.
    pub read_brep: bool,
    /// Resource `write.ascii`; default `true`.
    pub write_ascii: bool,
}

impl Default for DeStlInternalSection {
    fn default() -> Self {
        Self {
            read_merge_angle: 90.0,
            read_brep: false,
            write_ascii: true,
        }
    }
}

use martensite_tkernel::tcollection::AsciiString;

impl DeStlInternalSection {
    pub(super) fn resource_bindings(&self) -> [(&'static str, AsciiString); 3] {
        let value_0 = AsciiString::from_real(self.read_merge_angle);
        let value_1 = AsciiString::from_integer(i32::from(self.read_brep));
        let value_2 = AsciiString::from_integer(i32::from(self.write_ascii));
        [
            ("ReadMergeAngle", value_0),
            ("ReadBRep", value_1),
            ("WriteAscii", value_2),
        ]
    }
}
