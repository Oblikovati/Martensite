//! Interior-search report preserves the historical sampling entry points.
use super::common::{Sink, attempt, end, flag, int, pnt, real, xy, xyz};
use super::geometry_records::common::Reader;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::{Adaptor3dTopolTool, h_surface_tool::Adaptor3dSurfaceHandle};
use martensite_tkhlr::contap::{ContapSurfFunction, ContapTheSearchInside as Search};

fn inside_report(tokens: &mut Sink, search: &Search) {
    flag(tokens, search.is_done());
    attempt(tokens, |tokens| {
        let count = search.nb_points()?;
        int(tokens, count);
        (1..=count).try_for_each(|index| {
            point_report(tokens, search.value(index)?);
            Ok(())
        })
    });
    attempt(tokens, |_| search.value(0).map(|_| ()));
    attempt(tokens, |_| {
        search.value(search.nb_points()? + 1).map(|_| ())
    });
}

fn point_report(tokens: &mut Sink, point: &martensite_tkgeomalgo::int_surf::IntSurfInteriorPoint) {
    pnt(tokens, point.value());
    real(tokens, point.u_parameter());
    real(tokens, point.v_parameter());
    xyz(tokens, *point.direction().xyz());
    xy(tokens, point.direction2d().xy());
}

/// Preserve every original constructor/perform/report sequence; `inside_case(tokens, numbers)`.
pub fn inside_case(tokens: &mut Sink, values: &[f64]) {
    let mut reader = Reader::new(values);
    attempt(tokens, |tokens| replay_inside(tokens, &mut reader));
    end(tokens);
}

fn replay_inside(tokens: &mut Sink, reader: &mut Reader) -> Result<(), Failure> {
    let surface = super::surface::surface_adaptor(reader)?;
    let (mode, kind) = (reader.integer(), reader.integer());
    let (u, v) = (reader.real(), reader.real());
    let mut function = ContapSurfFunction::new();
    function.set_surface(&surface)?;
    configure_function(&mut function, reader, kind);
    let mut search = initial_search(tokens, &mut function, &surface, (mode, u, v))?;
    inside_report(tokens, &search);
    if mode != 2 {
        attempt(tokens, |_| {
            search.perform_from(&mut function, &surface, u, v)
        });
        inside_report(tokens, &search);
    }
    Ok(())
}

fn configure_function(function: &mut ContapSurfFunction, reader: &mut Reader, kind: i32) {
    // Read after SetSurface: failures must not consume later direction/eye fields.
    let direction = reader.dir();
    let eye = reader.pnt();
    let angle = reader.real();
    match kind {
        0 => function.set_direction(direction),
        1 => function.set_eye(eye),
        _ => function.set_direction_angle(direction, angle),
    }
    function.set_tolerance(1e-7);
}

fn initial_search(
    tokens: &mut Sink,
    function: &mut ContapSurfFunction,
    surface: &Adaptor3dSurfaceHandle,
    (mode, u, v): (i32, f64, f64),
) -> Result<Search, Failure> {
    let domain = Adaptor3dTopolTool::with_surface(surface)?;
    let mut search = Search::new();
    if mode == 0 {
        attempt(tokens, |_| {
            search = Search::with_domain(function, surface, &domain, 1e-7)?;
            Ok(())
        });
    }
    if mode == 1 {
        attempt(tokens, |_| search.perform_from(function, surface, u, v));
    }
    if mode == 3 {
        attempt(tokens, |_| search.perform(function, surface, &domain, 1e-7));
    }
    Ok(search)
}
