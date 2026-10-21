//! Complete native data family over the real load/update/sort/hiding pipeline.
use super::common::{Sink, attempt, end, int};
use super::geometry_records::common::Reader;
use super::{
    brep_curve_fixture::projector,
    brep_data_bounds::shape_box,
    brep_data_report::{data_edge, data_face},
    brep_data_walk::hide,
    brep_shape_fixture::shape,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkhlr::{
    hlr_algo::Projector,
    hlr_brep::{Data, ShapeToHLR},
    hlr_topo_brep::{MapOfShapeTool, OutLiner},
};

fn edges(tokens: &mut Sink, records: &Data, count: i32, boxes: bool) {
    for edge in 1..=count {
        data_edge(tokens, &records.e_data_array()[edge], boxes);
    }
}
fn report_records(
    tokens: &mut Sink,
    records: &Data,
    view: &Projector,
    ne: i32,
    nf: i32,
) -> Result<(), Failure> {
    int(tokens, records.nb_vertices());
    int(tokens, ne);
    int(tokens, nf);
    edges(tokens, records, ne, false);
    records.update(view)?;
    edges(tokens, records, ne, true);
    for face in 1..=nf {
        data_face(tokens, &records.f_data_array()[face])?;
    }
    Ok(())
}
fn hiding_faces(
    tokens: &mut Sink,
    records: &Data,
    ne: i32,
    nf: i32,
    tools: &mut MapOfShapeTool,
) -> Result<(), Failure> {
    if ne == 0 {
        return Ok(());
    }
    records.init_bound_sort(&shape_box(records, ne, nf)?, 1, ne)?;
    for face in 1..=nf {
        if records.f_data_array()[face].hiding() {
            attempt(tokens, |tokens| hide(tokens, records, face, tools));
        }
    }
    Ok(())
}
fn data_body(tokens: &mut Sink, reader: &mut Reader, view: &Projector) -> Result<(), Failure> {
    let source = shape(reader)?;
    let nb_iso = reader.integer();
    let outliner = Handle::new(OutLiner::with_original(&source));
    let mut tools = MapOfShapeTool::default();
    let handle = ShapeToHLR::load(&outliner, view, &mut tools, nb_iso)?;
    let records = handle.get().ok_or_else(|| Failure::new("no data"))?;
    let (ne, nf) = (records.nb_edges(), records.nb_faces());
    report_records(tokens, records, view, ne, nf)?;
    hiding_faces(tokens, records, ne, nf, &mut tools)
}
/// Replay exact projector/shape/isoline fields; `data_case(tokens, fields)`.
pub fn data_case(tokens: &mut Sink, fields: &[f64]) {
    let mut reader = Reader::new(fields);
    attempt(tokens, |tokens| {
        let view = projector(&mut reader)?;
        attempt(tokens, |tokens| data_body(tokens, &mut reader, &view));
        Ok(())
    });
    end(tokens);
}
