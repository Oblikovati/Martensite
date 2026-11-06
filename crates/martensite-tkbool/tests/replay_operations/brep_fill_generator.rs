//! BRepFillGenerator probe operations, the Rust twin of
//! the recorded capture (the records are documented there).
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shape::shape;
use super::shapes::{TpShapes, tp_shapes};
use martensite_tkbool::brep_fill::BRepFillGenerator;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;

/// Prints the exception of a failed construction and ends the line.
pub fn failed(output: &BufferedTokens, error: Failure) {
    output.emit(format_args!("{} ", error.exception_type()));
    output.end();
}

/// `bl_indexed`.
pub fn indexed(
    output: &BufferedTokens,
    result: &IndexedMapOfShape,
    s: &Shape,
) -> Result<(), Failure> {
    if s.is_null() {
        output.int(-1);
        return Ok(());
    }
    let i = result.find_index(s) as i32;
    output.int(i);
    output.int(s.orientation() as i32);
    if i == 0 {
        shape(output, s)?;
    }
    Ok(())
}

/// `bl_list`.
pub fn list(
    output: &BufferedTokens,
    result: &IndexedMapOfShape,
    l: &List<Shape>,
) -> Result<(), Failure> {
    output.int(l.len() as i32);
    for s in l.iter() {
        indexed(output, result, s)?;
    }
    Ok(())
}

/// The generator on the wires named by the request, with its input mode set.
fn generator_of(r: &mut Reader<'_>, t: &TpShapes) -> Result<BRepFillGenerator, Failure> {
    let mut generator = BRepFillGenerator::new();
    let n = r.integer();
    for _ in 0..n {
        let index = r.integer() as usize;
        let s = t
            .shapes
            .get(index)
            .cloned()
            .ok_or_else(|| Failure::out_of_range("bl_bf_generator: wire index"))?;
        generator.add_wire(&topods::wire(&s)?);
    }
    generator.set_mutable_input(r.flag());
    Ok(generator)
}

/// `bf_generator program n (wire)... mutable`.
pub fn generator_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    let mut generator = match generator_of(&mut r, &t) {
        Ok(generator) => generator,
        Err(error) => return failed(output, error),
    };
    output.flag(generator.is_mutable_input());
    if let Err(error) = generator.perform() {
        return failed(output, error);
    }
    output.int(generator.get_status() as i32);
    let shell: Shape = generator.shell().as_ref().clone();
    output.attempt(|| shape(output, &shell));
    let mut result = IndexedMapOfShape::default();
    if !shell.is_null() {
        topexp::map_shapes(&shell, &mut result, true, true);
    }
    output.int(generator.generated().extent() as i32);
    output.attempt(|| {
        list(
            output,
            &result,
            generator.generated_shapes(&Shape::default()),
        )
    });
    for s in &t.shapes {
        output.attempt(|| list(output, &result, generator.generated_shapes(s)));
        output.attempt(|| indexed(output, &result, &generator.result_shape(s)?));
    }
    output.end();
}
