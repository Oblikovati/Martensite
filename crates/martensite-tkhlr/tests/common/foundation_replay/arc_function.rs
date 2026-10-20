//! Saved contour arc states and native callback outputs.
use super::common::{Sink, attempt, end, flag, int, pnt, real, show};
use super::geometry_records::common::Reader;
use super::surface_setup::SurfaceSetup;
use martensite_tkhlr::contap::ContapArcFunction as Function;
use martensite_tkmath::math::{Function as MathFunction, FunctionWithDerivative};
fn arc_function_state(tokens: &mut Sink, function: &mut Function) {
    pnt(tokens, function.last_computed_point());
    flag(tokens, function.surface().is_null());
    show(tokens, function.nb_samples(), int);
    let index = function.get_state_number();
    int(tokens, index);
    for saved in [0, 1, index, index + 1] {
        show(tokens, function.valpoint(saved), pnt);
    }
    flag(tokens, std::ptr::eq(function.quadric(), function.quadric()));
}
fn arc_function_evaluate(tokens: &mut Sink, function: &mut Function, u: f64) {
    let mut value = 17.;
    let mut derivative = 19.;
    let success = MathFunction::value(function, u, &mut value);
    show_captured(tokens, function, success);
    real(tokens, value);
    arc_function_state(tokens, function);
    let success = FunctionWithDerivative::derivative(function, u + 0.125, &mut derivative);
    show_captured(tokens, function, success);
    real(tokens, derivative);
    arc_function_state(tokens, function);
    let success = FunctionWithDerivative::values(function, u + 0.25, &mut value, &mut derivative);
    show_captured(tokens, function, success);
    real(tokens, value);
    real(tokens, derivative);
    arc_function_state(tokens, function);
}
fn show_captured(tokens: &mut Sink, function: &mut Function, success: bool) {
    use martensite_tkgeomalgo::int_start::TheArcFunction;
    show(tokens, function.take_failure().map(|()| success), flag);
}
fn arc_function_modes(
    tokens: &mut Sink,
    function: &mut Function,
    u: f64,
    angle: f64,
    direction: martensite_tkmath::gp::Direction3,
    eye: martensite_tkmath::gp::Point3,
) {
    function.set_direction(direction);
    arc_function_evaluate(tokens, function, u);
    function.set_eye_angle(eye, angle);
    arc_function_evaluate(tokens, function, u);
    function.set_eye(eye);
    arc_function_evaluate(tokens, function, u);
    function.set_direction_angle(direction, angle);
    arc_function_evaluate(tokens, function, u);
}

/// All modes and saved-state resets; `arc_function_case(tokens, numbers)`.
pub fn arc_function_case(tokens: &mut Sink, values: &[f64]) {
    let mut reader = Reader::new(values);
    attempt(tokens, |tokens| {
        replay_arc_function(tokens, &mut reader, &mut SurfaceSetup::Historical)
    });
    end(tokens);
}

fn initial_state(
    tokens: &mut Sink,
    function: &mut Function,
) -> Result<(), martensite_tkernel::standard::Failure> {
    flag(tokens, function.surface().is_null());
    pnt(tokens, function.last_computed_point());
    int(tokens, function.get_state_number());
    pnt(tokens, function.valpoint(1)?);
    Ok(())
}

fn replay_arc_function(
    tokens: &mut Sink,
    reader: &mut Reader,
    setup: &mut SurfaceSetup,
) -> Result<(), martensite_tkernel::standard::Failure> {
    let surface = super::surface::surface_adaptor(reader)?;
    let arc = super::hcurve::arc(reader)?;
    let (u, angle) = (reader.real(), reader.real());
    let direction = reader.dir();
    let eye = reader.pnt();
    let mut function = Function::new();
    initial_state(tokens, &mut function)?;
    show(
        tokens,
        setup.set_arc_surface(&mut function, &surface),
        |_, ()| {},
    );
    function.set_arc(&arc);
    arc_function_state(tokens, &mut function);
    arc_function_modes(tokens, &mut function, u, angle, direction, eye);
    function.set_arc(&arc);
    arc_function_state(tokens, &mut function);
    show(
        tokens,
        setup.set_arc_surface(&mut function, &surface),
        |_, ()| {},
    );
    arc_function_state(tokens, &mut function);
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
        replay_arc_function(tokens, &mut reader, &mut setup)
    });
    end(tokens);
}
