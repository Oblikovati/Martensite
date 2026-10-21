//! Constructor, setter, handle and copied-limit behavior of Contap search records.
use super::common::{Sink, attempt, end, flag, pnt, real, show};
use super::geometry_records::common::Reader;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{adaptor3d::Adaptor3dHVertexBase, top_abs::Orientation};
use martensite_tkhlr::contap::{
    ContapThePathPointOfTheSearch as Point, ContapTheSegmentOfTheSearch as Segment,
};
fn path_point(tokens: &mut Sink, point: &Point) {
    pnt(tokens, point.value());
    real(tokens, point.tolerance());
    flag(tokens, point.is_new());
    flag(tokens, point.arc().is_null());
    real(tokens, point.parameter());
    show(tokens, point.vertex(), |tokens, vertex| {
        flag(tokens, vertex.is_null())
    });
}
fn segment(tokens: &mut Sink, segment: &Segment) {
    flag(tokens, segment.curve().is_null());
    flag(tokens, segment.has_first_point());
    flag(tokens, segment.has_last_point());
    show(tokens, segment.first_point(), path_point);
    show(tokens, segment.last_point(), path_point);
}
struct RecordFixture {
    arc: martensite_tkgeomalgo::int_start::TheArc,
    vertex: martensite_tkgeomalgo::int_start::TheVertex,
    position: martensite_tkmath::gp::Point3,
    tolerance: f64,
    parameter: f64,
}

fn read_fixture(
    reader: &mut Reader,
) -> Result<RecordFixture, martensite_tkernel::standard::Failure> {
    let arc = super::hcurve::arc(reader)?;
    let position = reader.pnt();
    let tolerance = reader.real();
    let parameter = reader.real();
    let vertex = Handle::new(Adaptor3dHVertexBase::new(
        Default::default(),
        Orientation::Forward,
        tolerance,
    ))
    .cast();
    Ok(RecordFixture {
        arc,
        vertex,
        position,
        tolerance,
        parameter,
    })
}

/// Check every record member on each arc; `records_case(tokens, numbers)`.
pub fn records_case(tokens: &mut Sink, values: &[f64]) {
    let mut reader = Reader::new(values);
    attempt(tokens, |tokens| {
        let fixture = read_fixture(&mut reader)?;
        let (mut point, mut segment) = initial_records(tokens, &fixture);
        mutate_last_limit(tokens, &fixture, &mut point, &mut segment);
        reset_records(tokens, &fixture, &mut point, &mut segment);
        Ok(())
    });
    end(tokens);
}

fn initial_records(tokens: &mut Sink, f: &RecordFixture) -> (Point, Segment) {
    let mut point = Point::new();
    path_point(tokens, &point);
    let mut record = Segment::new();
    segment(tokens, &record);
    record.set_value(&f.arc);
    segment(tokens, &record);
    point = Point::on_vertex(f.position, f.tolerance, &f.vertex, &f.arc, f.parameter);
    path_point(tokens, &point);
    record.set_limit_point(&point, true);
    segment(tokens, &record);
    (point, record)
}

fn mutate_last_limit(
    tokens: &mut Sink,
    f: &RecordFixture,
    point: &mut Point,
    record: &mut Segment,
) {
    point.set_value_on_arc(f.position, f.tolerance, &f.arc, f.parameter + 0.5);
    path_point(tokens, point);
    segment(tokens, record);
    record.set_limit_point(point, false);
    segment(tokens, record);
}

fn reset_records(tokens: &mut Sink, f: &RecordFixture, point: &mut Point, record: &mut Segment) {
    *point = Point::on_arc(f.position, f.tolerance, &f.arc, f.parameter - 0.5);
    path_point(tokens, point);
    point.set_value_on_vertex(f.position, f.tolerance, &f.vertex, &f.arc, f.parameter);
    path_point(tokens, point);
    record.set_value(&Default::default());
    segment(tokens, record);
}
