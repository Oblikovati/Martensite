//! Real Contap boundary roots and segments, including all result getters.
use super::boundary_arc_rank::arc_rank;
use super::common::{Sink, attempt, end, flag, int, pnt, pnt2d, real};
use super::geometry_records::common::Reader;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dTopolTool;
use martensite_tkhlr::contap::{
    ContapArcFunction, ContapThePathPointOfTheSearch as Point, ContapTheSearch as Search,
    ContapTheSegmentOfTheSearch as Segment,
};

fn path_point(tokens: &mut Sink, d: &Adaptor3dTopolTool, p: &Point) {
    pnt(tokens, p.value());
    real(tokens, p.tolerance());
    flag(tokens, p.is_new());
    real(tokens, p.parameter());
    int(tokens, arc_rank(d, &p.arc()));
    attempt(tokens, |tokens| {
        let v = p.vertex()?;
        pnt2d(tokens, v.value());
        real(tokens, v.resolution(&p.arc())?);
        Ok(())
    });
}

fn segment(tokens: &mut Sink, d: &Adaptor3dTopolTool, g: &Segment) {
    int(tokens, arc_rank(d, &g.curve()));
    flag(tokens, g.has_first_point());
    attempt(tokens, |tokens| {
        g.first_point().map(|p| path_point(tokens, d, p))
    });
    flag(tokens, g.has_last_point());
    attempt(tokens, |tokens| {
        g.last_point().map(|p| path_point(tokens, d, p))
    });
}

fn sob_report(tokens: &mut Sink, s: &Search, d: &Adaptor3dTopolTool) {
    flag(tokens, s.is_done());
    attempt(tokens, |tokens| {
        flag(tokens, s.all_arc_solution()?);
        Ok(())
    });
    attempt(tokens, |tokens| point_results(tokens, s, d));
    attempt(tokens, |tokens| segment_results(tokens, s, d));
    attempt(tokens, |_| s.point(0).map(|_| ()));
    attempt(tokens, |_| s.point(s.nb_points()? + 1).map(|_| ()));
    attempt(tokens, |_| s.segment(0).map(|_| ()));
    attempt(tokens, |_| s.segment(s.nb_segments()? + 1).map(|_| ()));
}
fn point_results(
    tokens: &mut Sink,
    search: &Search,
    domain: &Adaptor3dTopolTool,
) -> Result<(), Failure> {
    let count = search.nb_points()?;
    int(tokens, count);
    (1..=count).try_for_each(|index| {
        path_point(tokens, domain, search.point(index)?);
        Ok(())
    })
}

fn segment_results(
    tokens: &mut Sink,
    search: &Search,
    domain: &Adaptor3dTopolTool,
) -> Result<(), Failure> {
    let count = search.nb_segments()?;
    int(tokens, count);
    (1..=count).try_for_each(|index| {
        segment(tokens, domain, search.segment(index)?);
        Ok(())
    })
}

/// Compare unset and repeated boundary searches; `boundary_case(tokens, numbers)`.
pub fn boundary_case(tokens: &mut Sink, values: &[f64]) {
    let mut reader = Reader::new(values);
    attempt(tokens, |tokens| replay_boundary(tokens, &mut reader));
    end(tokens);
}

fn replay_boundary(tokens: &mut Sink, reader: &mut Reader) -> Result<(), Failure> {
    let surface = super::surface::surface_adaptor(reader)?;
    let (mode, kind, recheck) = (reader.integer(), reader.integer(), reader.flag());
    let mut function = configured_function(reader, &surface, kind)?;
    let domain = Adaptor3dTopolTool::with_surface(&surface)?;
    let mut search = Search::new();
    sob_report(tokens, &search, &domain);
    for _ in 0..mode {
        attempt(tokens, |_| {
            search.perform(&mut function, &domain, 1e-7, 1e-7, recheck)
        });
        sob_report(tokens, &search, &domain);
    }
    Ok(())
}

fn configured_function(
    reader: &mut Reader,
    surface: &martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle,
    kind: i32,
) -> Result<ContapArcFunction, Failure> {
    // Boundary input reads precede SetSurface, unlike the interior-search driver.
    let direction = reader.dir();
    let eye = reader.pnt();
    let angle = reader.real();
    let mut function = ContapArcFunction::new();
    function.set_surface(surface)?;
    match kind {
        0 => function.set_direction(direction),
        1 => function.set_eye(eye),
        _ => function.set_direction_angle(direction, angle),
    }
    Ok(function)
}
