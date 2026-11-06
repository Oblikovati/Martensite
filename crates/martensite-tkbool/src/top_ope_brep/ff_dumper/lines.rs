use super::*;
use crate::top_ope_brep::{TopOpeBRep, TopOpeBRepTypeLineCurve};
const RULE: &str = "--------------------------------------------------";
impl TopOpeBRepFFDumperView<'_, '_> {
    /// Dumps the indexed prepared line; release mode does not inspect the index; `dumper.dump_line_index(1, &mut output)?`.
    pub fn dump_line_index(&mut self, index: i32, output: &mut impl Write) -> Result<(), Failure> {
        if !self.cache.debug {
            return Ok(());
        }
        let filler = self.require_filler()?;
        let pair = filler
            .p_faces_intersector_dummy()
            .ok_or_else(missing_pair)?;
        let line = pair.change_line(index)?;
        self.dump_line(&line, output)
    }
    /// Dumps flags, all point drawings and kept-point details; `dumper.dump_line(&line, &mut output)?`.
    pub fn dump_line(
        &mut self,
        line: &TopOpeBRepLineInter,
        output: &mut impl Write,
    ) -> Result<(), Failure> {
        if !self.cache.debug {
            return Ok(());
        }
        self.cache.line_index = line.index();
        self.line_heading(line, output)?;
        self.line_flags(line, output)?;
        self.line_points(line, output)?;
        self.line_direction(line, output)?;
        line.dump_line_transitions_debug(output)?;
        emit(output, format_args!("\n{RULE}\n"))
    }
    fn line_heading(
        &self,
        line: &TopOpeBRepLineInter,
        output: &mut impl Write,
    ) -> Result<(), Failure> {
        let pair = self
            .require_filler()?
            .p_faces_intersector_dummy()
            .ok_or_else(missing_pair)?;
        emit(
            output,
            format_args!("\n{RULE}\nline {}/{} is a ", line.index(), pair.nb_lines()),
        )?;
        TopOpeBRep::print(line.type_line_curve(), output).map_err(io_failure)?;
        self.restriction_heading(line, output)?;
        emit(output, format_args!("\n"))
    }
    fn restriction_heading(
        &self,
        line: &TopOpeBRepLineInter,
        output: &mut impl Write,
    ) -> Result<(), Failure> {
        if line.type_line_curve() != TopOpeBRepTypeLineCurve::Restriction {
            return Ok(());
        }
        let label = if line.arc_is_edge(1)? {
            " of 1"
        } else if line.arc_is_edge(2)? {
            " of 2"
        } else {
            "of 0(BUG)"
        };
        emit(output, format_args!("{label}\n++++ line restriction"))?;
        self.restriction_label(line, output)
    }
    fn restriction_label(
        &self,
        line: &TopOpeBRepLineInter,
        output: &mut impl Write,
    ) -> Result<(), Failure> {
        let filler = self.require_filler()?;
        let edge = line.arc()?;
        if !filler
            .p_faces_intersector_dummy()
            .ok_or_else(missing_pair)?
            .is_restriction(&edge)
        {
            return Ok(());
        }
        let hds = filler.hdata_structure();
        let storage = hds.get().ok_or_else(|| {
            Failure::with_message(
                "Standard_NullObject",
                "FFDumper: storage=None; expected bound HDataStructure",
            )
        })?;
        emit(
            output,
            format_args!(" edge restriction {}", storage.shape_index(&edge, true)),
        )
    }
    fn line_flags(
        &self,
        line: &TopOpeBRepLineInter,
        output: &mut impl Write,
    ) -> Result<(), Failure> {
        emit(
            output,
            format_args!(
                "has {}vertex on restriction\nis {}closed by vertices\nis {}periodic\n",
                if line.has_vp_on_r() { "" } else { "no " },
                if line.is_v_closed() { "" } else { "not " },
                if line.is_periodic() { "" } else { "not " }
            ),
        )
    }
    fn line_points(
        &self,
        line: &TopOpeBRepLineInter,
        output: &mut impl Write,
    ) -> Result<(), Failure> {
        if line.nb_vpoint() > 0 {
            emit(output, format_args!("\n"))?;
        }
        for index in 1..=line.nb_vpoint() {
            let point = line.vpoint(index)?;
            points::emit_point(
                point,
                line.index(),
                output,
                &format!("; #draw {}\n", real(point.tolerance()?)),
            )?;
        }
        if line.nb_vpoint() > 0 {
            emit(output, format_args!("\n"))?;
        }
        for index in 1..=line.nb_vpoint() {
            let point = line.vpoint(index)?;
            if point.keep() {
                self.dump_vp(point, output)?;
                emit(output, format_args!("\n"))?;
            }
        }
        Ok(())
    }
    fn line_direction(
        &self,
        line: &TopOpeBRepLineInter,
        output: &mut impl Write,
    ) -> Result<(), Failure> {
        if line.type_line_curve() != TopOpeBRepTypeLineCurve::Line {
            return Ok(());
        }
        let direction = line
            .line_g()
            .get()
            .ok_or_else(|| {
                Failure::with_message(
                    "Standard_NullObject",
                    "FFDumper: geometric line=None; expected LINE source",
                )
            })?
            .line()?
            .direction();
        emit(
            output,
            format_args!(
                "LINE dir : {} {} {}\n",
                real(direction.x()),
                real(direction.y()),
                real(direction.z())
            ),
        )
    }
}
fn missing_pair() -> Failure {
    Failure::with_message(
        "Standard_NullObject",
        "FFDumper: intersector=None; expected bound face intersection",
    )
}
fn io_failure(error: std::io::Error) -> Failure {
    Failure::with_message(
        "Standard_Failure",
        &format!("FFDumper: output error={error}; expected writable stream"),
    )
}
