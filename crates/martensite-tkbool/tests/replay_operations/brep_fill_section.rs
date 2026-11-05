//! BRepFillSection and BRepFillSectionPlacement probe operations, the Rust twin of
//! the recorded capture (the records are documented there).
use super::brep_fill_generator::{failed, indexed};
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shape::shape;
use super::shapes::{TpShapes, tp_shape_index, tp_shapes};
use martensite_tkbool::brep_fill::{
    BRepFillEdge3DLaw, BRepFillLocationLaw, BRepFillSection, BRepFillSectionPlacement,
};
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{self, Shape, Vertex};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkgeomalgo::geom_fill::{
    GeomFillCorrectedFrenet, GeomFillCurveAndTrihedron, GeomFillLocationLaw, GeomFillTrihedronLaw,
};
use martensite_tkmath::gp::Transform3;

/// The shape of the program at a table index; a null shape for -1.
fn at(t: &TpShapes, index: i32) -> Result<Shape, Failure> {
    if index < 0 {
        return Ok(Shape::default());
    }
    t.shapes
        .get(index as usize)
        .cloned()
        .ok_or_else(|| Failure::out_of_range("replay_operations: shape index"))
}

/// The section of a request.
fn section_of(r: &mut Reader<'_>, t: &TpShapes) -> Result<BRepFillSection, Failure> {
    let profile = at(t, r.integer())?;
    let v: Vertex = topods::vertex(&at(t, r.integer())?)?;
    let (contact, correction) = (r.flag(), r.flag());
    BRepFillSection::with_profile(&profile, &v, contact, correction)
}

/// `bf_section program profile vertex contact correction`.
pub fn section_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    let mut section = match section_of(&mut r, &t) {
        Ok(section) => section,
        Err(error) => return failed(output, error),
    };
    section.set(true);
    output.flag(section.is_law());
    output.flag(section.is_punctual());
    output.flag(section.with_contact());
    output.flag(section.with_correction());
    output.int(tp_shape_index(&t, section.original_shape()));
    output.int(tp_shape_index(&t, section.vertex()));
    output.attempt(|| shape(output, section.wire()));
    let mut m = IndexedMapOfShape::default();
    topexp::map_shapes(section.wire(), &mut m, true, true);
    for s in &t.shapes {
        output.attempt(|| indexed(output, &m, &section.modified_shape(s)?));
    }
    output.end();
}

/// `bl_trsf`.
fn trsf(output: &BufferedTokens, t: &Transform3) -> Result<(), Failure> {
    for i in 1..=3 {
        for j in 1..=4 {
            output.real(t.value(i, j)?);
        }
    }
    Ok(())
}

/// The placement queries of a request.
fn placement(output: &BufferedTokens, r: &mut Reader<'_>, t: &TpShapes) -> Result<(), Failure> {
    let path = topods::wire(&at(t, r.integer())?)?;
    let section = at(t, r.integer())?;
    let v = at(t, r.integer())?;
    let (contact, correction) = (r.flag(), r.flag());
    let tri: Handle<dyn GeomFillTrihedronLaw> = Handle::new(GeomFillCorrectedFrenet::new()).cast();
    let model: Handle<dyn GeomFillLocationLaw> =
        Handle::new(GeomFillCurveAndTrihedron::new(&tri)).cast();
    let law: Handle<dyn BRepFillLocationLaw> =
        Handle::new(BRepFillEdge3DLaw::new(&path, &model)?).cast();
    let place = BRepFillSectionPlacement::with_vertex(&law, &section, &v, contact, correction)?;
    trsf(output, place.transformation())?;
    output.attempt(|| {
        output.real(place.abscissa_on_path()?);
        Ok(())
    });
    Ok(())
}

/// `bf_section_placement program path section vertex contact correction`.
pub fn section_placement_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    if let Err(error) = placement(output, &mut r, &t) {
        output.emit(format_args!("{} ", error.exception_type()));
    }
    output.end();
}
