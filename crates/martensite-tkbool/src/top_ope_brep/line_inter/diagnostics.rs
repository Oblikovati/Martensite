use super::super::TopOpeBRepBipoint;
use super::*;
use std::io::{self, Write};

impl TopOpeBRepLineInter {
    /// Mirrors release-mode DumpType's intentionally empty output; `line.dump_type()`.
    pub fn dump_type(&self) {}
    /// Mirrors release-mode DumpVPoint's intentionally empty output; `line.dump_vpoint(1, "", "")`.
    pub fn dump_vpoint(&self, _index: i32, _prefix: &str, _suffix: &str) {}
    /// Mirrors release-mode DumpBipoint's intentionally empty output; `line.dump_bipoint(&pair, "", "")`.
    pub fn dump_bipoint(&self, _pair: &TopOpeBRepBipoint, _prefix: &str, _suffix: &str) {}
    /// Preserves the release-mode output stream unchanged; `line.dump_line_transitions(&mut output)?`.
    pub fn dump_line_transitions<'a, W: Write>(&self, output: &'a mut W) -> io::Result<&'a mut W> {
        Ok(output)
    }
}

impl TopOpeBRepLineInter {
    /// Writes debug-only source face orientations; `line.dump_line_transitions_debug(&mut output)?`.
    pub fn dump_line_transitions_debug(&self, output: &mut impl Write) -> Result<(), Failure> {
        use martensite_tkg3d::top_abs::shape_orientation_to_string;
        write!(
            output,
            "transition from f1 / f2 {}transition from f2 / f1 {}",
            shape_orientation_to_string(self.faces[1].orientation()),
            shape_orientation_to_string(self.faces[0].orientation())
        )
        .map_err(|error| {
            Failure::with_message(
                "Standard_Failure",
                &format!("LineInter: output error={error}; expected writable stream"),
            )
        })
    }
}
