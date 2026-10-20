//! Complete reflection request body; native `tkhlr_probe_hlrbrep_algo`.
use super::common::{Sink, attempt, end};
use super::geometry_records::common::Reader;
use super::{brep_record_print::compound, brep_shape_fixture::shape};
use martensite_tkernel::standard::Failure;
use martensite_tkhlr::{hlr_appli::ReflectLines, hlr_brep::TypeOfResultingEdge};

fn reflect_body(tokens: &mut Sink, reader: &mut Reader) -> Result<(), Failure> {
    let source = shape(reader)?;
    let mut axes = [0.; 9];
    axes.iter_mut().for_each(|value| *value = reader.real());
    let mut lines = ReflectLines::new(&source)?;
    lines.set_axes(
        axes[0], axes[1], axes[2], axes[3], axes[4], axes[5], axes[6], axes[7], axes[8],
    )?;
    lines.perform()?;
    reflection_results(tokens, &lines);
    Ok(())
}
fn reflection_results(tokens: &mut Sink, lines: &ReflectLines) {
    attempt(tokens, |tokens| compound(tokens, &lines.get_result()?));
    for visible in [true, false] {
        attempt(tokens, |tokens| {
            compound(
                tokens,
                &lines.get_compound_of_3d_edges(TypeOfResultingEdge::Sharp, visible, true)?,
            )
        });
    }
}
/// Replay the complete shape/axes operation; `reflect_case(tokens, fields)`.
pub fn reflect_case(tokens: &mut Sink, fields: &[f64]) {
    let mut reader = Reader::new(fields);
    attempt(tokens, |tokens| reflect_body(tokens, &mut reader));
    end(tokens);
}
