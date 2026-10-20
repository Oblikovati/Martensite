//! Original contour arc projection, vertex forwarding and constant query reports.
use super::common::{Sink, attempt, end, flag, int, pnt2d, real, show};
use super::geometry_records::common::Reader;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::{adaptor3d::Adaptor3dHVertexBase, top_abs::Orientation};
use martensite_tkgeomalgo::int_start::TheArc;
use martensite_tkhlr::contap::ContapHContTool as Tool;
use martensite_tkmath::gp::Point2;

fn arc_constants(tokens: &mut Sink, arc: &TheArc) {
    flag(tokens, Tool::has_been_seen(arc));
    int(tokens, Tool::nb_points(arc));
    show(tokens, Tool::value(arc, 1), |_, _| {});
    flag(tokens, Tool::is_vertex(arc, 1));
    show(tokens, Tool::vertex(arc, 1), |_, _| {});
    int(tokens, Tool::nb_segments(arc));
    show(tokens, Tool::has_first_point(arc, 1), |_, _| {});
    show(tokens, Tool::has_last_point(arc, 1), |_, _| {});
    flag(tokens, Tool::is_all_solution(arc));
}

fn arc_projection(tokens: &mut Sink, arc: &TheArc, point: Point2) {
    show(tokens, Tool::nb_samples_on_arc(arc), int);
    show(tokens, Tool::bounds(arc), |tokens, (first, last)| {
        real(tokens, first);
        real(tokens, last);
    });
    show(tokens, Tool::project(arc, point), |tokens, projection| {
        flag(tokens, projection.is_some());
        if let Some((parameter, point)) = projection {
            real(tokens, parameter);
            pnt2d(tokens, point);
        }
    });
}

fn replay_arc(tokens: &mut Sink, reader: &mut Reader) -> Result<(), Failure> {
    let arc = super::hcurve::arc(reader)?;
    let point = reader.pnt2d();
    let vertex = Handle::new(Adaptor3dHVertexBase::new(
        point,
        Orientation::Forward,
        reader.real(),
    ))
    .cast();
    arc_projection(tokens, &arc, point);
    show(tokens, Tool::tolerance(&vertex, &arc), real);
    show(tokens, Tool::parameter(&vertex, &arc), real);
    arc_constants(tokens, &arc);
    Ok(())
}

/// Compare every original arc/vertex report; `arc_case(tokens, values)`.
pub fn arc_case(tokens: &mut Sink, values: &[f64]) {
    let mut reader = Reader::new(values);
    attempt(tokens, |tokens| replay_arc(tokens, &mut reader));
    end(tokens);
}
