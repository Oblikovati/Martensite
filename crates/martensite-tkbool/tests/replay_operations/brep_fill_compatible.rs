//! BRepFillCompatibleWires probe operations, the Rust twin of
//! the recorded capture (the record is documented there).
use super::brep_fill_generator::{failed, list};
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shape::shape;
use super::shapes::{TpShapes, tp_shapes};
use martensite_tkbool::brep_fill::BRepFillCompatibleWires;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{Edge, Shape};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

/// The sections named by the request.
fn sections_of(r: &mut Reader<'_>, t: &TpShapes) -> Result<Vec<Shape>, Failure> {
    let n = r.integer();
    let mut sections = Vec::new();
    for _ in 0..n {
        let index = r.integer() as usize;
        sections.push(
            t.shapes
                .get(index)
                .cloned()
                .ok_or_else(|| Failure::out_of_range("bl_bf_compatible: section index"))?,
        );
    }
    Ok(sections)
}

/// `bf_compatible program n (wire)... percent rotation`.
pub fn compatible_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    let sections = match sections_of(&mut r, &t) {
        Ok(sections) => sections,
        Err(error) => return failed(output, error),
    };
    let mut compatible = BRepFillCompatibleWires::with_sections(&sections);
    compatible.set_percent(r.real());
    let rotation = r.flag();
    if let Err(error) = compatible.perform(rotation) {
        return failed(output, error);
    }
    output.int(compatible.get_status() as i32);
    output.flag(compatible.is_done());
    output.flag(compatible.is_degenerated_first_section());
    output.flag(compatible.is_degenerated_last_section());
    output.int(compatible.shape().len() as i32);
    let mut all = IndexedMapOfShape::default();
    for s in compatible.shape() {
        output.attempt(|| shape(output, s));
        if !s.is_null() {
            topexp::map_shapes(s, &mut all, true, true);
        }
    }
    output.int(compatible.generated().extent() as i32);
    for s in &t.shapes {
        if s.is_null() || s.shape_type().ok() != Some(ShapeEnum::Edge) {
            continue;
        }
        output.attempt(|| {
            list(
                output,
                &all,
                compatible.generated_shapes(&Edge::unchecked(s.clone())),
            )
        });
    }
    output.end();
}
