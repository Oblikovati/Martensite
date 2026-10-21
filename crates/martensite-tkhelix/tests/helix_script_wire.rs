//! Native wire construction for exact the case harness inputs, without the downstream sweep.
mod support;
use super::helix_script_inputs::CASES;
use martensite_tkhelix::helix_brep::HelixBRepBuilderHelix;
use martensite_tkmath::gp::{Axis3, Direction3, Point3};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use support::brep::array;

pub(super) enum HelixMode {
    CylinderHeight,
    CompositeHeight,
    SpiralHeight,
    CylinderTurns,
    CompositeTurns,
    SpiralTurns,
}

pub(super) struct HelixInput {
    pub(super) name: &'static str,
    pub(super) mode: HelixMode,
    pub(super) diameters: &'static [f64],
    pub(super) first: &'static [f64],
    pub(super) second: &'static [f64],
    pub(super) flags: &'static [bool],
    pub(super) axis_y: bool,
}

fn configure_spiral_height(builder: &mut HelixBRepBuilderHelix, input: &HelixInput, axis: &Axis3) {
    builder
        .set_parameters_spiral(
            axis,
            input.diameters[0],
            input.diameters[1],
            &array(input.first),
            &array(input.second),
            &array(input.flags),
        )
        .unwrap();
}

fn configure_height(builder: &mut HelixBRepBuilderHelix, input: &HelixInput, axis: &Axis3) {
    if matches!(input.mode, HelixMode::SpiralHeight) {
        return configure_spiral_height(builder, input, axis);
    }
    let (first, second, flags) = (array(input.first), array(input.second), array(input.flags));
    match input.mode {
        HelixMode::CylinderHeight => {
            builder.set_parameters_diam(axis, input.diameters[0], &first, &second, &flags)
        }
        HelixMode::CompositeHeight => {
            builder.set_parameters(axis, &array(input.diameters), &first, &second, &flags)
        }
        _ => panic!("{}: expected a height-mode fixture", input.name),
    }
    .unwrap();
}

fn configure_spiral_turns(builder: &mut HelixBRepBuilderHelix, input: &HelixInput, axis: &Axis3) {
    builder
        .set_parameters_spiral_turns(
            axis,
            input.diameters[0],
            input.diameters[1],
            &array(input.first),
            &array(input.second),
        )
        .unwrap();
}

fn configure_turns(builder: &mut HelixBRepBuilderHelix, input: &HelixInput, axis: &Axis3) {
    if matches!(input.mode, HelixMode::SpiralTurns) {
        return configure_spiral_turns(builder, input, axis);
    }
    let (pitches, turns) = (array(input.first), array(input.second));
    match input.mode {
        HelixMode::CylinderTurns => {
            builder.set_parameters_diam_turns(axis, input.diameters[0], &pitches, &turns)
        }
        HelixMode::CompositeTurns => {
            builder.set_parameters_turns(axis, &array(input.diameters), &pitches, &turns)
        }
        _ => panic!("{}: expected a turns-mode fixture", input.name),
    }
    .unwrap();
}

fn fixture_axis(input: &HelixInput) -> Axis3 {
    let direction = if input.axis_y {
        Direction3::Y
    } else {
        Direction3::Z
    };
    Axis3::new(Point3::default(), direction, Direction3::X).unwrap()
}

pub(super) fn build_wire(input: &HelixInput) -> HelixBRepBuilderHelix {
    let axis = fixture_axis(input);
    let mut builder = HelixBRepBuilderHelix::new();
    match input.mode {
        HelixMode::CylinderHeight | HelixMode::CompositeHeight | HelixMode::SpiralHeight => {
            configure_height(&mut builder, input, &axis)
        }
        _ => configure_turns(&mut builder, input, &axis),
    }
    builder.perform().unwrap();
    builder
}

pub(super) fn check_wire_case(name: &str) {
    let input = CASES.iter().find(|case| case.name == name).unwrap();
    let builder = build_wire(input);
    assert_eq!(builder.error_status(), 0, "{name}");
    assert!(!builder.shape().is_null(), "{name}");
    assert!(
        BRepCheckAnalyzer::new(builder.shape(), true, false, false)
            .unwrap()
            .is_valid()
            .unwrap(),
        "{name}"
    );
}
