use super::*;
use crate::top_ope_brep::TopOpeBRepFFTransitionTool;
use crate::top_ope_brep_tool::TopOpeBRepToolShapeTool;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkg3d::top_abs::shape_orientation_to_string;
impl TopOpeBRepFFDumperView<'_, '_> {
    /// Prints a kept or discarded point in debug mode; `dumper.dump_vp(&point, &mut output)?`.
    pub fn dump_vp(
        &self,
        point: &TopOpeBRepVPointInter<'_>,
        output: &mut impl Write,
    ) -> Result<(), Failure> {
        if !self.cache.debug {
            return Ok(());
        }
        point_heading(point, output)?;
        emit_point(point, self.cache.line_index, output, "; #draw\n")?;
        for operand in [1, 2] {
            if point.shape_index() == operand || point.shape_index() == 3 {
                self.dump_vp_on_surface(point, operand, output)?;
            }
        }
        Ok(())
    }
    /// Prints the source's repeated boundary-edge diagnostic; `dumper.dump_vp_on_surface(&point, 1, &mut output)?`.
    pub fn dump_vp_on_surface(
        &self,
        point: &TopOpeBRepVPointInter<'_>,
        operand: i32,
        output: &mut impl Write,
    ) -> Result<(), Failure> {
        if !self.cache.debug {
            return Ok(());
        }
        let label = self.boundary_label(point, operand)?;
        if point.is_vertex(operand)? {
            emit(output, format_args!("is vertex of {operand}\n"))?;
        }
        emit(
            output,
            format_args!(
                "{label} : par : {}\n{label}",
                real(point.edge_parameter(operand)?)
            ),
        )
    }
    /// Prints geometry identity and its stored tolerance; `dumper.dump_dsp(&point, kind, index, true, &mut output)?`.
    pub fn dump_dsp(
        &self,
        point: &TopOpeBRepVPointInter<'_>,
        kind: TopOpeBRepDSKind,
        index: i32,
        is_new: bool,
        output: &mut impl Write,
    ) -> Result<(), Failure> {
        if !self.cache.debug {
            return Ok(());
        }
        let action = if is_new { "gives" } else { "equals" };
        let label = match kind {
            TopOpeBRepDSKind::Vertex => "DSV",
            TopOpeBRepDSKind::Point => "DSP",
            _ => "DS???",
        };
        emit(
            output,
            format_args!(
                "VP {} on {} {action} new {label} {index}",
                point.index(),
                point.shape_index()
            ),
        )?;
        let tolerance = self.stored_tolerance(kind, index)?;
        emit(output, format_args!(" tol = {}\n", real(tolerance)))
    }
    fn boundary_label(
        &self,
        point: &TopOpeBRepVPointInter<'_>,
        operand: i32,
    ) -> Result<String, Failure> {
        let filler = self.require_filler()?;
        let shape = point.edge(operand)?;
        let edge = topods::edge(&shape)?;
        let orientation = edge.orientation();
        let _ = TopOpeBRepFFTransitionTool::process_line_transition(point, operand, orientation)?;
        let prefix = edge_prefix(&shape, filler.face(operand)?.as_ref())?;
        let index = self.explore_index(&shape, operand)?;
        let hds = filler.hdata_structure();
        let storage = hds.get().ok_or_else(missing_storage)?;
        let ds = storage.shape_index(&shape, true);
        Ok(format!(
            "{prefix} (ds{ds}) (ex{index}) of face of {operand}"
        ))
    }
    fn stored_tolerance(&self, kind: TopOpeBRepDSKind, index: i32) -> Result<f64, Failure> {
        let hds = self.require_filler()?.hdata_structure();
        let storage = hds.get().ok_or_else(missing_storage)?;
        match kind {
            TopOpeBRepDSKind::Vertex => {
                BRepTool::vertex_tolerance(&topods::vertex(&storage.shape(index, true))?)
            }
            TopOpeBRepDSKind::Point => Ok(storage.point(index)?.tolerance()),
            _ => Ok(martensite_tkernel::precision::confusion()),
        }
    }
}
pub(super) fn emit_point(
    point: &TopOpeBRepVPointInter<'_>,
    line: i32,
    output: &mut impl Write,
    suffix: &str,
) -> Result<(), Failure> {
    let position = point.value()?;
    emit(
        output,
        format_args!(
            "dinp L{line}P{}{} {} {} {}{suffix}",
            point.index(),
            if point.keep() { "K" } else { "" },
            real(position.x()),
            real(position.y()),
            real(position.z())
        ),
    )
}
fn edge_prefix(shape: &Shape, face: &Shape) -> Result<String, Failure> {
    let closed = TopOpeBRepToolShapeTool::closed(shape, face)?;
    let edge = topods::edge(shape)?;
    let degenerate = BRepTool::degenerated(&edge)?;
    let closing = if closed {
        "on closing edge "
    } else {
        "on edge "
    };
    let degenerate = if degenerate {
        " on degenerated edge "
    } else {
        ""
    };
    let orientation = shape_orientation_to_string(edge.orientation());
    Ok(format!("{closing}{degenerate}{orientation}"))
}
fn missing_storage() -> Failure {
    Failure::with_message(
        "Standard_NullObject",
        "FFDumper: storage=None; expected bound HDataStructure",
    )
}

fn point_heading(
    point: &TopOpeBRepVPointInter<'_>,
    output: &mut impl Write,
) -> Result<(), Failure> {
    let discarded = if point.keep() { "" } else { " NOT kept" };
    let parameter = real(point.parameter_on_line()?);
    emit(
        output,
        format_args!(
            "VP {} on {} : on curve : {parameter}{discarded}\n",
            point.index(),
            point.shape_index()
        ),
    )
}
