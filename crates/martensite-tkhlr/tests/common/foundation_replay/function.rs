//! Contour function state and all evaluation paths against.
use super::common::{Sink, attempt, end, flag, int, pnt, real, show, xyz};
use super::geometry_records::common::Reader;
use super::surface_setup::SurfaceSetup;
use martensite_tkhlr::contap::ContapSurfFunction as Function;
use martensite_tkmath::{
    gp::{Direction3, Point3},
    math::{Matrix, Vector},
};

fn function_state(tokens: &mut Sink, function: &Function) {
    int(tokens, function.nb_variables());
    int(tokens, function.nb_equations());
    int(tokens, function.function_type() as i32);
    real(tokens, function.root());
    real(tokens, function.tolerance());
    real(tokens, function.angle());
    pnt(tokens, function.point());
    pnt(tokens, function.eye());
    xyz(tokens, function.direction().xyz());
    flag(tokens, function.surface() == function.p_surface());
    flag(tokens, function.surface().is_null());
}

fn function_tangent(tokens: &mut Sink, function: &mut Function) {
    show(tokens, function.is_tangent(), flag);
    show(tokens, function.direction3d(), |tokens, direction| {
        xyz(tokens, *direction.xyz())
    });
    show(tokens, function.direction2d(), |tokens, direction| {
        real(tokens, direction.x());
        real(tokens, direction.y());
    });
    show(tokens, function.is_tangent(), flag);
}

fn function_evaluate(tokens: &mut Sink, function: &mut Function, u: f64, v: f64) {
    let mut x = Vector::with_bounds(1, 2).unwrap();
    let mut value = Vector::filled(1, 1, 17.).unwrap();
    let mut grad = Matrix::with_bounds(1, 1, 1, 2).unwrap();
    x[1] = u;
    x[2] = v;
    grad[(1, 1)] = 19.;
    grad[(1, 2)] = 23.;
    show(tokens, function.value(&x, &mut value), flag);
    real(tokens, value[1]);
    function_state(tokens, function);
    function_tangent(tokens, function);
    x[1] = u + 0.125;
    derivatives_report(tokens, function, &x, &mut grad);
    x[2] = v + 0.25;
    values_report(tokens, function, &x, &mut value, &mut grad);
}

fn derivatives_report(tokens: &mut Sink, function: &mut Function, x: &Vector, grad: &mut Matrix) {
    show(tokens, function.derivatives(x, grad), flag);
    real(tokens, grad[(1, 1)]);
    real(tokens, grad[(1, 2)]);
    function_state(tokens, function);
    function_tangent(tokens, function);
}

fn values_report(
    tokens: &mut Sink,
    function: &mut Function,
    x: &Vector,
    value: &mut Vector,
    grad: &mut Matrix,
) {
    show(tokens, function.values(x, value, grad), flag);
    real(tokens, value[1]);
    real(tokens, grad[(1, 1)]);
    real(tokens, grad[(1, 2)]);
    function_state(tokens, function);
    function_tangent(tokens, function);
}

fn function_modes(
    tokens: &mut Sink,
    function: &mut Function,
    uv: (f64, f64),
    direction: Direction3,
    eye: Point3,
    angle: f64,
) {
    function.set_direction(direction);
    function_evaluate(tokens, function, uv.0, uv.1);
    function.set_eye_angle(eye, angle);
    function_tangent(tokens, function);
    function_evaluate(tokens, function, uv.0, uv.1);
    function.set_eye(eye);
    function_evaluate(tokens, function, uv.0, uv.1);
    function.set_direction_angle(direction, angle);
    function_evaluate(tokens, function, uv.0, uv.1);
    function.set_direction(direction);
    function_tangent(tokens, function);
}

/// Exercise retained outputs and cached tangents across setters; `function_case(tokens, numbers)`.
pub fn function_case(tokens: &mut Sink, values: &[f64]) {
    let mut reader = Reader::new(values);
    attempt(tokens, |tokens| {
        replay_function(tokens, &mut reader, &mut SurfaceSetup::Historical)
    });
    end(tokens);
}

fn replay_function(
    tokens: &mut Sink,
    reader: &mut Reader,
    setup: &mut SurfaceSetup,
) -> Result<(), martensite_tkernel::standard::Failure> {
    let surface = super::surface::surface_adaptor(reader)?;
    let (u, v, tolerance, angle) = (reader.real(), reader.real(), reader.real(), reader.real());
    let direction = reader.dir();
    let eye = reader.pnt();
    let mut function = Function::new();
    function_state(tokens, &function);
    show(
        tokens,
        setup.set_function_surface(&mut function, &surface),
        |_, ()| {},
    );
    function.set_tolerance(tolerance);
    function_state(tokens, &function);
    function_modes(tokens, &mut function, (u, v), direction, eye, angle);
    show(
        tokens,
        setup.set_function_surface(&mut function, &surface),
        |_, ()| {},
    );
    function_evaluate(tokens, &mut function, u, v);
    Ok(())
}

/// Replay the same requests through an explicitly owned sampling context.
/// `case_with_context(tokens, values, context)`.
pub fn case_with_context(
    tokens: &mut Sink,
    values: &[f64],
    context: &mut martensite_tkhlr::contap::ContapSamplingContext,
) {
    let mut reader = Reader::new(values);
    let mut setup = SurfaceSetup::Local(context);
    attempt(tokens, |tokens| {
        replay_function(tokens, &mut reader, &mut setup)
    });
    end(tokens);
}
