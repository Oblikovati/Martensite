//! Contap walking from boundary and interior seeds.
use super::common::{Sink, attempt, end, flag, int, pnt, real};
use super::geometry_records::common::Reader;
use super::iwline::line_report;
use martensite_tkernel::standard::Failure;
use martensite_tkgeomalgo::{
    int_surf::{IntSurfInteriorPoint, IntSurfPathPoint},
    int_walk::TheIWFunction,
};
use martensite_tkhlr::contap::{ContapSurfFunction as SurfFunction, ContapTheIWalking as Walk};
use martensite_tkmath::{
    gp::{Vector2, Vector3},
    math::{Matrix, Vector},
};
fn evaluate(f: &mut SurfFunction, u: f64, v: f64) -> Result<(), Failure> {
    let mut x = Vector::with_bounds(1, 2)?;
    let mut fv = Vector::with_bounds(1, 1)?;
    let mut d = Matrix::with_bounds(1, 1, 1, 2).expect("probe matrix");
    (x[1], x[2]) = (u, v);
    f.checked_values(&x, &mut fv, &mut d)?;
    Ok(())
}

/// `ga_iw_path_point`.
fn path_point(r: &mut Reader, f: &mut SurfFunction) -> Result<IntSurfPathPoint, Failure> {
    let (u, v) = (r.real(), r.real());
    let (pass, tangent, nmult) = (r.flag(), r.flag(), r.integer());
    evaluate(f, u, v)?;
    let mut p = IntSurfPathPoint::with_value(f.point(), u, v);
    if !tangent {
        let d3 = TheIWFunction::direction3d(f)?;
        p.set_directions(d3, TheIWFunction::direction2d(f)?);
    }
    p.set_passing(pass);
    for _ in 0..nmult {
        let a = r.real();
        p.add_uv(a, r.real())?;
    }
    Ok(p)
}

/// `ga_iw_interior_point`.
fn interior_point(r: &mut Reader, f: &mut SurfFunction) -> Result<IntSurfInteriorPoint, Failure> {
    let (u, v) = (r.real(), r.real());
    evaluate(f, u, v)?;
    if TheIWFunction::is_tangent(f)? {
        return Ok(IntSurfInteriorPoint::with_value(
            f.point(),
            u,
            v,
            Vector3::new(0.0, 0.0, 0.0),
            Vector2::new(1.0, 0.0),
        ));
    }
    let d2 = TheIWFunction::direction2d(f)?;
    let d3 = TheIWFunction::direction3d(f)?;
    Ok(IntSurfInteriorPoint::with_value(
        f.point(),
        u,
        v,
        d3,
        Vector2::new(d2.x(), d2.y()),
    ))
}

fn walk_report(tokens: &mut Sink, walking: &Walk) {
    flag(tokens, walking.is_done());
    attempt(tokens, |tokens| line_results(tokens, walking));
    attempt(tokens, |tokens| single_results(tokens, walking));
    attempt(tokens, |_| walking.value(0).map(|_| ()));
    attempt(tokens, |_| {
        walking.value(walking.nb_lines()? + 1).map(|_| ())
    });
    attempt(tokens, |_| walking.single_pnt(0).map(|_| ()));
}

fn line_results(tokens: &mut Sink, walking: &Walk) -> Result<(), Failure> {
    let count = walking.nb_lines()?;
    int(tokens, count);
    for index in 1..=count {
        line_report(tokens, walking.value(index)?);
    }
    Ok(())
}

fn single_results(tokens: &mut Sink, walking: &Walk) -> Result<(), Failure> {
    let count = walking.nb_single_pnts()?;
    int(tokens, count);
    for index in 1..=count {
        let point = walking.single_pnt(index)?;
        pnt(tokens, point.value());
        let (u, v) = point.value2d()?;
        real(tokens, u);
        real(tokens, v);
    }
    Ok(())
}

/// Trace original contour walks with both Perform overloads; `walking_case(tokens, numbers)`.
pub fn walking_case(tokens: &mut Sink, values: &[f64]) {
    let mut reader = Reader::new(values);
    attempt(tokens, |tokens| replay_walking(tokens, &mut reader));
    end(tokens);
}

fn replay_walking(tokens: &mut Sink, reader: &mut Reader) -> Result<(), Failure> {
    let surface = super::surface::surface_adaptor(reader)?;
    let mut function = SurfFunction::new();
    function.set_surface(&surface)?;
    function.set_direction(reader.dir());
    function.set_tolerance(1e-7);
    let settings = WalkingSettings::read(reader);
    let mut paths = Vec::new();
    for _ in 0..reader.integer() {
        paths.push(path_point(reader, &mut function)?);
    }
    let mut loops = Vec::new();
    for _ in 0..reader.integer() {
        loops.push(interior_point(reader, &mut function)?);
    }
    settings.perform(tokens, &paths, &loops, &mut function, &surface)
}

struct WalkingSettings {
    epsilon: f64,
    deflection: f64,
    step: f64,
    fill: bool,
    reversed: bool,
    mode: i32,
}

impl WalkingSettings {
    fn read(reader: &mut Reader) -> Self {
        Self {
            epsilon: reader.real(),
            deflection: reader.real(),
            step: reader.real(),
            fill: reader.flag(),
            reversed: reader.flag(),
            mode: reader.integer(),
        }
    }

    fn perform(
        &self,
        tokens: &mut Sink,
        paths: &[IntSurfPathPoint],
        loops: &[IntSurfInteriorPoint],
        function: &mut SurfFunction,
        surface: &martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle,
    ) -> Result<(), Failure> {
        let mut walking = Walk::new(self.epsilon, self.deflection, self.step, self.fill)?;
        walking.set_tolerance(self.epsilon, self.deflection, self.step);
        if self.mode != 2 {
            attempt(tokens, |_| {
                walking.perform(paths, loops, function, surface, self.reversed)
            });
        }
        if self.mode == 1 {
            attempt(tokens, |_| {
                walking.perform_open(paths, function, surface, self.reversed)
            });
        }
        walk_report(tokens, &walking);
        Ok(())
    }
}
