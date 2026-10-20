//! One complete multi-shape algorithm replay over retained native operation fields.
use super::common::{Sink, attempt, end};
use super::geometry_records::common::Reader;
use super::{brep_algo_report, brep_curve_fixture::projector, brep_shape_fixture::shape};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkhlr::{hlr_algo::Projector, hlr_brep::Algo};

fn shapes(reader: &mut Reader, count: i32) -> Result<Vec<Shape>, Failure> {
    let mut inputs = Vec::new();
    for _ in 0..count {
        inputs.push(shape(reader)?);
    }
    Ok(inputs)
}
fn visibility(tokens: &mut Sink, algorithm: &Algo, count: i32, mode: i32) -> Result<(), Failure> {
    match mode {
        0 => attempt(tokens, |_| algorithm.hide()),
        1 => attempt(tokens, |_| algorithm.partial_hide()),
        _ => {
            attempt(tokens, |_| algorithm.hide_shape(1));
            if count > 1 {
                attempt(tokens, |_| algorithm.hide_shapes(1, 2));
            }
            algorithm.select()?;
        }
    }
    Ok(())
}
fn evaluate(
    tokens: &mut Sink,
    view: &Projector,
    inputs: &[Shape],
    nb_iso: i32,
    mode: i32,
    count: i32,
) -> Result<(), Failure> {
    let handle = Handle::new(Algo::new()?);
    let algorithm = handle.get().ok_or_else(|| Failure::new("no algo"))?;
    for source in inputs {
        algorithm.add(source, nb_iso);
    }
    algorithm.set_projector(view);
    algorithm.update()?;
    brep_algo_report::bounds(tokens, algorithm, inputs)?;
    visibility(tokens, algorithm, count, mode)?;
    brep_algo_report::results(tokens, &handle, &inputs[0]);
    Ok(())
}
fn algo_body(tokens: &mut Sink, reader: &mut Reader, view: &Projector) -> Result<(), Failure> {
    let count = reader.integer();
    let inputs = shapes(reader, count)?;
    let (nb_iso, mode) = (reader.integer(), reader.integer());
    evaluate(tokens, view, &inputs, nb_iso, mode, count)
}
/// Replay exact projector/shapes/isoline/mode fields; `algo_case(tokens, fields)`.
pub fn algo_case(tokens: &mut Sink, fields: &[f64]) {
    let mut reader = Reader::new(fields);
    attempt(tokens, |tokens| {
        let view = projector(&mut reader)?;
        attempt(tokens, |tokens| algo_body(tokens, &mut reader, &view));
        Ok(())
    });
    end(tokens);
}
