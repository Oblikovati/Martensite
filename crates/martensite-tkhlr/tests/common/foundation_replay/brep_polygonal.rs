//! Complete deterministic polygonal HLR request over the existing owned grid/fan factory.
use super::common::{Sink, attempt, end, flag, int, real};
use super::geometry_records::common::Reader;
use super::polygon_mesh_fixture::mesh;
use super::{
    brep_curve_fixture::projector, brep_record_print::compound, brep_shape_fixture::shape,
};
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkhlr::{
    hlr_algo::{BiPointPoints, EdgeStatus, Projector},
    hlr_brep::{PolyAlgo, PolyAlgoSegment, PolyHLRToShape},
};

fn segment(
    tokens: &mut Sink,
    points: &BiPointPoints,
    source: &Shape,
    map: &IndexedMapOfShape,
    flags: [bool; 4],
) {
    for point in [points.pnt1, points.pnt2, points.pnt_p1, points.pnt_p2] {
        real(tokens, point.x());
        real(tokens, point.y());
        real(tokens, point.z());
    }
    int(tokens, map.find_index(source) as i32);
    flags.into_iter().for_each(|value| flag(tokens, value));
}
fn reported_segment(tokens: &mut Sink, item: &PolyAlgoSegment, map: &IndexedMapOfShape) {
    segment(
        tokens,
        &item.points,
        &item.shape,
        map,
        [item.reg1, item.regn, item.outl, item.intl],
    );
}
fn visible_parts(tokens: &mut Sink, status: &EdgeStatus) -> Result<(), Failure> {
    let count = status.nb_visible_part();
    int(tokens, count);
    for index in 1..=count {
        let (first, _, last, _) = status.visible_part(index)?;
        real(tokens, first);
        real(tokens, last);
    }
    Ok(())
}
fn hide_walk(
    tokens: &mut Sink,
    algo: &PolyAlgo,
    map: &IndexedMapOfShape,
    status: &mut EdgeStatus,
) -> Result<(), Failure> {
    let mut hidden = 0;
    algo.init_hide()?;
    while algo.more_hide() && hidden < 3000 {
        let item = algo.hide(status)?;
        reported_segment(tokens, &item, map);
        visible_parts(tokens, status)?;
        algo.next_hide()?;
        hidden += 1;
    }
    int(tokens, hidden);
    Ok(())
}
fn show_walk(tokens: &mut Sink, algo: &PolyAlgo, map: &IndexedMapOfShape) -> Result<(), Failure> {
    let mut shown = 0;
    algo.init_show()?;
    while algo.more_show() && shown < 3000 {
        let item = algo.show()?;
        reported_segment(tokens, &item, map);
        algo.next_show()?;
        shown += 1;
    }
    int(tokens, shown);
    Ok(())
}
fn walks(tokens: &mut Sink, algo: &PolyAlgo, map: &IndexedMapOfShape) -> Result<(), Failure> {
    let mut status = EdgeStatus::new();
    hide_walk(tokens, algo, map, &mut status)?;
    show_walk(tokens, algo, map)
}
fn compound_modes(tokens: &mut Sink, conversion: &mut PolyHLRToShape, source: &Shape) {
    for mode in 0..2 {
        if mode == 1 {
            conversion.show();
        }
        mode_results(tokens, conversion, source);
    }
}
fn mode_results(tokens: &mut Sink, conversion: &PolyHLRToShape, source: &Shape) {
    attempt(tokens, |tokens| compound(tokens, &conversion.v_compound()?));
    attempt(tokens, |tokens| compound(tokens, &conversion.h_compound()?));
    attempt(tokens, |tokens| {
        compound(tokens, &conversion.out_line_v_compound()?)
    });
    attempt(tokens, |tokens| {
        compound(tokens, &conversion.rg1_line_v_compound()?)
    });
    attempt(tokens, |tokens| {
        compound(tokens, &conversion.rgn_line_h_compound()?)
    });
    attempt(tokens, |tokens| {
        compound(tokens, &conversion.v_compound_of(source)?)
    });
}
fn poly_body(tokens: &mut Sink, reader: &mut Reader, view: &Projector) -> Result<(), Failure> {
    let source = shape(reader)?;
    let (n, m, tolerance) = (reader.integer(), reader.integer(), reader.real());
    mesh(&source, n, m)?;
    let mut map = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(&source, ShapeEnum::Edge, &mut map)?;
    topexp::map_shapes_of_type(&source, ShapeEnum::Face, &mut map)?;
    let handle = Handle::new(PolyAlgo::with_shape(&source)?);
    let algo = handle.get().ok_or_else(|| Failure::new("no algo"))?;
    algo.set_projector(view);
    algo.set_tol_coef(tolerance);
    algo.update()?;
    walks(tokens, algo, &map)?;
    conversion_results(tokens, &handle, algo, &source)
}
fn conversion_results(
    tokens: &mut Sink,
    handle: &Handle<PolyAlgo>,
    algo: &PolyAlgo,
    source: &Shape,
) -> Result<(), Failure> {
    let mut conversion = PolyHLRToShape::new();
    conversion.update(handle)?;
    compound_modes(tokens, &mut conversion, source);
    attempt(tokens, |tokens| {
        compound(tokens, &algo.out_lined_shape(source)?)
    });
    Ok(())
}
/// Replay the exact projector/shape/grid/tolerance request; `poly_case(tokens, fields)`.
pub fn poly_case(tokens: &mut Sink, fields: &[f64]) {
    let mut reader = Reader::new(fields);
    attempt(tokens, |tokens| {
        let view = projector(&mut reader)?;
        attempt(tokens, |tokens| poly_body(tokens, &mut reader, &view));
        Ok(())
    });
    end(tokens);
}
