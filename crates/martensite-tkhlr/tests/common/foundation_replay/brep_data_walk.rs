//! The original hiding walk of one face, without edge-result construction.
use super::brep_data_report::interference;
use super::common::{Sink, attempt, flag, int};
use martensite_tkernel::{ncollection::List, standard::Failure};
use martensite_tkhlr::{hlr_algo::Interference, hlr_brep::Data, hlr_topo_brep::MapOfShapeTool};

fn interferences(tokens: &mut Sink, records: &Data) -> Result<List<Interference>, Failure> {
    let mut items = List::default();
    records.init_interference()?;
    while records.more_interference() {
        let rejected = records.rejected_interference()?;
        flag(tokens, rejected);
        flag(tokens, records.above_interference());
        if !rejected {
            let item = records.interference();
            interference(tokens, records, &item);
            items.append(item);
        }
        records.next_interference()?;
    }
    Ok(items)
}
fn classifications(
    tokens: &mut Sink,
    records: &Data,
    edge: i32,
    bounds: (f64, f64),
    items: &List<Interference>,
) {
    let (first, last) = bounds;
    attempt(tokens, |tokens| {
        let (state, level) = records.classify(edge, true, (first + last) / 2.)?;
        int(tokens, state as i32);
        int(tokens, level);
        Ok(())
    });
    attempt(tokens, |tokens| {
        int(tokens, records.simpl_classify(edge, 3, first, last)? as i32);
        Ok(())
    });
    attempt(tokens, |tokens| {
        int(tokens, records.hiding_start_level(edge, items)?);
        Ok(())
    });
}
fn hide_edge(tokens: &mut Sink, records: &Data, edge: i32) -> Result<(), Failure> {
    let items = interferences(tokens, records)?;
    int(tokens, records.compare(edge)? as i32);
    let (first, _, last, _) = records
        .e_data_array()
        .value(edge)
        .map_err(|error| error.into_failure("probe"))?
        .status()
        .bounds();
    classifications(tokens, records, edge, (first, last), &items);
    let edges = records.e_data_array();
    flag(
        tokens,
        records.edge_of_the_hiding_face(
            edge,
            edges
                .value(edge)
                .map_err(|error| error.into_failure("probe"))?,
        ),
    );
    Ok(())
}
pub(super) fn hide(
    tokens: &mut Sink,
    records: &Data,
    face: i32,
    tools: &mut MapOfShapeTool,
) -> Result<(), Failure> {
    records.init_edge(face, tools)?;
    flag(tokens, records.is_bad_face()?);
    flag(tokens, records.hiding_the_face());
    flag(tokens, records.simple_hiding_face());
    let mut count = 0;
    while records.more_edge()? && count < 200 {
        let edge = records.edge()?;
        int(tokens, edge);
        attempt(tokens, |tokens| hide_edge(tokens, records, edge));
        records.next_edge(true)?;
        count += 1;
    }
    flag(tokens, records.more_edge()?);
    Ok(())
}
