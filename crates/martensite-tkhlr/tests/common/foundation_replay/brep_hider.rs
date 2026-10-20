//! One complete face-hiding operation over the original shape/projector fields.
use super::common::{Sink, attempt, end, flag, int};
use super::geometry_records::common::Reader;
use super::hlr_algo::print_parts;
use super::{
    brep_curve_fixture::projector, brep_data_bounds::shape_box, brep_shape_fixture::shape,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkhlr::{
    hlr_algo::{EdgeStatus, Projector},
    hlr_brep::{Data, Hider, ShapeToHLR},
    hlr_topo_brep::{MapOfShapeTool, OutLiner},
};

fn visible_parts(tokens: &mut Sink, status: &EdgeStatus) -> Result<(), Failure> {
    flag(tokens, status.all_hidden());
    flag(tokens, status.all_visible());
    let count = status.nb_visible_part();
    int(tokens, count);
    for part in 1..=count {
        print_parts(tokens, status.visible_part(part)?);
    }
    Ok(())
}
fn edge_status(tokens: &mut Sink, records: &Data) -> Result<(), Failure> {
    let edges = records.e_data_array();
    for edge in 1..=records.nb_edges() {
        let status = edges
            .value(edge)
            .map_err(|error| error.into_failure("probe"))?
            .status();
        visible_parts(tokens, status)?;
    }
    Ok(())
}
fn hide_faces(
    tokens: &mut Sink,
    handle: &Handle<Data>,
    tools: &mut MapOfShapeTool,
) -> Result<(), Failure> {
    let records = handle.get().ok_or_else(|| Failure::new("no data"))?;
    let (ne, nf) = (records.nb_edges(), records.nb_faces());
    if ne == 0 {
        return Ok(());
    }
    records.init_bound_sort(&shape_box(records, ne, nf)?, 1, ne)?;
    let hider = Hider::new(handle);
    for face in 1..=nf {
        if records.f_data_array()[face].hiding() {
            attempt(tokens, |_| hider.hide(face, tools));
        }
    }
    edge_status(tokens, records)
}
fn hider_body(tokens: &mut Sink, reader: &mut Reader, view: &Projector) -> Result<(), Failure> {
    let source = shape(reader)?;
    let nb_iso = reader.integer();
    let outliner = Handle::new(OutLiner::with_original(&source));
    let mut tools = MapOfShapeTool::default();
    let handle = ShapeToHLR::load(&outliner, view, &mut tools, nb_iso)?;
    let records = handle.get().ok_or_else(|| Failure::new("no data"))?;
    records.update(view)?;
    hide_faces(tokens, &handle, &mut tools)
}
/// Replay the unchanged projector/shape/isoline protocol; `hider_case(tokens, fields)`.
pub fn hider_case(tokens: &mut Sink, fields: &[f64]) {
    let mut reader = Reader::new(fields);
    attempt(tokens, |tokens| {
        let view = projector(&mut reader)?;
        attempt(tokens, |tokens| hider_body(tokens, &mut reader, &view));
        Ok(())
    });
    end(tokens);
}
