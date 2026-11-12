//! One compound-tool cut with five translated blind cylinders.
use super::prelude::*;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPICopy;

fn bore_tools() -> Result<Shape, Box<dyn Error>> {
    let axes = Axis2::with_x_direction(
        Point3::new(20., 20., 50.),
        Direction3::new(0., 0., 1.)?,
        Direction3::new(1., 0., 0.)?,
    )?;
    let mut maker = BRepPrimAPIMakeCylinder::with_axes(axes, 10., 80.);
    let mut tools = vec![maker.shape()?.clone()];
    for index in 1..5 {
        let mut copy = BRepBuilderAPICopy::with_shape(&tools[index - 1], true, false)?;
        tools.push(crate::primitive_shapes::translated(
            copy.shape()?,
            [40., 0., 0.],
        )?);
    }
    Ok(crate::primitive_shapes::compound_of(&tools)?)
}

/// Preserve opaque/translucent pixel queries and their requested tolerance; `five_blind_bores()?`.
pub(crate) fn five_blind_bores() -> Outcome {
    bore_banner();
    let cube = crate::primitive_shapes::box_sized([200., 100., 100.])?;
    let result = execute_boolean(
        BooleanRequest {
            case: "transparent_blind_bores",
            ordinal: 1,
            operation: Operation::Cut,
            operands: ["b", "co"],
            authority: super::boolean_outcome::OperandAuthority::ProceduralSource,
        },
        cube,
        bore_tools()?,
    )?;
    display_bores(result.shape())
}

fn display_bores(result: &Shape) -> Outcome {
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    run.session()
        .display("r", result, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().check_color(270, 230, [0.78, 0.55, 0.09])?;
    run.session().set_transparency("r", 0.5)?;
    run.session().check_color(270, 230, [0.8, 0.56, 0.1])?;
    run.dump("bug6145.png")?;
    run.finish()
}

fn bore_banner() {
    banner(&[
        "============",
        "Rendering a transparent solid with five blind bores",
        "============",
        "",
    ]);
}

#[cfg(test)]
fn check_bore_body(body: &super::boolean_outcome::CheckedLocalBody) {
    let (volume, area) = super::boolean_measurements::blind_bore_expectation();
    let measured = body.measurements();
    assert!((measured.volume - volume).abs() / volume < 1e-6);
    assert!((measured.surface.properties().mass() - area).abs() / area < 1e-6);
    assert_eq!(
        measured.volume_error,
        super::boolean_measurements::AchievedVolumeError::Unknown
    );
    assert!(!body.history().relations.is_empty());
    assert!(body.checks().measured_incidences > 0);
}

#[cfg(test)]
mod controls {
    use super::*;
    #[test]
    fn actual_blind_bore_cut_keeps_ordered_checks_and_independent_properties() {
        let cube = crate::primitive_shapes::box_sized([200., 100., 100.]).unwrap();
        let request = BooleanRequest {
            case: "owned_bop4_bores",
            ordinal: 1,
            operation: Operation::Cut,
            operands: ["b", "co"],
            authority: super::super::boolean_outcome::OperandAuthority::ProceduralSource,
        };
        let body = execute_boolean(request, cube, bore_tools().unwrap())
            .expect("genuine compound-tool bore cut must pass ordered existing checks");
        check_bore_body(&body);
    }
}
